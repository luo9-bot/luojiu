<template>
  <div class="ov-layout">
    <!-- 左侧：透明核心舞台（透出全屏 3D 背景） -->
    <div class="core-stage" ref="stageEl">
      <div class="core-stage-overlay">
        <div class="core-stage-label">LIVE EMOTION CORE</div>
        <div class="core-readout">
          <div class="core-readout-item">
            <span class="lbl">COHERENCE</span>
            <span class="val">{{ ui.coherence }}</span>
          </div>
          <div class="core-readout-item">
            <span class="lbl">RESONANCE</span>
            <span class="val">{{ ui.resonance }}</span>
          </div>
          <div class="core-readout-item">
            <span class="lbl">ENTROPY</span>
            <span class="val">{{ ui.entropy }}</span>
          </div>
        </div>
      </div>
      <div class="core-name">
        <div class="cn-title">洛玖</div>
        <div class="cn-sub">{{ stateLine }}</div>
      </div>
    </div>

    <!-- 右侧：情绪频谱 + 核心事件流 -->
    <div class="ov-right">
      <div class="glass ov-panel">
        <div class="panel-title">
          情绪频谱
          <span class="pt-right">实时</span>
        </div>
        <div v-for="e in emoDefs" :key="e.key" class="emo-row">
          <div class="emo-head">
            <span class="emo-name">
              <span class="color-dot" :style="{ background: e.color, boxShadow: '0 0 6px ' + e.color }"></span>
              {{ e.name }}
            </span>
            <span class="emo-val">{{ pct(ui[e.key]) }}</span>
          </div>
          <div class="emo-track">
            <div class="emo-fill" :style="{ width: pct(ui[e.key]), background: e.color, color: e.color }"></div>
          </div>
        </div>
      </div>

      <div class="glass ov-panel events-panel">
        <div class="panel-title">
          核心事件流
          <span class="pt-right">LIVE</span>
        </div>
        <div class="event-stream">
          <div v-for="(ev, i) in events" :key="i" class="event-line">
            <span class="event-time">{{ ev.time }}</span>
            <span class="event-text"><span class="hl">{{ ev.kind }}</span> {{ ev.content }}</span>
          </div>
          <div v-if="!events.length" class="event-empty">暂无近期事件 · 静默中</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, onUnmounted } from 'vue'
import * as THREE from 'three'
import { api } from '../api.js'

// ── 情绪维度定义（颜色与频谱条一致） ──
const emoDefs = [
  { key: 'joy', name: '愉悦度', color: '#F5A623' },
  { key: 'curious', name: '好奇心', color: '#FFC85C' },
  { key: 'empathy', name: '共情强度', color: '#B87A12' },
  { key: 'stress', name: '压力水平', color: '#E5484D' },
]

const STATE_LABELS = {
  Neutral: 'CALM · 平静', Happy: 'BRIGHT · 开心', Excited: 'CHARGED · 兴奋',
  Sad: 'HEAVY · 低落', Thinking: 'DRIFTING · 出神', Surprised: 'ALERT · 惊讶',
  Angry: 'STORMY · 烦躁', Shy: 'SOFT · 害羞', Worried: 'UNEASY · 不踏实',
  Tired: 'DIM · 疲惫', Like: 'RESONANT · 心动',
}
const KIND_LABELS = { sensation: '感官', inner: '内心', acted: '行动', digested: '沉淀', recall: '联想' }

// 3D 与 UI 共享的平滑状态（非响应式，避免高频触发 Vue 回流）
const S = {
  joy: 0.6, curious: 0.45, empathy: 0.5, stress: 0.2,
  target: { joy: 0.6, curious: 0.45, empathy: 0.5, stress: 0.2 },
}

// UI 展示层：10fps 节流更新
const ui = reactive({
  joy: 0.6, curious: 0.45, empathy: 0.5, stress: 0.2,
  coherence: 0.00, resonance: 0.00, entropy: 0.00,
})
const stateLine = ref('CALM · 平静')
const events = ref([])
const stageEl = ref(null)

