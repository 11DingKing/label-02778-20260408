import { describe, it, expect } from 'vitest'
import { formatTime, relativeTime } from '../utils/format'

describe('formatTime', () => {
  it('returns empty string for falsy input', () => {
    expect(formatTime('')).toBe('')
    expect(formatTime(null)).toBe('')
    expect(formatTime(undefined)).toBe('')
  })

  it('formats date without time (default)', () => {
    const result = formatTime('2024-01-15 14:30:00')
    expect(result).toBe('2024年01月15日')
  })

  it('formats date with time when full=true', () => {
    const result = formatTime('2024-01-15 14:30:00', true)
    expect(result).toBe('2024年01月15日 14:30:00')
  })

  it('handles ISO format with T separator', () => {
    const result = formatTime('2024-06-20T09:15:30')
    expect(result).toBe('2024年06月20日')
  })

  it('returns original string for invalid date', () => {
    expect(formatTime('not-a-date')).toBe('not-a-date')
  })

  it('pads single digit months and days', () => {
    const result = formatTime('2024-03-05 08:05:03', true)
    expect(result).toBe('2024年03月05日 08:05:03')
  })
})

describe('relativeTime', () => {
  it('returns empty string for falsy input', () => {
    expect(relativeTime('')).toBe('')
    expect(relativeTime(null)).toBe('')
  })

  it('returns 刚刚 for very recent time', () => {
    const now = new Date()
    const y = now.getFullYear()
    const m = String(now.getMonth() + 1).padStart(2, '0')
    const d = String(now.getDate()).padStart(2, '0')
    const h = String(now.getHours()).padStart(2, '0')
    const min = String(now.getMinutes()).padStart(2, '0')
    const s = String(now.getSeconds()).padStart(2, '0')
    const str = `${y}-${m}-${d} ${h}:${min}:${s}`
    expect(relativeTime(str)).toBe('刚刚')
  })

  it('returns minutes ago', () => {
    const d = new Date(Date.now() - 5 * 60 * 1000)
    const y = d.getFullYear()
    const mo = String(d.getMonth() + 1).padStart(2, '0')
    const day = String(d.getDate()).padStart(2, '0')
    const h = String(d.getHours()).padStart(2, '0')
    const min = String(d.getMinutes()).padStart(2, '0')
    const s = String(d.getSeconds()).padStart(2, '0')
    const str = `${y}-${mo}-${day} ${h}:${min}:${s}`
    const result = relativeTime(str)
    expect(result).toMatch(/\d+分钟前/)
  })

  it('returns hours ago', () => {
    const d = new Date(Date.now() - 3 * 60 * 60 * 1000)
    const y = d.getFullYear()
    const mo = String(d.getMonth() + 1).padStart(2, '0')
    const day = String(d.getDate()).padStart(2, '0')
    const h = String(d.getHours()).padStart(2, '0')
    const min = String(d.getMinutes()).padStart(2, '0')
    const s = String(d.getSeconds()).padStart(2, '0')
    const str = `${y}-${mo}-${day} ${h}:${min}:${s}`
    const result = relativeTime(str)
    expect(result).toMatch(/\d+小时前/)
  })

  it('returns days ago', () => {
    const d = new Date(Date.now() - 5 * 24 * 60 * 60 * 1000)
    const y = d.getFullYear()
    const mo = String(d.getMonth() + 1).padStart(2, '0')
    const day = String(d.getDate()).padStart(2, '0')
    const h = String(d.getHours()).padStart(2, '0')
    const min = String(d.getMinutes()).padStart(2, '0')
    const s = String(d.getSeconds()).padStart(2, '0')
    const str = `${y}-${mo}-${day} ${h}:${min}:${s}`
    const result = relativeTime(str)
    expect(result).toMatch(/\d+天前/)
  })

  it('returns formatted date for old dates', () => {
    const result = relativeTime('2020-01-01 00:00:00')
    expect(result).toBe('2020年01月01日')
  })

  it('returns original string for invalid date', () => {
    expect(relativeTime('invalid')).toBe('invalid')
  })
})
