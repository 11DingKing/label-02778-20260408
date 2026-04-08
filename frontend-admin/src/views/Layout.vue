<template>
  <el-container class="admin-layout">
    <el-aside width="240px" class="sidebar">
      <div class="sidebar-header">
        <div class="logo-badge">
          <el-icon :size="20"><Monitor /></el-icon>
        </div>
        <span>管理后台</span>
      </div>
      <el-menu
        :default-active="$route.path"
        router
        class="sidebar-menu"
        background-color="#0F172A"
        text-color="#94A3B8"
        active-text-color="#fff"
      >
        <el-menu-item index="/">
          <el-icon><DataAnalysis /></el-icon>
          <span>数据概览</span>
        </el-menu-item>
        <el-menu-item index="/users">
          <el-icon><User /></el-icon>
          <span>用户管理</span>
        </el-menu-item>
        <el-menu-item index="/enterprises">
          <el-icon><OfficeBuilding /></el-icon>
          <span>企业审核</span>
        </el-menu-item>
        <el-menu-item index="/logs">
          <el-icon><Document /></el-icon>
          <span>操作日志</span>
        </el-menu-item>
        <el-menu-item index="/market">
          <el-icon><Shop /></el-icon>
          <span>好市场管理</span>
        </el-menu-item>
        <el-menu-item index="/jianghu">
          <el-icon><ChatDotRound /></el-icon>
          <span>江湖说管理</span>
        </el-menu-item>
        <el-menu-item index="/jobs">
          <el-icon><Briefcase /></el-icon>
          <span>招聘管理</span>
        </el-menu-item>
      </el-menu>
      <div class="sidebar-footer">
        <div class="version-tag">v1.0.0</div>
      </div>
    </el-aside>
    <el-container>
      <el-header class="top-bar">
        <div class="breadcrumb">
          <span class="page-name">{{ currentPageName }}</span>
        </div>
        <el-dropdown trigger="click" @command="handleCommand">
          <span class="admin-user">
            <el-avatar :size="34" class="admin-avatar">A</el-avatar>
            <span class="admin-name">{{ userStore.user?.username }}</span>
            <el-icon class="arrow"><ArrowDown /></el-icon>
          </span>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="logout">
                <el-icon><SwitchButton /></el-icon> 退出登录
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </el-header>
      <el-main class="main-area">
        <router-view v-slot="{ Component }">
          <transition name="fade-slide" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup>
import { computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useUserStore } from '../store/user'
import { Monitor, DataAnalysis, User, OfficeBuilding, Document, ArrowDown, SwitchButton, Shop, ChatDotRound, Briefcase } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore()

const pageNames = { '/': '数据概览', '/users': '用户管理', '/enterprises': '企业审核', '/logs': '操作日志', '/market': '好市场管理', '/jianghu': '江湖说管理', '/jobs': '招聘管理' }
const currentPageName = computed(() => pageNames[route.path] || '')

function handleCommand(cmd) {
  if (cmd === 'logout') {
    userStore.logout()
    ElMessage({ message: '已安全退出', type: 'success', duration: 2000, showClose: true })
    router.push('/login')
  }
}
</script>

<style lang="scss" scoped>
.admin-layout { height: 100vh; }

.sidebar {
  background: #0F172A;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  border-right: 1px solid #1E293B;

  .sidebar-header {
    height: 64px;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 0 24px;
    color: #fff;
    font-size: 18px;
    font-weight: 700;
    border-bottom: 1px solid #1E293B;
    letter-spacing: -0.3px;

    .logo-badge {
      width: 34px;
      height: 34px;
      border-radius: 10px;
      background: linear-gradient(135deg, #2563EB, #7C3AED);
      display: flex;
      align-items: center;
      justify-content: center;
      color: #fff;
    }
  }

  .sidebar-menu {
    border-right: none;
    flex: 1;
    padding: 8px;

    .el-menu-item {
      border-radius: 10px;
      margin-bottom: 4px;
      height: 48px;
      line-height: 48px;
      font-size: 14px;
      font-weight: 500;
      transition: all 0.25s;

      &:hover {
        background: rgba(255, 255, 255, 0.06) !important;
      }

      &.is-active {
        background: linear-gradient(135deg, #2563EB, #3B82F6) !important;
        color: #fff !important;
        box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
      }
    }
  }

  .sidebar-footer {
    padding: 16px 24px;
    border-top: 1px solid #1E293B;

    .version-tag {
      font-size: 11px;
      color: #475569;
      text-align: center;
    }
  }
}

.top-bar {
  background: #fff;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 28px;
  height: 64px;
  border-bottom: 1px solid var(--border-light);

  .breadcrumb {
    .page-name {
      font-size: 16px;
      font-weight: 600;
      color: var(--text-primary);
    }
  }

  .admin-user {
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    padding: 6px 10px 6px 6px;
    border-radius: 24px;
    transition: all 0.2s;

    &:hover { background: #F3F4F6; }

    .admin-avatar {
      background: linear-gradient(135deg, #1E293B, #475569);
      color: #fff;
      font-weight: 600;
      font-size: 14px;
    }

    .admin-name {
      font-size: 14px;
      color: var(--text-primary);
      font-weight: 500;
    }

    .arrow {
      font-size: 12px;
      color: var(--text-muted);
    }
  }
}

.main-area {
  background: var(--bg);
  padding: 28px;
  overflow-y: auto;
}
</style>
