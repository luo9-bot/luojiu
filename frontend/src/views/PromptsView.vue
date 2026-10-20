<template>
  <div>
    <div class="card">
      <div class="card-header">
        <div>
          <h3>Prompt 管理</h3>
          <p class="hint">保存后立即热加载，后续请求直接使用新内容。</p>
        </div>
        <button class="btn btn-ghost btn-sm" @click="load">↻ 刷新</button>
      </div>

      <div v-if="loading" class="empty">加载中...</div>
      <template v-else>
        <div class="toolbar">
          <select v-model="selected" class="glass-select" @change="selectPrompt">
            <option value="__persona">当前人设（{{ persona.name }}）</option>
            <option v-for="p in prompts" :key="p.name" :value="p.name">{{ p.name }}</option>
          </select>
          <span class="status" :class="{ custom: isCustomized }">{{ isCustomized ? '已自定义' : '跟随内置版本' }}</span>
        </div>

        <textarea v-model="draft" class="prompt-editor" spellcheck="false"></textarea>

        <div class="editor-actions">
          <button class="btn btn-primary" @click="save">保存并热加载</button>
          <button v-if="isCustomized" class="btn btn-merge" @click="merge">合并新版，保留本地修改</button>
          <button class="btn btn-ghost" @click="reset">恢复内置版本</button>
          <button class="btn btn-ghost" @click="showDiff = !showDiff">{{ showDiff ? '隐藏逐行 Diff' : '查看逐行 Diff' }}</button>
          <button class="btn btn-ghost" @click="showBuiltin = !showBuiltin">{{ showBuiltin ? '隐藏内置版本' : '查看内置版本' }}</button>
        </div>

        <div v-if="showDiff" class="diff-block">
          <div class="builtin-title">当前生效版本 → 插件内置版本</div>
          <div class="diff-list">
            <div v-for="(line, i) in diff" :key="i" class="diff-line" :class="line.kind">
              <span class="diff-mark">{{ line.kind === 'added' ? '+' : line.kind === 'removed' ? '-' : ' ' }}</span>
              <span>{{ line.text || ' ' }}</span>
            </div>
          </div>
        </div>

        <div v-if="showBuiltin" class="builtin-block">
          <div class="builtin-title">当前插件内置版本</div>
          <pre>{{ builtin }}</pre>
        </div>
        <div v-if="notice" class="notice">{{ notice }}</div>
      </template>
    </div>
  </div>
</template>

<script setup>
import { computed, onMounted, ref } from 'vue'
import { api } from '../api.js'

const loading = ref(true)
const prompts = ref([])
const persona = ref({ name: 'default.txt', content: '', builtin: '' })
const selected = ref('__persona')
const draft = ref('')
const builtin = ref('')
const showBuiltin = ref(false)
const showDiff = ref(false)
const notice = ref('')

const current = computed(() => selected.value === '__persona'
  ? persona.value
  : prompts.value.find(p => p.name === selected.value) || {})
const isCustomized = computed(() => selected.value === '__persona'
  ? persona.value.content !== persona.value.builtin
  : Boolean(current.value.customized))
const diff = computed(() => current.value.diff || [])

function selectPrompt() {
  draft.value = selected.value === '__persona' ? persona.value.content : current.value.effective
  builtin.value = selected.value === '__persona' ? persona.value.builtin : current.value.builtin
  showBuiltin.value = false
  showDiff.value = false
  notice.value = ''
}

async function load() {
  loading.value = true
  try {
    const data = await api('/api/prompts')
    prompts.value = data.builtin || []
    persona.value = data.persona || persona.value
    selectPrompt()
  } catch (e) {
    notice.value = e.message || '加载失败'
  }
  loading.value = false
}

async function save() {
  try {
    const name = selected.value === '__persona' ? persona.value.name : selected.value
    const data = await api('/api/prompts/' + encodeURIComponent(name), {
      method: 'PUT', body: JSON.stringify({ content: draft.value })
    })
    notice.value = data.message || '已保存并热加载'
    await load()
  } catch (e) { notice.value = e.message || '保存失败' }
}

