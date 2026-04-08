import axios from 'axios'
import { ElMessage } from 'element-plus'
import router from '../router'

const api = axios.create({ baseURL: '/api', timeout: 15000 })

api.interceptors.request.use(config => {
  const token = localStorage.getItem('admin_token')
  if (token) config.headers.Authorization = `Bearer ${token}`
  return config
})

api.interceptors.response.use(
  res => {
    if (res.data.code !== 0) {
      ElMessage.error(res.data.message || '请求失败')
      return Promise.reject(new Error(res.data.message))
    }
    return res.data.data
  },
  err => {
    if (err.response?.status === 401) {
      localStorage.removeItem('admin_token')
      router.push('/login')
      ElMessage.error('登录已过期')
    } else {
      ElMessage.error(err.response?.data?.message || '网络错误')
    }
    return Promise.reject(err)
  }
)

export const authApi = {
  login: data => api.post('/auth/login', data),
  me: () => api.get('/auth/me')
}

export const adminApi = {
  stats: () => api.get('/admin/stats'),
  users: params => api.get('/admin/users', { params }),
  toggleUserStatus: id => api.put(`/admin/user/${id}/status`),
  enterprises: params => api.get('/admin/enterprises', { params }),
  verifyEnterprise: (id, data) => api.put(`/admin/enterprise/${id}/verify`, data),
  logs: params => api.get('/admin/logs', { params }),
  market: params => api.get('/admin/market', { params }),
  deleteMarket: id => api.delete(`/admin/market/${id}`),
  jianghu: params => api.get('/admin/jianghu', { params }),
  deleteJianghu: id => api.delete(`/admin/jianghu/${id}`),
  jobs: params => api.get('/admin/jobs', { params }),
  deleteJob: id => api.delete(`/admin/job/${id}`)
}

export default api
