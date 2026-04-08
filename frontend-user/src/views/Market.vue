<template>
  <div class="page-container">
    <div class="market-header">
      <h2 class="section-title">好市场</h2>
      <el-button type="primary" v-if="userStore.isEnterprise" @click="openForm" round>
        <el-icon><Plus /></el-icon> 发布商品
      </el-button>
    </div>

    <div class="card filter-bar">
      <el-row :gutter="12" align="middle">
        <el-col :span="7">
          <el-input v-model="query.keyword" placeholder="搜索标题、城市" clearable @keyup.enter="loadData" size="large">
            <template #prefix><el-icon><Search /></el-icon></template>
          </el-input>
        </el-col>
        <el-col :span="5">
          <el-select v-model="query.category" placeholder="全部分类" clearable @change="loadData" size="large" style="width:100%">
            <el-option label="门店转让" :value="0" />
            <el-option label="二手物品" :value="1" />
            <el-option label="其他" :value="2" />
          </el-select>
        </el-col>
        <el-col :span="4">
          <el-button type="primary" size="large" @click="loadData" round>搜索</el-button>
        </el-col>
      </el-row>
    </div>

    <div v-loading="loading" v-if="loading" style="min-height:200px"></div>

    <transition-group name="list" tag="div" class="market-grid">
      <div v-for="item in list" :key="item.id" class="market-card" @click="$router.push(`/market/${item.id}`)">
        <div class="market-card-top">
          <span class="tag" :class="categoryClass(item.category)">{{ categoryName(item.category) }}</span>
        </div>
        <div v-if="item.images && item.images.length" class="market-card-img">
          <img :src="item.images[0]" alt="商品图片" />
        </div>
        <h3 class="market-title">{{ item.title }}</h3>
        <p class="market-price">¥{{ item.price }}</p>
        <div class="market-meta">
          <span><el-icon><Location /></el-icon> {{ item.city || '未知' }}</span>
          <span><el-icon><Phone /></el-icon> {{ item.contact_info }}</span>
        </div>
      </div>
    </transition-group>

    <div v-if="list.length === 0 && !loading" class="empty-state">
      <el-icon><Shop /></el-icon>
      <p>暂无商品信息</p>
    </div>

    <div class="pagination-wrap" v-if="total > query.size">
      <el-pagination background layout="total, prev, pager, next" :total="total" :page-size="query.size" v-model:current-page="query.page" @current-change="loadData" />
    </div>

    <el-dialog v-model="showForm" title="发布商品" width="560px" destroy-on-close>
      <el-form ref="formRef" :model="form" :rules="formRules" label-width="80px">
        <el-form-item label="标题" prop="title">
          <el-input v-model="form.title" placeholder="请输入商品标题" />
        </el-form-item>
        <el-form-item label="分类" prop="category">
          <el-select v-model="form.category" style="width:100%">
            <el-option label="门店转让" :value="0" />
            <el-option label="二手物品" :value="1" />
            <el-option label="其他" :value="2" />
          </el-select>
        </el-form-item>
        <el-row :gutter="16">
          <el-col :span="12">
            <el-form-item label="价格" prop="price">
              <el-input v-model="form.price" type="number" placeholder="请输入价格">
                <template #prepend>¥</template>
              </el-input>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="城市">
              <el-input v-model="form.city" placeholder="所在城市" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="联系方式" prop="contact_info">
          <el-input v-model="form.contact_info" placeholder="手机号 / 微信号" />
        </el-form-item>
        <el-form-item label="图片">
          <div class="upload-area">
            <div v-for="(img, idx) in form.images" :key="idx" class="upload-preview">
              <img :src="img" alt="preview" />
              <span class="upload-remove" @click="form.images.splice(idx, 1)">&times;</span>
            </div>
            <label v-if="form.images.length < 6" class="upload-btn">
              <el-icon><Plus /></el-icon>
              <input type="file" accept="image/*" hidden @change="handleUpload($event, 'market')" />
            </label>
          </div>
        </el-form-item>
        <el-form-item label="详细描述">
          <el-input v-model="form.description" type="textarea" :rows="4" placeholder="详细描述商品信息" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showForm = false" round>取消</el-button>
        <el-button type="primary" :loading="submitting" @click="handleSubmit" round>发布</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { marketApi, uploadApi } from '../api'
