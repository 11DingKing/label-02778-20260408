# 人才网平台

基于 Rust (Actix-web) + Vue 3 + Element Plus + MySQL 的全栈人才招聘平台。

## 一键启动（Docker）

### 前置要求
- Docker & Docker Compose

### 启动

```bash
docker-compose up --build -d
```

首次构建 Rust 项目约需 3-5 分钟，请耐心等待。启动完成后数据库会自动初始化表结构和测试数据。

### 访问地址

| 服务 | 地址 | 说明 |
|------|------|------|
| 用户端 | http://localhost:8081 | 求职者/企业用户端 |
| 管理后台 | http://localhost:8082 | 管理员后台 |
| 后端 API | http://localhost:8080 | Rust Actix-web API |
| 接口文档 | http://localhost:8080/api-docs | 在线 API 接口文档 |

### 停止 / 清理

```bash
# 停止服务
docker-compose down

# 停止并清除所有数据（重新初始化）
docker-compose down -v
```

> 如果修改了 schema.sql 中的初始数据，需要先 `docker-compose down -v` 清除旧数据卷，再 `docker-compose up --build -d` 重新初始化。

---

## 测试账号

系统预置了以下测试账号，密码见下表，可直接登录体验完整功能。

### 管理员

| 用户名 | 密码 | 说明 |
|--------|------|------|
| admin | admin123 | 管理后台登录，可审核企业、管理用户 |

### 求职者（密码均为 `123456`）

| 用户名 | 技能方向 | 城市 |
|--------|----------|------|
| 张伟 | Java后端开发 | 北京 |
| 李娜 | 数据分析/机器学习 | 上海 |
| 王磊 | 前端架构 | 深圳 |
| 赵敏 | UI/UX设计 | 杭州 |
| 陈晓东 | Go/DevOps | 北京 |
| 刘芳 | 新媒体运营 | 成都 |

### 企业（密码均为 `123456`）

| 用户名 | 公司名称 | 行业 | 城市 |
|--------|----------|------|------|
| 华创科技 | 华创科技有限公司 | 互联网 | 北京 |
| 锦程教育 | 锦程教育科技有限公司 | 教育培训 | 上海 |
| 鼎盛餐饮 | 鼎盛餐饮管理有限公司 | 餐饮 | 深圳 |

---

## 质检测试指南

### 测试流程一：求职者完整体验

1. 打开 http://localhost:8081 ，用 `张伟 / 123456` 登录
2. 点击「个人中心」查看已有的求职档案
3. 点击「找工作」浏览职位列表（已有8个预置职位）
4. 点击任意职位查看详情
5. 点击「江湖说」查看动态，可发布新动态、点赞、评论
6. 点击「好市场」浏览商品信息
7. 点击「消息」查看与华创科技的聊天记录（已有预置对话）

### 测试流程二：企业完整体验

1. 用 `华创科技 / 123456` 登录
2. 点击「个人中心」查看企业档案（已认证通过）
3. 点击「个人中心」→ 职位管理，查看已发布的职位，可新增/编辑/删除
4. 点击「找人才」浏览求职者列表
5. 点击任意求职者查看详情，可发起联系（每天限3人，可附带招呼语）
6. 点击「消息」查看与求职者的聊天
7. 点击「好市场」可发布门店转让或二手物品信息（仅企业可发布）

### 测试流程三：管理后台

1. 打开 http://localhost:8082 ，用 `admin / admin123` 登录
2. 查看仪表盘数据统计
3. 「用户管理」：查看所有用户，可禁用/启用账号
4. 「企业管理」：查看企业列表，可审核企业认证
5. 「操作日志」：查看系统操作记录

### 测试流程四：核心业务规则验证

1. **每日联系限制**：用企业账号联系3个求职者后，第4个会被拒绝（403）
2. **求职者回复机制**：用 `李娜 / 123456` 登录，消息页有一条来自锦程教育的待回复请求，可点击「接受」或直接输入消息回复（自动接受）
3. **企业认证限制**：企业未认证前无法发布职位
4. **市场发布权限**：求职者账号看不到「发布商品」按钮，直接调API也会返回403

---

## 预置数据概览

| 数据类型 | 数量 | 说明 |
|----------|------|------|
| 用户 | 10 | 1管理员 + 6求职者 + 3企业 |
| 求职者档案 | 6 | 涵盖开发、设计、运营等方向 |
| 企业档案 | 3 | 均已通过认证 |
| 招聘职位 | 8 | 涵盖技术、教育、餐饮等行业 |
| 江湖说动态 | 6 | 含评论和点赞数据 |
| 好市场商品 | 5 | 含门店转让和二手物品 |
| 聊天记录 | 3组 | 含已接受和待回复状态 |
| 操作日志 | 9 | 注册、登录、审核等记录 |

