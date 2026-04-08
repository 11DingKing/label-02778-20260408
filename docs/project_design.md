# 人才网平台 - 项目设计文档

## 1. 系统架构

```mermaid
flowchart TD
    subgraph Client["客户端"]
        A[管理后台 frontend-admin :8082]
        B[用户端 frontend-user :8081]
    end

    subgraph Backend["后端服务 Rust Actix-web :8080"]
        C[Auth Module - JWT认证]
        D[User Module - 用户管理]
        E[Job Module - 求职/招聘]
        F[Chat Module - 聊天系统]
        G[Jianghu Module - 江湖说]
        H[Market Module - 好市场]
        I[Admin Module - 后台管理]
    end

    subgraph Storage["存储层"]
        J[(MySQL 8.0)]
    end

    A --> C
    B --> C
    C --> D
    C --> E
    C --> F
    C --> G
    C --> H
    C --> I
    D --> J
    E --> J
    F --> J
    G --> J
    H --> J
    I --> J
```

## 2. ER 图

```mermaid
erDiagram
    users {
        bigint id PK
        varchar username
        varchar password_hash
        varchar phone
        varchar avatar
        tinyint role "0=求职者 1=企业 2=管理员"
        tinyint status "0=正常 1=禁用"
        datetime created_at
        datetime updated_at
    }

    job_seeker_profiles {
        bigint id PK
        bigint user_id FK
        varchar name
        tinyint gender
        int age
        varchar education
        varchar experience_years
        varchar expected_salary
        varchar expected_city
        varchar skills
        text self_intro
        tinyint status "0=展示 1=隐藏"
        datetime created_at
        datetime updated_at
    }

    enterprise_profiles {
        bigint id PK
        bigint user_id FK
        varchar company_name
        varchar license_no
        varchar license_image
        varchar contact_person
        varchar contact_phone
        varchar industry
        varchar address
        tinyint verified "0=待审核 1=已通过 2=已拒绝"
        datetime created_at
        datetime updated_at
    }

    job_posts {
        bigint id PK
        bigint enterprise_id FK
        varchar title
        varchar salary_range
        varchar city
        varchar education_req
        varchar experience_req
        text description
        tinyint status "0=招聘中 1=已关闭"
        datetime created_at
        datetime updated_at
    }

    chat_contacts {
        bigint id PK
        bigint enterprise_user_id FK
        bigint seeker_user_id FK
        tinyint status "0=待回复 1=已接受 2=已拒绝"
        date contact_date
        datetime created_at
    }

    chat_messages {
        bigint id PK
        bigint contact_id FK
        bigint sender_id FK
        text content
        tinyint is_read
        datetime created_at
    }

    daily_contact_count {
        bigint id PK
        bigint enterprise_user_id FK
        date contact_date
        int count
    }

    jianghu_posts {
        bigint id PK
        bigint user_id FK
        text content
        varchar images
        int like_count
        int comment_count
        datetime created_at
    }

    jianghu_comments {
        bigint id PK
        bigint post_id FK
        bigint user_id FK
        text content
        datetime created_at
    }

    jianghu_likes {
        bigint id PK
        bigint post_id FK
        bigint user_id FK
        datetime created_at
    }

    market_posts {
        bigint id PK
        bigint user_id FK
        varchar title
        tinyint category "0=门店转让 1=二手物品 2=其他"
        decimal price
        varchar city
        varchar images
        text description
        varchar contact_info
        tinyint status "0=在售 1=已售"
        datetime created_at
        datetime updated_at
    }

    operation_logs {
        bigint id PK
        bigint user_id FK
        varchar action
        varchar target
        varchar ip
        datetime created_at
    }

    users ||--o| job_seeker_profiles : has
    users ||--o| enterprise_profiles : has
    enterprise_profiles ||--o{ job_posts : publishes
    users ||--o{ chat_contacts : initiates
    users ||--o{ chat_contacts : receives
    chat_contacts ||--o{ chat_messages : contains
    users ||--o{ daily_contact_count : tracks
    users ||--o{ jianghu_posts : publishes
    jianghu_posts ||--o{ jianghu_comments : has
    jianghu_posts ||--o{ jianghu_likes : has
    users ||--o{ market_posts : publishes
    users ||--o{ operation_logs : generates
```

