import { describe, it, expect, vi } from 'vitest'

vi.mock('../router', () => ({
  default: { push: vi.fn() }
}))

vi.mock('element-plus', () => ({
  ElMessage: { error: vi.fn() }
}))

describe('Admin API module', () => {
  it('authApi exports login and me', async () => {
    const { authApi } = await import('../api')
    expect(typeof authApi.login).toBe('function')
    expect(typeof authApi.me).toBe('function')
  })

  it('adminApi exports all management methods', async () => {
    const { adminApi } = await import('../api')
    const expectedMethods = [
      'stats', 'users', 'toggleUserStatus',
      'enterprises', 'verifyEnterprise',
      'logs', 'market', 'deleteMarket',
      'jianghu', 'deleteJianghu',
      'jobs', 'deleteJob'
    ]
    for (const method of expectedMethods) {
      expect(adminApi).toHaveProperty(method)
      expect(typeof adminApi[method]).toBe('function')
    }
  })
})
