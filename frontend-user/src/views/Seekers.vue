<template>
  <div class="page-container">
    <h2 class="section-title">找人才</h2>
    <div class="card filter-bar">
      <el-row :gutter="12" align="middle">
        <el-col :span="8">
          <el-input v-model="query.keyword" placeholder="搜索姓名、技能、城市" clearable @keyup.enter="loadData" size="large">
            <template #prefix><el-icon><Search /></el-icon></template>
          </el-input>
        </el-col>
        <el-col :span="4">
          <el-button type="primary" size="large" @click="loadData" round>
            <el-icon><Search /></el-icon> 搜索
          </el-button>
        </el-col>
      </el-row>
    </div>

    <transition-group name="list" tag="div" class="seeker-list">
      <div v-for="item in list" :key="item.id" class="card seeker-card" @click="$router.push(`/seekers/${item.id}`)">
        <div class="seeker-header">
          <div class="seeker-avatar">{{ item.name?.charAt(0) }}</div>
          <div class="seeker-info">
            <h3>{{ item.name }} <span class="age" v-if="item.age">{{ item.age }}岁</span></h3>
            <div class="seeker-sub">
              <span v-if="item.education">{{ item.education }}</span>
              <span v-if="item.experience_years">· {{ item.experience_years }}经验</span>
            </div>
          </div>
          <span class="expected-salary">{{ item.expected_salary || '面议' }}</span>
        </div>
        <div class="seeker-tags">
          <span class="tag tag-blue" v-if="item.expected_city">{{ item.expected_city }}</span>
          <span class="tag tag-orange" v-for="s in (item.skills || '').split(',').filter(Boolean).slice(0, 4)" :key="s">{{ s.trim() }}</span>
        </div>
      </div>
    </transition-group>

    <div v-if="list.length === 0 && !loading" class="empty-state">
      <el-icon><User /></el-icon>
      <p>暂无求职者信息</p>
    </div>

    <div v-loading="loading" v-if="loading" style="min-height:200px"></div>

    <div class="pagination-wrap" v-if="total > query.size">
      <el-pagination background layout="total, prev, pager, next" :total="total" :page-size="query.size" v-model:current-page="query.page" @current-change="loadData" />
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { seekerApi } from '../api'
import { Search, User } from '@element-plus/icons-vue'

const loading = ref(false)
const list = ref([])
const total = ref(0)
const query = ref({ page: 1, size: 10, keyword: '' })

async function loadData() {
  loading.value = true
  try {
    const res = await seekerApi.list(query.value)
    list.value = res.list
    total.value = res.total
  } finally { loading.value = false }
}

onMounted(loadData)
</script>

<style lang="scss" scoped>
.filter-bar { margin-bottom: 20px; }

.list-enter-active { transition: all 0.4s ease; }
.list-enter-from { opacity: 0; transform: translateY(16px); }

.seeker-card {
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border: 1px solid transparent;
  &:hover { border-color: var(--primary-lighter); transform: translateX(4px); }
}

.seeker-header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 12px;
}

.seeker-avatar {
  width: 48px;
  height: 48px;
  border-radius: 14px;
  background: linear-gradient(135deg, #2563EB, #7C3AED);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: 700;
  flex-shrink: 0;
}

.seeker-info {
  flex: 1;
  h3 {
    font-size: 17px;
    font-weight: 600;
    .age { font-size: 13px; color: var(--text-muted); font-weight: 400; margin-left: 6px; }
  }
  .seeker-sub {
    font-size: 13px;
    color: var(--text-secondary);
    margin-top: 2px;
  }
}

.expected-salary {
  font-size: 18px;
  color: #EF4444;
  font-weight: 700;
  white-space: nowrap;
}

.seeker-tags { margin-top: 4px; }

.pagination-wrap {
  display: flex;
  justify-content: center;
  margin-top: 28px;
}
</style>
