<template>
  <div v-if="!loggedIn" class="login-page">
    <div class="login-card">
      <div class="login-core">
        <span class="core-ring"></span>
        <span class="core-ring r2"></span>
        <span class="core-dot"></span>
      </div>
      <h1 class="login-title">Luo9 AI Chat</h1>
      <p class="login-sub num">SENTIENT KERNEL · 管理控制台</p>
      <div class="login-input-group">
        <input v-model="tokenInput" type="password" placeholder="输入管理员 Token" @keydown.enter="doLogin" autofocus />
        <button @click="doLogin" :disabled="!tokenInput.trim()" class="login-btn">
          <span>进入圣所</span>
          <svg viewBox="0 0 20 20" fill="none" width="16" height="16"><path d="M4 10h12M12 6l4 4-4 4" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/></svg>
        </button>
      </div>
      <div class="login-err" v-if="loginErr">{{ loginErr }}</div>
      <div class="login-footer num" v-if="appVersion">
        <span>v{{ appVersion }}</span>
      </div>
    </div>
  </div>
  <div v-else class="shell">
    <aside class="rail" :class="{ open: sidebarOpen }">
      <div class="rail-logo" title="Luo9 AI Chat">洛</div>
      <div class="rail-scroll">
        <template v-for="(group, gi) in navGroups" :key="group.label">
          <div class="rail-sep" v-if="gi > 0"></div>
          <div class="rail-group-label">{{ group.label }}</div>
          <a v-for="t in group.items" :key="t.id"
             class="rail-btn"
             :class="{ active: currentTab === t.id }"
             @click="currentTab = t.id; sidebarOpen = false"
             :title="group.label + ' · ' + t.name">
            <span class="rail-icon" v-html="t.icon"></span>
            <span class="rail-text">{{ t.name }}</span>
          </a>
        </template>
      </div>
    </aside>
    <div v-if="sidebarOpen" class="overlay" @click="sidebarOpen = false"></div>

    <header class="topbar">
      <div class="tb-left">
        <button class="menu-btn" @click="sidebarOpen = !sidebarOpen" aria-label="菜单">
          <svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M3 5h14M3 10h14M3 15h14" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
        </button>
        <span class="tb-title">{{ currentTabMeta?.name || '此刻' }}</span>
        <span class="tb-sub">{{ subtitle }}</span>
      </div>
      <div class="tb-right">
        <div class="tb-status" :class="{ offline: coreStatus === 'offline' }">
          <span class="dot"></span>
          <span>{{ coreStatus === 'offline' ? '核心离线' : '核心在线 · 情感引擎活跃' }}</span>
        </div>
        <button class="tb-icon" @click="toggleTheme" :title="isDark ? '切换亮金模式' : '切换黑金模式'">
          <svg v-if="isDark" viewBox="0 0 20 20" fill="none" width="16" height="16"><circle cx="10" cy="10" r="4" stroke="currentColor" stroke-width="1.5"/><path d="M10 2v2M10 16v2M2 10h2M16 10h2M4.22 4.22l1.42 1.42M14.36 14.36l1.42 1.42M4.22 15.78l1.42-1.42M14.36 5.64l1.42-1.42" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
          <svg v-else viewBox="0 0 20 20" fill="none" width="16" height="16"><path d="M10 2a8 8 0 100 16 6 6 0 010-12 6 6 0 000-4z" fill="currentColor"/></svg>
        </button>
        <button class="tb-icon" @click="refreshAll" title="刷新">
          <svg viewBox="0 0 20 20" fill="none" width="16" height="16"><path d="M14.5 5.5A6.5 6.5 0 104 10.5M14.5 2v3.5H11M5.5 14.5A6.5 6.5 0 0016 9.5M5.5 18V14.5H9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
        </button>
        <button class="tb-icon" @click="doLogout" title="退出">
          <svg viewBox="0 0 20 20" fill="none" width="16" height="16"><path d="M7 17H4a1 1 0 01-1-1V4a1 1 0 011-1h3M13 14l4-4-4-4M17 10H7" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
        </button>
      </div>
    </header>

    <main class="stage">
      <div class="page-content" :key="currentTab">
        <component :is="tabComponent" />
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { getToken, setToken, clearToken, tryLogin, api } from './api.js'
import CoreView from './views/CoreView.vue'
import MindView from './views/MindView.vue'
import StreamView from './views/StreamView.vue'
import MindMemoryView from './views/MindMemoryView.vue'
import SecurityView from './views/SecurityView.vue'
import SocialView from './views/SocialView.vue'
import DashboardView from './views/DashboardView.vue'
import ConfigView from './views/ConfigView.vue'
import ConversationsView from './views/ConversationsView.vue'
import QuotaView from './views/QuotaView.vue'
import StickerView from './views/StickerView.vue'
import UserMemory from './views/UserMemory.vue'
import WorkingMemory from './views/WorkingMemory.vue'
import EmotionView from './views/EmotionView.vue'
import BlocklistView from './views/BlocklistView.vue'
import AntiInjectionView from './views/AntiInjectionView.vue'
import ArchiveView from './views/ArchiveView.vue'
import BackupsView from './views/BackupsView.vue'
import MemoryOpsLog from './views/MemoryOpsLog.vue'
import ScheduleView from './views/ScheduleView.vue'
import AnalyticsView from './views/AnalyticsView.vue'
import HumanityView from './views/HumanityView.vue'
import RelationshipsView from './views/RelationshipsView.vue'