import { useUserStore } from '../store/user'
import { Search, Shop, Plus, Location, Phone } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

const userStore = useUserStore()
const loading = ref(false)
const submitting = ref(false)
const showForm = ref(false)
const formRef = ref(null)
const list = ref([])
const total = ref(0)
const query = ref({ page: 1, size: 12, keyword: '', category: null })

const defaultForm = { title: '', category: 0, price: '0', city: '', contact_info: '', description: '', images: [] }
const form = ref({ ...defaultForm, images: [] })
const formRules = {
  title: [{ required: true, message: '请输入标题', trigger: 'blur' }],
  category: [{ required: true, message: '请选择分类', trigger: 'change' }],
  contact_info: [{ required: true, message: '请输入联系方式', trigger: 'blur' }]
}

const categoryNames = ['门店转让', '二手物品', '其他']
const categoryClasses = ['tag-orange', 'tag-blue', 'tag-gray']
function categoryName(c) { return categoryNames[c] || '其他' }
function categoryClass(c) { return categoryClasses[c] || 'tag-gray' }

function openForm() {
  form.value = { ...defaultForm, images: [] }
  showForm.value = true
}

async function handleUpload(event) {
  const file = event.target.files?.[0]
  if (!file) return
  try {
    const urls = await uploadApi.upload(file)
    if (urls && urls.length > 0) {
      form.value.images.push(urls[0])
    }
  } catch (e) {
    ElMessage.error('图片上传失败')
  }
  event.target.value = ''
}

async function loadData() {
  loading.value = true
  try {
    const params = { ...query.value }
    if (params.category === null || params.category === undefined) delete params.category
    const res = await marketApi.list(params)
    list.value = res.list
    total.value = res.total
  } finally { loading.value = false }
}

async function handleSubmit() {
  await formRef.value?.validate()
  submitting.value = true
  try {
    await marketApi.create({ ...form.value, price: parseFloat(form.value.price) || 0 })
    ElMessage({ message: '发布成功', type: 'success', duration: 2000, showClose: true })
    showForm.value = false
    loadData()
  } finally { submitting.value = false }
}

onMounted(loadData)
</script>

<style lang="scss" scoped>
.market-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.filter-bar { margin-bottom: 20px; }

.list-enter-active { transition: all 0.4s ease; }
.list-enter-from { opacity: 0; transform: translateY(16px) scale(0.97); }

.market-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
}

.market-card {
  background: #fff;
  border-radius: var(--radius);
  padding: 24px;
  box-shadow: var(--shadow-sm);
  border: 1px solid var(--border-light);
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);

  &:hover {
    transform: translateY(-4px);
    box-shadow: var(--shadow-lg);
    border-color: var(--primary-lighter);
  }

  .market-card-top { margin-bottom: 10px; }

  .market-title {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 8px;
    line-height: 1.4;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .market-price {
    font-size: 22px;
    color: #EF4444;
    font-weight: 800;
    margin-bottom: 12px;
  }

  .market-meta {
    display: flex;
    justify-content: space-between;
    font-size: 13px;
    color: var(--text-muted);

    span {
      display: flex;
      align-items: center;
      gap: 3px;
    }
  }
}

.pagination-wrap { display: flex; justify-content: center; margin-top: 28px; }

.upload-area {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;

  .upload-preview {
    position: relative;
    width: 80px;
    height: 80px;
    border-radius: 8px;
    overflow: hidden;
    border: 1px solid var(--border-light);

    img { width: 100%; height: 100%; object-fit: cover; }

    .upload-remove {
      position: absolute;
      top: 2px;
      right: 2px;
      width: 20px;
      height: 20px;
      background: rgba(0,0,0,0.5);
      color: #fff;
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      cursor: pointer;
      font-size: 14px;
      line-height: 1;
    }
  }

  .upload-btn {
    width: 80px;
    height: 80px;
    border: 2px dashed var(--border);
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    color: var(--text-muted);
    font-size: 24px;
    transition: var(--transition);

    &:hover { border-color: var(--primary); color: var(--primary); }
  }
}

.market-card-img {
  width: 100%;
  height: 140px;
  border-radius: 8px;
  overflow: hidden;
  margin-bottom: 10px;

  img { width: 100%; height: 100%; object-fit: cover; }
}

@media (max-width: 768px) { .market-grid { grid-template-columns: 1fr; } }
</style>
