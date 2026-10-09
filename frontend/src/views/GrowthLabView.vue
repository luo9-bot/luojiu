<template>
  <div class="growth-page">
    <div class="page-head">
      <div>
        <div class="eyebrow">GROWTH LAB / PHASE A</div>
        <h2>成长观测台</h2>
        <p class="muted">只观测现有行为，不自动修改人格、提示词、记忆或决策阈值。</p>
      </div>
      <button class="btn btn-ghost" :disabled="loading" @click="load">{{ loading ? '刷新中…' : '刷新数据' }}</button>
    </div>

    <div v-if="error" class="notice error">{{ error }}</div>
    <div v-else-if="!report" class="empty">正在读取观测数据…</div>
    <template v-else>
      <div class="status-line">
        <span class="status-dot" :class="{ live: report.available }"></span>
        <strong>{{ report.available ? '观测器数据可用' : '等待第一条观测' }}</strong>
        <span class="muted">每 15 秒自动刷新</span>
        <span class="muted stamp" v-if="lastUpdated">最近刷新 {{ lastUpdated }}</span>
      </div>

      <div class="metric-grid">
        <article class="metric card">
          <span class="metric-label">累计观测</span>
          <strong>{{ number(report.event_count) }}</strong>
          <small>SpeakScore 门控事件</small>
        </article>
        <article class="metric card">
          <span class="metric-label">允许发言</span>
          <strong>{{ number(report.pass_count) }}</strong>
          <small>score ≥ threshold</small>
        </article>
        <article class="metric card">
          <span class="metric-label">选择沉默</span>
          <strong>{{ number(report.silent_count) }}</strong>
          <small>score &lt; threshold</small>
        </article>
        <article class="metric card">
          <span class="metric-label">发言通过率</span>
          <strong>{{ percent(report.pass_rate) }}</strong>
          <small>不是回复质量指标</small>
        </article>
      </div>

      <div class="detail-grid">
        <section class="card panel">
          <div class="panel-head">
            <div><h3>门控评分分布摘要</h3><p class="muted">描述已有行为，不代表成长已经发生</p></div>
          </div>
          <div class="summary-row">
            <span>平均评分</span><strong>{{ fixed(report.average_score) }}</strong>
          </div>
          <div class="bar-track"><span :style="{ width: bar(report.average_score) }"></span></div>
          <div class="summary-row">
            <span>平均门限</span><strong>{{ fixed(report.average_threshold) }}</strong>
          </div>
          <div class="bar-track threshold"><span :style="{ width: bar(report.average_threshold) }"></span></div>
          <div class="summary-row">
            <span>最近事件</span><strong>{{ report.recent?.length || 0 }}</strong>
          </div>
          <p class="muted footnote">比较不同时间段前，必须控制群聊流量、批次大小及场景构成；单看通过率不能证明行为变好。</p>
        </section>

        <section class="card panel">
          <div class="panel-head"><div><h3>采集范围与限制</h3><p class="muted">当前阶段的数据解释边界</p></div></div>
          <div class="scope-item"><span class="scope-mark"></span><div><strong>已采集</strong><p>进入普通群聊 SpeakScore 门控的批次、分项评分、阈值与门控结果。</p></div></div>
          <div class="scope-item"><span class="scope-mark muted-mark"></span><div><strong>尚未覆盖</strong><p>{{ report.coverage || '其他提前返回路径、私聊与特殊强制回复路径尚未完整采集。' }}</p></div></div>
          <div class="scope-item"><span class="scope-mark"></span><div><strong>隐私边界</strong><p>接口只返回白名单字段；观测数据不应包含原始消息、用户 ID 或群号。</p></div></div>
          <div class="scope-item"><span class="scope-mark muted-mark"></span><div><strong>下一阶段</strong><p>建立固定回放集、盲评与留出样本；观测数据本身不是成长证明。</p></div></div>
        </section>
      </div>

      <section class="card events-panel">
        <div class="panel-head">
          <div><h3>最近门控事件</h3><p class="muted">最多展示最近 40 条，不展示原始聊天内容或身份标识</p></div>
          <span class="count-chip">{{ report.recent?.length || 0 }} 条</span>
        </div>
        <div v-if="!report.recent?.length" class="empty inner-empty">暂无事件。洛玖处理新的普通群聊批次后，这里会出现观测记录。</div>
        <div v-else class="table-wrap">
          <table>
            <thead><tr><th>时间</th><th>结果</th><th>评分 / 门限</th><th>批次</th><th>话题相关</th><th>注意力</th><th>社交风险</th><th>距上次发言</th></tr></thead>
            <tbody>
              <tr v-for="(e, i) in report.recent.slice(0, 40)" :key="String(e.timestamp_unix) + '-' + i">
                <td class="time-cell">{{ time(e.timestamp_unix) }}</td>
                <td><span class="decision" :class="e.decision === 'pass' ? 'pass' : 'silent'">{{ e.decision === 'pass' ? '发言' : e.decision === 'silent' ? '沉默' : e.decision }}</span></td>
                <td class="num-cell">{{ fixed(e.score) }} / {{ fixed(e.threshold) }}</td>
                <td>{{ e.batch_size }}</td>
                <td>{{ fixed(e.topic_relevance) }}</td>
                <td>{{ fixed(e.attention_max) }}</td>
                <td>{{ fixed(e.social_risk) }}</td>
                <td>{{ e.secs_since_spoke == null ? '—' : e.secs_since_spoke + 's' }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>
    </template>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { api } from '../api.js'

const report = ref(null)
const error = ref('')
const loading = ref(false)
const lastUpdated = ref('')
let timer = null

function number(v) { return Number(v || 0).toLocaleString() }
function fixed(v) { return v == null || !Number.isFinite(Number(v)) ? '—' : Number(v).toFixed(3) }
function percent(v) { return v == null ? '—' : (Number(v) * 100).toFixed(1) + '%' }
function bar(v) { return Math.max(0, Math.min(100, Number(v || 0) * 100)) + '%' }
function time(v) {
  if (v == null) return '—'
  const d = new Date(Number(v) * 1000)
  return d.toLocaleString()
}

async function load() {
  loading.value = true
  try {
    report.value = await api('/api/growth-lab')
    error.value = ''
    lastUpdated.value = new Date().toLocaleTimeString()
  } catch (e) {
    error.value = '无法读取 Growth Lab 数据：' + e.message
  } finally {
    loading.value = false
  }
}
function refresh() { load() }

onMounted(() => {
  load()
  timer = setInterval(load, 15000)
  window.addEventListener('refresh-all', refresh)
})
onUnmounted(() => {
  if (timer) clearInterval(timer)
  window.removeEventListener('refresh-all', refresh)
})
</script>

<style scoped>
.growth-page { display: flex; flex-direction: column; gap: 16px; }
.page-head, .status-line, .panel-head, .summary-row { display:flex; align-items:center; justify-content:space-between; gap:12px; }
.page-head h2 { font-size:23px; letter-spacing:-.4px; margin:4px 0 6px; }
.eyebrow { color:var(--gold); font-size:10px; font-weight:700; letter-spacing:1.8px; }
.muted { color:var(--text-2); font-size:12px; line-height:1.65; }
.status-line { justify-content:flex-start; flex-wrap:wrap; padding:10px 12px; border:1px solid var(--border); border-radius:var(--radius-sm); background:var(--surface); font-size:12px; }
.status-dot,.scope-mark { width:7px; height:7px; border-radius:50%; background:var(--text-3); flex-shrink:0; }
.status-dot.live,.scope-mark { background:var(--success); }
.stamp { margin-left:auto; }
.metric-grid { display:grid; grid-template-columns:repeat(4,minmax(0,1fr)); gap:12px; }
.metric { display:flex; flex-direction:column; gap:8px; padding:18px; }
.metric-label { color:var(--text-2); font-size:12px; }
.metric strong { font-size:27px; line-height:1.1; font-weight:650; letter-spacing:-.7px; font-variant-numeric:tabular-nums; }
.metric small { color:var(--text-3); font-size:11px; }
.detail-grid { display:grid; grid-template-columns:1fr 1fr; gap:12px; }
.panel,.events-panel { padding:18px; min-width:0; }
.panel-head { margin-bottom:18px; }
.panel-head h3 { font-size:14px; font-weight:650; margin-bottom:4px; }
.summary-row { font-size:12px; margin:13px 0 8px; color:var(--text-2); }
.summary-row strong { color:var(--text); font-variant-numeric:tabular-nums; }
.bar-track { height:5px; border-radius:99px; background:var(--surface-hover); overflow:hidden; }
.bar-track span { display:block; height:100%; background:var(--success); border-radius:99px; transition:width .25s ease; }
.bar-track.threshold span { background:var(--gold); }
.footnote { margin-top:18px; border-top:1px solid var(--border); padding-top:12px; }
.scope-item { display:flex; align-items:flex-start; gap:10px; margin:13px 0; }
.scope-mark { margin-top:6px; }
.scope-mark.muted-mark { background:var(--text-3); }
.scope-item strong { font-size:12px; }
.scope-item p { color:var(--text-2); font-size:12px; line-height:1.65; margin-top:3px; }
.count-chip { border:1px solid var(--border); border-radius:99px; padding:4px 9px; color:var(--text-2); font-size:11px; white-space:nowrap; }
.table-wrap { overflow:auto; }
table { width:100%; border-collapse:collapse; font-size:12px; white-space:nowrap; }
th { text-align:left; color:var(--text-3); font-size:10px; font-weight:600; padding:10px 9px; border-bottom:1px solid var(--border); }
td { padding:10px 9px; border-bottom:1px solid var(--border); color:var(--text-2); }
tbody tr:last-child td { border-bottom:none; }
tbody tr:hover { background:var(--surface-hover); }
.time-cell { color:var(--text-3); font-variant-numeric:tabular-nums; }
.num-cell { color:var(--text); font-variant-numeric:tabular-nums; }
.decision { display:inline-block; border-radius:5px; padding:3px 7px; font-size:10px; }
.decision.pass { color:var(--success); background:color-mix(in srgb,var(--success) 12%,transparent); }
.decision.silent { color:var(--text-2); background:var(--surface-hover); }
.empty { padding:30px; text-align:center; color:var(--text-2); font-size:13px; }
.inner-empty { padding:22px 8px; }
.notice { padding:12px 14px; border-radius:var(--radius-sm); font-size:12px; }
.notice.error { color:var(--danger); border:1px solid color-mix(in srgb,var(--danger) 30%,transparent); background:var(--surface); }
.btn { border:1px solid var(--border); border-radius:var(--radius-xs); padding:8px 12px; background:var(--surface); color:var(--text); cursor:pointer; font-size:12px; }
.btn:disabled { opacity:.5; cursor:wait; }
@media(max-width:900px) { .metric-grid { grid-template-columns:repeat(2,minmax(0,1fr)); } .detail-grid { grid-template-columns:1fr; } }
@media(max-width:560px) { .page-head { align-items:flex-start; } .metric { padding:13px; } .metric strong { font-size:22px; } .panel,.events-panel { padding:13px; } }
</style>