const I = {
  core: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><circle cx="10" cy="10" r="7.5" stroke="currentColor" stroke-width="1.5"/><circle cx="10" cy="10" r="3" stroke="currentColor" stroke-width="1.5"/><path d="M10 2.5v2M10 15.5v2M2.5 10h2M15.5 10h2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',
  dashboard: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><rect x="3" y="3" width="6" height="6" rx="1.5" stroke="currentColor" stroke-width="1.5"/><rect x="11" y="3" width="6" height="6" rx="1.5" stroke="currentColor" stroke-width="1.5"/><rect x="3" y="11" width="6" height="6" rx="1.5" stroke="currentColor" stroke-width="1.5"/><rect x="11" y="11" width="6" height="6" rx="1.5" stroke="currentColor" stroke-width="1.5"/></svg>',
  analytics: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M3 17V7l4 4 3-6 3 3 4-5v14H3z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/></svg>',
  schedule: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><rect x="3" y="4" width="14" height="14" rx="2" stroke="currentColor" stroke-width="1.5"/><path d="M3 8h14M7 1v3M13 1v3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',
  config: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><circle cx="10" cy="10" r="3" stroke="currentColor" stroke-width="1.5"/><path d="M10 1v2M10 17v2M1 10h2M17 10h2M4.22 4.22l1.42 1.42M14.36 14.36l1.42 1.42M4.22 15.78l1.42-1.42M14.36 5.64l1.42-1.42" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',
  conversations: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M3 10a7 7 0 1114 0 7 7 0 01-7 7H3l2-3a7 7 0 01-2-4z" stroke="currentColor" stroke-width="1.5"/><path d="M7 8h6M7 11h4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',
  quota: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><circle cx="10" cy="10" r="7" stroke="currentColor" stroke-width="1.5"/><path d="M10 6v4l3 2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>',
  sticker: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><rect x="3" y="3" width="14" height="14" rx="2" stroke="currentColor" stroke-width="1.5"/><circle cx="7.5" cy="8.5" r="1.5" fill="currentColor"/><path d="M5 15l3-4 3 4H5z" fill="currentColor" opacity="0.5"/><path d="M11 13l3-5 3 5H11z" fill="currentColor" opacity="0.5"/></svg>',
  mind: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M2 10h3l2-5 3 10 2-5h6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>',
  memory: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><rect x="4" y="3" width="12" height="14" rx="2" stroke="currentColor" stroke-width="1.5"/><path d="M7 7h6M7 10h6" stroke="currentColor" stroke-width="1.5"/></svg>',
  'working-memory': '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><rect x="3" y="3" width="14" height="14" rx="2" stroke="currentColor" stroke-width="1.5"/><path d="M7 7h6M7 10h6" stroke="currentColor" stroke-width="1.5"/></svg>',
  stream: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M2 6c3-2 5 2 8 0s5 2 8 0" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><path d="M2 10c3-2 5 2 8 0s5 2 8 0" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" opacity="0.7"/><path d="M2 14c3-2 5 2 8 0s5 2 8 0" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" opacity="0.4"/></svg>',
  emotion: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><circle cx="10" cy="10" r="7" stroke="currentColor" stroke-width="1.5"/><circle cx="7.5" cy="8.5" r="1" fill="currentColor"/><circle cx="12.5" cy="8.5" r="1" fill="currentColor"/><path d="M7 12.5c.8 1 2 1.5 3 1.5s2.2-.5 3-1.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',
  diary: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M4 3h11a1 1 0 011 1v12a1 1 0 01-1 1H4V3z" stroke="currentColor" stroke-width="1.5"/><path d="M4 3v14M13 3v14" stroke="currentColor" stroke-width="1.5"/><path d="M7 7h4M7 10h4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',
  humanity: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M10 3a7 7 0 017 7c0 2.5-1 4.5-2.5 5.5S10 17 10 17s-3.5-.5-4.5-1.5S3 12.5 3 10a7 7 0 017-7z" stroke="currentColor" stroke-width="1.5"/><circle cx="8" cy="9" r="1" fill="currentColor"/><circle cx="12" cy="9" r="1" fill="currentColor"/><path d="M7 12.5c.8 1 2 1.5 3 1.5s2.2-.5 3-1.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',
  blocklist: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><circle cx="10" cy="10" r="7" stroke="currentColor" stroke-width="1.5"/><path d="M6 6l8 8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',
  'anti-injection': '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M10 2l7 3v5c0 4-3 7-7 8-4-1-7-4-7-8V5l7-3z" stroke="currentColor" stroke-width="1.5"/><path d="M7 10l2 2 4-4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>',
  archive: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><rect x="3" y="6" width="14" height="11" rx="2" stroke="currentColor" stroke-width="1.5"/><path d="M2 4a1 1 0 011-1h14a1 1 0 011 1v2H2V4z" stroke="currentColor" stroke-width="1.5"/><path d="M8 10h4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',
  backups: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M10 3a5 5 0 00-4.5 2.8A4 4 0 003 10a4 4 0 004 4h1.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><path d="M10 3a5 5 0 014.5 2.8A4 4 0 0117 10a4 4 0 01-4 4h-1.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><path d="M10 10v5M7 12.5l3-2.5 3 2.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>',
  'memory-ops-log': '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M3 3h14v14H3z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/><path d="M7 7h6M7 10h4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><circle cx="14" cy="14" r="2" fill="currentColor" opacity="0.6"/></svg>',
  relationships: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><circle cx="7" cy="7" r="3" stroke="currentColor" stroke-width="1.5"/><circle cx="13" cy="7" r="3" stroke="currentColor" stroke-width="1.5"/><path d="M3 17c0-2.2 1.8-4 4-4s4 1.8 4 4M9 17c0-2.2 1.8-4 4-4s4 1.8 4 4" stroke="currentColor" stroke-width="1.5"/></svg>',
  security: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M3 3h14l-5.5 6.5V16l-3 1.5v-8L3 3z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/></svg>',
  social: '<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><circle cx="6" cy="6" r="2.5" stroke="currentColor" stroke-width="1.5"/><circle cx="14.5" cy="8" r="2" stroke="currentColor" stroke-width="1.5"/><circle cx="9" cy="14" r="2" stroke="currentColor" stroke-width="1.5"/><path d="M7.5 7.5L13 9M7.8 12.5L10.5 9.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" opacity="0.6"/></svg>',
}

