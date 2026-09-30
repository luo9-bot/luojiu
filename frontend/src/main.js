import { createApp } from 'vue'
import App from './App.vue'

const app = createApp(App)
app.mount('#app')

const style = document.createElement('style')
style.textContent = `
  /* ── 黑金圣所 · 高级黑金拟人控制台 ────────────────────────── */
  :root {
    --gold: #F5A623;
    --gold-dim: rgba(245, 166, 35, 0.5);
    --gold-glow: rgba(245, 166, 35, 0.3);
    /* NEXUS 参考稿的命名别名（新旧两套 token 并存） */
    --gold-light: #FFC85C;
    --gold-deep: #B87A12;
    --black: #0A0A0D;
    --text-0: #F2EFE8;
    --text-1: #A8A49B;
    --border-strong: rgba(245, 166, 35, 0.25);
    --font-mono: 'JetBrains Mono', 'Cascadia Code', 'SF Mono', 'Fira Code', Consolas, monospace;
    --font-sans: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;

    /* 亮色 = 金调纸面（不用纯白，保持金色血统） */
    --bg: #F6F0E2;
    --bg-alt: #FBF6EA;
    --surface: rgba(253, 249, 239, 0.82);
    --surface-hover: rgba(255, 252, 244, 0.96);
    --surface-solid: #FDFAF2;
    --glass: rgba(253, 249, 239, 0.7);
    --glass-border: rgba(154, 111, 27, 0.22);
    --glass-shadow: 0 6px 22px rgba(90, 66, 15, 0.10);
    --glass-shadow-lg: 0 12px 34px rgba(90, 66, 15, 0.16);
    --text: #2B230F;
    --text-2: #7C6E4E;
    --text-3: #A99A76;
    --text-0: #2B230F;
    --text-1: #7C6E4E;
    --border-strong: rgba(154, 111, 27, 0.3);
    --primary: #B47A12;
    --primary-hover: #96630C;
    --primary-subtle: rgba(180, 122, 18, 0.10);
    --primary-glow: rgba(180, 122, 18, 0.20);
    --accent: #C15F3C;
    --accent-subtle: rgba(193, 95, 60, 0.10);
    --success: #6E8440;
    --success-subtle: rgba(110, 132, 64, 0.12);
    --warning: #B8862B;
    --warning-subtle: rgba(184, 134, 43, 0.12);
    --danger: #A94438;
    --danger-subtle: rgba(169, 68, 56, 0.10);
    --info: #6F7E8C;
    --info-subtle: rgba(111, 126, 140, 0.12);
    --border: rgba(154, 111, 27, 0.26);
    --border-light: rgba(154, 111, 27, 0.13);
    --chip-solid: rgba(154, 111, 27, 0.06);
    --radius: 14px;
    --radius-sm: 12px;
    --radius-xs: 10px;
    --radius-full: 9999px;
    --transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    --transition-fast: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
  }
  [data-theme="dark"] {
    --bg: #0A0A0D;
    --bg-alt: #0E0E13;
    --surface: rgba(18, 18, 22, 0.75);
    --surface-hover: rgba(24, 24, 29, 0.9);
    --surface-solid: #121216;
    --glass: rgba(18, 18, 22, 0.75);
    --glass-border: rgba(245, 166, 35, 0.25);
    --glass-shadow: 0 8px 28px rgba(0, 0, 0, 0.5);
    --glass-shadow-lg: 0 10px 34px rgba(245, 166, 35, 0.08);
    --text: #EAEef2;
    --text-2: #8A8F98;
    --text-3: #5C616A;
    --text-0: #F2EFE8;
    --text-1: #A8A49B;
    --border-strong: rgba(245, 166, 35, 0.25);
    --primary: #F5A623;
    --primary-hover: #FFB84D;
    --primary-subtle: rgba(245, 166, 35, 0.08);
    --primary-glow: rgba(245, 166, 35, 0.18);
    --accent: #E09520;
    --accent-subtle: rgba(224, 149, 32, 0.10);
    --success: #A8B863;
    --success-subtle: rgba(168, 184, 99, 0.12);
    --warning: #D2A754;
    --warning-subtle: rgba(210, 167, 84, 0.12);
    --danger: #D0684F;
    --danger-subtle: rgba(208, 104, 79, 0.12);
    --info: #93A6C0;
    --info-subtle: rgba(147, 166, 192, 0.12);
    --border: rgba(245, 166, 35, 0.20);
    --border-light: rgba(245, 166, 35, 0.09);
    --chip-solid: rgba(245, 166, 35, 0.06);
  }
  *, *::before, *::after { box-sizing: border-box; }
  body {
    font-family: var(--font-sans);
    background: var(--bg);
    color: var(--text);
    margin: 0;
    line-height: 1.5;
    letter-spacing: 0.02em;
    font-variant-numeric: tabular-nums;
    -webkit-font-smoothing: antialiased;
  }
  /* 深空星云 + 极淡能量网格（固定背景层） */
  body::before, body::after {
    content: '';
    position: fixed;
    inset: 0;
    pointer-events: none;
    z-index: 0;
  }
  body::before {
    background:
      radial-gradient(circle at 30% 40%, rgba(245, 166, 35, 0.035) 0%, transparent 50%),
      radial-gradient(circle at 80% 70%, rgba(245, 166, 35, 0.022) 0%, transparent 40%);
  }
  body::after {
    background-image:
      linear-gradient(rgba(245, 166, 35, 0.02) 1px, transparent 1px),
      linear-gradient(90deg, rgba(245, 166, 35, 0.02) 1px, transparent 1px);
    background-size: 48px 48px;
    opacity: 0.7;
  }
  [data-theme="light"] body::after { opacity: 0.35; }
  input, select, textarea, button { font-family: inherit; }
  ::-webkit-scrollbar { width: 5px; height: 5px; }
  ::-webkit-scrollbar-track { background: transparent; }
  ::-webkit-scrollbar-thumb { background: var(--gold-dim); border-radius: 4px; }
  ::-webkit-scrollbar-thumb:hover { background: var(--gold); }
  ::selection { background: var(--primary); color: #0A0A0D; }
  [data-theme="light"] ::selection { color: #FDFAF2; }
  @keyframes fadeIn { from { opacity: 0; transform: translateY(8px); } to { opacity: 1; transform: translateY(0); } }
  @keyframes slideUp { from { opacity: 0; transform: translateY(16px); } to { opacity: 1; transform: translateY(0); } }
  @keyframes pulseDot { 0%, 100% { opacity: 1; transform: scale(1); } 50% { opacity: 0.45; transform: scale(0.75); } }

  /* ── 玻璃拟态卡片：半透明 + 模糊 + 顶部金色渐变线 + 细金边 ── */
  .card {
    position: relative;
    padding: 18px;
    border-radius: var(--radius);
    background: var(--surface);
    backdrop-filter: blur(14px);
    -webkit-backdrop-filter: blur(14px);
    border: 1px solid var(--glass-border);
    box-shadow: var(--glass-shadow);
    overflow: hidden;
    transition: var(--transition);
    margin-bottom: 16px;
    z-index: 1;
  }
  .card::before {
    content: '';
    position: absolute;
    top: 0; left: 0; right: 0;
    height: 1px;
    background: linear-gradient(90deg, transparent, var(--gold), transparent);
    opacity: 0.7;
  }
  .card:hover { border-color: rgba(245, 166, 35, 0.45); box-shadow: var(--glass-shadow-lg); }
  .card-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 14px; }
  .card-header h3 {
    font-size: 13px; font-weight: 500; display: flex; align-items: center; gap: 8px;
    font-family: var(--font-mono); letter-spacing: 0.14em; text-transform: uppercase;
    color: var(--text);
  }

  /* ── 通用表单元件（金调） ── */
  .btn {
    display: inline-flex; align-items: center; justify-content: center; gap: 6px;
    padding: 7px 14px; border: 1px solid transparent; border-radius: var(--radius-sm);
    font-size: 13px; font-weight: 500; cursor: pointer;
    transition: var(--transition-fast);
  }
  .btn-primary { background: var(--primary); color: #0A0A0D; border-color: var(--primary); }
  [data-theme="light"] .btn-primary { color: #FDFAF2; }
  .btn-primary:hover { background: var(--primary-hover); border-color: var(--primary-hover); }
  .btn-secondary { background: var(--primary-subtle); color: var(--primary); border-color: var(--glass-border); }
  .btn-secondary:hover { background: var(--primary-glow); border-color: var(--gold-dim); }
  .btn-danger { background: var(--danger-subtle); color: var(--danger); border-color: transparent; }
  .btn-danger:hover { background: var(--danger); color: white; }
  .btn-ghost { background: transparent; color: var(--text-2); }
  .btn-ghost:hover { background: var(--primary-subtle); color: var(--primary); }
  .btn-sm { padding: 4px 10px; font-size: 12px; }

  .input {
    padding: 7px 12px; border-radius: var(--radius-sm);
    border: 1px solid var(--border); background: var(--chip-solid);
    color: var(--text); font-size: 13px; outline: none; transition: var(--transition-fast);
  }
  .input:focus { border-color: var(--gold-dim); box-shadow: 0 0 0 3px var(--primary-glow); }
  .input-sm { padding: 4px 8px; font-size: 12px; }
  select.input option { background: var(--surface-solid); color: var(--text); }

  .tag {
    display: inline-flex; align-items: center; gap: 4px;
    padding: 2px 8px; border-radius: var(--radius-full);
    font-size: 11px; font-weight: 500;
    font-family: var(--font-mono); letter-spacing: 0.05em;
  }
  .tag-primary { background: var(--primary-subtle); color: var(--primary); }
  .tag-info { background: var(--info-subtle); color: var(--info); }
  .tag-warning { background: var(--warning-subtle); color: var(--warning); }
  .tag-danger { background: var(--danger-subtle); color: var(--danger); }
  .tag-success { background: var(--success-subtle); color: var(--success); }

  .table { width: 100%; border-collapse: collapse; font-size: 13px; }
  .table th {
    text-align: left; padding: 8px 12px; font-weight: 500; color: var(--text-3);
    border-bottom: 1px solid var(--border); font-size: 11px;
    font-family: var(--font-mono); letter-spacing: 0.12em; text-transform: uppercase;
  }
  .table td { padding: 8px 12px; border-bottom: 1px solid var(--border-light); }
  .table tr:hover td { background: var(--primary-subtle); }

  .empty { text-align: center; padding: 32px; color: var(--text-3); font-size: 13px; }
  .loading { text-align: center; padding: 32px; color: var(--text-3); font-size: 13px; }

  /* 全屏 3D 核心画布（由 CoreView 挂载/卸载） */
  #core-webgl {
    position: fixed;
    inset: 0;
    z-index: 0;
    display: block;
    pointer-events: none;
  }
  [data-theme="light"] #core-webgl { opacity: 0.5; }

  /* ── 精密仪表：大数字一律等宽细字重 ── */
  .stat-value, .balance-number {
    font-family: var(--font-mono) !important;
    font-weight: 300 !important;
    letter-spacing: 0.05em;
    font-variant-numeric: tabular-nums;
  }
  .num {
    font-family: var(--font-mono);
    font-weight: 300;
    letter-spacing: 0.05em;
    font-variant-numeric: tabular-nums;
  }
  .gold-text { color: var(--gold); }
`
document.head.appendChild(style)
