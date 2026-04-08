import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'

// Mock router
vi.mock('../router', () => ({
  default: { push: vi.fn() }
}))

// Mock element-plus
vi.mock('element-plus', () => ({
  ElMessage: { error: vi.fn() },
  ElButton: { name: 'ElButton', template: '<button @click="$emit(\'click\')"><slot /></button>', props: ['type', 'round'] },
  ElIcon: { name: 'ElIcon', template: '<i><slot /></i>', props: ['size'] },
  vLoading: { mounted() {}, updated() {} }
}))

// Mock icons
vi.mock('@element-plus/icons-vue', () => ({
  User: { template: '<i />' },
  UserFilled: { template: '<i />' },
  OfficeBuilding: { template: '<i />' },
  Briefcase: { template: '<i />' },
  ChatDotRound: { template: '<i />' },
  Shop: { template: '<i />' },
  Clock: { template: '<i />' },
}))

const mockStats = vi.fn()

vi.mock('../api', () => ({
  adminApi: {
    stats: (...args) => mockStats(...args),
  },
  authApi: {
    login: vi.fn(),
    me: vi.fn()
  }
}))

import Dashboard from '../views/Dashboard.vue'
import { useUserStore } from '../store/user'

const sampleStats = {
  total_users: 10,
  total_seekers: 6,
  total_enterprises: 3,
  total_jobs: 8,
  total_jianghu: 6,
  total_market: 5,
  pending_verify: 1,
}

function mountDashboard() {
  const pinia = createPinia()
  setActivePinia(pinia)
  const store = useUserStore()
  store.token = 'admin-token'
  store.user = { id: 1, username: 'admin', role: 2 }

  return mount(Dashboard, {
    global: {
      plugins: [pinia],
      directives: { loading: () => {} },
      stubs: {
        'el-button': { template: '<button @click="$emit(\'click\')"><slot /></button>', props: ['type', 'round'] },
        'el-icon': { template: '<i><slot /></i>' },
      }
    }
  })
}

describe('Dashboard.vue', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mockStats.mockResolvedValue(sampleStats)
  })

  it('fetches stats on mount', async () => {
    const wrapper = mountDashboard()
    await vi.waitFor(() => {
      expect(mockStats).toHaveBeenCalled()
    })
  })

  it('renders page header', () => {
    const wrapper = mountDashboard()
    expect(wrapper.find('.page-header').exists()).toBe(true)
    expect(wrapper.text()).toContain('数据概览')
  })

  it('renders stat cards grid', () => {
    const wrapper = mountDashboard()
    expect(wrapper.find('.stat-grid').exists()).toBe(true)
    const cards = wrapper.findAll('.stat-card')
    expect(cards.length).toBe(7)
  })

  it('displays stat labels correctly', () => {
    const wrapper = mountDashboard()
    const labels = ['总用户数', '求职者', '企业用户', '待审核企业', '招聘职位', '江湖说动态', '好市场商品']
    for (const label of labels) {
      expect(wrapper.text()).toContain(label)
    }
  })

  it('renders welcome card with username', () => {
    const wrapper = mountDashboard()
    expect(wrapper.find('.welcome-card').exists()).toBe(true)
    expect(wrapper.text()).toContain('admin')
    expect(wrapper.text()).toContain('人才网管理后台')
  })

  it('renders action buttons', () => {
    const wrapper = mountDashboard()
    expect(wrapper.text()).toContain('审核企业')
    expect(wrapper.text()).toContain('管理用户')
  })

  it('displays stats values after load', async () => {
    const wrapper = mountDashboard()
    await vi.waitFor(() => expect(mockStats).toHaveBeenCalled())
    // After stats load, the component should show the values
    await wrapper.vm.$nextTick()
    expect(wrapper.vm.stats).toEqual(sampleStats)
  })
})