const loggedIn = ref(false)
const tokenInput = ref('')
const loginErr = ref('')
const currentTab = ref('core')
const sidebarOpen = ref(false)
const appVersion = ref('')
const isDark = ref(true)
const coreStatus = ref('online')
let statusTimer = null

const tabs = [
  { id: 'core', name: '控制总览', group: '概览', icon: I.core, desc: 'AI 情绪核心 · 四维实时频谱', comp: CoreView },
  { id: 'mind', name: '此刻', group: '心灵', icon: I.mind, desc: '她的此刻：状态、身体与意识流', comp: MindView },
  { id: 'stream', name: '意识流', group: '心灵', icon: I.stream, desc: '她的人生时间线', comp: StreamView },
  { id: 'social', name: '群里的势', group: '心灵', icon: I.social, desc: '她的社会感知：话题线、注意力、关系与等待', comp: SocialView },
  { id: 'mind-memory', name: '日记与人物', group: '心灵', icon: I.diary, desc: '她的日记与人物档案', comp: MindMemoryView },
  { id: 'security', name: '滤壳与安全', group: '心灵', icon: I.security, desc: '滤壳审计记录', comp: SecurityView },
  { id: 'dashboard', name: '仪表盘', group: '概览', icon: I.dashboard, desc: '系统总览与关键指标', comp: DashboardView },
  { id: 'analytics', name: 'Token 分析', group: '概览', icon: I.analytics, desc: 'API 用量与 Prompt 统计', comp: AnalyticsView },
  { id: 'schedule', name: '日程计划', group: '管理', icon: I.schedule, desc: '周/月计划管理', comp: ScheduleView },
  { id: 'config', name: '配置', group: '管理', icon: I.config, desc: 'Bot 与 AI 参数', comp: ConfigView },
  { id: 'conversations', name: '对话管理', group: '管理', icon: I.conversations, desc: '活跃会话控制', comp: ConversationsView },
  { id: 'quota', name: '配额', group: '管理', icon: I.quota, desc: '回复配额', comp: QuotaView },
  { id: 'sticker', name: '表情包', group: '管理', icon: I.sticker, desc: '表情管理', comp: StickerView },
  { id: 'memory', name: '用户记忆', group: '数据', icon: I.memory, desc: '长期记忆', comp: UserMemory },
  { id: 'working-memory', name: '工作记忆', group: '数据', icon: I['working-memory'], desc: '短期工作记忆', comp: WorkingMemory },
  { id: 'memory-ops-log', name: '内存监视', group: '数据', icon: I['memory-ops-log'], desc: '内存操作日志', comp: MemoryOpsLog },
  { id: 'emotion', name: '情绪', group: '数据', icon: I.emotion, desc: '情绪状态', comp: EmotionView },
  { id: 'humanity', name: '人性化', group: '数据', icon: I.humanity, desc: '人性化状态监控', comp: HumanityView },
  { id: 'relationships', name: '关系', group: '数据', icon: I.relationships, desc: '关系动力学', comp: RelationshipsView },
  { id: 'blocklist', name: '黑名单', group: '系统', icon: I.blocklist, desc: '用户管理', comp: BlocklistView },
  { id: 'anti-injection', name: '防注入', group: '系统', icon: I['anti-injection'], desc: '安全防护', comp: AntiInjectionView },
  { id: 'archive', name: '归档', group: '系统', icon: I.archive, desc: '数据归档', comp: ArchiveView },
  { id: 'backups', name: '备份', group: '系统', icon: I.backups, desc: '数据备份', comp: BackupsView },
]

