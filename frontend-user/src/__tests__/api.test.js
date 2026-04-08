import { describe, it, expect, beforeEach, vi } from 'vitest'

// Mock router
vi.mock('../router', () => ({
  default: { push: vi.fn() }
}))

// Mock element-plus
vi.mock('element-plus', () => ({
  ElMessage: { error: vi.fn() }
}))

import router from '../router'
import { ElMessage } from 'element-plus'

describe('API module', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    localStorage.clear()
  })

  it('authApi exports expected methods', async () => {
    const { authApi } = await import('../api')
    expect(authApi).toHaveProperty('register')
    expect(authApi).toHaveProperty('login')
    expect(authApi).toHaveProperty('me')
    expect(typeof authApi.register).toBe('function')
    expect(typeof authApi.login).toBe('function')
    expect(typeof authApi.me).toBe('function')
  })

  it('seekerApi exports expected methods', async () => {
    const { seekerApi } = await import('../api')
    expect(seekerApi).toHaveProperty('upsertProfile')
    expect(seekerApi).toHaveProperty('getProfile')
    expect(seekerApi).toHaveProperty('list')
    expect(seekerApi).toHaveProperty('detail')
  })

  it('enterpriseApi exports expected methods', async () => {
    const { enterpriseApi } = await import('../api')
    expect(enterpriseApi).toHaveProperty('upsertProfile')
    expect(enterpriseApi).toHaveProperty('getProfile')
    expect(enterpriseApi).toHaveProperty('createJob')
    expect(enterpriseApi).toHaveProperty('updateJob')
    expect(enterpriseApi).toHaveProperty('deleteJob')
    expect(enterpriseApi).toHaveProperty('myJobs')
  })

  it('chatApi exports expected methods', async () => {
    const { chatApi } = await import('../api')
    expect(chatApi).toHaveProperty('contact')
    expect(chatApi).toHaveProperty('reply')
    expect(chatApi).toHaveProperty('contacts')
    expect(chatApi).toHaveProperty('sendMessage')
    expect(chatApi).toHaveProperty('messages')
    expect(chatApi).toHaveProperty('markRead')
  })

  it('jianghuApi exports expected methods', async () => {
    const { jianghuApi } = await import('../api')
    expect(jianghuApi).toHaveProperty('create')
    expect(jianghuApi).toHaveProperty('list')
    expect(jianghuApi).toHaveProperty('like')
    expect(jianghuApi).toHaveProperty('comment')
    expect(jianghuApi).toHaveProperty('comments')
    expect(jianghuApi).toHaveProperty('delete')
  })

  it('marketApi exports expected methods', async () => {
    const { marketApi } = await import('../api')
    expect(marketApi).toHaveProperty('create')
    expect(marketApi).toHaveProperty('list')
    expect(marketApi).toHaveProperty('detail')
    expect(marketApi).toHaveProperty('update')
    expect(marketApi).toHaveProperty('delete')
  })

  it('uploadApi exports upload method', async () => {
    const { uploadApi } = await import('../api')
    expect(uploadApi).toHaveProperty('upload')
    expect(typeof uploadApi.upload).toBe('function')
  })
})
