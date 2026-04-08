import { describe, it, expect } from 'vitest'
import { formatTime } from '../utils/format'

describe('formatTime (admin)', () => {
  it('returns empty string for falsy input', () => {
    expect(formatTime('')).toBe('')
    expect(formatTime(null)).toBe('')
    expect(formatTime(undefined)).toBe('')
  })

  it('formats with full time by default (admin default is full=true)', () => {
    const result = formatTime('2024-01-15 14:30:00')
    expect(result).toBe('2024年01月15日 14:30:00')
  })

  it('formats date only when full=false', () => {
    const result = formatTime('2024-01-15 14:30:00', false)
    expect(result).toBe('2024年01月15日')
  })

  it('handles ISO format', () => {
    const result = formatTime('2024-12-25T10:00:00')
    expect(result).toBe('2024年12月25日 10:00:00')
  })

  it('returns original string for invalid date', () => {
    expect(formatTime('invalid')).toBe('invalid')
  })

  it('pads single digits correctly', () => {
    const result = formatTime('2024-01-05 03:07:09')
    expect(result).toBe('2024年01月05日 03:07:09')
  })
})