const navGroups = computed(() => {
  const groups = []
  for (const tab of tabs) {
    let group = groups.find(g => g.label === tab.group)
    if (!group) groups.push(group = { label: tab.group, items: [] })
    group.items.push(tab)
  }
  return groups
})

const currentTabMeta = computed(() => tabs.find(t => t.id === currentTab.value))
const tabComponent = computed(() => tabs.find(t => t.id === currentTab.value)?.comp)
const subtitle = computed(() => {
  const meta = currentTabMeta.value
  if (!meta) return 'NEXUS // SENTIENT CONSOLE'
  return `${meta.group} · ${meta.desc}`
})

function applyTheme(dark) {
  isDark.value = dark
  document.documentElement.setAttribute('data-theme', dark ? 'dark' : 'light')
  localStorage.setItem('ai-chat-theme', dark ? 'dark' : 'light')
}
function toggleTheme() { applyTheme(!isDark.value) }
function initTheme() {
  // 黑金是默认身份，仅在用户显式选择过亮金时才切换
  const saved = localStorage.getItem('ai-chat-theme')
  applyTheme(saved !== 'light')
}

async function doLogin() {
  loginErr.value = ''
  const t = tokenInput.value.trim()
  if (!t) return
  try { if (await tryLogin(t)) loggedIn.value = true; else loginErr.value = 'Token 验证失败' }
  catch { loginErr.value = '网络错误' }
}
function doLogout() { clearToken(); loggedIn.value = false }
function refreshAll() { window.dispatchEvent(new CustomEvent('refresh-all')) }

