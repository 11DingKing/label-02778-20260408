<template>
  <div class="login-page">
    <div class="login-bg-shapes">
      <div class="shape shape-1"></div>
      <div class="shape shape-2"></div>
    </div>
    <div class="login-card">
      <div class="login-header">
        <div class="logo-icon">
          <el-icon :size="36"><Setting /></el-icon>
        </div>
        <h1>管理后台</h1>
        <p>人才网运营管理系统</p>
      </div>
      <el-form ref="formRef" :model="form" :rules="rules" @submit.prevent="handleLogin" hide-required-asterisk>
        <el-form-item prop="username">
          <el-input v-model="form.username" placeholder="管理员账号" :prefix-icon="User" size="large" />
        </el-form-item>
        <el-form-item prop="password">
          <el-input v-model="form.password" type="password" placeholder="管理员密码" :prefix-icon="Lock" size="large" show-password @keyup.enter="handleLogin" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" size="large" :loading="loading" @click="handleLogin" round style="width:100%;height:46px;font-size:15px;font-weight:600">
            {{ loading ? '登录中...' : '登 录' }}
          </el-button>
        </el-form-item>
      </el-form>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '../store/user'
import { User, Lock, Setting } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

const router = useRouter()
const userStore = useUserStore()
const formRef = ref(null)
const loading = ref(false)
const form = ref({ username: '', password: '' })
const rules = {
  username: [{ required: true, message: '请输入账号', trigger: 'blur' }],
  password: [{ required: true, message: '请输入密码', trigger: 'blur' }]
}

async function handleLogin() {
  await formRef.value?.validate()
  loading.value = true
  try {
    await userStore.login(form.value)
    ElMessage({ message: '登录成功', type: 'success', duration: 2000, showClose: true })
    router.push('/')
  } catch (e) {
    if (e.message === '非管理员账号') ElMessage({ message: '该账号不是管理员', type: 'error', duration: 3000 })
  } finally { loading.value = false }
}
</script>

<style lang="scss" scoped>
.login-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #0F172A 0%, #1E293B 50%, #1E3A5F 100%);
  position: relative;
  overflow: hidden;
}

.login-bg-shapes {
  position: absolute; inset: 0; overflow: hidden;
  .shape {
    position: absolute; border-radius: 50%; opacity: 0.06; background: #fff;
  }
  .shape-1 { width: 500px; height: 500px; top: -180px; right: -80px; animation: float 18s ease-in-out infinite; }
  .shape-2 { width: 350px; height: 350px; bottom: -100px; left: -60px; animation: float 14s ease-in-out infinite reverse; }
}

@keyframes float {
  0%, 100% { transform: translate(0, 0) scale(1); }
  50% { transform: translate(20px, -15px) scale(1.03); }
}

.login-card {
  width: 420px;
  background: rgba(255, 255, 255, 0.98);
  border-radius: 20px;
  padding: 48px 40px 40px;
  box-shadow: 0 24px 80px rgba(0, 0, 0, 0.3);
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
    width: 76px; height: 76px; border-radius: 20px;
    background: linear-gradient(135deg, #1E293B, #475569);
    display: flex; align-items: center; justify-content: center;
    margin: 0 auto 20px; color: #fff;
    box-shadow: 0 8px 24px rgba(30, 41, 59, 0.3);
  }
  h1 { font-size: 26px; color: #111827; margin-bottom: 6px; font-weight: 700; }
  p { color: #6B7280; font-size: 14px; }
}
</style>
