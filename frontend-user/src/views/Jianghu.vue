<template>
  <div class="page-container">
    <h2 class="section-title">江湖说</h2>

    <div class="card post-form">
      <div class="form-top">
        <el-avatar :size="40" class="form-avatar">{{ userStore.isLoggedIn ? userStore.user?.username?.charAt(0)?.toUpperCase() : '匿' }}</el-avatar>
        <el-input v-model="newContent" type="textarea" :rows="3" :placeholder="userStore.isLoggedIn ? '分享你的职场见闻、行业动态...' : '匿名发布你的职场见闻、行业动态...'" maxlength="500" show-word-limit resize="none" />
      </div>
      <div class="form-bottom">
        <div class="form-bottom-left">
          <label class="upload-trigger" v-if="userStore.isLoggedIn">
            <el-icon><Picture /></el-icon> 图片
            <input type="file" accept="image/*" hidden @change="handleJianghuUpload" />
          </label>
          <span v-if="!userStore.isLoggedIn" class="anon-hint">未登录将以匿名身份发布</span>
        </div>
        <el-button type="primary" :loading="posting" @click="handlePost" :disabled="!newContent.trim()" round>
          <el-icon><Promotion /></el-icon> 发布
        </el-button>
      </div>
      <div v-if="newImages.length" class="upload-area" style="margin-top:10px;">
        <div v-for="(img, idx) in newImages" :key="idx" class="upload-preview">
          <img :src="img" alt="preview" />
          <span class="upload-remove" @click="newImages.splice(idx, 1)">&times;</span>
        </div>
      </div>
    </div>

    <div v-loading="loading" v-if="loading" style="min-height:200px"></div>

    <transition-group name="list" tag="div">
      <div v-for="item in list" :key="item.id" class="card post-card">
        <div class="post-header">
          <el-avatar :size="44" class="post-avatar">{{ item.username?.charAt(0)?.toUpperCase() }}</el-avatar>
          <div class="post-user">
            <span class="name">{{ item.username }}</span>
            <span class="time">{{ relativeTime(item.created_at) }}</span>
          </div>
          <span v-if="userStore.user?.id && userStore.user?.id === item.user_id" class="action-text danger" @click="handleDelete(item.id)">删除</span>
        </div>
        <p class="post-content">{{ item.content }}</p>
        <div v-if="item.images && item.images.length" class="post-images">
          <el-image v-for="(img, idx) in item.images" :key="idx" :src="img" :preview-src-list="item.images" :initial-index="idx" fit="cover" class="post-img" lazy />
        </div>
        <div class="post-actions">
          <div class="post-action" :class="{ active: item.liked }" @click="handleLike(item)">
            <el-icon><Star /></el-icon>
            <span>{{ item.like_count || '点赞' }}</span>
          </div>
          <div class="post-action" @click="toggleComments(item)">
            <el-icon><ChatDotSquare /></el-icon>
            <span>{{ item.comment_count || '评论' }}</span>
          </div>
        </div>

        <transition name="fade">
          <div class="comments-section" v-if="item.showComments">
            <div v-for="c in item.comments" :key="c.id" class="comment-item">
              <el-avatar :size="28" class="comment-avatar">{{ c.username?.charAt(0)?.toUpperCase() }}</el-avatar>
              <div class="comment-body">
                <span class="comment-user">{{ c.username }}</span>
                <span class="comment-text">{{ c.content }}</span>
              </div>
            </div>
            <div v-if="item.comments.length === 0" class="no-comments">暂无评论，来说两句吧</div>
            <div class="comment-input" v-if="userStore.isLoggedIn">
              <el-input v-model="item.newComment" placeholder="写评论..." size="default" @keyup.enter="handleComment(item)">
                <template #append>
                  <el-button @click="handleComment(item)" :disabled="!item.newComment?.trim()">发送</el-button>
                </template>
              </el-input>
            </div>
          </div>
        </transition>
      </div>
    </transition-group>

    <div v-if="list.length === 0 && !loading" class="empty-state">
      <el-icon><ChatDotRound /></el-icon>
      <p>还没有人发布动态，来做第一个吧</p>
    </div>

    <div class="pagination-wrap" v-if="total > query.size">
      <el-pagination background layout="total, prev, pager, next" :total="total" :page-size="query.size" v-model:current-page="query.page" @current-change="loadData" />
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { jianghuApi, uploadApi } from '../api'
import { useUserStore } from '../store/user'
import { ChatDotRound, ChatDotSquare, Star, Promotion, Picture } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { relativeTime } from '../utils/format'

const userStore = useUserStore()
const loading = ref(false)
const posting = ref(false)
const list = ref([])
const total = ref(0)
const query = ref({ page: 1, size: 10 })
const newContent = ref('')
const newImages = ref([])

async function loadData() {
  loading.value = true
  try {
    const res = await jianghuApi.list(query.value)
    list.value = res.list.map(p => ({ ...p, showComments: false, comments: [], newComment: '' }))
    total.value = res.total
  } finally { loading.value = false }
}

async function handlePost() {
  if (!newContent.value.trim()) return
  posting.value = true
  try {
    await jianghuApi.create({ content: newContent.value, images: newImages.value.length ? newImages.value : [] })
    newContent.value = ''
    newImages.value = []
    ElMessage({ message: '发布成功', type: 'success', duration: 2000, showClose: true })
    loadData()
  } finally { posting.value = false }
}