const pct = v => Math.round((v || 0) * 100) + '%'
const fmt2 = v => (v == null ? '--' : Number(v).toFixed(2))

// ── 数据轮询 ──
async function loadCore() {
  try {
    const j = await api('/api/emotion/core')
    S.target.joy = num(j.joy, S.target.joy)
    S.target.curious = num(j.curiosity, S.target.curious)
    S.target.empathy = num(j.empathy, S.target.empathy)
    S.target.stress = num(j.stress, S.target.stress)
    ui.coherence = fmt2(j.coherence)
    ui.resonance = fmt2(j.resonance)
    ui.entropy = fmt2(j.entropy)
    stateLine.value = STATE_LABELS[j.state] || String(j.state || 'CALM').toUpperCase()
  } catch {}
}
async function loadEvents() {
  try {
    const j = await api('/api/mind/now')
    events.value = (j.recent_stream || []).slice(0, 12).map(e => ({
      time: fmtTime(e.time),
      kind: e.recall ? '联想' : (KIND_LABELS[e.kind] || e.kind),
      content: e.content,
    }))
  } catch {}
}
function num(v, d) { const n = Number(v); return Number.isFinite(n) ? Math.min(Math.max(n, 0), 1) : d }
function fmtTime(secs) {
  if (!secs) return '--:--'
  return new Date(secs * 1000).toLocaleTimeString('zh-CN', { hour12: false, hour: '2-digit', minute: '2-digit' })
}

// ── Three.js 全屏背景 ──
let renderer, scene, camera, canvas, clock
let rafId = 0, uiTimer = 0, pollTimer = 0, eventTimer = 0, themeObserver = null
let coreGroup, innerCore, glowCore, wireCore, neurons, centerGlow, coreLight
let originalPositions = null
let orbitRings = [], flowPoints = null, flowData = [], stars = null
let camTheta = 0, camPhi = 1.437
let dragging = false, lastX = 0, lastY = 0
let disposed = false

const clamp = (v, a, b) => Math.min(Math.max(v, a), b)

function isLight() { return document.documentElement.getAttribute('data-theme') === 'light' }

