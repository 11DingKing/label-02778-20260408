<template>
  <div>
    <div class="page-header">
      <h2>用户管理</h2>
      <el-input
        v-model="keyword"
        placeholder="搜索用户名 / 手机号"
        style="width:280px"
        clearable
        size="large"
        @keyup.enter="loadData"
        @clear="loadData"
      >
        <template #prefix><el-icon><Search /></el-icon></template>
      </el-input>
    </div>

    <el-card shadow="never" class="table-card">
      <el-table :data="list" v-loading="loading" stripe :empty-text="'暂无用户数据'" style="width:100%">
        <el-table-column prop="id" label="ID" width="70" align="center" />
        <el-table-column label="用户名" min-width="140">
          <template #default="{ row }">
            <div class="user-cell">
              <el-avatar :size="32" class="user-cell-avatar">{{ row.username?.charAt(0)?.toUpperCase() }}</el-avatar>
              <span>{{ row.username }}</span>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="phone" label="手机号" width="140">
          <template #default="{ row }">
            <span>{{ row.phone || '-' }}</span>
          </template>
        </el-table-column>
        <el-table-column label="角色" width="100" align="center">
          <template #default="{ row }">
            <el-tag v-if="row.role === 0" type="info" effect="light" round>求职者</el-tag>
            <el-tag v-else-if="row.role === 1" type="warning" effect="light" round>企业</el-tag>
            <el-tag v-else type="danger" effect="light" round>管理员</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="状态" width="90" align="center">
          <template #default="{ row }">
            <div class="status-dot-wrap">
              <span class="status-dot" :class="row.status === 0 ? 'online' : 'offline'"></span>
              <span>{{ row.status === 0 ? '正常' : '禁用' }}</span>
            </div>
          </template>
        </el-table-column>
        <el-table-column label="注册时间" width="170">
          <template #default="{ row }">{{ formatTime(row.created_at) }}</template>
        </el-table-column>
        <el-table-column label="操作" width="100" align="center">
          <template #default="{ row }">
            <span
              v-if="row.role !== 2"
              class="action-text"
              :class="row.status === 0 ? 'danger' : 'success'"
              @click="toggleStatus(row)"
            >
              {{ row.status === 0 ? '禁用' : '启用' }}
            </span>
            <span v-else class="text-muted">-</span>
          </template>
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
    const res = await adminApi.users({ ...query.value, keyword: keyword.value })
    list.value = res.list
    total.value = res.total
  } finally { loading.value = false }
}

async function toggleStatus(row) {
  const action = row.status === 0 ? '禁用' : '启用'
  try {
    await ElMessageBox.confirm(
      `确定要${action}用户「${row.username}」吗？${row.status === 0 ? '禁用后该用户将无法登录。' : ''}`,
      `${action}确认`,
      {
        confirmButtonText: `确定${action}`,
        cancelButtonText: '取消',
        type: row.status === 0 ? 'warning' : 'info',
        confirmButtonClass: row.status === 0 ? 'el-button--danger' : 'el-button--success'
      }
    )
    await adminApi.toggleUserStatus(row.id)
    ElMessage({ message: `已${action}用户「${row.username}」`, type: 'success', duration: 2000, showClose: true })
    row.status = row.status === 0 ? 1 : 0
  } catch (e) { /* cancelled */ }
}

onMounted(loadData)
</script>

<style lang="scss" scoped>
.table-card {
  border-radius: 14px;
  border: 1px solid #F3F4F6;
}

.user-cell {
  display: flex;
  align-items: center;
  gap: 10px;

  .user-cell-avatar {
    background: linear-gradient(135deg, #6366F1, #8B5CF6);
    color: #fff;
    font-weight: 600;
    font-size: 13px;
    flex-shrink: 0;
  }

  span {
    font-weight: 500;
    color: #111827;
  }
}

.status-dot-wrap {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  font-size: 13px;

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;

    &.online {
      background: #10B981;
      box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.15);
    }

    &.offline {
      background: #EF4444;
      box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.15);
    }
  }
}

.text-muted {
  color: #D1D5DB;
  font-size: 13px;
}

.table-footer {
  display: flex;
  justify-content: center;
  padding-top: 20px;
}
</style>