async function handleJianghuUpload(event) {
  const file = event.target.files?.[0]
  if (!file) return
  if (newImages.value.length >= 9) { ElMessage.warning('最多上传9张图片'); return }
  try {
    const urls = await uploadApi.upload(file)
    if (urls && urls.length > 0) {
      newImages.value.push(urls[0])
    }
  } catch (e) {
    ElMessage.error('图片上传失败')
  }
  event.target.value = ''
}

async function handleLike(item) {
  if (!userStore.isLoggedIn) { ElMessage({ message: '请先登录', type: 'warning', duration: 2000 }); return }
  const liked = await jianghuApi.like(item.id)
  item.liked = liked
  item.like_count += liked ? 1 : -1
}

async function toggleComments(item) {
  item.showComments = !item.showComments
  if (item.showComments && item.comments.length === 0) {
    item.comments = await jianghuApi.comments(item.id)
  }
}

async function handleComment(item) {
  if (!item.newComment?.trim()) return
  await jianghuApi.comment(item.id, { content: item.newComment })
  item.newComment = ''
  item.comment_count++
  item.comments = await jianghuApi.comments(item.id)
  ElMessage({ message: '评论成功', type: 'success', duration: 1500 })
}

async function handleDelete(id) {
  try {
    await ElMessageBox.confirm('确定要删除这条动态吗？删除后不可恢复。', '删除确认', {
      confirmButtonText: '确定删除',
      cancelButtonText: '取消',
      type: 'warning',
      confirmButtonClass: 'el-button--danger'
    })
    await jianghuApi.delete(id)
    ElMessage({ message: '已删除', type: 'success', duration: 2000 })
    loadData()
  } catch (e) { /* cancelled */ }
}

onMounted(loadData)
</script>

<style lang="scss" scoped>
.post-form {
  .form-top {
    display: flex;
    gap: 14px;
    align-items: flex-start;
  }
  .form-avatar {
    background: linear-gradient(135deg, var(--primary), #7C3AED);
    color: #fff;
    font-weight: 600;
    flex-shrink: 0;
  }
  .form-bottom {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 14px;
    .form-bottom-left {
      display: flex;
      align-items: center;
      gap: 8px;
    }
    .anon-hint {
      font-size: 12px;
      color: #9ca3af;
    }
  }
}

.list-enter-active { transition: all 0.4s ease; }
.list-enter-from { opacity: 0; transform: translateY(16px); }

.post-card {
  .post-header {
    display: flex;
    align-items: center;
    gap: 14px;
    margin-bottom: 14px;

    .post-avatar {
      background: linear-gradient(135deg, #6366F1, #8B5CF6);
      color: #fff;
      font-weight: 600;
      flex-shrink: 0;
    }

    .post-user {
      flex: 1;
      .name { display: block; font-weight: 600; font-size: 15px; color: var(--text-primary); }
      .time { font-size: 12px; color: var(--text-muted); }
    }
  }

  .post-content {
    font-size: 15px;
    line-height: 1.8;
    margin-bottom: 16px;
    white-space: pre-wrap;
    color: var(--text-primary);
  }

  .post-actions {
    display: flex;
    gap: 8px;
    padding-top: 14px;
    border-top: 1px solid var(--border-light);
  }

  .post-action {
    display: flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
    color: var(--text-muted);
    font-size: 14px;
    padding: 6px 16px;
    border-radius: var(--radius-sm);
    transition: var(--transition);

    &:hover {
      background: #F3F4F6;
      color: var(--text-secondary);
    }

    &.active {
      color: #F59E0B;
      background: #FFFBEB;
    }
  }
}

.comments-section {
  margin-top: 14px;
  padding: 16px;
  background: #F8FAFC;
  border-radius: 12px;

  .comment-item {
    display: flex;
    gap: 10px;
    padding: 10px 0;
    border-bottom: 1px solid #F0F0F0;

    &:last-of-type { border-bottom: none; }

    .comment-avatar {
      background: linear-gradient(135deg, #10B981, #059669);
      color: #fff;
      font-size: 11px;
      font-weight: 600;
      flex-shrink: 0;
    }

    .comment-body {
      .comment-user { font-weight: 600; font-size: 13px; color: var(--primary); margin-right: 8px; }
      .comment-text { font-size: 14px; color: var(--text-primary); line-height: 1.6; }
    }
  }

  .no-comments {
    text-align: center;
    color: var(--text-muted);
    font-size: 13px;
    padding: 16px 0;
  }

  .comment-input { margin-top: 12px; }
}

.pagination-wrap { display: flex; justify-content: center; margin-top: 28px; }

.upload-trigger {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
  color: var(--text-muted);
  font-size: 14px;
  padding: 6px 12px;
  border-radius: var(--radius-sm);
  transition: var(--transition);
  &:hover { background: #F3F4F6; color: var(--primary); }
}

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
}

.post-images {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 14px;

  .post-img {
    width: 120px;
    height: 120px;
    object-fit: cover;
    border-radius: 8px;
    border: 1px solid var(--border-light);
    cursor: pointer;
  }
}
</style>