function initThree() {
  canvas = document.createElement('canvas')
  canvas.id = 'core-webgl'
  document.body.appendChild(canvas)

  renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true })
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
  renderer.setSize(window.innerWidth, window.innerHeight)
  renderer.setClearColor(0x000000, 0)

  scene = new THREE.Scene()
  applyTheme3D()

  camera = new THREE.PerspectiveCamera(55, window.innerWidth / window.innerHeight, 0.1, 200)
  clock = new THREE.Clock()

  // ── 1. 核心球体（多层叠加） ──
  coreGroup = new THREE.Group()
  scene.add(coreGroup)

  innerCore = new THREE.Mesh(
    new THREE.SphereGeometry(1.0, 64, 64),
    new THREE.MeshBasicMaterial({ color: 0xF5A623, transparent: true, opacity: 0.15, blending: THREE.AdditiveBlending })
  )
  coreGroup.add(innerCore)

  glowCore = new THREE.Mesh(
    new THREE.SphereGeometry(1.35, 48, 48),
    new THREE.MeshBasicMaterial({ color: 0xF5A623, transparent: true, opacity: 0.06, blending: THREE.AdditiveBlending, side: THREE.BackSide })
  )
  coreGroup.add(glowCore)

  const wireGeo = new THREE.IcosahedronGeometry(1.7, 4)
  wireCore = new THREE.Mesh(
    wireGeo,
    new THREE.MeshBasicMaterial({ color: 0xF5A623, wireframe: true, transparent: true, opacity: 0.35, blending: THREE.AdditiveBlending })
  )
  coreGroup.add(wireCore)
  originalPositions = wireGeo.attributes.position.array.slice()

  // 400 个神经元粒子
  const neuronCount = 400
  const neuronPos = new Float32Array(neuronCount * 3)
  const neuronColors = new Float32Array(neuronCount * 3)
  for (let i = 0; i < neuronCount; i++) {
    const r = Math.random() * 1.6
    const theta = Math.random() * Math.PI * 2
    const phi = Math.acos(Math.random() * 2 - 1)
    neuronPos[i * 3] = r * Math.sin(phi) * Math.cos(theta)
    neuronPos[i * 3 + 1] = r * Math.sin(phi) * Math.sin(theta)
    neuronPos[i * 3 + 2] = r * Math.cos(phi)
    const t = Math.random()
    neuronColors[i * 3] = 0.96
    neuronColors[i * 3 + 1] = 0.65 + t * 0.2
    neuronColors[i * 3 + 2] = 0.15 + t * 0.2
  }
  const neuronGeo = new THREE.BufferGeometry()
  neuronGeo.setAttribute('position', new THREE.BufferAttribute(neuronPos, 3))
  neuronGeo.setAttribute('color', new THREE.BufferAttribute(neuronColors, 3))
  neurons = new THREE.Points(neuronGeo, new THREE.PointsMaterial({
    size: 0.03, vertexColors: true, transparent: true, opacity: 0.9,
    blending: THREE.AdditiveBlending, depthWrite: false,
  }))
  coreGroup.add(neurons)

  centerGlow = new THREE.Mesh(
    new THREE.SphereGeometry(0.15, 16, 16),
    new THREE.MeshBasicMaterial({ color: 0xFFE0A0, transparent: true, opacity: 0.9 })
  )
  coreGroup.add(centerGlow)

  coreLight = new THREE.PointLight(0xF5A623, 2, 20, 2)
  coreGroup.add(coreLight)

  // ── 2. 四个情绪轨道环 ──
  orbitRings = []
  emoDefs.forEach((emo, idx) => {
    const radius = 2.8 + idx * 0.55
    const segments = 128
    const ringPos = new Float32Array(segments * 3)
    for (let i = 0; i < segments; i++) {
      const angle = (i / segments) * Math.PI * 2
      ringPos[i * 3] = Math.cos(angle) * radius
      ringPos[i * 3 + 1] = 0
      ringPos[i * 3 + 2] = Math.sin(angle) * radius
    }
    const ringGeo = new THREE.BufferGeometry()
    ringGeo.setAttribute('position', new THREE.BufferAttribute(ringPos, 3))
    const ring = new THREE.Line(ringGeo, new THREE.LineBasicMaterial({
      color: new THREE.Color(emo.color), transparent: true, opacity: 0.5, blending: THREE.AdditiveBlending,
    }))
    ring.rotation.x = idx * 0.35 - 0.5
    ring.rotation.y = idx * 0.5
    ring.rotation.z = idx * 0.2
    scene.add(ring)
    orbitRings.push({
      mesh: ring, baseRotation: { x: ring.rotation.x, y: ring.rotation.y, z: ring.rotation.z },
      emoKey: emo.key, phase: idx * 1.2, speed: 0.3 + idx * 0.15,
    })
  })

  // ── 3. 800 个情感数据流粒子 ──
  const flowCount = 800
  const flowPos = new Float32Array(flowCount * 3)
  const flowColors = new Float32Array(flowCount * 3)
  flowData = []
  for (let i = 0; i < flowCount; i++) {
    const emoIdx = Math.floor(Math.random() * 4)
    flowData.push({
      emoIdx,
      startAngle: Math.random() * Math.PI * 2,
      radius: 2.8 + emoIdx * 0.55,
      t: Math.random(),
      speed: 0.15 + Math.random() * 0.2,
      offset: Math.random() * Math.PI * 2,
    })
    const c = new THREE.Color(emoDefs[emoIdx].color)
    flowColors[i * 3] = c.r; flowColors[i * 3 + 1] = c.g; flowColors[i * 3 + 2] = c.b
  }
  const flowGeo = new THREE.BufferGeometry()
  flowGeo.setAttribute('position', new THREE.BufferAttribute(flowPos, 3))
  flowGeo.setAttribute('color', new THREE.BufferAttribute(flowColors, 3))
  flowPoints = new THREE.Points(flowGeo, new THREE.PointsMaterial({
    size: 0.045, vertexColors: true, transparent: true, opacity: 0.75,
    blending: THREE.AdditiveBlending, depthWrite: false, sizeAttenuation: true,
  }))
  scene.add(flowPoints)

  // ── 4. 深空星空（1200） ──
  const starCount = 1200
  const starPos = new Float32Array(starCount * 3)
  const starColor = new Float32Array(starCount * 3)
  for (let i = 0; i < starCount; i++) {
    const r = 30 + Math.random() * 50
    const theta = Math.random() * Math.PI * 2
    const phi = Math.acos(Math.random() * 2 - 1)
    starPos[i * 3] = r * Math.sin(phi) * Math.cos(theta)
    starPos[i * 3 + 1] = r * Math.sin(phi) * Math.sin(theta) * 0.6
    starPos[i * 3 + 2] = r * Math.cos(phi) - 20
    const t = Math.random()
    if (t < 0.7) { starColor[i * 3] = 0.96; starColor[i * 3 + 1] = 0.65; starColor[i * 3 + 2] = 0.15 }
    else if (t < 0.9) { starColor[i * 3] = 0.6; starColor[i * 3 + 1] = 0.7; starColor[i * 3 + 2] = 0.9 }
    else { starColor[i * 3] = 0.9; starColor[i * 3 + 1] = 0.9; starColor[i * 3 + 2] = 0.95 }
  }
  const starGeo = new THREE.BufferGeometry()
  starGeo.setAttribute('position', new THREE.BufferAttribute(starPos, 3))
  starGeo.setAttribute('color', new THREE.BufferAttribute(starColor, 3))
  stars = new THREE.Points(starGeo, new THREE.PointsMaterial({
    size: 0.15, vertexColors: true, transparent: true, opacity: 0.7,
    blending: THREE.AdditiveBlending, depthWrite: false, sizeAttenuation: true,
  }))
  scene.add(stars)

  // ── 5. 远景能量网格 ──
  const grid = new THREE.GridHelper(80, 40, 0xF5A623, 0x1a1a20)
  grid.position.y = -4
  grid.material.opacity = 0.08
  grid.material.transparent = true
  scene.add(grid)
}

