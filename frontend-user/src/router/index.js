import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  { path: '/login', name: 'Login', component: () => import('../views/Login.vue'), meta: { guest: true } },
  { path: '/register', name: 'Register', component: () => import('../views/Register.vue'), meta: { guest: true } },
  {
    path: '/',
    component: () => import('../views/Layout.vue'),
    children: [
      { path: '', name: 'Home', component: () => import('../views/Home.vue') },
      { path: 'jobs', name: 'Jobs', component: () => import('../views/Jobs.vue') },
      { path: 'jobs/:id', name: 'JobDetail', component: () => import('../views/JobDetail.vue') },
      { path: 'seekers', name: 'Seekers', component: () => import('../views/Seekers.vue') },
      { path: 'seekers/:id', name: 'SeekerDetail', component: () => import('../views/SeekerDetail.vue') },
      { path: 'jianghu', name: 'Jianghu', component: () => import('../views/Jianghu.vue') },
      { path: 'market', name: 'Market', component: () => import('../views/Market.vue') },
      { path: 'market/:id', name: 'MarketDetail', component: () => import('../views/MarketDetail.vue') },
      { path: 'chat', name: 'Chat', component: () => import('../views/Chat.vue'), meta: { auth: true } },
      { path: 'profile', name: 'Profile', component: () => import('../views/Profile.vue'), meta: { auth: true } },
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

router.beforeEach((to, from, next) => {
  const token = localStorage.getItem('token')
  if (to.meta.auth && !token) {
    next('/login')
  } else {
    next()
  }
})

export default router
