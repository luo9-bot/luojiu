//! 批次处理：把一段时间的话凑成一批，一次交给她看
//!
//! 批次就绪后按群/私聊分组：私聊独立线程直接处理，
//! 群聊进入消息队列串行处理（真正的决策在 `handler::process_group_batch`）。
//!
//! 聚合的粒度是**对话流**而不是 (群, 用户)：同一个人隔几秒说的几句话、
//! 或者几个人你一言我一语的一段聊，都属于同一段话，要在一次调用里
//! 一起递给她，由她自己决定接哪几条——而不是每条消息各回一句。
//!
//! 本函数运行在 **1ms tick 的主事件循环**上，因此这里绝不能阻塞：
//! 早先版本为了"等尾部消息一起合并"在这里 `sleep(500ms)`，等于每处理
//! 一批就把整个插件的事件循环停半秒——用户消息被推迟看见、新消息在
//! 队列里堆积、回神与定时任务一起被拖慢。合并现在由 `CoalescePolicy`
//! 决定：说完了才取，取出来就立刻送走。

use std::collections::HashMap;
use std::thread;

use tracing::{info, warn};

use super::handler::{GroupBatch, process_message};
use crate::state::CoalescePolicy;
use crate::util::MutexExt;
use crate::{MESSAGE_QUEUE, ProcessingTask, batches, config, processing_users};

/// 批次按真实到达时刻排序：聚合后的这批话要按人说话的顺序递给她
fn chronological(list: &mut [GroupBatch]) {
    list.sort_by_key(GroupBatch::sort_key_ms);
}

/// 群聊批次按群号归堆（一次决策看完整个群的这段话）
type GroupedBatches = HashMap<u64, Vec<GroupBatch>>;

/// 按对话流分开：群聊归堆到群号，私聊散给个人
fn split_by_stream(ready: Vec<GroupBatch>) -> (GroupedBatches, Vec<(u64, String)>) {
    ready.into_iter().fold(
        (HashMap::new(), Vec::new()),
        |(mut groups, mut privates), batch| {
            if batch.group_id > 0 {
                groups.entry(batch.group_id).or_default().push(batch);
            } else {
                privates.push((batch.user_id, batch.taken.messages));
            }
            (groups, privates)
        },
    )
}

pub(crate) fn process_expired_batches() {
    let cfg = config::get();
    let policy = CoalescePolicy {
        quiet_ms: cfg.conversation.batch_timeout_ms,
        max_wait_ms: cfg.conversation.batch_max_wait_ms,
    };

    // 收集所有"这段话说完了"的批次，跳过正在处理中的用户。
    // `take_ready` 一次性返回结果，因此这里不把缓冲借用带出闭包。
    let ready: Vec<GroupBatch> = {
        let processing = processing_users().lock_recover();
        batches(|b| b.take_ready(policy, |key| processing.contains(&key)))
            .into_iter()
            .map(|((group_id, user_id), taken)| GroupBatch {
                group_id,
                user_id,
                taken,
            })
            .collect()
    };

    if ready.is_empty() {
        return;
    }

    info!(count = ready.len(), "batch: processing coalesced batches");

    // 按对话流聚合：同一个群的所有消息一起进入表达决策
    let (group_msgs, private_batches) = split_by_stream(ready);

    // 处理私聊批次 (独立线程，不阻塞主循环)
    private_batches.into_iter().for_each(|(user_id, messages)| {
        thread::spawn(move || {
            process_message(user_id, &messages);
        });
    });

    // 处理群聊批次: 通过消息队列串行化处理，避免并发混乱
    group_msgs.into_iter().for_each(|(group_id, mut user_msgs)| {
        chronological(&mut user_msgs);
        if let Some(queue) = MESSAGE_QUEUE.get()
            && queue
                .tx
                .send(ProcessingTask { group_id, user_msgs })
                .is_err()
        {
            warn!(group_id, "queue: 发送失败");
        }
    });
}
