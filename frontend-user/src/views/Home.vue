<template>
  <div class="home-page">
    <section class="hero">
      <div class="hero-bg">
        <div class="hero-circle c1"></div>
        <div class="hero-circle c2"></div>
        <div class="hero-circle c3"></div>
      </div>
      <div class="hero-content">
        <h1 class="hero-title">找到理想的<span class="highlight">工作</span>与<span class="highlight">人才</span></h1>
        <p class="hero-subtitle">个人免费发布求职信息，企业认证后发布招聘，连接每一个机遇</p>
        <div class="hero-actions">
          <el-button type="primary" size="large" round @click="$router.push('/jobs')">
            <el-icon><Search /></el-icon> 浏览职位
          </el-button>
          <el-button size="large" round class="btn-ghost" @click="$router.push('/seekers')">
            <el-icon><UserFilled /></el-icon> 浏览人才
          </el-button>
        </div>
        <div class="hero-stats">
          <div class="stat-item">
            <span class="stat-num">1000+</span>
            <span class="stat-label">活跃职位</span>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <span class="stat-num">500+</span>
            <span class="stat-label">认证企业</span>
          </div>
          <div class="stat-divider"></div>
          <div class="stat-item">
            <span class="stat-num">2000+</span>
            <span class="stat-label">求职者</span>
          </div>
        </div>
      </div>
    </section>

    <div class="home-container">
      <div class="feature-grid">
        <div class="feature-card" v-for="(f, i) in features" :key="i" @click="$router.push(f.path)" :style="{ animationDelay: `${i * 0.1}s` }">
          <div class="feature-icon" :class="f.color">
            <el-icon :size="30"><component :is="f.icon" /></el-icon>
          </div>
          <h3>{{ f.title }}</h3>
          <p>{{ f.desc }}</p>
          <div class="feature-arrow">
            <el-icon><ArrowRight /></el-icon>
          </div>
        </div>
      </div>
    </div>

    <!-- 最新职位 -->
    <section class="section">
      <div class="home-container">
        <div class="section-header">
          <h2><el-icon><Briefcase /></el-icon> 最新职位</h2>
          <el-button text type="primary" @click="$router.push('/jobs')">查看全部 <el-icon><ArrowRight /></el-icon></el-button>
        </div>
        <div class="job-grid" v-if="latestJobs.length">
          <div class="job-card" v-for="job in latestJobs" :key="job.id" @click="$router.push(`/jobs/${job.id}`)">
            <div class="job-top">
              <h3 class="job-title">{{ job.title }}</h3>
              <span class="job-salary">{{ job.salary_range || '面议' }}</span>
            </div>
            <div class="job-company">{{ job.company_name || '企业用户' }}</div>
            <div class="job-tags">
              <el-tag size="small" type="info" v-if="job.city">{{ job.city }}</el-tag>
              <el-tag size="small" type="info" v-if="job.job_type">{{ ['全职','兼职','实习'][job.job_type] || '其他' }}</el-tag>
            </div>
          </div>
        </div>
        <el-empty v-else description="暂无职位，快来发布第一个吧" :image-size="80" />
      </div>
    </section>

    <!-- 热门求职者 -->
    <section class="section section-alt">
      <div class="home-container">
        <div class="section-header">
          <h2><el-icon><User /></el-icon> 优秀人才</h2>
          <el-button text type="primary" @click="$router.push('/seekers')">查看全部 <el-icon><ArrowRight /></el-icon></el-button>
        </div>
        <div class="seeker-grid" v-if="latestSeekers.length">
          <div class="seeker-card" v-for="s in latestSeekers" :key="s.id" @click="$router.push(`/seekers/${s.id}`)">
            <el-avatar :size="52" class="seeker-avatar">{{ s.real_name?.charAt(0) || '?' }}</el-avatar>
            <div class="seeker-info">
              <h4>{{ s.real_name || '求职者' }}</h4>
              <p class="seeker-skills">{{ s.skills || '暂无技能标签' }}</p>
              <div class="seeker-meta">
                <span v-if="s.city"><el-icon><Location /></el-icon> {{ s.city }}</span>
                <span v-if="s.expected_salary" class="salary-tag">期望 {{ s.expected_salary }}</span>
              </div>
            </div>
          </div>
        </div>
        <el-empty v-else description="暂无求职者档案" :image-size="80" />
      </div>
    </section>

    <!-- 江湖说动态 -->
    <section class="section">
      <div class="home-container">
        <div class="section-header">
          <h2><el-icon><ChatDotRound /></el-icon> 江湖说 · 最新动态</h2>
          <el-button text type="primary" @click="$router.push('/jianghu')">查看全部 <el-icon><ArrowRight /></el-icon></el-button>
        </div>
        <div class="jianghu-grid" v-if="latestPosts.length">
          <div class="jianghu-card" v-for="p in latestPosts" :key="p.id">
            <div class="jianghu-header">
              <el-avatar :size="36" class="jianghu-avatar">{{ p.username?.charAt(0)?.toUpperCase() }}</el-avatar>
              <div>
                <span class="jianghu-user">{{ p.username }}</span>
                <span class="jianghu-time">{{ formatTime(p.created_at) }}</span>
              </div>
            </div>
            <p class="jianghu-content">{{ p.content?.length > 120 ? p.content.slice(0, 120) + '...' : p.content }}</p>
            <div class="jianghu-footer">
              <span><el-icon><Star /></el-icon> {{ p.like_count || 0 }}</span>
              <span><el-icon><ChatDotRound /></el-icon> {{ p.comment_count || 0 }}</span>
            </div>
          </div>
        </div>
        <el-empty v-else description="还没有人发布动态，来做第一个吧" :image-size="80" />
      </div>
    </section>

    <!-- 平台优势 -->
    <section class="section section-alt">
      <div class="home-container">
        <h2 class="center-title">为什么选择我们</h2>
        <div class="advantage-grid">
          <div class="advantage-item" v-for="(a, i) in advantages" :key="i">
            <div class="adv-icon">{{ a.emoji }}</div>
            <h4>{{ a.title }}</h4>
            <p>{{ a.desc }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- Footer -->
    <footer class="home-footer">
      <div class="footer-inner">
        <div class="footer-brand">
          <h3>人才网</h3>
          <p>连接人才与机遇的桥梁</p>
        </div>
        <div class="footer-links">
          <div class="footer-col">
            <h4>求职者</h4>
            <router-link to="/jobs">浏览职位</router-link>
            <router-link to="/register">免费注册</router-link>
            <router-link to="/profile">个人档案</router-link>
          </div>
          <div class="footer-col">
            <h4>企业</h4>
            <router-link to="/seekers">浏览人才</router-link>
            <router-link to="/profile">企业认证</router-link>
            <router-link to="/market">好市场</router-link>
          </div>
          <div class="footer-col">
            <h4>社区</h4>
            <router-link to="/jianghu">江湖说</router-link>
            <router-link to="/chat">在线沟通</router-link>
          </div>
        </div>
      </div>
      <div class="footer-bottom">
        <span>© 2024 人才网 · 让每一次连接都有价值</span>
      </div>
    </footer>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { Search, UserFilled, Briefcase, User, ChatDotRound, Shop, ArrowRight, Star, Location } from '@element-plus/icons-vue'
import { jobApi, seekerApi, jianghuApi } from '../api'

const features = [
  { title: '找工作', desc: '海量优质职位，精准匹配你的技能', path: '/jobs', icon: 'Briefcase', color: 'blue' },
  { title: '找人才', desc: '浏览求职者档案，发现合适的候选人', path: '/seekers', icon: 'User', color: 'green' },
  { title: '江湖说', desc: '分享职场见闻，交流行业动态', path: '/jianghu', icon: 'ChatDotRound', color: 'purple' },
  { title: '好市场', desc: '门店转让、二手交易，一站搞定', path: '/market', icon: 'Shop', color: 'orange' }
]

const advantages = [
  { emoji: '🆓', title: '免费发布', desc: '求职者完全免费发布个人档案，零门槛入驻' },
  { emoji: '🔒', title: '安全可靠', desc: '企业实名认证，聊天限制防骚扰，保障双方权益' },
  { emoji: '💬', title: '即时沟通', desc: '内置聊天系统，企业与求职者高效对接' },
  { emoji: '🏪', title: '一站服务', desc: '招聘、社区、二手市场，满足多元需求' }
]

const latestJobs = ref([])
const latestSeekers = ref([])
const latestPosts = ref([])

function formatTime(t) {
  if (!t) return ''
  const d = new Date(t.replace(' ', 'T'))
  if (isNaN(d.getTime())) return ''
  const now = new Date()
  const diff = Math.floor((now - d) / 60000)
  if (diff < 1) return '刚刚'
  if (diff < 60) return `${diff}分钟前`
  if (diff < 1440) return `${Math.floor(diff / 60)}小时前`
  return `${Math.floor(diff / 1440)}天前`
}

onMounted(async () => {
  try { const r = await jobApi.list({ page: 1, size: 6 }); latestJobs.value = r.list || [] } catch {}
  try { const r = await seekerApi.list({ page: 1, size: 4 }); latestSeekers.value = r.list || [] } catch {}
  try { const r = await jianghuApi.list({ page: 1, size: 3 }); latestPosts.value = r.list || [] } catch {}
})
</script>

<style lang="scss" scoped>
.hero {
  background: linear-gradient(135deg, #0F172A 0%, #1E3A5F 50%, #1E40AF 100%);
  padding: 100px 20px 80px;
  text-align: center;
  color: #fff;
  position: relative;
  overflow: hidden;
}

.hero-bg {
  position: absolute; inset: 0;
  .hero-circle { position: absolute; border-radius: 50%; opacity: 0.06; background: #fff; }
  .c1 { width: 500px; height: 500px; top: -200px; right: -100px; animation: heroFloat 20s ease-in-out infinite; }
  .c2 { width: 300px; height: 300px; bottom: -80px; left: -50px; animation: heroFloat 16s ease-in-out infinite reverse; }
  .c3 { width: 150px; height: 150px; top: 30%; left: 15%; animation: heroFloat 12s ease-in-out infinite; }
}

@keyframes heroFloat {
  0%, 100% { transform: translate(0, 0); }
  50% { transform: translate(20px, -15px); }
}

.hero-content {
  max-width: 680px; margin: 0 auto; position: relative; z-index: 1;
  animation: fadeUp 0.8s cubic-bezier(0.16, 1, 0.3, 1);
}

@keyframes fadeUp {
  from { opacity: 0; transform: translateY(24px); }
  to { opacity: 1; transform: translateY(0); }
}

.hero-title {
  font-size: 44px; font-weight: 800; margin-bottom: 16px; letter-spacing: -1px; line-height: 1.2;
  .highlight {
    background: linear-gradient(135deg, #60A5FA, #A78BFA);
    -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;
  }
}

.hero-subtitle { font-size: 17px; opacity: 0.75; margin-bottom: 36px; line-height: 1.6; }

.hero-actions {
  display: flex; gap: 14px; justify-content: center; margin-bottom: 48px;
  .btn-ghost {
    background: rgba(255,255,255,0.12); border: 1px solid rgba(255,255,255,0.25); color: #fff;
    &:hover { background: rgba(255,255,255,0.2); border-color: rgba(255,255,255,0.4); }
  }
}

.hero-stats {
  display: flex; align-items: center; justify-content: center; gap: 32px;
  .stat-item {
    .stat-num { display: block; font-size: 28px; font-weight: 800; letter-spacing: -0.5px; }
    .stat-label { font-size: 13px; opacity: 0.6; }
  }
  .stat-divider { width: 1px; height: 36px; background: rgba(255,255,255,0.15); }
}

.feature-grid {
  display: grid; grid-template-columns: repeat(4, 1fr); gap: 20px;
  margin-top: -48px; margin-bottom: 8px; position: relative; z-index: 2;
}

.feature-card {
  background: #fff; border-radius: 16px; padding: 32px 24px 28px; text-align: center;
  box-shadow: 0 4px 24px rgba(0,0,0,0.06); cursor: pointer;
  transition: all 0.35s cubic-bezier(0.4,0,0.2,1); border: 1px solid transparent;
  position: relative; overflow: hidden; animation: cardUp 0.6s cubic-bezier(0.16,1,0.3,1) both;
  &:hover {
    transform: translateY(-6px); box-shadow: 0 12px 40px rgba(0,0,0,0.1); border-color: #E5E7EB;
    .feature-arrow { opacity: 1; transform: translateX(0); }
    .feature-icon { transform: scale(1.08); }
  }
  h3 { font-size: 18px; margin: 18px 0 8px; color: #111827; font-weight: 700; }
  p { font-size: 13px; color: #6B7280; line-height: 1.5; }
}

@keyframes cardUp {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}

.feature-icon {
  width: 64px; height: 64px; border-radius: 18px; display: flex; align-items: center;
  justify-content: center; margin: 0 auto; color: #fff; transition: transform 0.3s;
  &.blue { background: linear-gradient(135deg, #2563EB, #3B82F6); box-shadow: 0 6px 20px rgba(37,99,235,0.25); }
  &.green { background: linear-gradient(135deg, #059669, #10B981); box-shadow: 0 6px 20px rgba(16,185,129,0.25); }
  &.purple { background: linear-gradient(135deg, #7C3AED, #8B5CF6); box-shadow: 0 6px 20px rgba(124,58,237,0.25); }
  &.orange { background: linear-gradient(135deg, #D97706, #F59E0B); box-shadow: 0 6px 20px rgba(245,158,11,0.25); }
}

.feature-arrow {
  position: absolute; right: 20px; bottom: 20px; opacity: 0;
  transform: translateX(-8px); transition: all 0.3s; color: var(--text-muted);
}

/* Sections */
.section { padding: 56px 0; }
.section-alt { background: #F8FAFC; }

.home-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
  box-sizing: border-box;
}

.section-header {
  display: flex; justify-content: space-between; align-items: center; margin-bottom: 28px;
  h2 {
    font-size: 22px; font-weight: 700; color: #111827;
    display: flex; align-items: center; gap: 8px;
    .el-icon { color: var(--primary); }
  }
}

.center-title {
  text-align: center; font-size: 24px; font-weight: 700; color: #111827; margin-bottom: 40px;
}

/* Job cards */
.job-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px; }

.job-card {
  background: #fff; border-radius: 12px; padding: 20px; cursor: pointer;
  border: 1px solid var(--border-light); transition: all 0.25s;
  &:hover { transform: translateY(-3px); box-shadow: 0 8px 24px rgba(0,0,0,0.08); border-color: #DBEAFE; }
  .job-top { display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 8px; }
  .job-title { font-size: 15px; font-weight: 600; color: #111827; flex: 1; margin-right: 12px; }
  .job-salary { font-size: 15px; font-weight: 700; color: #E85D04; white-space: nowrap; }
  .job-company { font-size: 13px; color: #6B7280; margin-bottom: 10px; }
  .job-tags { display: flex; gap: 6px; }
}

/* Seeker cards */
.seeker-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px; }

.seeker-card {
  background: #fff; border-radius: 12px; padding: 20px; cursor: pointer;
  border: 1px solid var(--border-light); transition: all 0.25s;
  display: flex; gap: 14px; align-items: flex-start; min-width: 0; overflow: hidden;
  &:hover { transform: translateY(-3px); box-shadow: 0 8px 24px rgba(0,0,0,0.08); border-color: #D1FAE5; }
  .seeker-avatar {
    background: linear-gradient(135deg, #059669, #10B981); color: #fff;
    font-weight: 600; flex-shrink: 0;
  }
  .seeker-info { flex: 1; min-width: 0; overflow: hidden; }
  h4 { font-size: 15px; font-weight: 600; color: #111827; margin-bottom: 4px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .seeker-skills {
    font-size: 12px; color: #6B7280; margin-bottom: 8px;
    overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
  }
  .seeker-meta {
    display: flex; gap: 10px; font-size: 12px; color: #9CA3AF; overflow: hidden;
    .el-icon { font-size: 13px; vertical-align: -1px; }
    .salary-tag { color: #E85D04; white-space: nowrap; }
  }
}

/* Jianghu cards */
.jianghu-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px; }

.jianghu-card {
  background: #fff; border-radius: 12px; padding: 20px;
  border: 1px solid var(--border-light); transition: all 0.25s;
  &:hover { box-shadow: 0 8px 24px rgba(0,0,0,0.06); border-color: #E9D5FF; }
  .jianghu-header {
    display: flex; gap: 10px; align-items: center; margin-bottom: 12px;
    .jianghu-avatar { background: linear-gradient(135deg, #7C3AED, #8B5CF6); color: #fff; font-weight: 600; flex-shrink: 0; }
    .jianghu-user { font-size: 14px; font-weight: 600; color: #111827; display: block; }
    .jianghu-time { font-size: 12px; color: #9CA3AF; }
  }
  .jianghu-content { font-size: 14px; color: #374151; line-height: 1.7; margin-bottom: 14px; }
  .jianghu-footer {
    display: flex; gap: 16px; font-size: 13px; color: #9CA3AF;
    .el-icon { vertical-align: -2px; margin-right: 3px; }
  }
}

/* Advantages */
.advantage-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 24px; }

.advantage-item {
  text-align: center; padding: 28px 16px;
  .adv-icon { font-size: 36px; margin-bottom: 14px; }
  h4 { font-size: 16px; font-weight: 700; color: #111827; margin-bottom: 8px; }
  p { font-size: 13px; color: #6B7280; line-height: 1.6; }
}

/* Footer */
.home-footer {
  background: #0F172A; color: #94A3B8; padding: 64px 20px 0;
  .footer-inner {
    max-width: 1200px; margin: 0 auto; display: flex;
    justify-content: space-between; gap: 48px;
    padding-bottom: 48px; border-bottom: 1px solid rgba(255,255,255,0.08);
  }
  .footer-brand {
    h3 { font-size: 22px; font-weight: 800; color: #fff; margin-bottom: 8px; }
    p { font-size: 14px; opacity: 0.6; }
  }
  .footer-links { display: flex; gap: 56px; }
  .footer-col {
    display: flex; flex-direction: column; gap: 10px;
    h4 { font-size: 14px; font-weight: 600; color: #E2E8F0; margin-bottom: 4px; }
    a { font-size: 13px; color: #94A3B8; text-decoration: none; transition: color 0.2s; &:hover { color: #fff; } }
  }
  .footer-bottom {
    max-width: 1200px; margin: 0 auto; padding: 24px 0;
    text-align: center; font-size: 13px; opacity: 0.5;
  }
}

@media (max-width: 768px) {
  .feature-grid { grid-template-columns: repeat(2, 1fr); }
  .job-grid { grid-template-columns: 1fr; }
  .seeker-grid { grid-template-columns: repeat(2, 1fr); }
  .jianghu-grid { grid-template-columns: 1fr; }
  .advantage-grid { grid-template-columns: repeat(2, 1fr); }
  .hero-title { font-size: 30px; }
  .hero-stats { gap: 20px; .stat-item .stat-num { font-size: 22px; } }
  .footer-inner { flex-direction: column; gap: 32px; }
  .footer-links { gap: 24px; }
}
</style>
