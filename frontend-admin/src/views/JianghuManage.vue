<template>
  <div>
    <div class="page-header">
      <h2>江湖说管理</h2>
    </div>

    <el-card shadow="never" class="table-card">
      <el-table :data="list" v-loading="loading" stripe :empty-text="'暂无江湖说动态'" style="width:100%">
        <el-table-column prop="id" label="ID" width="70" align="center" />
        <el-table-column label="发布者" width="130">
          <template #default="{ row }">
            <div class="user-cell">
              <el-avatar :size="30" class="user-cell-avatar">{{ row.username?.charAt(0)?.toUpperCase() }}</el-avatar>
              <span>{{ row.username }}</span>
            </div>
          </template>
        </el-table-column>
        <el-table-column label="内容" min-width="280">
          <template #default="{ row }">
            <span class="content-text">{{ row.content?.length > 80 ? row.content.slice(0, 80) + '...' : row.content }}</span>
          </template>
        </el-table-column>
        <el-table-column label="图片" width="80" align="center">
          <template #default="{ row }">
            <el-image v-if="row.images?.length" :src="row.images[0]" :preview-src-list="row.images" fit="cover" class="thumb-img" />
            <span v-else class="text-muted">-</span>
          </template>
        </el-table-column>
        <el-table-column label="点赞" width="70" align="center">
          <template #default="{ row }">{{ row.like_count || 0 }}</template>
        </el-table-column>
        <el-table-column label="评论" width="70" align="center">
          <template #default="{ row }">{{ row.comment_count || 0 }}</template>
        </el-table-column>
        <el-table-column label="发布时间" width="170">
          <template #default="{ row }">{{ formatTime(row.created_at) }}</template>
        </el-table-column>
        <el-table-column label="操作" width="80" align="center">
          <template #default="{ row }">
            <span class="action-text danger" @click="handleDelete(row)">删除</span>
          </template>
        </el-table-column>
      </el-table>

      <div class="table-footer" v-if="total > 0">
        <el-pagination background layout="total, prev, pager, next" :total="total" :page-size="query.size" v-model:current-page="query.page" @current-change="loadData" />
      </div>
    </el-card>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { adminApi } from '../api'
import { ElMessage, ElMessageBox } from 'element-plus'
import { formatTime } from '../utils/format'

const loading = ref(false)
const list = ref([])
const total = ref(0)
const query = ref({ page: 1, size: 20 })

async function loadData() {
  loading.value = true
  try {
    const res = await adminApi.jianghu(query.value)
    list.value = res.list
    total.value = res.total
  } finally { loading.value = false }
}

async function handleDelete(row) {
  try {
    await ElMessageBox.confirm(
      `确定要删除该动态吗？内容：「${row.content?.slice(0, 30)}...」`,
      '删除确认',
      { confirmButtonText: '确定删除', cancelButtonText: '取消', type: 'warning', confirmButtonClass: 'el-button--danger' }
    )
    await adminApi.deleteJianghu(row.id)
    ElMessage({ message: '删除成功', type: 'success', duration: 2000, showClose: true })
    loadData()
  } catch (e) { /* cancelled */ }
}

onMounted(loadData)
</script>

<style lang="scss" scoped>
.table-card { border-radius: 14px; border: 1px solid #F3F4F6; }

.user-cell {
  display: flex; align-items: center; gap: 8px;
  .user-cell-avatar { background: linear-gradient(135deg, #7C3AED, #8B5CF6); color: #fff; font-weight: 600; font-size: 12px; flex-shrink: 0; }
  span { font-weight: 500; color: #111827; font-size: 13px; }
}

.content-text { font-size: 13px; color: #374151; line-height: 1.5; }
.text-muted { color: #D1D5DB; font-size: 13px; }
.thumb-img { width: 40px; height: 40px; border-radius: 6px; border: 1px solid #E5E7EB; cursor: pointer; }
.table-footer { display: flex; justify-content: center; padding-top: 20px; }
</style>
