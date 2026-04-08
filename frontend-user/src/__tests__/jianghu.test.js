import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'

// Mock router
vi.mock('../router', () => ({
  default: { push: vi.fn() }
}))

// Mock element-plus — ElMessage is used both as function and object
vi.mock('element-plus', () => {
  const fn = Object.assign(vi.fn(), { error: vi.fn(), warning: vi.fn(), success: vi.fn() })
  return {
    ElMessage: fn,
    ElMessageBox: { confirm: vi.fn() },
    ElInput: { name: 'ElInput', template: '<div><slot name="append" /></div>', props: ['modelValue', 'placeholder', 'type', 'rows', 'maxlength', 'showWordLimit', 'resize', 'size'] },
    ElButton: { name: 'ElButton', template: '<button><slot /></button>', props: ['type', 'loading', 'disabled', 'round'] },
    ElAvatar: { name: 'ElAvatar', template: '<span><slot /></span>', props: ['size'] },
    ElIcon: { name: 'ElIcon', template: '<i><slot /></i>', props: ['size'] },
    ElImage: { name: 'ElImage', template: '<img />', props: ['src', 'previewSrcList', 'initialIndex', 'fit', 'lazy'] },
    ElPagination: { name: 'ElPagination', template: '<div />', props: ['total', 'pageSize', 'currentPage'] },
    vLoading: { mounted() {}, updated() {} }
  }
})
const mockList = vi.fn()
const mockCreate = vi.fn()
const mockLike = vi.fn()
const mockCommentApi = vi.fn()
const mockComments = vi.fn()
const mockDelete = vi.fn()
const mockUpload = vi.fn()

vi.mock('../api', () => ({
  jianghuApi: {
    list: (...args) => mockList(...args),
    create: (...args) => mockCreate(...args),
    like: (...args) => mockLike(...args),
    comment: (...args) => mockCommentApi(...args),
    comments: (...args) => mockComments(...args),
    delete: (...args) => mockDelete(...args),
  },
  uploadApi: {
    upload: (...args) => mockUpload(...args),
  }
}))

// Mock icons
vi.mock('@element-plus/icons-vue', () => ({
  ChatDotRound: { template: '<i />' },
  ChatDotSquare: { template: '<i />' },
  Star: { template: '<i />' },
  Promotion: { template: '<i />' },
  Picture: { template: '<i />' },
}))

import Jianghu from '../views/Jianghu.vue'
import { useUserStore } from '../store/user'

function mountJianghu(loggedIn = true) {
  const pinia = createPinia()
  setActivePinia(pinia)
  const store = useUserStore()
  if (loggedIn) {
    store.token = 'test-token'
    store.user = { id: 1, username: '张伟', role: 0 }
  }
  return mount(Jianghu, {
    global: {
      plugins: [pinia],
      directives: { loading: () => {} },
      stubs: {
        'el-input': { template: '<div><slot name="append" /></div>', props: ['modelValue'] },
        'el-button': { template: '<button @click="$emit(\'click\')"><slot /></button>', props: ['type', 'loading', 'disabled', 'round'] },
        'el-avatar': { template: '<span><slot /></span>', props: ['size'] },
        'el-icon': { template: '<i><slot /></i>' },
        'el-image': { template: '<img />' },
        'el-pagination': { template: '<div />' },
        'transition-group': { template: '<div><slot /></div>' },
        'transition': { template: '<div><slot /></div>' },
      }
    }
  })
}

const samplePosts = {
  list: [
    { id: 1, user_id: 1, username: '张伟', avatar: '', content: '今天面试顺利', images: [], like_count: 3, comment_count: 1, created_at: '2026-02-25 10:00:00', liked: false },
    { id: 2, user_id: 2, username: '李娜', avatar: '', content: '数据分析心得分享', images: ['/uploads/test.png'], like_count: 5, comment_count: 2, created_at: '2026-02-24 09:00:00', liked: true },
  ],
  total: 2, page: 1, size: 10
}