async function pingCore() {
  try { await api('/api/version'); coreStatus.value = 'online' }
  catch { coreStatus.value = 'offline' }
}

onMounted(async () => {
  initTheme()
  const t = getToken()
  if (t) { try { if (await tryLogin(t)) loggedIn.value = true } catch {} }
  try { const info = await api('/api/version'); appVersion.value = info.version } catch {}
  pingCore()
  statusTimer = setInterval(pingCore, 30000)
})
onUnmounted(() => { if (statusTimer) clearInterval(statusTimer) })
</script>

<style>
/* ── NEXUS 外壳：68px 图标栏 × 56px 顶栏 × 舞台 ── */
.shell {
  position: relative;
  z-index: 10;
  height: 100vh;
  display: grid;
  grid-template-columns: 68px 1fr;
  grid-template-rows: 56px 1fr;
  grid-template-areas:
    "rail topbar"
    "rail stage";
  overflow: hidden;
}

/* ── 左侧竖排图标栏（68px，悬停展开 224px） ── */
.rail {
  grid-area: rail;
  background: rgba(16, 16, 21, 0.6);
  backdrop-filter: blur(24px) saturate(140%);
  -webkit-backdrop-filter: blur(24px) saturate(140%);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  align-items: stretch;
  padding: 12px 0;
  position: relative;
  z-index: 30;
  overflow: visible;
  transition: width 0.2s ease;
}
[data-theme="light"] .rail { background: rgba(246, 240, 226, 0.72); }

