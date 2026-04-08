<template>
  <div class="page-container">
    <h2 class="section-title">消息</h2>
    <div class="chat-layout">
      <div class="contact-list">
        <div class="contact-search">
          <el-input placeholder="搜索联系人" size="default" disabled>
            <template #prefix><el-icon><Search /></el-icon></template>
          </el-input>
        </div>
        <div v-if="contacts.length === 0" class="empty-contacts">
          <el-icon :size="36"><ChatDotRound /></el-icon>
          <p>暂无联系人</p>
        </div>
        <div v-for="c in contacts" :key="c.id" class="contact-item" :class="{ active: activeContact?.id === c.id }" @click="selectContact(c)">
          <el-avatar :size="44" class="contact-avatar">{{ c.other_username?.charAt(0)?.toUpperCase() }}</el-avatar>
          <div class="contact-info">
            <div class="contact-name-row">
              <span class="contact-name">{{ c.other_username }}</span>
              <el-badge :value="c.unread_count" v-if="c.unread_count > 0" :max="99" class="unread-badge" />
            </div>
            <div class="contact-status-row">
              <span v-if="c.status === 0" class="status-dot pending"></span>
              <span v-else-if="c.status === 1" class="status-dot accepted"></span>
              <span v-else class="status-dot rejected"></span>
              <span class="status-text">{{ ['待回复', '已接受', '已拒绝'][c.status] }}</span>
            </div>
          </div>
        </div>
      </div>

      <div class="chat-area">
        <template v-if="activeContact">
          <div class="chat-header">
            <h3>{{ activeContact.other_username }}</h3>
            <div v-if="activeContact.status === 0 && userStore.isSeeker" class="reply-actions">
              <el-button type="success" size="small" round @click="handleReply(true)">
                <el-icon><Check /></el-icon> 接受
              </el-button>
              <el-button type="danger" size="small" round @click="handleReply(false)">
                <el-icon><Close /></el-icon> 拒绝
              </el-button>
            </div>
            <span v-else-if="activeContact.status === 0" class="tag tag-orange">等待对方回复</span>
            <span v-else-if="activeContact.status === 2" class="tag tag-red">已拒绝</span>
          </div>
          <div class="messages" ref="messagesRef">
            <div v-if="activeContact.status !== 1" class="chat-notice">
              <el-icon :size="24"><InfoFilled /></el-icon>
              <span v-if="activeContact.status === 0 && userStore.isSeeker">您可以直接回复消息来接受联系，或点击上方按钮操作</span>
              <span v-else-if="activeContact.status === 0">等待求职者回复后才能开始聊天</span>
              <span v-else>对方已拒绝联系请求</span>
            </div>
            <div v-if="activeContact.greeting" class="greeting-card">
              <div class="greeting-label">招呼语</div>
              <div class="greeting-text">{{ activeContact.greeting }}</div>
            </div>
            <transition-group name="msg-list" tag="div">
              <div v-for="msg in messages" :key="msg.id" class="message" :class="{ mine: msg.sender_id === userStore.user?.id }">
                <div class="bubble">{{ msg.content }}</div>
                <div class="msg-time">{{ formatMsgTime(msg.created_at) }}</div>
              </div>
            </transition-group>
          </div>
          <div class="chat-input" v-if="activeContact.status === 1 || (activeContact.status === 0 && userStore.isSeeker)">
            <el-input v-model="newMessage" :placeholder="activeContact.status === 0 ? '输入消息直接回复（自动接受联系）...' : '输入消息...'" size="large" @keyup.enter="handleSend">
              <template #append>
                <el-button @click="handleSend" :disabled="!newMessage.trim()" type="primary">发送</el-button>
              </template>
            </el-input>
          </div>
        </template>
        <div v-else class="empty-chat">
          <el-icon :size="56"><ChatDotRound /></el-icon>
          <p>选择一个联系人开始聊天</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, nextTick } from 'vue'
import { chatApi } from '../api'
import { useUserStore } from '../store/user'
import { ChatDotRound, Search, Check, Close, InfoFilled } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

const userStore = useUserStore()
const contacts = ref([])
const activeContact = ref(null)
const messages = ref([])
const newMessage = ref('')
const messagesRef = ref(null)

function formatMsgTime(t) {
  if (!t) return ''
  const d = new Date(t.replace(' ', 'T'))
  if (isNaN(d.getTime())) return ''
  return `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`
}

async function loadContacts() {
  contacts.value = await chatApi.contacts()
}

async function selectContact(c) {
  activeContact.value = c
  if (c.status === 1 || (c.status === 0 && userStore.isSeeker)) {
    messages.value = await chatApi.messages(c.id)
    await chatApi.markRead(c.id)
    c.unread_count = 0
    await nextTick()
    scrollToBottom()
  } else {
    messages.value = []
  }
}

function scrollToBottom() {
  if (messagesRef.value) messagesRef.value.scrollTop = messagesRef.value.scrollHeight
}

