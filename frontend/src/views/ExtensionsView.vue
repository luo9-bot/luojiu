<template>
  <div class="extensions-page">
    <div class="page-heading">
      <div>
        <div class="eyebrow">EXTENSION CENTER</div>
        <h1>扩展中心</h1>
        <p>统一管理内置能力、外部扩展和可接入的市场目录。</p>
      </div>
      <button class="btn btn-secondary" @click="load" :disabled="loading">刷新目录</button>
    </div>

    <div v-if="error" class="card error-card">{{ error }}</div>
    <div v-if="loading" class="card empty">正在读取扩展注册表…</div>

    <template v-else>
      <div class="stats-row">
        <div class="card stat-card"><span>已注册</span><strong>{{ extensions.length }}</strong></div>
        <div class="card stat-card"><span>运行中</span><strong>{{ activeCount }}</strong></div>
        <div class="card stat-card"><span>外部扩展</span><strong>{{ externalCount }}</strong></div>
        <div class="card stat-card"><span>市场状态</span><strong>{{ market?.configured ? '已连接' : '未配置' }}</strong></div>
      </div>

      <div class="card">
        <div class="card-header">
          <h3>已安装扩展</h3>
          <input v-model="query" class="input input-sm" placeholder="搜索扩展或能力" />
        </div>
        <div class="extension-grid">
          <article v-for="item in filteredExtensions" :key="item.id" class="extension-card">
            <div class="extension-topline">
              <span class="extension-kind" :class="item.kind">{{ item.kind === 'builtin' ? '内置' : '外部' }}</span>
              <span class="extension-status" :class="item.status">{{ statusLabel(item) }}</span>
            </div>
            <h4>{{ item.name }}</h4>
            <p>{{ item.description || '暂无描述' }}</p>
            <div class="tag-row"><span v-for="tag in item.tags" :key="tag" class="tag tag-info">{{ tag }}</span></div>
            <div v-if="item.permissions?.length" class="permission-line">权限：{{ item.permissions.join('、') }}</div>
            <div v-if="item.requires?.length" class="permission-line">依赖：{{ item.requires.join('、') }}</div>
            <div v-if="item.config_schema?.length" class="permission-line">可配置项：{{ item.config_schema.length }}</div>
            <div class="extension-meta">v{{ item.version }} · {{ item.author || 'Luo9' }}</div>
            <div class="extension-actions">
              <button class="btn btn-sm" :class="item.enabled ? 'btn-secondary' : 'btn-primary'" @click="toggle(item)">
                {{ item.enabled ? '停用' : '启用' }}
              </button>
              <span v-if="item.status === 'manifest_only'" class="muted">等待运行时适配</span>
            </div>
          </article>
        </div>
        <div v-if="!filteredExtensions.length" class="empty">没有匹配的扩展。</div>
      </div>

      <div class="card">
        <div class="card-header"><h3>市场目录</h3><span class="tag" :class="market?.configured ? 'tag-success' : 'tag-warning'">{{ market?.configured ? '已配置' : '待配置' }}</span></div>
        <div v-if="market?.error" class="error-card">{{ market.error }}</div>
        <div v-else-if="!market?.entries?.length" class="empty">市场目录尚未配置。设置 LUO9_EXTENSION_MARKET_URL 后，这里会显示你自己的扩展源。</div>
        <div v-else class="market-list">
          <div v-for="entry in market.entries" :key="entry.id" class="market-item">
            <div><strong>{{ entry.name }}</strong><p>{{ entry.description }}</p></div>
            <span class="extension-meta">v{{ entry.version }} · {{ entry.author }}</span>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup>
import { computed, onMounted, ref } from 'vue'
import { api } from '../api.js'

const extensions = ref([])
const market = ref(null)
const query = ref('')
const loading = ref(false)
const error = ref('')