---

## 本地开发（可选）

如需本地开发调试而非 Docker 运行：

```bash
# 1. 仅启动数据库
docker-compose up db -d

# 2. 启动后端（需安装 Rust: https://rustup.rs）
cd backend
cp .env.example .env   # 首次需复制环境变量示例文件，按需修改
# 确认 .env 中 DATABASE_URL 的 host 为 127.0.0.1
cargo run

# 3. 启动前端（需安装 Node.js >= 18）
# 确认 vite.config.js 中 proxy target 为 http://localhost:8080
cd frontend-user && npm install && npm run dev
cd frontend-admin && npm install && npm run dev
```

## 运行测试

### 前端单元测试

```bash
cd frontend-user && npx vitest --run
cd frontend-admin && npx vitest --run
```

### 后端集成测试

后端测试为集成测试，直接连接 MySQL 数据库执行真实 API 调用。

```bash
# 前置条件：MySQL 必须运行（可通过 docker-compose up db -d 启动）
cd backend && cargo test -- --test-threads=1
```

**测试数据隔离策略：**
- 每个测试用例使用独立的测试用户名（如 `test_seeker_reg`、`test_chat_ent`），与种子数据互不干扰
- 每个测试在开始前和结束后均调用 `cleanup_test_data()` 清理自身创建的所有关联数据（用户、档案、职位、聊天记录、动态等）
- 使用 `#[serial]` 宏确保测试串行执行，避免并发写入冲突
- 测试使用独立的 JWT 密钥（`test_jwt_secret_key_for_testing`），不影响正常运行的服务

**如需重置测试环境：**
```bash
# 清除数据库并重新初始化
docker-compose down -v
docker-compose up db -d
# 等待数据库就绪后重新运行测试
```

---

## 技术栈

- Backend: Rust + Actix-web 4 + SQLx + MySQL 8.0
- Frontend User: Vue 3 + Vite + Element Plus + Pinia + Axios
- Frontend Admin: Vue 3 + Vite + Element Plus + Pinia + Axios
- Deploy: Docker + Docker Compose
- CI/CD: GitHub Actions（配置文件位于 `.github/workflows/ci.yml`，包含后端编译检查 + Clippy 静态分析 + 集成测试（含 MySQL Service Container）+ 前端构建 + 单元测试）

## 生产环境安全须知

当前配置为开发/演示环境，部署到生产环境前请注意：

1. **JWT 密钥**：`docker-compose.yml` 中的 `JWT_SECRET` 默认值为占位符（`CHANGE_ME...`），首次部署必须替换为高强度随机密钥（建议 64 字符以上，可用 `openssl rand -base64 48` 生成）。推荐通过 Docker secrets 或独立的 `.env` 文件注入，不要在版本控制中明文存储。
2. **数据库密码**：`MYSQL_ROOT_PASSWORD`、`MYSQL_PASSWORD` 同理，生产环境应使用强密码。所有敏感配置均已支持环境变量覆盖（`${VAR:-默认值}` 语法），可通过项目根目录 `.env` 文件或 CI/CD 密钥管理注入。
3. **分环境配置**：建议创建 `docker-compose.prod.yml` 覆盖敏感配置，使用 `docker-compose -f docker-compose.yml -f docker-compose.prod.yml up` 启动。
4. **CORS 策略**：默认仅允许 `localhost:8081` 和 `localhost:8082`（本地开发前端地址）。生产环境应通过 `CORS_ORIGIN` 环境变量设置为实际域名，多个域名用逗号分隔。示例：`CORS_ORIGIN=https://example.com,https://admin.example.com`。
5. **HTTPS**：生产环境应在 Nginx 或负载均衡器层配置 TLS 证书。

生产环境 `.env` 文件示例：
```bash
JWT_SECRET=your_64_char_random_secret_here_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
MYSQL_ROOT_PASSWORD=strong_root_password_here
MYSQL_PASSWORD=strong_app_password_here
CORS_ORIGIN=https://your-domain.com,https://admin.your-domain.com
```

## 功能模块

1. **用户系统**：注册/登录（JWT认证），支持求职者和企业两种角色
2. **求职者**：免费发布和管理求职档案
3. **企业端**：提交营业执照认证，认证通过后发布招聘职位
4. **聊天系统**：企业每天最多联系3名求职者，求职者接受后才能聊天
5. **江湖说**：类朋友圈功能，所有人均可发布动态（登录用户关联身份，未登录以匿名身份发布），支持点赞、评论
6. **好市场**：企业发布门店转让、二手物品等信息
7. **管理后台**：用户管理、企业审核、操作日志、数据统计
8. **图片上传**：支持头像、营业执照、动态图片、商品图片上传
