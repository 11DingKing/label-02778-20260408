<template>
  <div>
    <div class="page-header">
      <h2>招聘管理</h2>
      <el-input v-model="keyword" placeholder="搜索职位名称 / 城市" style="width:280px" clearable size="large" @keyup.enter="loadData" @clear="loadData">
        <template #prefix><el-icon><Search /></el-icon></template>
      </el-input>
    </div>

    <el-card shadow="never" class="table-card">
      <el-table :data="list" v-loading="loading" stripe :empty-text="'暂无招聘信息'" style="width:100%">
        <el-table-column prop="id" label="ID" width="70" align="center" />
        <el-table-column label="职位名称" min-width="180">
          <template #default="{ row }">
            <span class="title-text">{{ row.title }}</span>
          </template>
        </el-table-column>
        <el-table-column label="薪资" width="120">
          <template #default="{ row }">
            <span class="salary-text">{{ row.salary_range || '面议' }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="city" label="城市" width="100" />
        <el-table-column label="企业" width="160">
          <template #default="{ row }">{{ row.company_name || '-' }}</template>
        </el-table-column>
        <el-table-column label="学历" width="100">
          <template #default="{ row }">{{ row.education_req || '-' }}</template>
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
import { Search } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { formatTime } from '../utils/format'

const loading = ref(false)
const list = ref([])
const total = ref(0)
const keyword = ref('')
const query = ref({ page: 1, size: 20 })

async function loadData() {
  loading.value = true
  try {
    const res = await adminApi.jobs({ ...query.value, keyword: keyword.value })
    list.value = res.list
    total.value = res.total
  } finally { loading.value = false }
}

async function handleDelete(row) {
  try {
    await ElMessageBox.confirm(
      `确定要删除职位「${row.title}」吗？删除后不可恢复。`,
      '删除确认',
      { confirmButtonText: '确定删除', cancelButtonText: '取消', type: 'warning', confirmButtonClass: 'el-button--danger' }
    )
    await adminApi.deleteJob(row.id)
    ElMessage({ message: '删除成功', type: 'success', duration: 2000, showClose: true })
    loadData()
  } catch (e) { /* cancelled */ }
}

onMounted(loadData)
</script>

<style lang="scss" scoped>
.table-card { border-radius: 14px; border: 1px solid #F3F4F6; }
.title-text { font-weight: 500; color: #111827; }
.salary-text { font-weight: 700; color: #EF4444; }
.table-footer { display: flex; justify-content: center; padding-top: 20px; }
</style>
