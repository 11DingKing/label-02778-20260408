<template>
  <div class="page-container">
    <h2 class="section-title">找工作</h2>
    <div class="card filter-bar">
      <el-row :gutter="12" align="middle">
        <el-col :span="8">
          <el-input v-model="query.keyword" placeholder="搜索职位名称、关键词" clearable @clear="loadData" @keyup.enter="loadData" size="large">
            <template #prefix><el-icon><Search /></el-icon></template>
          </el-input>
        </el-col>
        <el-col :span="6">
          <el-input v-model="query.city" placeholder="城市" clearable @clear="loadData" @keyup.enter="loadData" size="large">
            <template #prefix><el-icon><Location /></el-icon></template>
          </el-input>
        </el-col>
        <el-col :span="4">
          <el-button type="primary" size="large" @click="loadData" round>
            <el-icon><Search /></el-icon> 搜索
          </el-button>
        </el-col>
      </el-row>
    </div>

    <transition-group name="list" tag="div" class="job-list">
      <div v-for="item in list" :key="item.id" class="card job-card" @click="$router.push(`/jobs/${item.id}`)">
        <div class="job-header">
          <div class="job-title-row">
            <h3>{{ item.title }}</h3>
            <span class="salary">{{ item.salary_range || '面议' }}</span>
          </div>
          <div class="job-company">{{ item.company_name }}</div>
        </div>
        <div class="job-tags">
          <span class="tag tag-green">{{ item.city || '不限城市' }}</span>
          <span class="tag tag-gray">{{ item.education_req || '不限学历' }}</span>
          <span class="tag tag-gray">{{ item.experience_req || '不限经验' }}</span>
        </div>
        <p class="job-desc" v-if="item.description">{{ item.description.substring(0, 120) }}{{ item.description.length > 120 ? '...' : '' }}</p>
        <div class="job-footer">
          <span class="time">{{ formatTime(item.created_at) }}</span>
        </div>
      </div>
    </transition-group>

    <div v-if="list.length === 0 && !loading" class="empty-state">
      <el-icon><Briefcase /></el-icon>
      <p>暂无职位信息，换个关键词试试</p>
    </div>

    <div v-loading="loading" v-if="loading" style="min-height:200px"></div>

    <div class="pagination-wrap" v-if="total > query.size">
      <el-pagination
        background
        layout="total, prev, pager, next"
        :total="total"
        :page-size="query.size"
        v-model:current-page="query.page"
        @current-change="loadData"
      />
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { jobApi } from '../api'
import { Search, Briefcase, Location } from '@element-plus/icons-vue'
import { formatTime } from '../utils/format'

const loading = ref(false)
const list = ref([])
const total = ref(0)
const query = ref({ page: 1, size: 10, keyword: '', city: '' })

async function loadData() {
  loading.value = true
  try {
    const res = await jobApi.list(query.value)
    list.value = res.list
    total.value = res.total
  } finally { loading.value = false }
}

onMounted(loadData)
</script>

<style lang="scss" scoped>
.filter-bar { margin-bottom: 20px; }

.job-list {
  position: relative;
}

.list-enter-active { transition: all 0.4s ease; }
.list-leave-active { transition: all 0.3s ease; }
.list-enter-from { opacity: 0; transform: translateY(16px); }
.list-leave-to { opacity: 0; transform: translateX(-16px); }

.job-card {
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border: 1px solid transparent;

  &:hover {
    border-color: var(--primary-lighter);
    transform: translateX(4px);
  }
}

.job-header {
  margin-bottom: 12px;

  .job-title-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 4px;

    h3 {
      font-size: 18px;
      color: var(--text-primary);
      font-weight: 600;
      flex: 1;
    }

    .salary {
      font-size: 20px;
      color: #EF4444;
      font-weight: 700;
      white-space: nowrap;
      margin-left: 16px;
    }
  }

  .job-company {
    font-size: 14px;
    color: var(--primary);
    font-weight: 500;
  }
}

.job-tags { margin-bottom: 10px; }

.job-desc {
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.7;
  margin-bottom: 12px;
}

.job-footer {
  display: flex;
  justify-content: flex-end;
  .time { font-size: 12px; color: var(--text-muted); }
}

.pagination-wrap {
  display: flex;
  justify-content: center;
  margin-top: 28px;
  padding-bottom: 20px;
}
</style>