function applyTheme3D() {
  if (!scene) return
  scene.fog = new THREE.FogExp2(isLight() ? 0xF6F0E2 : 0x0A0A0D, 0.025)
}

function onResize() {
  if (!renderer || !camera) return
  camera.aspect = window.innerWidth / window.innerHeight
  camera.updateProjectionMatrix()
  renderer.setSize(window.innerWidth, window.innerHeight)
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
}

function animate() {
  if (disposed) return
  rafId = requestAnimationFrame(animate)
  const dt = Math.min(clock.getDelta(), 0.1)
  const time = clock.elapsedTime

  // 情绪值平滑过渡（朝 API 目标收敛）
  for (const k of Object.keys(S.target)) {
    S[k] += (S.target[k] - S[k]) * 0.02
  }

  // 核心脉动：活跃度驱动频率
  const vitality = (S.joy + S.curious + S.empathy) / 3 - S.stress * 0.3
  const pulse = 1 + Math.sin(time * (1.5 + vitality * 2)) * 0.04 * Math.max(vitality, 0.1)
  coreGroup.scale.setScalar(pulse)
  coreGroup.rotation.y += dt * 0.15
  coreGroup.rotation.x += dt * 0.05

  // 表面顶点扰动：压力驱动强度
  const posAttr = wireCore.geometry.attributes.position
  const arr = posAttr.array
  const noiseAmp = 0.08 * (0.5 + S.stress)
  for (let i = 0; i < arr.length; i += 3) {
    const ox = originalPositions[i], oy = originalPositions[i + 1], oz = originalPositions[i + 2]
    const noise = Math.sin(ox * 3 + time * 2) * Math.cos(oy * 3 + time * 1.7) * Math.sin(oz * 3 + time * 1.3)
    const d = 1 + noise * noiseAmp
    arr[i] = ox * d; arr[i + 1] = oy * d; arr[i + 2] = oz * d
  }
  posAttr.needsUpdate = true

  // 颜色随愉悦度：深金 → 亮金
  const currentGold = new THREE.Color(0xB87A12).lerp(new THREE.Color(0xFFC85C), S.joy)
  innerCore.material.color.copy(currentGold)
  glowCore.material.color.copy(currentGold)
  wireCore.material.color.copy(currentGold)
  coreLight.color.copy(currentGold)
  coreLight.intensity = 1.5 + S.joy * 1.5

  // 神经元亮度随压力升高而降低
  neurons.material.opacity = 0.5 + (1 - S.stress) * 0.5
  neurons.rotation.y += 0.001

  // 中心亮点呼吸
  centerGlow.scale.setScalar(1 + Math.sin(time * (2 + vitality * 3)) * 0.2)

  // 轨道环：旋转速度/透明度/大小由对应情绪值驱动
  for (const ring of orbitRings) {
    const v = S[ring.emoKey] ?? 0.5
    ring.mesh.rotation.y = ring.baseRotation.y + time * ring.speed * (0.3 + v)
    ring.mesh.rotation.x = ring.baseRotation.x + Math.sin(time * 0.4 + ring.phase) * 0.15
    const s = 0.9 + v * 0.25
    ring.mesh.scale.set(s, s, s)
    ring.mesh.material.opacity = 0.15 + v * 0.6
  }

  // 数据流粒子：核心 → 轨道
  const fp = flowPoints.geometry.attributes.position.array
  for (let i = 0; i < flowData.length; i++) {
    const fd = flowData[i]
    fd.t += dt * fd.speed
    if (fd.t > 1) fd.t -= 1
    const v = S[fd.emoKey ?? emoDefs[fd.emoIdx].key] ?? 0.5
    const progress = fd.t * (0.5 + v * 0.8)
    const eased = 1 - Math.pow(1 - progress, 2)
    const angle = fd.startAngle + time * 0.3
    fp[i * 3] = Math.cos(angle) * fd.radius * eased
    fp[i * 3 + 1] = Math.sin(progress * Math.PI) * 0.5 * (1 + Math.sin(time + fd.offset) * 0.3)
    fp[i * 3 + 2] = Math.sin(angle) * fd.radius * eased
  }
  flowPoints.geometry.attributes.position.needsUpdate = true

  // 星空缓转
  stars.rotation.y += dt * 0.005

  // 相机：缓慢浮动 + 鼠标拖拽轨道
  const r = 9
  const th = camTheta + Math.sin(time * 0.1) * 0.06
  const ph = clamp(camPhi + Math.cos(time * 0.15) * 0.04, 0.3, Math.PI - 0.3)
  camera.position.set(
    r * Math.sin(ph) * Math.sin(th),
    r * Math.cos(ph),
    r * Math.sin(ph) * Math.cos(th)
  )
  camera.lookAt(0, 0, 0)

  renderer.render(scene, camera)
}