.rail-logo {
  width: 36px;
  height: 36px;
  margin: 0 auto 14px;
  border-radius: 11px;
  background: linear-gradient(135deg, var(--gold), var(--gold-deep, #B87A12));
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  font-weight: 800;
  color: var(--black, #0A0A0D);
  flex-shrink: 0;
  box-shadow: 0 0 24px rgba(245, 166, 35, 0.4), inset 0 1px 0 rgba(255, 255, 255, 0.3);
  position: relative;
}
.rail-logo::after {
  content: '';
  position: absolute;
  inset: -3px;
  border-radius: 14px;
  border: 1px solid rgba(245, 166, 35, 0.3);
  animation: logoPulse 3s ease-in-out infinite;
}
@keyframes logoPulse {
  0%, 100% { opacity: 0.3; transform: scale(1); }
  50% { opacity: 0.8; transform: scale(1.05); }
}

.rail-scroll {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 4px 12px 16px;
  display: flex;
  flex-direction: column;
  align-items: stretch;
}
.rail-sep {
  height: 1px;
  margin: 9px 4px;
  background: linear-gradient(90deg, transparent, var(--border-strong), transparent);
  flex-shrink: 0;
}
.rail-group-label {
  font-size: 9px;
  font-weight: 600;
  letter-spacing: 1.5px;
  text-transform: uppercase;
  color: var(--text-3);
  padding: 4px 6px 3px;
  white-space: nowrap;
  opacity: 0;
  height: 0;
  overflow: hidden;
  transition: opacity 0.18s ease;
}
.rail-btn {
  width: 42px;
  height: 42px;
  margin: 0 auto 3px;
  border-radius: 11px;
  display: flex;
  align-items: center;
  justify-content: flex-start;
  gap: 12px;
  padding: 0 12px;
  cursor: pointer;
  color: var(--text-2);
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  flex-shrink: 0;
  text-decoration: none;
  white-space: nowrap;
}
.rail-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  flex-shrink: 0;
}
.rail-text {
  font-size: 13px;
  font-weight: 500;
  opacity: 0;
  transition: opacity 0.18s ease;
}
.rail-btn:hover:not(.active) { background: rgba(245, 166, 35, 0.08); color: var(--gold); }
[data-theme="light"] .rail-btn:hover:not(.active) { background: var(--primary-subtle); color: var(--primary); }
.rail-btn.active { background: rgba(245, 166, 35, 0.12); color: var(--gold); }
[data-theme="light"] .rail-btn.active { background: var(--primary-subtle); color: var(--primary); }
.rail-btn.active::before {
  content: '';
  position: absolute;
  left: -12px;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 20px;
  background: var(--gold);
  border-radius: 0 3px 3px 0;
  box-shadow: 0 0 12px var(--gold);
}

/* 悬停展开：露出文字（68px 默认保持纯图标） */
.rail:hover {
  width: 224px;
  box-shadow: 12px 0 34px rgba(0, 0, 0, 0.4);
  background: rgba(16, 16, 21, 0.92);
}
[data-theme="light"] .rail:hover { background: rgba(246, 240, 226, 0.95); box-shadow: 12px 0 34px rgba(90, 66, 15, 0.14); }
.rail:hover .rail-text { opacity: 1; }
.rail:hover .rail-group-label { opacity: 1; height: auto; padding: 4px 6px 3px; }
.rail:hover .rail-btn { width: auto; align-self: stretch; margin-left: 0; margin-right: 0; }
.rail:not(:hover) .rail-btn { align-self: center; }

/* ── 顶栏 ── */
.topbar {
  grid-area: topbar;
  background: rgba(16, 16, 21, 0.4);
  backdrop-filter: blur(24px) saturate(140%);
  -webkit-backdrop-filter: blur(24px) saturate(140%);
  border-bottom: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 26px;
  position: relative;
  z-index: 20;
}
[data-theme="light"] .topbar { background: rgba(246, 240, 226, 0.6); }
.tb-left { display: flex; align-items: baseline; gap: 16px; min-width: 0; }
.menu-btn {
  display: none;
  align-items: center; justify-content: center;
  width: 32px; height: 32px; padding: 0;
  background: none; border: 1px solid var(--border);
  border-radius: 9px; cursor: pointer;
  color: var(--text-1); align-self: center;
}
.menu-btn:hover { color: var(--gold); border-color: var(--border-strong); }
.tb-title {
  font-size: 15px;
  font-weight: 600;
  letter-spacing: 0.3px;
  white-space: nowrap;
}
.tb-sub {
  font-size: 11.5px;
  color: var(--text-2);
  font-family: var(--font-mono);
  letter-spacing: 0.5px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  min-width: 0;
}
.tb-right { display: flex; align-items: center; gap: 10px; flex-shrink: 0; }
.tb-status {
  display: flex;
  align-items: center;
  gap: 7px;
  font-size: 11.5px;
  color: var(--text-1);
  padding: 6px 12px;
  border-radius: 20px;
  background: rgba(245, 166, 35, 0.06);
  border: 1px solid var(--border);
  font-family: var(--font-mono);
  letter-spacing: 0.3px;
}
.tb-status.offline { color: var(--danger); background: var(--danger-subtle); border-color: var(--danger-subtle); }
.tb-status .dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--gold);
  box-shadow: 0 0 8px var(--gold);
  animation: pulse 2s infinite;
}
.tb-status.offline .dot { background: var(--danger); box-shadow: 0 0 8px var(--danger); }
@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.3; } }
.tb-icon {
  width: 34px;
  height: 34px;
  border-radius: 9px;
  border: 1px solid var(--border);
  background: transparent;
  color: var(--text-1);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s;
  padding: 0;
}
.tb-icon:hover {
  color: var(--gold);
  border-color: var(--border-strong);
  background: rgba(245, 166, 35, 0.05);
}

/* ── 舞台 ── */
.stage {
  grid-area: stage;
  position: relative;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 24px 28px;
  z-index: 10;
}
.stage::-webkit-scrollbar { width: 5px; }
.stage::-webkit-scrollbar-thumb { background: rgba(245, 166, 35, 0.25); border-radius: 5px; }
.stage::-webkit-scrollbar-track { background: transparent; }
.page-content { min-height: 100%; animation: fadeIn 0.3s ease; }
/* 控制总览需要撑满舞台高度（ov-layout height:100% 依赖确定的父高度） */
.stage:has(.ov-layout) .page-content { height: 100%; min-height: 0; }