const activeCount = computed(() => extensions.value.filter(item => item.enabled).length)
const externalCount = computed(() => extensions.value.filter(item => item.kind === 'external').length)
const filteredExtensions = computed(() => {
  const needle = query.value.trim().toLowerCase()
  if (!needle) return extensions.value
  return extensions.value.filter(item => [item.id, item.name, item.description, ...(item.tags || []), ...(item.capabilities || [])].join(' ').toLowerCase().includes(needle))
})

function statusLabel(item) {
  if (item.status === 'active') return '运行中'
  if (item.status === 'disabled') return '已停用'
  if (item.status === 'manifest_only') return '仅清单'
  return item.status || '未知'
}

async function load() {
  loading.value = true
  error.value = ''
  try {
    const data = await api('/api/extensions')
    extensions.value = data.extensions || []
    market.value = data.market || null
  } catch (cause) { error.value = cause.message || '读取扩展目录失败' }
  finally { loading.value = false }
}

async function toggle(item) {
  try {
    const data = await api('/api/extensions/' + encodeURIComponent(item.id) + '/toggle', { method: 'POST', body: JSON.stringify({ enabled: !item.enabled }) })
    extensions.value = data.extensions || []
    market.value = data.market || market.value
  } catch (cause) { error.value = cause.message || '更新扩展状态失败' }
}

onMounted(load)
</script>

<style scoped>
.extensions-page { max-width: 1280px; margin: 0 auto; }
.page-heading { display:flex; justify-content:space-between; align-items:flex-start; gap:20px; margin-bottom:22px; }
.eyebrow { color:var(--primary); font:11px var(--font-mono); letter-spacing:.16em; }
h1 { margin:5px 0; font-size:26px; font-weight:500; }
.page-heading p { color:var(--text-2); margin:0; }
.stats-row { display:grid; grid-template-columns:repeat(4,minmax(0,1fr)); gap:14px; margin-bottom:16px; }
.stat-card { display:flex; flex-direction:column; gap:8px; margin-bottom:0; }
.stat-card span { color:var(--text-2); font-size:12px; }
.stat-card strong { font:24px var(--font-mono); color:var(--primary); }
.error-card { color:var(--danger); background:var(--danger-subtle); padding:14px; border-radius:var(--radius-sm); }
.extension-grid { display:grid; grid-template-columns:repeat(auto-fill,minmax(260px,1fr)); gap:14px; }
.extension-card { border:1px solid var(--border-light); border-radius:var(--radius-sm); padding:16px; background:var(--chip-solid); }
.extension-card h4 { margin:12px 0 5px; font-size:16px; }
.extension-card p { min-height:42px; color:var(--text-2); font-size:13px; margin:0 0 12px; }
.extension-topline,.extension-actions,.extension-meta,.tag-row { display:flex; align-items:center; gap:8px; }
.extension-topline,.extension-actions { justify-content:space-between; }
.extension-kind,.extension-status { font:10px var(--font-mono); padding:3px 7px; border-radius:999px; }
.extension-kind.builtin { color:var(--primary); background:var(--primary-subtle); }
.extension-kind.external { color:var(--info); background:var(--info-subtle); }
.extension-status.active { color:var(--success); background:var(--success-subtle); }
.extension-status.disabled { color:var(--text-3); background:var(--chip-solid); }
.extension-status.manifest_only { color:var(--warning); background:var(--warning-subtle); }
.extension-meta,.muted { color:var(--text-3); font-size:11px; }
.permission-line { color:var(--text-3); font-size:11px; margin-top:5px; }
.tag-row { flex-wrap:wrap; min-height:24px; }
.market-list { display:flex; flex-direction:column; gap:8px; }
.market-item { display:flex; justify-content:space-between; gap:16px; padding:12px; border-bottom:1px solid var(--border-light); }
.market-item p { margin:4px 0 0; color:var(--text-2); font-size:13px; }
@media (max-width:700px) { .stats-row { grid-template-columns:repeat(2,minmax(0,1fr)); } .page-heading { flex-direction:column; } }
</style>
