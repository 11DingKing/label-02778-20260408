import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi } from '../api'

export const useUserStore = defineStore('user', () => {
  const user = ref(JSON.parse(localStorage.getItem('user') || 'null'))
  const token = ref(localStorage.getItem('token') || '')

  const isLoggedIn = computed(() => !!token.value)
  const isSeeker = computed(() => user.value?.role === 0)
  const isEnterprise = computed(() => user.value?.role === 1)
  const isAdmin = computed(() => user.value?.role === 2)

  async function login(data) {
    const res = await authApi.login(data)
    token.value = res.token
    user.value = res.user
    localStorage.setItem('token', res.token)
    localStorage.setItem('user', JSON.stringify(res.user))
    return res
  }

  async function fetchMe() {
    const res = await authApi.me()
    user.value = res
    localStorage.setItem('user', JSON.stringify(res))
  }

  function logout() {
    token.value = ''
    user.value = null
    localStorage.removeItem('token')
    localStorage.removeItem('user')
  }

  return { user, token, isLoggedIn, isSeeker, isEnterprise, isAdmin, login, fetchMe, logout }
})