## 3. 接口清单

### Auth Controller
| Method | Path | Description |
|--------|------|-------------|
| POST | /api/auth/register | 用户注册 |
| POST | /api/auth/login | 用户登录 |
| GET | /api/auth/me | 获取当前用户信息 |

### User Controller
| Method | Path | Description |
|--------|------|-------------|
| PUT | /api/user/profile | 更新个人资料 |
| PUT | /api/user/avatar | 上传头像 |
| PUT | /api/user/password | 修改密码 |

### JobSeeker Controller
| Method | Path | Description |
|--------|------|-------------|
| POST | /api/seeker/profile | 创建/更新求职档案 |
| GET | /api/seeker/profile | 获取自己的求职档案 |
| GET | /api/seeker/list | 求职者列表(企业浏览) |
| GET | /api/seeker/{id} | 求职者详情 |

### Enterprise Controller
| Method | Path | Description |
|--------|------|-------------|
| POST | /api/enterprise/profile | 提交企业认证 |
| GET | /api/enterprise/profile | 获取企业档案 |
| POST | /api/enterprise/job | 发布招聘 |
| PUT | /api/enterprise/job/{id} | 编辑招聘 |
| DELETE | /api/enterprise/job/{id} | 删除招聘 |
| GET | /api/enterprise/jobs | 我的招聘列表 |
| GET | /api/jobs | 招聘列表(公开) |
| GET | /api/jobs/{id} | 招聘详情 |

### Chat Controller
| Method | Path | Description |
|--------|------|-------------|
| POST | /api/chat/contact | 企业发起联系 |
| PUT | /api/chat/contact/{id}/reply | 求职者回复(接受/拒绝) |
| GET | /api/chat/contacts | 我的联系列表 |
| POST | /api/chat/message | 发送消息 |
| GET | /api/chat/messages/{contact_id} | 获取聊天记录 |
| PUT | /api/chat/messages/{contact_id}/read | 标记已读 |

### Jianghu Controller
| Method | Path | Description |
|--------|------|-------------|
| POST | /api/jianghu/post | 发布动态 |
| GET | /api/jianghu/posts | 动态列表 |
| POST | /api/jianghu/post/{id}/like | 点赞/取消 |
| POST | /api/jianghu/post/{id}/comment | 评论 |
| GET | /api/jianghu/post/{id}/comments | 评论列表 |
| DELETE | /api/jianghu/post/{id} | 删除动态 |

### Market Controller
| Method | Path | Description |
|--------|------|-------------|
| POST | /api/market/post | 发布商品 |
| GET | /api/market/posts | 商品列表 |
| GET | /api/market/post/{id} | 商品详情 |
| PUT | /api/market/post/{id} | 编辑商品 |
| DELETE | /api/market/post/{id} | 删除商品 |

### Admin Controller
| Method | Path | Description |
|--------|------|-------------|
| GET | /api/admin/users | 用户列表 |
| PUT | /api/admin/user/{id}/status | 禁用/启用用户 |
| GET | /api/admin/enterprises | 企业认证审核列表 |
| PUT | /api/admin/enterprise/{id}/verify | 审核企业 |
| GET | /api/admin/logs | 操作日志 |
| GET | /api/admin/stats | 统计数据 |

## 4. UI/UX 规范

| 属性 | 值 |
|------|-----|
| 主色调 | #2563EB (蓝色) |
| 辅助色 | #10B981 (绿色) |
| 警告色 | #F59E0B (橙色) |
| 危险色 | #EF4444 (红色) |
| 背景色 | #F3F4F6 |
| 卡片背景 | #FFFFFF |
| 文字主色 | #1F2937 |
| 文字次色 | #6B7280 |
| 字体 | -apple-system, "PingFang SC", "Microsoft YaHei" |
| 卡片圆角 | 12px |
| 按钮圆角 | 8px |
| 阴影 | 0 2px 12px rgba(0,0,0,0.08) |
| 间距基数 | 8px (8/16/24/32) |
