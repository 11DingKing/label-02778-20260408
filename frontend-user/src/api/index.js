import axios from 'axios'
import { ElMessage } from 'element-plus'
import router from '../router'

const api = axios.create({
  baseURL: '/api',
  timeout: 15000
})

api.interceptors.request.use(config => {
  const token = localStorage.getItem('token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

api.interceptors.response.use(
  res => {
    const data = res.data
    if (data.code !== 0) {
      ElMessage.error(data.message || '请求失败')
      return Promise.reject(new Error(data.message))
    }
    return data.data
  },
  err => {
    if (err.response?.status === 401) {
      localStorage.removeItem('token')
      localStorage.removeItem('user')
      router.push('/login')
      ElMessage.error('登录已过期，请重新登录')
    } else {
      ElMessage.error(err.response?.data?.message || '网络错误')
    }
    return Promise.reject(err)
  }
)

// Auth
export const authApi = {
  register: data => api.post('/auth/register', data),
  login: data => api.post('/auth/login', data),
  me: () => api.get('/auth/me')
}

// Seeker
export const seekerApi = {
  upsertProfile: data => api.post('/seeker/profile', data),
  getProfile: () => api.get('/seeker/profile'),
  list: params => api.get('/seeker/list', { params }),
  detail: id => api.get(`/seeker/${id}`)
}

// Enterprise
export const enterpriseApi = {
  upsertProfile: data => api.post('/enterprise/profile', data),
  getProfile: () => api.get('/enterprise/profile'),
  createJob: data => api.post('/enterprise/job', data),
  updateJob: (id, data) => api.put(`/enterprise/job/${id}`, data),
  deleteJob: id => api.delete(`/enterprise/job/${id}`),
  myJobs: params => api.get('/enterprise/jobs', { params })
}

// Jobs
export const jobApi = {
  list: params => api.get('/jobs', { params }),
  detail: id => api.get(`/jobs/${id}`)
}

// Chat
export const chatApi = {
  contact: data => api.post('/chat/contact', data),
  reply: (id, data) => api.put(`/chat/contact/${id}/reply`, data),
  contacts: () => api.get('/chat/contacts'),
  sendMessage: data => api.post('/chat/message', data),
  messages: contactId => api.get(`/chat/messages/${contactId}`),
  markRead: contactId => api.put(`/chat/messages/${contactId}/read`)
}

// Jianghu
export const jianghuApi = {
  create: data => api.post('/jianghu/post', data),
  list: params => api.get('/jianghu/posts', { params }),
  like: id => api.post(`/jianghu/post/${id}/like`),
  comment: (id, data) => api.post(`/jianghu/post/${id}/comment`, data),
  comments: id => api.get(`/jianghu/post/${id}/comments`),
  delete: id => api.delete(`/jianghu/post/${id}`)
}

// Market
export const marketApi = {
  create: data => api.post('/market/post', data),
  list: params => api.get('/market/posts', { params }),
  detail: id => api.get(`/market/post/${id}`),
  update: (id, data) => api.put(`/market/post/${id}`, data),
  delete: id => api.delete(`/market/post/${id}`)
}

// Upload
export const uploadApi = {
  upload: file => {
    // 前端校验：文件大小不超过 5MB
    if (file.size > 5 * 1024 * 1024) {
      ElMessage.error('文件大小不能超过 5MB')
      return Promise.reject(new Error('文件大小超过限制'))
    }
    // 前端校验：仅允许图片格式
    const allowedTypes = ['image/jpeg', 'image/png', 'image/gif', 'image/webp', 'image/bmp', 'image/tiff']
    if (!allowedTypes.includes(file.type)) {
      ElMessage.error('仅支持 JPG/PNG/GIF/WebP/BMP/TIFF 格式图片')
      return Promise.reject(new Error('不支持的文件格式'))
    }
    const formData = new FormData()
    formData.append('file', file)
    return api.post('/upload', formData, { headers: { 'Content-Type': 'multipart/form-data' } })
  }
}

export default api
