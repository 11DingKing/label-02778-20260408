<template>
  <div class="page-container" v-loading="loading">
    <el-button @click="$router.back()" round class="back-btn">
      <el-icon><ArrowLeft /></el-icon> 返回列表
    </el-button>
    <transition name="fade-slide">
      <div class="card detail-card" v-if="job">
        <div class="detail-header">
          <div>
            <h1>{{ job.title }}</h1>
            <div class="company-name">
              <el-icon><OfficeBuilding /></el-icon> {{ job.company_name }}
            </div>
          </div>
          <span class="salary">{{ job.salary_range || '面议' }}</span>
        </div>
        <div class="detail-tags">
          <span class="tag tag-green"><el-icon><Location /></el-icon> {{ job.city || '不限' }}</span>
          <span class="tag tag-blue">{{ job.education_req || '不限学历' }}</span>
          <span class="tag tag-gray">{{ job.experience_req || '不限经验' }}</span>
          <span class="tag tag-gray">{{ formatTime(job.created_at) }} 发布</span>
        </div>
        <el-divider />
        <h3 class="desc-title">职位描述</h3>
        <p class="description">{{ job.description || '暂无详细描述' }}</p>

        <el-divider />
        <h3 class="desc-title">企业信息</h3>
        <div class="company-info">
          <div class="info-item" v-if="job.contact_person">
            <el-icon><User /></el-icon>
            <span class="info-label">联系人</span>
            <span class="info-val">{{ job.contact_person }}</span>
          </div>
          <div class="info-item" v-if="job.contact_phone">
            <el-icon><Phone /></el-icon>
            <span class="info-label">联系电话</span>
            <span class="info-val">{{ job.contact_phone }}</span>
          </div>
          <div class="info-item" v-if="job.industry">
            <el-icon><OfficeBuilding /></el-icon>
            <span class="info-label">所属行业</span>
            <span class="info-val">{{ job.industry }}</span>
          </div>
          <div class="info-item" v-if="job.address">
            <el-icon><Location /></el-icon>
            <span class="info-label">公司地址</span>
            <span class="info-val">{{ job.address }}</span>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { jobApi } from '../api'
import { ArrowLeft, OfficeBuilding, Location, User, Phone } from '@element-plus/icons-vue'
import { formatTime } from '../utils/format'

const route = useRoute()
const loading = ref(false)
const job = ref(null)

onMounted(async () => {
  loading.value = true
  try { job.value = await jobApi.detail(route.params.id) } finally { loading.value = false }
})
</script>

<style lang="scss" scoped>
.back-btn { margin-bottom: 20px; }

.detail-card { animation: fadeUp 0.5s ease; }

@keyframes fadeUp {
  from { opacity: 0; transform: translateY(12px); }
  to { opacity: 1; transform: translateY(0); }
}

.detail-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 20px;

  h1 { font-size: 26px; font-weight: 700; margin-bottom: 8px; letter-spacing: -0.3px; }

  .company-name {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 15px;
    color: var(--primary);
    font-weight: 500;
  }

  .salary {
    font-size: 28px;
    color: #EF4444;
    font-weight: 800;
    white-space: nowrap;
  }
}

.detail-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 4px;

  .tag {
    display: inline-flex;
    align-items: center;
    gap: 4px;
  }
}

.desc-title {
  font-size: 17px;
  font-weight: 600;
  margin-bottom: 14px;
  color: var(--text-primary);
}

.description {
  white-space: pre-wrap;
  line-height: 2;
  color: var(--text-secondary);
  font-size: 15px;
}

.company-info {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;

  .info-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 14px 18px;
    background: #F8FAFC;
    border-radius: 10px;
    border: 1px solid var(--border-light);

    .el-icon { color: var(--primary); font-size: 18px; flex-shrink: 0; }
    .info-label { font-size: 13px; color: var(--text-muted); white-space: nowrap; }
    .info-val { font-size: 14px; font-weight: 500; color: var(--text-primary); }
  }
}
</style>
