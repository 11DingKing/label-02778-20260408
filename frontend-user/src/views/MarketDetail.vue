<template>
  <div class="page-container" v-loading="loading">
    <el-button @click="$router.back()" round class="back-btn">
      <el-icon><ArrowLeft /></el-icon> 返回列表
    </el-button>
    <transition name="fade-slide">
      <div class="card detail-card" v-if="item">
        <span class="tag" :class="categoryClass(item.category)">{{ categoryName(item.category) }}</span>
        <h1 class="detail-title">{{ item.title }}</h1>
        <p class="detail-price">¥{{ item.price }}</p>
        <div class="detail-info-row">
          <div class="info-chip"><el-icon><Location /></el-icon> {{ item.city || '未知' }}</div>
          <div class="info-chip"><el-icon><User /></el-icon> {{ item.username }}</div>
          <div class="info-chip"><el-icon><Phone /></el-icon> {{ item.contact_info }}</div>
          <div class="info-chip"><el-icon><Clock /></el-icon> {{ formatTime(item.created_at) }}</div>
        </div>
        <div v-if="item.images && item.images.length" class="detail-images">
          <el-image v-for="(img, idx) in item.images" :key="idx" :src="img" :preview-src-list="item.images" :initial-index="idx" fit="cover" class="detail-img" lazy />
        </div>
        <el-divider />
        <h3 class="desc-title">详细描述</h3>
        <p class="description">{{ item.description || '暂无详细描述' }}</p>
      </div>
    </transition>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { marketApi } from '../api'
import { ArrowLeft, Location, User, Phone, Clock } from '@element-plus/icons-vue'
import { formatTime } from '../utils/format'

const route = useRoute()
const loading = ref(false)
const item = ref(null)

const categoryNames = ['门店转让', '二手物品', '其他']
const categoryClasses = ['tag-orange', 'tag-blue', 'tag-gray']
function categoryName(c) { return categoryNames[c] || '其他' }
function categoryClass(c) { return categoryClasses[c] || 'tag-gray' }

onMounted(async () => {
  loading.value = true
  try { item.value = await marketApi.detail(route.params.id) } finally { loading.value = false }
})
</script>

<style lang="scss" scoped>
.back-btn { margin-bottom: 20px; }
.detail-card { animation: fadeUp 0.5s ease; }
@keyframes fadeUp { from { opacity: 0; transform: translateY(12px); } to { opacity: 1; transform: translateY(0); } }

.detail-title { font-size: 26px; font-weight: 700; margin: 14px 0 8px; letter-spacing: -0.3px; }
.detail-price { font-size: 32px; color: #EF4444; font-weight: 800; margin-bottom: 20px; }

.detail-info-row {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;

  .info-chip {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 6px 14px;
    background: #F8FAFC;
    border-radius: 20px;
    font-size: 14px;
    color: var(--text-secondary);
    border: 1px solid var(--border-light);
  }
}

.desc-title { font-size: 17px; font-weight: 600; margin-bottom: 14px; }
.description { white-space: pre-wrap; line-height: 2; color: var(--text-secondary); font-size: 15px; }

.detail-images {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-top: 20px;

  .detail-img {
    width: 160px;
    height: 160px;
    border-radius: 10px;
    border: 1px solid var(--border-light);
    cursor: pointer;
  }
}
</style>
