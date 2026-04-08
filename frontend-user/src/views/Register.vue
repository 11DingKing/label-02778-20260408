<template>
  <div class="login-page">
    <div class="login-bg-shapes">
      <div class="shape shape-1"></div>
      <div class="shape shape-2"></div>
    </div>
    <div class="login-card">
      <div class="login-header">
        <div class="logo-icon">
          <el-icon :size="36"><UserFilled /></el-icon>
        </div>
        <h1>创建账号</h1>
        <p>选择您的身份，开始使用人才网</p>
      </div>
      <el-form ref="formRef" :model="form" :rules="rules" @submit.prevent="handleRegister" hide-required-asterisk>
        <el-form-item prop="username">
          <el-input v-model="form.username" placeholder="用户名" :prefix-icon="User" size="large" />
        </el-form-item>
        <el-form-item prop="password">
          <el-input v-model="form.password" type="password" placeholder="密码（至少6位）" :prefix-icon="Lock" size="large" show-password />
        </el-form-item>
        <el-form-item prop="phone">
          <el-input v-model="form.phone" placeholder="手机号（选填）" :prefix-icon="Phone" size="large" />
        </el-form-item>
        <el-form-item prop="role">
          <div class="role-selector">
            <div class="role-option" :class="{ active: form.role === 0 }" @click="form.role = 0">
              <el-icon :size="28"><User /></el-icon>
              <span class="role-label">我是求职者</span>
              <span class="role-desc">免费发布求职信息</span>
            </div>
            <div class="role-option" :class="{ active: form.role === 1 }" @click="form.role = 1">
              <el-icon :size="28"><OfficeBuilding /></el-icon>
              <span class="role-label">我是企业</span>
              <span class="role-desc">认证后发布招聘</span>
            </div>
          </div>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" size="large" :loading="loading" @click="handleRegister" round style="width:100%;height:46px;font-size:15px;font-weight:600">
            {{ loading ? '注册中...' : '立即注册' }}
          </el-button>
        </el-form-item>
      </el-form>
      <div class="login-footer">
        已有账号？<router-link to="/login">去登录</router-link>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { authApi } from '../api'
import { User, Lock, Phone, UserFilled, OfficeBuilding } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

const router = useRouter()
const formRef = ref(null)
const loading = ref(false)
const form = ref({ username: '', password: '', phone: '', role: 0 })
const rules = {
  username: [{ required: true, message: '请输入用户名', trigger: 'blur' }, { min: 2, max: 50, message: '长度2-50', trigger: 'blur' }],
  password: [{ required: true, message: '请输入密码', trigger: 'blur' }, { min: 6, message: '至少6位', trigger: 'blur' }]
}

async function handleRegister() {
  await formRef.value?.validate()
  loading.value = true
  try {
    await authApi.register(form.value)
    ElMessage({ message: '注册成功，请登录', type: 'success', duration: 2500, showClose: true })
    router.push('/login')
  } catch (e) {} finally { loading.value = false }
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
  position: absolute; inset: 0; overflow: hidden;
  .shape {
    position: absolute; border-radius: 50%; opacity: 0.08; background: #fff;
  }
  .shape-1 { width: 500px; height: 500px; top: -150px; right: -80px; animation: float 18s ease-in-out infinite; }
  .shape-2 { width: 350px; height: 350px; bottom: -80px; left: -60px; animation: float 14s ease-in-out infinite reverse; }
}

@keyframes float {
  0%, 100% { transform: translate(0, 0) scale(1); }
  50% { transform: translate(20px, -15px) scale(1.03); }
}

.login-card {
  width: 440px;
  background: rgba(255, 255, 255, 0.98);
  border-radius: 20px;
  padding: 44px 40px 36px;
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
  margin-bottom: 32px;
  .logo-icon {
    width: 76px; height: 76px; border-radius: 20px;
    background: linear-gradient(135deg, #10B981, #059669);
    display: flex; align-items: center; justify-content: center;
    margin: 0 auto 20px; color: #fff;
    box-shadow: 0 8px 24px rgba(16, 185, 129, 0.3);
  }
  h1 { font-size: 26px; color: #111827; margin-bottom: 6px; font-weight: 700; }
  p { color: #6B7280; font-size: 14px; }
}

.role-selector {
  display: flex;
  gap: 12px;
  width: 100%;
}

.role-option {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: 18px 12px;
  border: 2px solid #E5E7EB;
  border-radius: 14px;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  color: #6B7280;

  .role-label { font-size: 14px; font-weight: 600; color: #374151; }
  .role-desc { font-size: 11px; color: #9CA3AF; }

  &:hover {
    border-color: #93C5FD;
    background: #F0F7FF;
  }

  &.active {
    border-color: #2563EB;
    background: #EFF6FF;
    color: #2563EB;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
    .role-label { color: #2563EB; }
  }
}

.login-footer {
  text-align: center; color: #6B7280; font-size: 14px; margin-top: 8px;
  a { color: #2563EB; text-decoration: none; font-weight: 500; &:hover { color: #1D4ED8; } }
}
</style>
