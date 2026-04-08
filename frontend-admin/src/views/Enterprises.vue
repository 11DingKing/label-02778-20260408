<template>
  <div>
    <div class="page-header">
      <h2>企业审核</h2>
      <div class="header-stats">
        <span class="header-stat">
          <span class="header-stat-dot pending"></span>
          待审核 {{ pendingCount }}
        </span>
      </div>
    </div>

    <el-card shadow="never" class="table-card">
      <el-table :data="list" v-loading="loading" stripe :empty-text="'暂无企业认证申请'" style="width:100%">
        <el-table-column prop="id" label="ID" width="70" align="center" />
        <el-table-column label="公司名称" min-width="180">
          <template #default="{ row }">
            <div class="company-cell">
              <div class="company-icon">
                <el-icon><OfficeBuilding /></el-icon>
              </div>
              <div>
                <div class="company-name">{{ row.company_name }}</div>
                <div class="company-industry">{{ row.industry || '-' }}</div>
              </div>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="license_no" label="营业执照号" min-width="160">
          <template #default="{ row }">
            <span class="mono-text">{{ row.license_no }}</span>
          </template>
        </el-table-column>
        <el-table-column label="联系人" width="120">
          <template #default="{ row }">
            <div>{{ row.contact_person || '-' }}</div>
            <div class="sub-text">{{ row.contact_phone }}</div>
          </template>
        </el-table-column>
        <el-table-column label="营业执照" width="120" align="center">
          <template #default="{ row }">
            <el-image v-if="row.license_image" :src="row.license_image" :preview-src-list="[row.license_image]" fit="cover" class="license-thumb" />
            <span v-else class="text-muted">未上传</span>
          </template>
        </el-table-column>
        <el-table-column label="状态" width="100" align="center">
          <template #default="{ row }">
            <el-tag v-if="row.verified === 0" type="warning" effect="light" round>待审核</el-tag>
            <el-tag v-else-if="row.verified === 1" type="success" effect="light" round>已通过</el-tag>
            <el-tag v-else type="danger" effect="light" round>已拒绝</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="提交时间" width="150">
          <template #default="{ row }">{{ formatTime(row.created_at, false) }}</template>
        </el-table-column>
        <el-table-column label="操作" width="140" align="center">
          <template #default="{ row }">
            <span v-if="row.verified !== 1" class="action-text success" @click="verify(row, 1)">通过</span>
            <span v-if="row.verified !== 2" class="action-text danger" @click="verify(row, 2)">拒绝</span>
            <span v-if="row.verified === 1 && row.verified === 2" class="text-muted">-</span>
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
import { ref, computed, onMounted } from 'vue'
import { adminApi } from '../api'
import { OfficeBuilding } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { formatTime } from '../utils/format'

const loading = ref(false)
const list = ref([])
const total = ref(0)
const query = ref({ page: 1, size: 20 })

const pendingCount = computed(() => list.value.filter(e => e.verified === 0).length)

async function loadData() {
  loading.value = true
  try {
    const res = await adminApi.enterprises(query.value)
    list.value = res.list
    total.value = res.total
  } finally { loading.value = false }
}

async function verify(row, status) {
  const action = status === 1 ? '通过' : '拒绝'
  const desc = status === 1
    ? `通过后「${row.company_name}」将可以发布招聘信息。`
    : `拒绝后「${row.company_name}」需要重新提交认证。`

  try {
    await ElMessageBox.confirm(
      desc,
      `${action}认证 - ${row.company_name}`,
      {
        confirmButtonText: `确定${action}`,
        cancelButtonText: '取消',
        type: status === 1 ? 'success' : 'warning',
        confirmButtonClass: status === 1 ? 'el-button--success' : 'el-button--danger'
      }
    )
    await adminApi.verifyEnterprise(row.id, { verified: status })
    ElMessage({ message: `已${action}「${row.company_name}」的认证申请`, type: 'success', duration: 2500, showClose: true })
    row.verified = status
  } catch (e) { /* cancelled */ }
}

onMounted(loadData)
</script>

<style lang="scss" scoped>
.header-stats {
  display: flex;
  gap: 16px;
}

.header-stat {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: #6B7280;
  padding: 6px 14px;
  background: #FEF3C7;
  border-radius: 20px;
  font-weight: 500;
  color: #92400E;

  .header-stat-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;

    &.pending {
      background: #F59E0B;
      animation: pulse 2s infinite;
    }
  }
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.table-card {
  border-radius: 14px;
  border: 1px solid #F3F4F6;
}

.company-cell {
  display: flex;
  align-items: center;
  gap: 12px;

  .company-icon {
    width: 40px;
    height: 40px;
    border-radius: 10px;
    background: #EFF6FF;
    color: #2563EB;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 18px;
    flex-shrink: 0;
  }

  .company-name {
    font-weight: 600;
    font-size: 14px;
    color: #111827;
  }

  .company-industry {
    font-size: 12px;
    color: #9CA3AF;
    margin-top: 1px;
  }
}

.mono-text {
  font-family: "SF Mono", "Fira Code", monospace;
  font-size: 12px;
  color: #6B7280;
  letter-spacing: 0.3px;
}

.sub-text {
  font-size: 12px;
  color: #9CA3AF;
  margin-top: 2px;
}

.text-muted {
  color: #D1D5DB;
  font-size: 13px;
}

.license-thumb {
  width: 60px;
  height: 42px;
  border-radius: 6px;
  border: 1px solid #E5E7EB;
  cursor: pointer;
}

.table-footer {
  display: flex;
  justify-content: center;
  padding-top: 20px;
}
</style>
