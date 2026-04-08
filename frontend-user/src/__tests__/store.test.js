import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useUserStore } from '../store/user'

// Mock localStorage
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

// Mock the API module
vi.mock('../api', () => ({
  authApi: {
    login: vi.fn(),
    me: vi.fn()
  }
}))

import { authApi } from '../api'

describe('useUserStore', () => {
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

  it('computes isSeeker correctly', () => {
    const store = useUserStore()
    store.user = { role: 0 }
    expect(store.isSeeker).toBe(true)
    expect(store.isEnterprise).toBe(false)
    expect(store.isAdmin).toBe(false)
  })

  it('computes isEnterprise correctly', () => {
    const store = useUserStore()
    store.user = { role: 1 }
    expect(store.isSeeker).toBe(false)
    expect(store.isEnterprise).toBe(true)
    expect(store.isAdmin).toBe(false)
  })

  it('computes isAdmin correctly', () => {
    const store = useUserStore()
    store.user = { role: 2 }
    expect(store.isSeeker).toBe(false)
    expect(store.isEnterprise).toBe(false)
    expect(store.isAdmin).toBe(true)
  })

  it('login sets token and user', async () => {
    const store = useUserStore()
    const mockResp = {
      token: 'test-jwt-token',
      user: { id: 1, username: 'testuser', role: 0 }
    }
    authApi.login.mockResolvedValue(mockResp)

    await store.login({ username: 'testuser', password: '123456' })

    expect(store.token).toBe('test-jwt-token')
    expect(store.user.username).toBe('testuser')
    expect(store.isLoggedIn).toBe(true)
    expect(localStorageMock.setItem).toHaveBeenCalledWith('token', 'test-jwt-token')
  })

  it('logout clears state', () => {
    const store = useUserStore()
    store.token = 'some-token'
    store.user = { id: 1, username: 'test' }

    store.logout()

    expect(store.token).toBe('')
    expect(store.user).toBeNull()
    expect(store.isLoggedIn).toBe(false)
    expect(localStorageMock.removeItem).toHaveBeenCalledWith('token')
    expect(localStorageMock.removeItem).toHaveBeenCalledWith('user')
  })

  it('fetchMe updates user', async () => {
    const store = useUserStore()
    const mockUser = { id: 1, username: 'testuser', role: 0 }
    authApi.me.mockResolvedValue(mockUser)

    await store.fetchMe()

    expect(store.user).toEqual(mockUser)
    expect(localStorageMock.setItem).toHaveBeenCalledWith('user', JSON.stringify(mockUser))
  })
})