async function reset() {
  if (!confirm('确认恢复当前插件内置版本？本地修改会被覆盖。')) return
  try {
    const name = selected.value === '__persona' ? persona.value.name : selected.value
    const data = await api('/api/prompts/' + encodeURIComponent(name) + '/reset', { method: 'POST' })
    notice.value = data.message || '已恢复并热加载'
    await load()
  } catch (e) { notice.value = e.message || '恢复失败' }
}

async function merge() {
  if (!confirm('将新版新增内容合并到当前 Prompt；存在冲突时会生成冲突标记，是否继续？')) return
  try {
    const name = selected.value === '__persona' ? persona.value.name : selected.value
    const data = await api('/api/prompts/' + encodeURIComponent(name) + '/merge', { method: 'POST' })
    if (data.content) draft.value = data.content
    notice.value = data.conflicts
      ? `已生成合并草稿，发现 ${data.conflicts} 处冲突；请检查后点击“保存并热加载”`
      : (data.message || '已合并新版并热加载')
    if (!data.conflicts) await load()
  } catch (e) { notice.value = e.message || '合并失败' }
}

onMounted(() => { load(); window.addEventListener('refresh-all', load) })
</script>

<style scoped>
.card-header { display:flex; align-items:flex-start; justify-content:space-between; gap:12px; margin-bottom:16px; }
.card-header h3 { font-size:15px; font-weight:600; }
.hint { margin-top:4px; color:var(--text-3); font-size:12px; }
.toolbar { display:flex; align-items:center; gap:10px; margin-bottom:12px; }
.glass-select { min-width:260px; padding:8px 10px; border-radius:var(--radius-xs); border:1px solid var(--border); background:var(--surface); color:var(--text); }
.status { padding:4px 8px; border-radius:999px; font-size:11px; background:var(--surface-hover); color:var(--text-2); }
.status.custom { color:var(--warning); }
.prompt-editor { width:100%; min-height:430px; resize:vertical; box-sizing:border-box; padding:14px; border:1px solid var(--border); border-radius:var(--radius-sm); background:var(--surface-hover); color:var(--text); font:12px/1.65 monospace; outline:none; }
.prompt-editor:focus { border-color:var(--primary); }
.editor-actions { display:flex; flex-wrap:wrap; gap:8px; margin-top:12px; }
.btn { padding:8px 14px; border:none; border-radius:var(--radius-xs); font-size:13px; cursor:pointer; }
.btn-primary { background:var(--primary); color:#fff; }
.btn-merge { background:var(--warning); color:#1b1300; }
.btn-ghost { background:var(--surface); color:var(--text); border:1px solid var(--border); }
.btn-sm { padding:4px 10px; font-size:12px; }
.builtin-block { margin-top:16px; }
.diff-block { margin-top:16px; }
.diff-list { max-height:320px; overflow:auto; border:1px solid var(--border); border-radius:var(--radius-sm); background:var(--surface-hover); font:12px/1.6 monospace; }
.diff-line { display:flex; gap:8px; padding:2px 10px; white-space:pre-wrap; }
.diff-line.added { background:color-mix(in srgb, var(--success) 14%, transparent); color:var(--success); }
.diff-line.removed { background:color-mix(in srgb, var(--danger) 14%, transparent); color:var(--danger); }
.diff-mark { width:10px; flex:0 0 10px; color:var(--text-3); }
.builtin-title { color:var(--text-2); font-size:12px; margin-bottom:6px; }
pre { max-height:280px; overflow:auto; padding:12px; border-radius:var(--radius-sm); background:var(--surface-hover); color:var(--text-2); white-space:pre-wrap; font:12px/1.6 monospace; }
.notice { margin-top:12px; color:var(--success); font-size:12px; }
.empty { text-align:center; padding:24px; color:var(--text-3); }
</style>