/* ── 登录页：黑金圣所 ── */
.login-page {
  position: relative; z-index: 1;
  min-height: 100vh;
  display: flex; align-items: center; justify-content: center;
  background: var(--bg);
  padding: 20px;
}
.login-card {
  width: 360px; padding: 40px 36px; text-align: center;
  background: var(--surface);
  backdrop-filter: blur(14px);
  border-radius: var(--radius);
  border: 1px solid var(--glass-border);
  box-shadow: var(--glass-shadow-lg);
  position: relative; overflow: hidden;
}
.login-card::before {
  content: '';
  position: absolute; top: 0; left: 0; right: 0; height: 1px;
  background: linear-gradient(90deg, transparent, var(--gold), transparent);
}
.login-core { position: relative; width: 72px; height: 72px; margin: 0 auto 18px; }
.login-core .core-ring {
  position: absolute; inset: 0;
  border: 1px solid var(--gold-dim);
  border-radius: 50%;
  animation: pulse 2.4s ease-in-out infinite;
}
.login-core .core-ring.r2 { inset: -8px; opacity: 0.4; animation-delay: 0.6s; }
.login-core .core-dot {
  position: absolute; inset: 22px;
  border-radius: 50%;
  background: radial-gradient(circle, var(--gold) 0%, rgba(245, 166, 35, 0.2) 70%, transparent 100%);
  box-shadow: 0 0 24px var(--gold-glow);
  animation: pulse 1.8s ease-in-out infinite;
}
.login-title { font-size: 22px; font-weight: 400; letter-spacing: 0.12em; }
.login-sub { font-size: 11px; color: var(--gold-dim); margin: 6px 0 22px; letter-spacing: 0.18em; text-transform: uppercase; }
[data-theme="light"] .login-sub { color: var(--primary); }
.login-input-group { display: flex; flex-direction: column; gap: 10px; }
.login-input-group input {
  padding: 11px 14px; border-radius: var(--radius-sm);
  border: 1px solid var(--border); background: var(--chip-solid);
  color: var(--text); font-size: 14px; outline: none; transition: var(--transition-fast);
}
.login-input-group input:focus { border-color: var(--gold-dim); box-shadow: 0 0 0 3px var(--primary-glow); }
.login-btn {
  display: flex; align-items: center; justify-content: center; gap: 8px;
  padding: 11px; border: 1px solid var(--primary);
  border-radius: var(--radius-sm);
  background: var(--primary);
  color: #0A0A0D; font-size: 14px; font-weight: 600; letter-spacing: 0.08em;
  cursor: pointer; transition: var(--transition-fast);
}
[data-theme="light"] .login-btn { color: #FDFAF2; }
.login-btn:hover { background: var(--primary-hover); border-color: var(--primary-hover); }
.login-btn:disabled { opacity: 0.5; cursor: not-allowed; }
.login-err { color: var(--danger); font-size: 13px; margin-top: 10px; }
.login-footer { margin-top: 20px; font-size: 12px; color: var(--text-3); display: flex; justify-content: center; gap: 6px; }

/* 遮罩（移动端抽屉） */
.overlay { position: fixed; inset: 0; background: rgba(0, 0, 0, 0.45); z-index: 29; }

/* ── 响应式：移动端 ── */
@media (max-width: 768px) {
  .shell {
    grid-template-columns: 1fr;
    grid-template-rows: 56px 1fr;
    grid-template-areas:
      "topbar"
      "stage";
  }
  .rail {
    position: fixed;
    top: 0; left: 0; bottom: 0;
    width: 224px;
    transform: translateX(-260px);
    transition: transform 0.25s ease;
    z-index: 40;
    background: rgba(16, 16, 21, 0.96);
    box-shadow: none;
  }
  [data-theme="light"] .rail { background: rgba(246, 240, 226, 0.97); }
  .rail.open { transform: translateX(0); box-shadow: 12px 0 34px rgba(0, 0, 0, 0.4); }
  .rail .rail-text { opacity: 1; }
  .rail .rail-group-label { opacity: 1; height: auto; }
  .rail .rail-btn { width: auto; align-self: stretch; }
  .menu-btn { display: flex; }
  .topbar { padding: 0 14px; gap: 10px; }
  .tb-left { gap: 10px; }
  .tb-sub { display: none; }
  .tb-status { display: none; }
  .stage { padding: 14px 14px 28px; }
}
</style>
