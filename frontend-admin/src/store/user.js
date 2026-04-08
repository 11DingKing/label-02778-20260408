import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi } from '../api'

export const useUserStore = defineStore('adminUser', () => {
  const user = ref(JSON.parse(localStorage.getItem('admin_user') || 'null'))
  const token = ref(localStorage.getItem('admin_token') || '')
  const isLoggedIn = computed(() => !!token.value)

  async function login(data) {
    const res = await authApi.login(data)
    if (res.user.role !== 2) throw new Error('非管理员账号')
    token.value = res.token
    user.value = res.user
    localStorage.setItem('admin_token', res.token)
    localStorage.setItem('admin_user', JSON.stringify(res.user))
    return res
  }

  function logout() {
    token.value = ''
    user.value = null
    localStorage.removeItem('admin_token')
    localStorage.removeItem('admin_user')
  }

  return { user, token, isLoggedIn, login, logout }
})
