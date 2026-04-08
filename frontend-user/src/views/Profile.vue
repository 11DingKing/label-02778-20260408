<template>
  <div class="page-container">
    <h2 class="section-title">个人中心</h2>

    <!-- 求职者档案 -->
    <div class="card" v-if="userStore.isSeeker">
      <div class="card-title-row">
        <h3><el-icon><Document /></el-icon> 求职档案</h3>
      </div>
      <el-form ref="seekerFormRef" :model="seekerForm" :rules="seekerRules" label-width="100px" hide-required-asterisk>
        <el-row :gutter="16">
          <el-col :span="8">
            <el-form-item label="姓名" prop="name">
              <el-input v-model="seekerForm.name" placeholder="您的姓名" />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="性别">
              <el-select v-model="seekerForm.gender" style="width:100%">
                <el-option label="未知" :value="0" />
                <el-option label="男" :value="1" />
                <el-option label="女" :value="2" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="年龄">
              <el-input-number v-model="seekerForm.age" :min="16" :max="65" style="width:100%" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="16">
          <el-col :span="8">
            <el-form-item label="学历">
              <el-select v-model="seekerForm.education" style="width:100%" placeholder="选择学历">
                <el-option v-for="e in educations" :key="e" :label="e" :value="e" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="工作经验">
              <el-input v-model="seekerForm.experience_years" placeholder="如: 3年" />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="期望薪资">
              <el-input v-model="seekerForm.expected_salary" placeholder="如: 8k-12k" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="期望城市">
          <el-input v-model="seekerForm.expected_city" placeholder="如: 北京" />
        </el-form-item>
        <el-form-item label="技能标签">
          <el-input v-model="seekerForm.skills" placeholder="用逗号分隔，如: Vue,Java,MySQL" />
        </el-form-item>
        <el-form-item label="自我介绍">
          <el-input v-model="seekerForm.self_intro" type="textarea" :rows="4" placeholder="介绍一下自己的优势和经历" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" :loading="saving" @click="saveSeeker" round>
            <el-icon><Check /></el-icon> 保存档案
          </el-button>
        </el-form-item>
      </el-form>
    </div>

    <!-- 企业档案 -->
    <div class="card" v-if="userStore.isEnterprise">
      <div class="card-title-row">
        <h3><el-icon><OfficeBuilding /></el-icon> 企业认证</h3>
        <span v-if="enterpriseForm.verified === 1" class="tag tag-green">已认证</span>
        <span v-else-if="enterpriseForm.verified === 0" class="tag tag-orange">审核中</span>
        <span v-else-if="enterpriseForm.verified === 2" class="tag tag-red">已拒绝</span>
      </div>
      <el-form ref="entFormRef" :model="enterpriseForm" :rules="entRules" label-width="100px" hide-required-asterisk>
        <el-row :gutter="16">
          <el-col :span="12">
            <el-form-item label="公司名称" prop="company_name">
              <el-input v-model="enterpriseForm.company_name" placeholder="公司全称" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="营业执照号" prop="license_no">
              <el-input v-model="enterpriseForm.license_no" placeholder="统一社会信用代码" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="16">
          <el-col :span="12">
            <el-form-item label="联系人">
              <el-input v-model="enterpriseForm.contact_person" placeholder="联系人姓名" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="联系电话">
              <el-input v-model="enterpriseForm.contact_phone" placeholder="联系电话" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="16">
          <el-col :span="12">
            <el-form-item label="行业">
              <el-input v-model="enterpriseForm.industry" placeholder="所属行业" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="地址">
              <el-input v-model="enterpriseForm.address" placeholder="公司地址" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="营业执照">
          <div class="license-upload">
            <div v-if="enterpriseForm.license_image" class="license-preview">
              <el-image :src="enterpriseForm.license_image" :preview-src-list="[enterpriseForm.license_image]" fit="cover" class="license-img" />
              <span class="upload-remove" @click="enterpriseForm.license_image = ''">&times;</span>
            </div>
            <label v-else class="upload-btn">
              <el-icon :size="24"><Plus /></el-icon>
              <span>上传营业执照</span>
              <input type="file" accept="image/*" hidden @change="handleLicenseUpload" />
            </label>
          </div>
          <div class="form-tip">请上传清晰的营业执照照片，支持 jpg/png 格式</div>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" :loading="saving" @click="saveEnterprise" round>
            <el-icon><Check /></el-icon> 提交认证
          </el-button>
        </el-form-item>
      </el-form>
    </div>

    <!-- 企业招聘管理 -->
    <div class="card" v-if="userStore.isEnterprise">
      <div class="card-title-row">
        <h3><el-icon><Briefcase /></el-icon> 我的招聘</h3>
        <el-button type="primary" size="small" @click="openJobForm" round>
          <el-icon><Plus /></el-icon> 发布招聘
        </el-button>
      </div>
      <el-table :data="myJobs" stripe :empty-text="'暂无招聘信息'">
        <el-table-column prop="title" label="职位名称" min-width="160" />
        <el-table-column prop="salary_range" label="薪资" width="120" />
        <el-table-column prop="city" label="城市" width="100" />
        <el-table-column label="发布时间" width="140">
          <template #default="{ row }">{{ formatTime(row.created_at) }}</template>
        </el-table-column>
        <el-table-column label="操作" width="120" align="center">
          <template #default="{ row }">
            <span class="action-text" @click="editJob(row)">编辑</span>
            <span class="action-text danger" @click="removeJob(row.id)">删除</span>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- 发布/编辑招聘弹窗 -->
    <el-dialog v-model="showJobForm" :title="editingJob ? '编辑招聘' : '发布招聘'" width="560px" destroy-on-close>
      <el-form ref="jobFormRef" :model="jobForm" :rules="jobRules" label-width="80px" hide-required-asterisk>
        <el-form-item label="职位名称" prop="title">
          <el-input v-model="jobForm.title" placeholder="如: 前端开发工程师" />
        </el-form-item>
        <el-row :gutter="16">
          <el-col :span="12">
            <el-form-item label="薪资范围">
              <el-input v-model="jobForm.salary_range" placeholder="如: 8k-15k" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="城市">
              <el-input v-model="jobForm.city" placeholder="工作城市" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="16">
          <el-col :span="12">
            <el-form-item label="学历要求">
              <el-select v-model="jobForm.education_req" style="width:100%" placeholder="选择学历">
                <el-option v-for="e in educations" :key="e" :label="e" :value="e" />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="经验要求">
              <el-input v-model="jobForm.experience_req" placeholder="如: 3年以上" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="职位描述">
          <el-input v-model="jobForm.description" type="textarea" :rows="5" placeholder="详细描述职位要求和福利" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showJobForm = false" round>取消</el-button>
        <el-button type="primary" :loading="saving" @click="saveJob" round>{{ editingJob ? '保存修改' : '发布' }}</el-button>
      </template>
    </el-dialog>

    <!-- 我的江湖说 -->
    <div class="card" v-if="myPosts.length">
      <div class="card-title-row">
        <h3><el-icon><ChatDotRound /></el-icon> 我的江湖说</h3>
      </div>
      <el-table :data="myPosts" stripe>
        <el-table-column label="内容" min-width="300">
          <template #default="{ row }">{{ row.content?.length > 60 ? row.content.slice(0, 60) + '...' : row.content }}</template>
        </el-table-column>
        <el-table-column label="点赞" width="80" align="center">
          <template #default="{ row }">{{ row.like_count || 0 }}</template>
        </el-table-column>
        <el-table-column label="评论" width="80" align="center">
          <template #default="{ row }">{{ row.comment_count || 0 }}</template>
        </el-table-column>
        <el-table-column label="发布时间" width="140">
          <template #default="{ row }">{{ formatTime(row.created_at) }}</template>
        </el-table-column>
        <el-table-column label="操作" width="80" align="center">
          <template #default="{ row }">
            <span class="action-text danger" @click="removePost(row.id)">删除</span>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- 我的市场发布 -->
    <div class="card" v-if="userStore.isEnterprise && myMarket.length">
      <div class="card-title-row">
        <h3><el-icon><Shop /></el-icon> 我的市场</h3>
      </div>
      <el-table :data="myMarket" stripe>
        <el-table-column prop="title" label="标题" min-width="200" />
        <el-table-column label="分类" width="100">
          <template #default="{ row }">{{ ['门店转让','二手物品','其他'][row.category] || '其他' }}</template>
        </el-table-column>
        <el-table-column label="价格" width="120">
          <template #default="{ row }">¥{{ row.price }}</template>
        </el-table-column>
        <el-table-column prop="city" label="城市" width="100" />
        <el-table-column label="发布时间" width="140">
          <template #default="{ row }">{{ formatTime(row.created_at) }}</template>
        </el-table-column>
        <el-table-column label="操作" width="80" align="center">
          <template #default="{ row }">
            <span class="action-text danger" @click="removeMarket(row.id)">删除</span>
          </template>
        </el-table-column>
      </el-table>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { seekerApi, enterpriseApi, jianghuApi, marketApi, uploadApi } from '../api'
