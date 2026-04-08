<template>
  <div>
    <div class="page-header">
      <h2>操作日志</h2>
      <div class="header-info">
        <el-icon><Clock /></el-icon>
        <span>共 {{ total }} 条记录</span>
      </div>
    </div>

    <el-card shadow="never" class="table-card">
      <el-table :data="list" v-loading="loading" stripe :empty-text="'暂无操作日志'" style="width:100%">
        <el-table-column prop="id" label="ID" width="70" align="center" />
        <el-table-column label="用户ID" width="80" align="center">
          <template #default="{ row }">
            <el-tag type="info" effect="plain" round size="small">{{ row.user_id }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="140">
          <template #default="{ row }">
            <div class="action-badge" :class="actionColor(row.action)">
              {{ row.action }}
            </div>
          </template>
        </el-table-column>
        <el-table-column label="目标" min-width="200">
          <template #default="{ row }">
            <span class="target-text">{{ row.target || '-' }}</span>
          </template>
        </el-table-column>
        <el-table-column label="IP 地址" width="140">
          <template #default="{ row }">
            <span class="mono-text">{{ row.ip || '-' }}</span>
          </template>
        </el-table-column>
        <el-table-column label="操作时间" width="190">
          <template #default="{ row }">{{ formatTime(row.created_at) }}</template>
        </el-table-column>
      </el-table>

      <div class="table-footer" v-if="total > 0">
        <el-pagination
          background
          layout="total, prev, pager, next"
          :total="total"
          :page-size="query.size"
          v-model:current-page="query.page"
          @current-change="loadData"
        />
      </div>
    </el-card>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { adminApi } from '../api'
import { Clock } from '@element-plus/icons-vue'
import { formatTime } from '../utils/format'

const loading = ref(false)
const list = ref([])
const total = ref(0)
const query = ref({ page: 1, size: 20 })

function actionColor(action) {
  if (!action) return 'gray'
  if (action.includes('删除')) return 'red'
  if (action.includes('审核') || action.includes('切换')) return 'orange'
  if (action.includes('发布') || action.includes('更新') || action.includes('创建')) return 'blue'
  if (action.includes('接受')) return 'green'
  if (action.includes('拒绝')) return 'red'
  return 'gray'
}

async function loadData() {
  loading.value = true
  try {
    const res = await adminApi.logs(query.value)
    list.value = res.list
    total.value = res.total
  } finally { loading.value = false }
}

onMounted(loadData)
</script>

<style lang="scss" scoped>
.header-info {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: #6B7280;
  padding: 6px 14px;
  background: #F3F4F6;
  border-radius: 20px;
}

.table-card {
  border-radius: 14px;
  border: 1px solid #F3F4F6;
}

.action-badge {
  display: inline-flex;
  align-items: center;
  padding: 3px 12px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 0.2px;

  &.blue { background: #DBEAFE; color: #1E40AF; }
  &.green { background: #D1FAE5; color: #065F46; }
  &.orange { background: #FEF3C7; color: #92400E; }
  &.red { background: #FEE2E2; color: #991B1B; }
  &.gray { background: #F3F4F6; color: #4B5563; }
}

.target-text {
  font-size: 13px;
  color: #4B5563;
  word-break: break-all;
}

.mono-text {
  font-family: "SF Mono", "Fira Code", monospace;
  font-size: 12px;
  color: #6B7280;
  letter-spacing: 0.3px;
}

.table-footer {
  display: flex;
  justify-content: center;
  padding-top: 20px;
}
</style>
