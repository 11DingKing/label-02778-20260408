<template>
  <div class="page-container" v-loading="loading">
    <el-button @click="$router.back()" round class="back-btn">
      <el-icon><ArrowLeft /></el-icon> 返回列表
    </el-button>
    <transition name="fade-slide">
      <div class="card detail-card" v-if="profile">
        <div class="profile-top">
          <div class="profile-avatar">{{ profile.name?.charAt(0) }}</div>
          <div class="profile-main">
            <h1>{{ profile.name }}</h1>
            <div class="profile-sub">
              {{ ['未知','男','女'][profile.gender] }}
              <span v-if="profile.age"> · {{ profile.age }}岁</span>
              <span v-if="profile.education"> · {{ profile.education }}</span>
            </div>
          </div>
          <el-button type="primary" v-if="userStore.isEnterprise" @click="showGreetingDialog = true" round size="large">
            <el-icon><ChatDotRound /></el-icon> 联系TA
          </el-button>
        </div>

        <div class="info-cards">
          <div class="info-card">
            <div class="info-icon blue"><el-icon><Money /></el-icon></div>
            <div><div class="info-label">期望薪资</div><div class="info-value salary">{{ profile.expected_salary || '面议' }}</div></div>
          </div>
          <div class="info-card">
            <div class="info-icon green"><el-icon><Location /></el-icon></div>
            <div><div class="info-label">期望城市</div><div class="info-value">{{ profile.expected_city || '不限' }}</div></div>
          </div>
          <div class="info-card">
            <div class="info-icon purple"><el-icon><Timer /></el-icon></div>
            <div><div class="info-label">工作经验</div><div class="info-value">{{ profile.experience_years || '不限' }}</div></div>
          </div>
          <div class="info-card">
            <div class="info-icon orange"><el-icon><Phone /></el-icon></div>
            <div><div class="info-label">联系电话</div><div class="info-value">{{ profile.phone || '未填写' }}</div></div>
          </div>
        </div>

        <div v-if="profile.skills" class="skills-section">
          <h3>技能标签</h3>
          <div class="skills-wrap">
            <span class="tag tag-orange" v-for="s in profile.skills.split(',').filter(Boolean)" :key="s">{{ s.trim() }}</span>
          </div>
        </div>

        <el-divider />
        <h3 class="desc-title">自我介绍</h3>
        <p class="description">{{ profile.self_intro || '暂无自我介绍' }}</p>

        <el-divider />
        <div class="detail-footer">
          <div class="status-badge">
            <span class="dot green"></span> 正在求职中
          </div>
          <span class="update-time">档案更新于 {{ formatTime(profile.updated_at) }}</span>
        </div>
      </div>
    </transition>

    <!-- 推荐职位 -->
    <div class="recommend-section" v-if="recommendJobs.length">
      <h3 class="recommend-title"><el-icon><Briefcase /></el-icon> 可能适合的职位</h3>
      <div class="recommend-grid">
        <div class="recommend-card" v-for="job in recommendJobs" :key="job.id" @click="$router.push(`/jobs/${job.id}`)">
          <div class="rec-top">
            <h4>{{ job.title }}</h4>
            <span class="rec-salary">{{ job.salary_range || '面议' }}</span>
          </div>
          <div class="rec-company">{{ job.company_name || '企业用户' }}</div>
          <div class="rec-tags">
            <el-tag size="small" type="info" v-if="job.city">{{ job.city }}</el-tag>
            <el-tag size="small" type="info" v-if="job.education_req">{{ job.education_req }}</el-tag>
          </div>
        </div>
      </div>
    </div>

    <el-dialog v-model="showGreetingDialog" title="发起联系" width="460px" destroy-on-close>
      <p style="margin-bottom:12px;color:var(--text-secondary);font-size:14px;">请输入一段招呼语，让求职者了解您的来意：</p>
      <el-input v-model="greetingText" type="textarea" :rows="3" placeholder="例如：您好，我们公司正在招聘XX岗位，觉得您很合适..." maxlength="200" show-word-limit />
      <template #footer>
        <el-button @click="showGreetingDialog = false" round>取消</el-button>
        <el-button type="primary" :loading="contacting" @click="handleContact" round>发送请求</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { seekerApi, chatApi, jobApi } from '../api'
import { useUserStore } from '../store/user'
import { ArrowLeft, ChatDotRound, Money, Location, Timer, Briefcase, Phone } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

const route = useRoute()
const userStore = useUserStore()
const loading = ref(false)
const contacting = ref(false)
const profile = ref(null)
const showGreetingDialog = ref(false)
const greetingText = ref('')
const recommendJobs = ref([])

