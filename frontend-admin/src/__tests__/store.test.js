import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useUserStore } from '../store/user'

const localStorageMock = (() => {
  let store = {}
  return {
    getItem: vi.fn(key => store[key] || null),
    setItem: vi.fn((key, value) => { store[key] = value }),
    removeItem: vi.fn(key => { delete store[key] }),
    clear: vi.fn(() => { store = {} })
  }
})()
Object.defineProperty(global, 'localStorage', { value: localStorageMock })

vi.mock('../api', () => ({
  authApi: {
    login: vi.fn(),
    me: vi.fn()
  }
}))

import { authApi } from '../api'

describe('useUserStore (admin)', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    localStorageMock.clear()
    vi.clearAllMocks()
  })

  it('initializes with null user and empty token', () => {
    const store = useUserStore()
    expect(store.user).toBeNull()
    expect(store.token).toBe('')
    expect(store.isLoggedIn).toBe(false)
  })

  it('login sets token and user for admin role', async () => {
    const store = useUserStore()
    const mockResp = {
      token: 'admin-jwt-token',
      user: { id: 1, username: 'admin', role: 2 }
    }
    authApi.login.mockResolvedValue(mockResp)

    await store.login({ username: 'admin', password: 'admin123' })

    expect(store.token).toBe('admin-jwt-token')
    expect(store.user.username).toBe('admin')
    expect(store.isLoggedIn).toBe(true)
    expect(localStorageMock.setItem).toHaveBeenCalledWith('admin_token', 'admin-jwt-token')
    expect(localStorageMock.setItem).toHaveBeenCalledWith('admin_user', JSON.stringify(mockResp.user))
  })

  it('login rejects non-admin role', async () => {
    const store = useUserStore()
    authApi.login.mockResolvedValue({
      token: 'user-token',
      user: { id: 2, username: 'seeker', role: 0 }
    })

    await expect(store.login({ username: 'seeker', password: '123456' }))
      .rejects.toThrow('非管理员账号')
    expect(store.token).toBe('')
    expect(store.isLoggedIn).toBe(false)
  })

  it('logout clears state and localStorage', () => {
    const store = useUserStore()
    store.token = 'some-token'
    store.user = { id: 1, username: 'admin' }

    store.logout()

    expect(store.token).toBe('')
    expect(store.user).toBeNull()
    expect(store.isLoggedIn).toBe(false)
    expect(localStorageMock.removeItem).toHaveBeenCalledWith('admin_token')
    expect(localStorageMock.removeItem).toHaveBeenCalledWith('admin_user')
  })
})