async function handleReply(accept) {
  await chatApi.reply(activeContact.value.id, { accept })
  ElMessage({ message: accept ? '已接受联系请求' : '已拒绝', type: accept ? 'success' : 'info', duration: 2000, showClose: true })
  activeContact.value.status = accept ? 1 : 2
  if (accept) {
    messages.value = await chatApi.messages(activeContact.value.id)
  }
}

async function handleSend() {
  if (!newMessage.value.trim()) return
  const msg = await chatApi.sendMessage({ contact_id: activeContact.value.id, content: newMessage.value })
  // 求职者直接回复待处理联系时，后端自动接受，前端同步状态
  if (activeContact.value.status === 0) {
    activeContact.value.status = 1
    messages.value = await chatApi.messages(activeContact.value.id)
  }
  messages.value.push(msg)
  newMessage.value = ''
  await nextTick()
  scrollToBottom()
}

onMounted(loadContacts)
</script>

<style lang="scss" scoped>
.chat-layout {
  display: flex;
  gap: 0;
  height: calc(100vh - 180px);
  background: #fff;
  border-radius: var(--radius);
  box-shadow: var(--shadow);
  overflow: hidden;
  border: 1px solid var(--border-light);
}

.contact-list {
  width: 300px;
  border-right: 1px solid var(--border-light);
  display: flex;
  flex-direction: column;
  background: #FAFBFC;
}

.contact-search {
  padding: 16px;
  border-bottom: 1px solid var(--border-light);
}

.empty-contacts {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  gap: 8px;
  p { font-size: 14px; }
}

.contact-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 16px;
  cursor: pointer;
  border-bottom: 1px solid #F3F4F6;
  transition: var(--transition);

  &:hover { background: #F0F4FF; }
  &.active { background: #EBF0FF; border-left: 3px solid var(--primary); }

  .contact-avatar {
    background: linear-gradient(135deg, #6366F1, #8B5CF6);
    color: #fff;
    font-weight: 600;
    flex-shrink: 0;
  }

  .contact-info { flex: 1; min-width: 0; }

  .contact-name-row {
    display: flex;
    align-items: center;
    gap: 8px;
    .contact-name { font-weight: 600; font-size: 14px; color: var(--text-primary); }
  }

  .contact-status-row {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-top: 4px;

    .status-dot {
      width: 7px; height: 7px; border-radius: 50%;
      &.pending { background: #F59E0B; }
      &.accepted { background: #10B981; }
      &.rejected { background: #EF4444; }
    }

    .status-text { font-size: 12px; color: var(--text-muted); }
  }
}

.chat-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: #fff;
}

.chat-header {
  padding: 16px 24px;
  border-bottom: 1px solid var(--border-light);
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: #FAFBFC;
  h3 { font-size: 16px; font-weight: 600; }
}

.messages {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
  background: #F8FAFC;
}

.chat-notice {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: var(--text-muted);
  font-size: 14px;
  padding: 24px;
  background: #fff;
  border-radius: 12px;
  border: 1px dashed var(--border);
}

.greeting-card {
  margin-top: 12px;
  padding: 16px 20px;
  background: #fff;
  border-radius: 12px;
  border: 1px solid #E0E7FF;
  background: linear-gradient(135deg, #EEF2FF, #F5F3FF);

  .greeting-label {
    font-size: 12px;
    color: #6366F1;
    font-weight: 600;
    margin-bottom: 6px;
  }

  .greeting-text {
    font-size: 14px;
    color: var(--text-primary);
    line-height: 1.6;
    white-space: pre-wrap;
  }
}

.msg-list-enter-active { transition: all 0.3s ease; }
.msg-list-enter-from { opacity: 0; transform: translateY(8px); }

.message {
  margin-bottom: 16px;
  animation: msgIn 0.3s ease;

  .bubble {
    display: inline-block;
    padding: 12px 18px;
    border-radius: 16px 16px 16px 4px;
    background: #fff;
    max-width: 70%;
    word-break: break-all;
    line-height: 1.6;
    font-size: 14px;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
    border: 1px solid var(--border-light);
  }

  .msg-time { font-size: 11px; color: var(--text-muted); margin-top: 4px; }

  &.mine {
    text-align: right;
    .bubble {
      background: var(--primary);
      color: #fff;
      border-radius: 16px 16px 4px 16px;
      border: none;
      box-shadow: 0 2px 8px rgba(37, 99, 235, 0.2);
    }
  }
}

@keyframes msgIn {
  from { opacity: 0; transform: translateY(6px); }
  to { opacity: 1; transform: translateY(0); }
}

.chat-input {
  padding: 16px 24px;
  border-top: 1px solid var(--border-light);
  background: #fff;
}

.empty-chat {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  gap: 12px;
  .el-icon { opacity: 0.3; }
  p { font-size: 15px; }
}
</style>