import { useUserStore } from '../store/user'
import { Plus, Check, Document, OfficeBuilding, Briefcase, ChatDotRound, Shop } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { formatTime } from '../utils/format'

const userStore = useUserStore()
const saving = ref(false)
const educations = ['初中及以下', '高中/中专', '大专', '本科', '硕士', '博士']

const seekerFormRef = ref(null)
const seekerForm = ref({ name: '', gender: 0, age: 25, education: '', experience_years: '', expected_salary: '', expected_city: '', skills: '', self_intro: '' })
const seekerRules = { name: [{ required: true, message: '请输入姓名', trigger: 'blur' }] }

const entFormRef = ref(null)
const enterpriseForm = ref({ company_name: '', license_no: '', license_image: '', contact_person: '', contact_phone: '', industry: '', address: '', verified: -1 })
const entRules = {
  company_name: [{ required: true, message: '请输入公司名称', trigger: 'blur' }],
  license_no: [{ required: true, message: '请输入营业执照号', trigger: 'blur' }]
}

const showJobForm = ref(false)
const editingJob = ref(null)
const jobFormRef = ref(null)
const jobForm = ref({ title: '', salary_range: '', city: '', education_req: '', experience_req: '', description: '' })
const jobRules = { title: [{ required: true, message: '请输入职位名称', trigger: 'blur' }] }
const myJobs = ref([])
const myPosts = ref([])
const myMarket = ref([])