// ── 拖拽旋转（在核心舞台上，鼠标/触摸水平拖动） ──
function onPointerDown(e) {
  if (e.pointerType !== 'mouse' && e.pointerType !== 'touch') return
  if (e.pointerType === 'mouse' && e.button !== 0) return
  dragging = true
  lastX = e.clientX; lastY = e.clientY
  e.currentTarget.setPointerCapture?.(e.pointerId)
}
function onPointerMove(e) {
  if (!dragging) return
  const dx = e.clientX - lastX, dy = e.clientY - lastY
  lastX = e.clientX; lastY = e.clientY
  camTheta -= dx * 0.006
  camPhi = clamp(camPhi - dy * 0.006, 0.35, Math.PI - 0.35)
}
function onPointerUp() { dragging = false }

// ── 生命周期 ──
onMounted(async () => {
  initThree()
  onResize()
  animate()

  const stage = stageEl.value
  stage?.addEventListener('pointerdown', onPointerDown)
  stage?.addEventListener('pointermove', onPointerMove)
  stage?.addEventListener('pointerup', onPointerUp)
  stage?.addEventListener('pointercancel', onPointerUp)
  window.addEventListener('resize', onResize)

  themeObserver = new MutationObserver(applyTheme3D)
  themeObserver.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] })

  await loadCore()
  loadEvents()
  pollTimer = setInterval(loadCore, 10000)
  eventTimer = setInterval(loadEvents, 15000)

  // UI 更新节流到 10fps，避免频繁回流
  uiTimer = setInterval(() => {
    ui.joy = S.joy; ui.curious = S.curious; ui.empathy = S.empathy; ui.stress = S.stress
  }, 100)
})

