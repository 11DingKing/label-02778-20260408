<template>
  <div>
    <div class="page-header"><h2>数据概览</h2></div>
    <div class="stat-grid" v-loading="loading">
      <div class="stat-card" v-for="(s, i) in statItems" :key="i" :style="{ animationDelay: `${i * 0.08}s` }">
        <div class="stat-icon" :class="s.color">
          <el-icon :size="24"><component :is="s.icon" /></el-icon>
        </div>
        <div class="stat-body">
          <div class="stat-value">{{ stats[s.key] || 0 }}</div>
          <div class="stat-label">{{ s.label }}</div>
        </div>
      </div>
    </div>

    <div class="welcome-card">
      <div class="welcome-content">
        <h3>欢迎回来，{{ userStore.user?.username }}</h3>
        <p>这里是人才网管理后台，您可以管理用户、审核企业认证、查看操作日志。</p>
      </div>
      <div class="welcome-actions">
        <el-button type="primary" round @click="$router.push('/enterprises')">
          <el-icon><OfficeBuilding /></el-icon> 审核企业
        </el-button>
        <el-button round @click="$router.push('/users')">
          <el-icon><User /></el-icon> 管理用户
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { adminApi } from '../api'
import { useUserStore } from '../store/user'
import { User, OfficeBuilding, Briefcase, ChatDotRound, Shop, Clock, UserFilled } from '@element-plus/icons-vue'

const userStore = useUserStore()
const loading = ref(false)
const stats = ref({})

const statItems = [
  { key: 'total_users', label: '总用户数', icon: 'UserFilled', color: 'blue' },
  { key: 'total_seekers', label: '求职者', icon: 'User', color: 'green' },
  { key: 'total_enterprises', label: '企业用户', icon: 'OfficeBuilding', color: 'purple' },
  { key: 'pending_verify', label: '待审核企业', icon: 'Clock', color: 'orange' },
  { key: 'total_jobs', label: '招聘职位', icon: 'Briefcase', color: 'cyan' },
  { key: 'total_jianghu', label: '江湖说动态', icon: 'ChatDotRound', color: 'pink' },
  { key: 'total_market', label: '好市场商品', icon: 'Shop', color: 'teal' },
]

onMounted(async () => {
  loading.value = true
  try { stats.value = await adminApi.stats() } finally { loading.value = false }
})
</script>

<style lang="scss" scoped>
.stat-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  background: #fff;
  border-radius: 14px;
  padding: 22px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04);
  border: 1px solid #F3F4F6;
  display: flex;
  align-items: center;
  gap: 16px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  animation: cardUp 0.5s cubic-bezier(0.16, 1, 0.3, 1) both;

  &:hover {
    transform: translateY(-3px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08);
  }
}

@keyframes cardUp {
  from { opacity: 0; transform: translateY(12px); }
  to { opacity: 1; transform: translateY(0); }
}

.stat-icon {
  width: 52px;
  height: 52px;
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  flex-shrink: 0;

  &.blue { background: linear-gradient(135deg, #2563EB, #3B82F6); }
  &.green { background: linear-gradient(135deg, #059669, #10B981); }
  &.purple { background: linear-gradient(135deg, #7C3AED, #8B5CF6); }
  &.orange { background: linear-gradient(135deg, #D97706, #F59E0B); }
  &.cyan { background: linear-gradient(135deg, #0891B2, #06B6D4); }
  &.pink { background: linear-gradient(135deg, #DB2777, #EC4899); }
  &.teal { background: linear-gradient(135deg, #0D9488, #14B8A6); }
}

.stat-body {
  .stat-value {
    font-size: 28px;
    font-weight: 800;
    color: #111827;
    letter-spacing: -0.5px;
    line-height: 1.2;
  }
  .stat-label {
    font-size: 13px;
    color: #9CA3AF;
    margin-top: 2px;
    font-weight: 500;
  }
}

.welcome-card {
  background: linear-gradient(135deg, #0F172A, #1E3A5F);
  border-radius: 16px;
  padding: 36px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  color: #fff;
  box-shadow: 0 4px 24px rgba(15, 23, 42, 0.2);

  .welcome-content {
    h3 { font-size: 22px; font-weight: 700; margin-bottom: 8px; }
    p { font-size: 14px; opacity: 0.7; max-width: 480px; line-height: 1.6; }
  }

  .welcome-actions {
    display: flex;
    gap: 12px;
  }
}
</style>
