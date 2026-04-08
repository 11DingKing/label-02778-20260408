<template>
  <div class="layout">
    <header class="top-nav">
      <div class="nav-inner">
        <router-link to="/" class="nav-logo">
          <div class="logo-badge">
            <el-icon :size="20"><Briefcase /></el-icon>
          </div>
          <span>人才网</span>
        </router-link>
        <nav class="nav-links">
          <router-link to="/jobs" class="nav-link">
            <el-icon><Search /></el-icon> 找工作
          </router-link>
          <router-link to="/seekers" class="nav-link">
            <el-icon><User /></el-icon> 找人才
          </router-link>
          <router-link to="/jianghu" class="nav-link">
            <el-icon><ChatDotRound /></el-icon> 江湖说
          </router-link>
          <router-link to="/market" class="nav-link">
            <el-icon><Shop /></el-icon> 好市场
          </router-link>
          <router-link to="/chat" v-if="userStore.isLoggedIn" class="nav-link">
            <el-icon><Message /></el-icon> 消息
          </router-link>
        </nav>
        <div class="nav-right">
          <template v-if="userStore.isLoggedIn">
            <el-dropdown trigger="click" @command="handleCommand">
              <span class="user-trigger">
                <el-avatar :size="34" class="user-avatar">
                  {{ userStore.user?.username?.charAt(0)?.toUpperCase() }}
                </el-avatar>
                <span class="username">{{ userStore.user?.username }}</span>
                <el-icon class="arrow"><ArrowDown /></el-icon>
              </span>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item command="profile">
                    <el-icon><UserFilled /></el-icon> 个人中心
                  </el-dropdown-item>
                  <el-dropdown-item command="logout" divided>
                    <el-icon><SwitchButton /></el-icon> 退出登录
                  </el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </template>
          <template v-else>
            <el-button @click="$router.push('/login')" round>登录</el-button>
            <el-button type="primary" @click="$router.push('/register')" round>注册</el-button>
          </template>
        </div>
      </div>
    </header>
    <main class="main-content">
      <router-view v-slot="{ Component }">
        <transition name="fade-slide" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </main>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router'
import { useUserStore } from '../store/user'
import { Briefcase, Search, User, ChatDotRound, Shop, Message, ArrowDown, UserFilled, SwitchButton } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

const router = useRouter()
const userStore = useUserStore()

function handleCommand(cmd) {
  if (cmd === 'profile') router.push('/profile')
  else if (cmd === 'logout') {
    userStore.logout()
    ElMessage({ message: '已安全退出', type: 'success', duration: 2000, showClose: true })
    router.push('/')
  }
}
</script>

<style lang="scss" scoped>
.layout { min-height: 100vh; display: flex; flex-direction: column; background: var(--bg); }

.top-nav {
  background: rgba(255, 255, 255, 0.92);
  backdrop-filter: blur(20px) saturate(180%);
  -webkit-backdrop-filter: blur(20px) saturate(180%);
  border-bottom: 1px solid rgba(0, 0, 0, 0.04);
  position: sticky;
  top: 0;
  z-index: 100;
  transition: box-shadow 0.3s;

  &:hover {
    box-shadow: 0 2px 20px rgba(0, 0, 0, 0.06);
  }
}

.nav-inner {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
  height: 64px;
  display: flex;
  align-items: center;
  gap: 36px;
}

.nav-logo {
  display: flex;
  align-items: center;
  gap: 10px;
  text-decoration: none;
  color: var(--primary);
  font-size: 20px;
  font-weight: 800;
  letter-spacing: -0.5px;

  .logo-badge {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    background: linear-gradient(135deg, var(--primary), #7C3AED);
    display: flex;
    align-items: center;
    justify-content: center;
    color: #fff;
    transition: transform 0.3s;
  }

  &:hover .logo-badge {
    transform: rotate(-8deg) scale(1.05);
  }
}

.nav-links {
  display: flex;
  gap: 4px;
  flex: 1;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 5px;
  text-decoration: none;
  color: var(--text-secondary);
  font-size: 14px;
  font-weight: 500;
  padding: 8px 14px;
  border-radius: var(--radius-sm);
  transition: var(--transition);
  position: relative;

  &:hover {
    color: var(--primary);
    background: var(--primary-lighter);
  }

  &.router-link-active {
    color: var(--primary);
    background: var(--primary-lighter);
    font-weight: 600;
  }
}

.nav-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.user-trigger {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  padding: 4px 8px 4px 4px;
  border-radius: 24px;
  transition: var(--transition);

  &:hover {
    background: #F3F4F6;
  }

  .user-avatar {
    background: linear-gradient(135deg, var(--primary), #7C3AED);
    color: #fff;
    font-weight: 600;
    font-size: 14px;
  }

  .username {
    font-size: 14px;
    color: var(--text-primary);
    font-weight: 500;
  }

  .arrow {
    font-size: 12px;
    color: var(--text-muted);
    transition: transform 0.2s;
  }
}

.main-content { flex: 1; }
</style>