onUnmounted(() => {
  disposed = true
  cancelAnimationFrame(rafId)
  clearInterval(uiTimer); clearInterval(pollTimer); clearInterval(eventTimer)
  const stage = stageEl.value
  stage?.removeEventListener('pointerdown', onPointerDown)
  stage?.removeEventListener('pointermove', onPointerMove)
  stage?.removeEventListener('pointerup', onPointerUp)
  stage?.removeEventListener('pointercancel', onPointerUp)
  window.removeEventListener('resize', onResize)
  themeObserver?.disconnect()

  if (scene) {
    scene.traverse(obj => {
      obj.geometry?.dispose?.()
      if (Array.isArray(obj.material)) obj.material.forEach(m => m.dispose?.())
      else obj.material?.dispose?.()
    })
  }
  renderer?.dispose()
  canvas?.remove()
  renderer = scene = camera = canvas = null
})
</script>

<style scoped>
/* ── 控制总览布局：1fr + 340px ── */
.ov-layout {
  display: grid;
  grid-template-columns: 1fr 340px;
  gap: 20px;
  height: 100%;
  min-height: 480px;
}

/* 透明核心舞台：让全屏 3D 背景透出 */
.core-stage {
  position: relative;
  border-radius: 14px;
  overflow: hidden;
  background: radial-gradient(ellipse at center, rgba(245, 166, 35, 0.04), transparent 65%);
  border: 1px solid var(--border);
  cursor: grab;
  user-select: none;
  touch-action: pan-y;
}
.core-stage:active { cursor: grabbing; }
.core-stage-overlay {
  position: absolute;
  inset: 0;
  pointer-events: none;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 22px 24px;
}
.core-stage-label {
  font-size: 10.5px;
  letter-spacing: 2.5px;
  text-transform: uppercase;
  color: rgba(245, 166, 35, 0.7);
  font-family: var(--font-mono);
  display: flex;
  align-items: center;
  gap: 10px;
}
.core-stage-label::before {
  content: '';
  width: 24px; height: 1px;
  background: var(--gold);
  opacity: 0.6;
}
.core-readout { display: flex; gap: 28px; }
.core-readout-item { display: flex; flex-direction: column; gap: 4px; }
.core-readout-item .lbl {
  font-size: 10px;
  letter-spacing: 1.5px;
  text-transform: uppercase;
  color: var(--text-3);
  font-family: var(--font-mono);
}
.core-readout-item .val {
  font-size: 20px;
  font-weight: 300;
  font-family: var(--font-mono);
  font-variant-numeric: tabular-nums;
  color: var(--gold);
  letter-spacing: 0.5px;
}

