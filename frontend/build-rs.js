import { readFileSync, writeFileSync } from 'fs'
import { resolve, dirname } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))
// Rust raw string 不允许裸 CR：统一归一成 LF（CRLF 行尾的 index.html 会带 \r 进来）
const html = readFileSync(resolve(__dirname, 'dist/index.html'), 'utf-8')
  .replace(/\r\n/g, '\n')
  .replace(/\r/g, '')

// 动态选择 raw string 的 # 数量，避免 HTML 内容出现 "## 提前终止
let hashes = '##'
while (html.includes(`"${hashes}`)) hashes += '#'
const rs = `pub const HTML: &str = r${hashes}"${html}"${hashes};\n`
writeFileSync(resolve(__dirname, '../src/admin/ui.rs'), rs)
console.log(`Generated admin_ui.rs (${(rs.length / 1024).toFixed(1)} KB)`)
