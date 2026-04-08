<template>
  <div class="login-page">
    <div class="login-bg-shapes">
      <div class="shape shape-1"></div>
      <div class="shape shape-2"></div>
      <div class="shape shape-3"></div>
    </div>
    <div class="login-card">
      <div class="login-header">
        <div class="logo-icon">
          <el-icon :size="36"><Briefcase /></el-icon>
        </div>
        <h1>欢迎回来</h1>
        <p>登录人才网，开启职业新旅程</p>
      </div>
      <el-form ref="formRef" :model="form" :rules="rules" @submit.prevent="handleLogin" hide-required-asterisk>
        <el-form-item prop="username">
          <el-input v-model="form.username" placeholder="用户名" :prefix-icon="User" size="large" />
        </el-form-item>
        <el-form-item prop="password">
          <el-input v-model="form.password" type="password" placeholder="密码" :prefix-icon="Lock" size="large" show-password @keyup.enter="handleLogin" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" size="large" :loading="loading" @click="handleLogin" round style="width:100%;height:46px;font-size:15px;font-weight:600">
            {{ loading ? '登录中...' : '登 录' }}
          </el-button>
        </el-form-item>
      </el-form>
      <div class="login-footer">
        还没有账号？<router-link to="/register">立即注册</router-link>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '../store/user'
import { User, Lock, Briefcase } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

const router = useRouter()
const userStore = useUserStore()
const formRef = ref(null)
const loading = ref(false)
const form = ref({ username: '', password: '' })
const rules = {
  username: [{ required: true, message: '请输入用户名', trigger: 'blur' }],
  password: [{ required: true, message: '请输入密码', trigger: 'blur' }]
}

async function handleLogin() {
  await formRef.value?.validate()
  loading.value = true
  try {
    await userStore.login(form.value)
    ElMessage({ message: '登录成功，欢迎回来', type: 'success', duration: 2000, showClose: true })
    router.push('/')
  } catch (e) {
    // handled by interceptor
  } finally {
    loading.value = false
  }
}
</script>

<style lang="scss" scoped>
.login-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #0F172A 0%, #1E3A5F 40%, #2563EB 100%);
  position: relative;
  overflow: hidden;
}

.login-bg-shapes {
  position: absolute;
  inset: 0;
  overflow: hidden;

  .shape {
    position: absolute;
    border-radius: 50%;
    opacity: 0.08;
    background: #fff;
  }

  .shape-1 {
    width: 600px; height: 600px;
    top: -200px; right: -100px;
    animation: float 20s ease-in-out infinite;
  }

  .shape-2 {
    width: 400px; height: 400px;
    bottom: -100px; left: -80px;
    animation: float 15s ease-in-out infinite reverse;
  }

  .shape-3 {
    width: 200px; height: 200px;
    top: 40%; left: 20%;
    animation: float 12s ease-in-out infinite;
  }
}

@keyframes float {
  0%, 100% { transform: translate(0, 0) scale(1); }
  33% { transform: translate(30px, -20px) scale(1.05); }
  66% { transform: translate(-20px, 20px) scale(0.95); }
}

.login-card {
  width: 420px;
  background: rgba(255, 255, 255, 0.98);
  backdrop-filter: blur(20px);
  border-radius: 20px;
  padding: 48px 40px 40px;
  box-shadow: 0 24px 80px rgba(0, 0, 0, 0.25);
  position: relative;
  z-index: 1;
  animation: cardIn 0.6s cubic-bezier(0.16, 1, 0.3, 1);
}

@keyframes cardIn {
  from { opacity: 0; transform: translateY(30px) scale(0.96); }
  to { opacity: 1; transform: translateY(0) scale(1); }
}

.login-header {
  text-align: center;
  margin-bottom: 36px;

  .logo-icon {
    width: 76px;
    height: 76px;
    border-radius: 20px;
    background: linear-gradient(135deg, #2563EB, #7C3AED);
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 0 auto 20px;
    color: #fff;
    box-shadow: 0 8px 24px rgba(37, 99, 235, 0.3);
  }

  h1 {
    font-size: 26px;
    color: #111827;
    margin-bottom: 6px;
    font-weight: 700;
    letter-spacing: -0.5px;
  }

  p { color: #6B7280; font-size: 14px; }
}

.login-footer {
  text-align: center;
  color: #6B7280;
  font-size: 14px;
  margin-top: 8px;

  a {
    color: #2563EB;
    text-decoration: none;
    font-weight: 500;
    transition: color 0.2s;
    &:hover { color: #1D4ED8; }
  }
}
</style>