/* 浮动大字：洛玖 + 状态 */
.core-name {
  text-align: center;
  position: absolute;
  bottom: 24%;
  left: 0; right: 0;
  pointer-events: none;
}
.core-name .cn-title {
  font-size: 40px;
  font-weight: 300;
  letter-spacing: 14px;
  text-indent: 14px;
  color: var(--text-0, #F2EFE8);
  text-shadow: 0 0 30px rgba(245, 166, 35, 0.55);
}
[data-theme="light"] .core-name .cn-title { color: var(--text); text-shadow: 0 0 24px rgba(245, 166, 35, 0.35); }
.core-name .cn-sub {
  font-size: 10.5px;
  letter-spacing: 4px;
  color: var(--gold);
  text-transform: uppercase;
  margin-top: 8px;
  font-family: var(--font-mono);
  opacity: 0.85;
}

/* ── 右侧面板 ── */
.ov-right { display: flex; flex-direction: column; gap: 20px; }
.glass {
  background: rgba(22, 22, 29, 0.55);
  backdrop-filter: blur(20px) saturate(140%);
  -webkit-backdrop-filter: blur(20px) saturate(140%);
  border: 1px solid var(--border);
  border-radius: 14px;
  position: relative;
  overflow: hidden;
}
[data-theme="light"] .glass { background: rgba(253, 249, 239, 0.7); border-color: var(--glass-border); }
.glass::before {
  content: '';
  position: absolute;
  top: 0; left: 0; right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, rgba(245, 166, 35, 0.4), transparent);
}
.ov-panel { padding: 20px 22px; }
.events-panel { flex: 1; min-height: 200px; display: flex; flex-direction: column; }
.panel-title {
  font-size: 10.5px;
  letter-spacing: 2px;
  text-transform: uppercase;
  color: var(--text-2);
  font-family: var(--font-mono);
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 18px;
}
.panel-title::before {
  content: '';
  width: 3px; height: 12px;
  background: var(--gold);
  border-radius: 3px;
  box-shadow: 0 0 8px var(--gold);
}
.panel-title .pt-right {
  margin-left: auto;
  font-size: 10px;
  letter-spacing: 0.5px;
  color: var(--text-3);
  text-transform: none;
}

/* 情绪频谱条 */
.emo-row { margin-bottom: 15px; }
.emo-row:last-child { margin-bottom: 0; }
.emo-head {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: 7px;
}
.emo-name { font-size: 12px; color: var(--text-1); display: flex; align-items: center; gap: 8px; }
.emo-name .color-dot { width: 6px; height: 6px; border-radius: 2px; }
.emo-val {
  font-size: 13px;
  font-weight: 600;
  font-family: var(--font-mono);
  font-variant-numeric: tabular-nums;
  color: var(--gold);
}
.emo-track {
  height: 3px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 3px;
  overflow: hidden;
  position: relative;
}
[data-theme="light"] .emo-track { background: rgba(154, 111, 27, 0.12); }
.emo-fill {
  height: 100%;
  border-radius: 3px;
  position: relative;
  transition: width 0.6s cubic-bezier(0.4, 0, 0.2, 1);
}
.emo-fill::after {
  content: '';
  position: absolute;
  right: 0; top: 50%;
  transform: translateY(-50%);
  width: 6px; height: 6px;
  border-radius: 50%;
  background: currentColor;
  box-shadow: 0 0 8px currentColor;
}

/* 事件流 */
.event-stream {
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  flex: 1;
  max-height: 320px;
}
.event-line {
  display: flex;
  gap: 12px;
  padding: 9px 0;
  border-bottom: 1px solid rgba(245, 166, 35, 0.05);
  font-size: 12px;
  line-height: 1.45;
  color: var(--text-1);
}
.event-line:last-child { border-bottom: none; }
.event-time {
  font-size: 10.5px;
  color: var(--text-3);
  font-family: var(--font-mono);
  flex-shrink: 0;
  padding-top: 2px;
  min-width: 42px;
}
.event-text { min-width: 0; word-break: break-all; }
.event-text .hl { color: var(--gold); margin-right: 4px; }
.event-empty {
  font-size: 12px;
  color: var(--text-3);
  text-align: center;
  padding: 28px 0;
  font-family: var(--font-mono);
  letter-spacing: 1px;
}

/* ── 响应式 ── */
@media (max-width: 1180px) {
  .ov-layout { grid-template-columns: 1fr; height: auto; }
  .core-stage { min-height: 420px; }
  .events-panel { min-height: unset; }
  .event-stream { max-height: 260px; }
}
</style>