function formatTime(t) {
  if (!t) return ''
  const d = new Date(String(t).replace(' ', 'T'))
  if (isNaN(d.getTime())) return ''
  const now = new Date()
  const diff = Math.floor((now - d) / 60000)
  if (diff < 1) return '刚刚'
  if (diff < 60) return `${diff}分钟前`
  if (diff < 1440) return `${Math.floor(diff / 60)}小时前`
  if (diff < 43200) return `${Math.floor(diff / 1440)}天前`
  return d.toLocaleDateString('zh-CN')
}

onMounted(async () => {
  loading.value = true
  try {
    profile.value = await seekerApi.detail(route.params.id)
    // 加载推荐职位（按求职者期望城市匹配）
    const keyword = profile.value?.expected_city || ''
    const res = await jobApi.list({ page: 1, size: 3, keyword })
    recommendJobs.value = res.list || []
  } finally { loading.value = false }
})

async function handleContact() {
  if (!profile.value) return
  contacting.value = true
  try {
    await chatApi.contact({ seeker_user_id: profile.value.user_id, greeting: greetingText.value || undefined })
    ElMessage({ message: '已发起联系请求，等待对方回复', type: 'success', duration: 3000, showClose: true })
    showGreetingDialog.value = false
    greetingText.value = ''
  } catch (e) {} finally { contacting.value = false }
}
</script>

<style lang="scss" scoped>
.back-btn { margin-bottom: 20px; }
.detail-card { animation: fadeUp 0.5s ease; }
@keyframes fadeUp { from { opacity: 0; transform: translateY(12px); } to { opacity: 1; transform: translateY(0); } }

.profile-top {
  display: flex;
  align-items: center;
  gap: 20px;
  margin-bottom: 28px;
}

.profile-avatar {
  width: 72px; height: 72px; border-radius: 20px;
  background: linear-gradient(135deg, #2563EB, #7C3AED);
  color: #fff; display: flex; align-items: center; justify-content: center;
  font-size: 32px; font-weight: 700; flex-shrink: 0;
}

.profile-main {
  flex: 1;
  h1 { font-size: 24px; font-weight: 700; margin-bottom: 4px; }
  .profile-sub { font-size: 14px; color: var(--text-secondary); }
}

.info-cards {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.info-card {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 18px 20px;
  background: #F8FAFC;
  border-radius: 12px;
  border: 1px solid var(--border-light);

  .info-icon {
    width: 44px; height: 44px; border-radius: 12px;
    display: flex; align-items: center; justify-content: center;
    color: #fff; font-size: 20px;
    &.blue { background: linear-gradient(135deg, #2563EB, #3B82F6); }
    &.green { background: linear-gradient(135deg, #059669, #10B981); }
    &.purple { background: linear-gradient(135deg, #7C3AED, #8B5CF6); }
    &.orange { background: linear-gradient(135deg, #D97706, #F59E0B); }
  }

  .info-label { font-size: 12px; color: var(--text-muted); margin-bottom: 2px; }
  .info-value { font-size: 16px; font-weight: 600; color: var(--text-primary); }
  .salary { color: #EF4444; }
}

.skills-section {
  margin-bottom: 8px;
  h3 { font-size: 15px; font-weight: 600; margin-bottom: 10px; }
  .skills-wrap { display: flex; flex-wrap: wrap; gap: 6px; }
}

.desc-title { font-size: 17px; font-weight: 600; margin-bottom: 14px; }
.description { white-space: pre-wrap; line-height: 2; color: var(--text-secondary); font-size: 15px; }

.detail-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.status-badge {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--text-secondary);
  .dot {
    width: 8px; height: 8px; border-radius: 50%;
    &.green { background: #10B981; box-shadow: 0 0 6px rgba(16,185,129,0.4); }
  }
}

.update-time { font-size: 13px; color: var(--text-muted); }

.recommend-section {
  margin-top: 20px;
  animation: fadeUp 0.5s ease 0.2s both;
}

.recommend-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 16px;
  display: flex;
  align-items: center;
  gap: 8px;
  .el-icon { color: var(--primary); }
}

.recommend-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 14px;
}

.recommend-card {
  background: #fff;
  border-radius: 12px;
  padding: 18px;
  cursor: pointer;
  border: 1px solid var(--border-light);
  transition: all 0.25s;
  box-shadow: var(--shadow-sm);
  &:hover { transform: translateY(-3px); box-shadow: 0 8px 24px rgba(0,0,0,0.08); border-color: #DBEAFE; }
  .rec-top { display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 6px; }
  h4 { font-size: 15px; font-weight: 600; color: #111827; flex: 1; margin-right: 10px; }
  .rec-salary { font-size: 15px; font-weight: 700; color: #E85D04; white-space: nowrap; }
  .rec-company { font-size: 13px; color: #6B7280; margin-bottom: 8px; }
  .rec-tags { display: flex; gap: 6px; }
}

@media (max-width: 768px) {
  .recommend-grid { grid-template-columns: 1fr; }
}
</style>