async function loadProfile() {
  if (userStore.isSeeker) {
    const p = await seekerApi.getProfile()
    if (p) Object.assign(seekerForm.value, p)
  }
  if (userStore.isEnterprise) {
    const p = await enterpriseApi.getProfile()
    if (p) Object.assign(enterpriseForm.value, p)
    const res = await enterpriseApi.myJobs({ page: 1, size: 100 })
    myJobs.value = res.list
  }
  // 加载我的江湖说和市场发布
  try {
    const uid = userStore.user?.id
    if (uid) {
      const jRes = await jianghuApi.list({ page: 1, size: 100 })
      myPosts.value = (jRes.list || []).filter(p => p.user_id === uid)
      if (userStore.isEnterprise) {
        const mRes = await marketApi.list({ page: 1, size: 100 })
        myMarket.value = (mRes.list || []).filter(p => p.user_id === uid)
      }
    }
  } catch {}
}

async function saveSeeker() {
  await seekerFormRef.value?.validate()
  saving.value = true
  try {
    await seekerApi.upsertProfile(seekerForm.value)
    ElMessage({ message: '档案保存成功', type: 'success', duration: 2000, showClose: true })
  } finally { saving.value = false }
}

async function saveEnterprise() {
  await entFormRef.value?.validate()
  saving.value = true
  try {
    const res = await enterpriseApi.upsertProfile(enterpriseForm.value)
    enterpriseForm.value.verified = res.verified
    ElMessage({ message: '认证信息已提交，等待审核', type: 'success', duration: 2500, showClose: true })
  } finally { saving.value = false }
}

