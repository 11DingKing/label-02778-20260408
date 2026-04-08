/**
 * 格式化时间为中文格式
 * @param {string} t - ISO 时间字符串
 * @param {boolean} full - 是否包含时分秒
 * @returns {string}
 */
export function formatTime(t, full = false) {
  if (!t) return ''
  const d = new Date(t.replace(' ', 'T'))
  if (isNaN(d.getTime())) return t
  const y = d.getFullYear()
  const m = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  if (!full) return `${y}年${m}月${day}日`
  const h = String(d.getHours()).padStart(2, '0')
  const min = String(d.getMinutes()).padStart(2, '0')
  const s = String(d.getSeconds()).padStart(2, '0')
  return `${y}年${m}月${day}日 ${h}:${min}:${s}`
}

/**
 * 相对时间（几分钟前、几小时前）
 */
export function relativeTime(t) {
  if (!t) return ''
  const d = new Date(t.replace(' ', 'T'))
  if (isNaN(d.getTime())) return t
  const now = Date.now()
  const diff = now - d.getTime()
  const minutes = Math.floor(diff / 60000)
  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  const hours = Math.floor(minutes / 60)
  if (hours < 24) return `${hours}小时前`
  const days = Math.floor(hours / 24)
  if (days < 30) return `${days}天前`
  return formatTime(t)
}