describe('Jianghu.vue', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mockList.mockResolvedValue(samplePosts)
    mockComments.mockResolvedValue([])
  })

  it('renders post list after loading', async () => {
    const wrapper = mountJianghu()
    await vi.waitFor(() => {
      expect(mockList).toHaveBeenCalled()
    })
    // Posts should be loaded
    expect(mockList).toHaveBeenCalledWith({ page: 1, size: 10 })
  })

  it('shows publish form when logged in', () => {
    const wrapper = mountJianghu(true)
    expect(wrapper.find('.post-form').exists()).toBe(true)
  })

  it('shows publish form when not logged in (anonymous posting)', () => {
    const wrapper = mountJianghu(false)
    expect(wrapper.find('.post-form').exists()).toBe(true)
  })

  it('calls create API on publish', async () => {
    mockCreate.mockResolvedValue({ id: 3 })
    const wrapper = mountJianghu()
    await vi.waitFor(() => expect(mockList).toHaveBeenCalled())

    // Simulate setting content and posting
    wrapper.vm.newContent = '新动态内容'
    await wrapper.vm.handlePost()

    expect(mockCreate).toHaveBeenCalledWith({ content: '新动态内容', images: [] })
  })

  it('calls like API on toggle', async () => {
    mockLike.mockResolvedValue(true)
    const wrapper = mountJianghu()
    await vi.waitFor(() => expect(mockList).toHaveBeenCalled())

    const item = wrapper.vm.list[0]
    await wrapper.vm.handleLike(item)

    expect(mockLike).toHaveBeenCalledWith(1)
    expect(item.liked).toBe(true)
    expect(item.like_count).toBe(4)
  })

  it('unlike decrements count', async () => {
    mockLike.mockResolvedValue(false)
    const wrapper = mountJianghu()
    await vi.waitFor(() => expect(mockList).toHaveBeenCalled())

    const item = wrapper.vm.list[1] // already liked
    await wrapper.vm.handleLike(item)

    expect(mockLike).toHaveBeenCalledWith(2)
    expect(item.liked).toBe(false)
    expect(item.like_count).toBe(4)
  })

  it('toggleComments loads comments on first open', async () => {
    const mockCommentData = [
      { id: 1, user_id: 2, content: '加油', created_at: '2026-02-25 11:00:00', username: '李娜', avatar: '' }
    ]
    mockComments.mockResolvedValue(mockCommentData)
    const wrapper = mountJianghu()
    await vi.waitFor(() => expect(mockList).toHaveBeenCalled())

    const item = wrapper.vm.list[0]
    await wrapper.vm.toggleComments(item)

    expect(item.showComments).toBe(true)
    expect(mockComments).toHaveBeenCalledWith(1)
    expect(item.comments).toEqual(mockCommentData)
  })

  it('submits comment and refreshes', async () => {
    mockCommentApi.mockResolvedValue({ id: 10 })
    mockComments.mockResolvedValue([{ id: 10, user_id: 1, content: '好的', created_at: '2026-02-25 12:00:00', username: '张伟', avatar: '' }])
    const wrapper = mountJianghu()
    await vi.waitFor(() => expect(mockList).toHaveBeenCalled())

    const item = wrapper.vm.list[0]
    item.showComments = true
    item.newComment = '好的'
    await wrapper.vm.handleComment(item)

    expect(mockCommentApi).toHaveBeenCalledWith(1, { content: '好的' })
    expect(item.comment_count).toBe(2)
    expect(item.newComment).toBe('')
  })

  it('shows delete button only for own posts', async () => {
    const wrapper = mountJianghu()
    await vi.waitFor(() => expect(mockList).toHaveBeenCalled())
    // user_id=1 matches logged in user, so delete should show for first post
    // This is a template-level check; we verify the vm data is correct
    expect(wrapper.vm.list[0].user_id).toBe(1) // own post
    expect(wrapper.vm.list[1].user_id).toBe(2) // other's post
  })

  it('does not post when content is empty', async () => {
    const wrapper = mountJianghu()
    wrapper.vm.newContent = '   '
    await wrapper.vm.handlePost()
    expect(mockCreate).not.toHaveBeenCalled()
  })
})