async function handleLicenseUpload(event) {
  const file = event.target.files?.[0]
  if (!file) return
  try {
    const urls = await uploadApi.upload(file)
    if (urls && urls.length > 0) {
      enterpriseForm.value.license_image = urls[0]
      ElMessage({ message: '营业执照上传成功', type: 'success', duration: 2000 })
    }
  } catch (e) {
    ElMessage.error('图片上传失败')
  }
  event.target.value = ''
}

function openJobForm() {
  editingJob.value = null
  jobForm.value = { title: '', salary_range: '', city: '', education_req: '', experience_req: '', description: '' }
  showJobForm.value = true
}

function editJob(row) {
  editingJob.value = row
  jobForm.value = { title: row.title, salary_range: row.salary_range, city: row.city, education_req: row.education_req, experience_req: row.experience_req, description: row.description || '' }
  showJobForm.value = true
}

async function saveJob() {
  await jobFormRef.value?.validate()
  saving.value = true
  try {
    if (editingJob.value) {
      await enterpriseApi.updateJob(editingJob.value.id, jobForm.value)
      ElMessage({ message: '招聘信息已更新', type: 'success', duration: 2000, showClose: true })
    } else {
      await enterpriseApi.createJob(jobForm.value)
      ElMessage({ message: '招聘发布成功', type: 'success', duration: 2000, showClose: true })
    }
    showJobForm.value = false
    editingJob.value = null
    const res = await enterpriseApi.myJobs({ page: 1, size: 100 })
    myJobs.value = res.list
  } finally { saving.value = false }
}

async function removeJob(id) {
  try {
    await ElMessageBox.confirm('确定要删除该招聘信息吗？删除后不可恢复。', '删除确认', {
      confirmButtonText: '确定删除',
      cancelButtonText: '取消',
      type: 'warning',
      confirmButtonClass: 'el-button--danger'
    })
    await enterpriseApi.deleteJob(id)
    ElMessage({ message: '已删除', type: 'success', duration: 2000 })
    myJobs.value = myJobs.value.filter(j => j.id !== id)
  } catch (e) { /* cancelled */ }
}

async function removePost(id) {
  try {
    await ElMessageBox.confirm('确定要删除这条动态吗？', '删除确认', {
      confirmButtonText: '确定删除', cancelButtonText: '取消', type: 'warning', confirmButtonClass: 'el-button--danger'
    })
    await jianghuApi.delete(id)
    ElMessage({ message: '已删除', type: 'success', duration: 2000 })
    myPosts.value = myPosts.value.filter(p => p.id !== id)
  } catch {}
}

async function removeMarket(id) {
  try {
    await ElMessageBox.confirm('确定要删除该商品信息吗？', '删除确认', {
      confirmButtonText: '确定删除', cancelButtonText: '取消', type: 'warning', confirmButtonClass: 'el-button--danger'
    })
    await marketApi.delete(id)
    ElMessage({ message: '已删除', type: 'success', duration: 2000 })
    myMarket.value = myMarket.value.filter(p => p.id !== id)
  } catch {}
}

onMounted(loadProfile)
</script>

<style lang="scss" scoped>
.card-title-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;

  h3 {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 17px;
    font-weight: 600;
    color: var(--text-primary);
  }
}

.license-upload {
  .license-preview {
    position: relative;
    display: inline-block;
    .license-img { width: 200px; height: 140px; border-radius: 8px; border: 1px solid var(--border-light); }
    .upload-remove {
      position: absolute; top: 4px; right: 4px; width: 24px; height: 24px;
      background: rgba(0,0,0,0.5); color: #fff; border-radius: 50%;
      display: flex; align-items: center; justify-content: center;
      cursor: pointer; font-size: 16px; line-height: 1;
    }
  }
  .upload-btn {
    width: 200px; height: 140px; border: 2px dashed var(--border);
    border-radius: 8px; display: flex; flex-direction: column;
    align-items: center; justify-content: center; gap: 8px;
    cursor: pointer; color: var(--text-muted); font-size: 14px;
    transition: var(--transition);
    &:hover { border-color: var(--primary); color: var(--primary); }
  }
}

.form-tip { font-size: 12px; color: var(--text-muted); margin-top: 6px; }
</style>
