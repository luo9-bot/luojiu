//! 配额状态：段计数与段日志
//!
//! 两者都住在状态库里。原先它们在一个进程内 `Mutex<Option<QuotaStore>>` 里
//! 缓存，并在**每次改动后重写整份 `quota.json`**——而那个文件包含
//! 48 小时的消息文本，所以每来一条消息就要重写一份越来越大的文件。
//!
//! 现在：计数一行 UPSERT、日志一行 INSERT，跨天重置与 48 小时裁剪各是一条
//! 范围删除。

use tracing::debug;

use crate::config;
use crate::util::{hour_cst_at, now_secs, segment_start_cst};

// ── 初始化 ──────────────────────────────────────────────────

/// 初始化：跨天则重置计数
pub(crate) fn init() {
    let db = crate::db::db();
    match db.quota_roll_day(&crate::util::today_str()) {
        Ok(true) => debug!("quota: 跨天，计数已重置"),
        Ok(false) => {}
        Err(error) => tracing::warn!(%error, "quota: 跨天检查失败"),
    }
}

// ── 配额段计算 ──────────────────────────────────────────────

/// 获取当前配额段的起始时间 (unix seconds)
///
/// 段边界对齐到**东八区当天零点**（见 `util::segment_start_cst`）。
/// 曾经按绝对 epoch 对齐，于是 5 分钟段会跨过整点（13:58–14:02）：
/// 段内的小时上限中途变化，而该段的计数不重置。
pub(super) fn current_segment_start() -> u64 {
    let segment_secs = config::get().quota.segment_minutes as u64 * 60;
    segment_start_cst(now_secs(), segment_secs)
}

/// 当前小时对应的配额上限
///
/// `None` 表示**没有任何时段覆盖当前小时**（配置有空档）。这与
/// `Some(0)`（明确配成 0 次）是两件事：前者通常是时段表写漏了，
/// 后者是刻意的。旧签名把两者都返回 `0`，于是整小时的静默不回复
/// 无法与"配额用尽"区分。
pub(super) fn current_max_replies() -> Option<u32> {
    let cfg = config::get().quota;
    let hour = hour_cst_at(now_secs());
    cfg.segments
        .iter()
        .find(|seg| hour >= seg.start_hour && hour < seg.end_hour)
        .map(|seg| seg.max_replies)
}

#[cfg(test)]
mod tests {
    /// 段计数是原子的：并发扣减不能超发
    #[test]
    fn consuming_never_exceeds_the_limit() {
        let db = crate::db::db();
        let group = 993_001;
        let segment = 1_000_000u64;
        let max = 3u32;

        db.quota_set_segment_count(group, segment, 0).expect("清零");

        let mut granted = 0;
        for _ in 0..10 {
            if db.quota_consume(group, segment, max).expect("扣减") {
                granted += 1;
            }
        }
        assert_eq!(granted, max, "最多只能放行 {max} 次");
        assert_eq!(db.quota_segment_count(group, segment).expect("计数"), max);
    }
}
