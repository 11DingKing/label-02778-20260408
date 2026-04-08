import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  { path: '/login', name: 'Login', component: () => import('../views/Login.vue') },
  {
    path: '/',
    component: () => import('../views/Layout.vue'),
    meta: { auth: true },
    children: [
      { path: '', name: 'Dashboard', component: () => import('../views/Dashboard.vue') },
      { path: 'users', name: 'Users', component: () => import('../views/Users.vue') },
      { path: 'enterprises', name: 'Enterprises', component: () => import('../views/Enterprises.vue') },
      { path: 'logs', name: 'Logs', component: () => import('../views/Logs.vue') },
      { path: 'market', name: 'MarketManage', component: () => import('../views/MarketManage.vue') },
      { path: 'jianghu', name: 'JianghuManage', component: () => import('../views/JianghuManage.vue') },
      { path: 'jobs', name: 'JobManage', component: () => import('../views/JobManage.vue') }
    ]
  }
]

const router = createRouter({ history: createWebHistory(), routes })

router.beforeEach((to, from, next) => {
  const token = localStorage.getItem('admin_token')
  if (to.meta.auth && !token) next('/login')
  else next()
})

export default router
