SET NAMES utf8mb4;
SET CHARACTER SET utf8mb4;

CREATE DATABASE IF NOT EXISTS talent_db DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE talent_db;

CREATE TABLE users (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    phone VARCHAR(20) DEFAULT '',
    avatar VARCHAR(500) DEFAULT '',
    role TINYINT NOT NULL DEFAULT 0 COMMENT '0=求职者 1=企业 2=管理员',
    status TINYINT NOT NULL DEFAULT 0 COMMENT '0=正常 1=禁用',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE job_seeker_profiles (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT NOT NULL UNIQUE,
    name VARCHAR(50) NOT NULL DEFAULT '',
    gender TINYINT NOT NULL DEFAULT 0 COMMENT '0=未知 1=男 2=女',
    age INT NOT NULL DEFAULT 0,
    education VARCHAR(50) NOT NULL DEFAULT '',
    experience_years VARCHAR(50) NOT NULL DEFAULT '',
    expected_salary VARCHAR(50) NOT NULL DEFAULT '',
    expected_city VARCHAR(100) NOT NULL DEFAULT '',
    skills VARCHAR(500) NOT NULL DEFAULT '',
    self_intro TEXT,
    status TINYINT NOT NULL DEFAULT 0 COMMENT '0=展示 1=隐藏',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE enterprise_profiles (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT NOT NULL UNIQUE,
    company_name VARCHAR(200) NOT NULL DEFAULT '',
    license_no VARCHAR(100) NOT NULL DEFAULT '',
    license_image VARCHAR(500) NOT NULL DEFAULT '',
    contact_person VARCHAR(50) NOT NULL DEFAULT '',
    contact_phone VARCHAR(20) NOT NULL DEFAULT '',
    industry VARCHAR(100) NOT NULL DEFAULT '',
    address VARCHAR(500) NOT NULL DEFAULT '',
    verified TINYINT NOT NULL DEFAULT 0 COMMENT '0=待审核 1=已通过 2=已拒绝',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE job_posts (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    enterprise_id BIGINT NOT NULL,
    title VARCHAR(200) NOT NULL,
    salary_range VARCHAR(100) NOT NULL DEFAULT '',
    city VARCHAR(100) NOT NULL DEFAULT '',
    education_req VARCHAR(50) NOT NULL DEFAULT '',
    experience_req VARCHAR(50) NOT NULL DEFAULT '',
    description TEXT,
    status TINYINT NOT NULL DEFAULT 0 COMMENT '0=招聘中 1=已关闭',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (enterprise_id) REFERENCES enterprise_profiles(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE chat_contacts (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    enterprise_user_id BIGINT NOT NULL,
    seeker_user_id BIGINT NOT NULL,
    status TINYINT NOT NULL DEFAULT 0 COMMENT '0=待回复 1=已接受 2=已拒绝',
    contact_date DATE NOT NULL,
    greeting VARCHAR(500) NOT NULL DEFAULT '' COMMENT '企业发起联系时的招呼语',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (enterprise_user_id) REFERENCES users(id),
    FOREIGN KEY (seeker_user_id) REFERENCES users(id),
    UNIQUE KEY uk_enterprise_seeker (enterprise_user_id, seeker_user_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE chat_messages (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    contact_id BIGINT NOT NULL,
    sender_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    is_read TINYINT NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (contact_id) REFERENCES chat_contacts(id),
    FOREIGN KEY (sender_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE daily_contact_count (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    enterprise_user_id BIGINT NOT NULL,
    contact_date DATE NOT NULL,
    count INT NOT NULL DEFAULT 0,
    UNIQUE KEY uk_user_date (enterprise_user_id, contact_date),
    FOREIGN KEY (enterprise_user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE jianghu_posts (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT NULL,
    content TEXT NOT NULL,
    images VARCHAR(2000) NOT NULL DEFAULT '',
    like_count INT NOT NULL DEFAULT 0,
    comment_count INT NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE jianghu_comments (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    post_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (post_id) REFERENCES jianghu_posts(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE jianghu_likes (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    post_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE KEY uk_post_user (post_id, user_id),
    FOREIGN KEY (post_id) REFERENCES jianghu_posts(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE market_posts (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    title VARCHAR(200) NOT NULL,
    category TINYINT NOT NULL DEFAULT 0 COMMENT '0=门店转让 1=二手物品 2=其他',
    price DECIMAL(12,2) NOT NULL DEFAULT 0,
    city VARCHAR(100) NOT NULL DEFAULT '',
    images VARCHAR(2000) NOT NULL DEFAULT '',
    description TEXT,
    contact_info VARCHAR(200) NOT NULL DEFAULT '',
    status TINYINT NOT NULL DEFAULT 0 COMMENT '0=在售 1=已售',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE operation_logs (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT NOT NULL DEFAULT 0,
    action VARCHAR(100) NOT NULL,
    target VARCHAR(500) NOT NULL DEFAULT '',
    ip VARCHAR(50) NOT NULL DEFAULT '',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 初始管理员账号 admin/admin123
INSERT INTO users (username, password_hash, role, status) VALUES
('admin', '$2b$12$KCl5i5IX1gX0ikfKeClOsea35PU8NuE6QsnNs1PiZ55vtHmaasS7m', 2, 0);

-- ========== 测试求职者 (密码均为 123456) ==========
INSERT INTO users (username, password_hash, phone, role, status) VALUES
('张伟', '$2b$12$4SQyUtJ4D.n6RUs0U47pnuE3/t.OL.7KOj1Ye3hbIvCwNTAvJJQ5G', '13800001001', 0, 0),
('李娜', '$2b$12$4SQyUtJ4D.n6RUs0U47pnuE3/t.OL.7KOj1Ye3hbIvCwNTAvJJQ5G', '13800001002', 0, 0),
('王磊', '$2b$12$4SQyUtJ4D.n6RUs0U47pnuE3/t.OL.7KOj1Ye3hbIvCwNTAvJJQ5G', '13800001003', 0, 0),
('赵敏', '$2b$12$4SQyUtJ4D.n6RUs0U47pnuE3/t.OL.7KOj1Ye3hbIvCwNTAvJJQ5G', '13800001004', 0, 0),
('陈晓东', '$2b$12$4SQyUtJ4D.n6RUs0U47pnuE3/t.OL.7KOj1Ye3hbIvCwNTAvJJQ5G', '13800001005', 0, 0),
('刘芳', '$2b$12$4SQyUtJ4D.n6RUs0U47pnuE3/t.OL.7KOj1Ye3hbIvCwNTAvJJQ5G', '13800001006', 0, 0);

-- ========== 测试企业 (密码均为 123456) ==========
INSERT INTO users (username, password_hash, phone, role, status) VALUES
('华创科技', '$2b$12$4SQyUtJ4D.n6RUs0U47pnuE3/t.OL.7KOj1Ye3hbIvCwNTAvJJQ5G', '13800002001', 1, 0),
('锦程教育', '$2b$12$4SQyUtJ4D.n6RUs0U47pnuE3/t.OL.7KOj1Ye3hbIvCwNTAvJJQ5G', '13800002002', 1, 0),
('鼎盛餐饮', '$2b$12$4SQyUtJ4D.n6RUs0U47pnuE3/t.OL.7KOj1Ye3hbIvCwNTAvJJQ5G', '13800002003', 1, 0);

-- ========== 求职者档案 ==========
INSERT INTO job_seeker_profiles (user_id, name, gender, age, education, experience_years, expected_salary, expected_city, skills, self_intro) VALUES
(2, '张伟', 1, 28, '本科', '5年', '15000-20000', '北京', 'Java,Spring Boot,MySQL,Redis,微服务', '五年Java后端开发经验，熟悉分布式系统架构，参与过多个大型电商项目的核心模块开发，具备良好的代码规范意识和团队协作能力。'),
(3, '李娜', 2, 25, '硕士', '2年', '12000-18000', '上海', 'Python,数据分析,机器学习,TensorFlow,SQL', '复旦大学计算机硕士毕业，专注数据分析与机器学习方向，曾在知名互联网公司实习，独立完成用户画像系统的建模工作。'),
(4, '王磊', 1, 30, '本科', '7年', '20000-30000', '深圳', 'React,Vue,TypeScript,Node.js,前端架构', '资深前端工程师，精通React和Vue全家桶，有丰富的前端工程化和性能优化经验，主导过多个中大型项目的前端架构设计。'),
(5, '赵敏', 2, 26, '本科', '3年', '10000-15000', '杭州', 'UI设计,Figma,Sketch,交互设计,用户研究', '三年UI/UX设计经验，擅长移动端和Web端产品设计，注重用户体验和设计规范，作品曾获公司内部设计大赛一等奖。'),
(6, '陈晓东', 1, 32, '本科', '8年', '25000-35000', '北京', 'Go,Kubernetes,Docker,AWS,DevOps', '八年后端及运维开发经验，精通Go语言和云原生技术栈，负责过日活千万级系统的架构设计和运维保障工作。'),
(7, '刘芳', 2, 24, '本科', '1年', '8000-12000', '成都', '新媒体运营,内容策划,短视频,数据分析,文案', '一年新媒体运营经验，擅长内容策划和短视频制作，曾独立运营公司抖音账号，三个月内粉丝从零增长到五万。');

-- ========== 企业档案 ==========
INSERT INTO enterprise_profiles (user_id, company_name, license_no, contact_person, contact_phone, industry, address, verified) VALUES
(8, '华创科技有限公司', '91110108MA01ABCDEF', '李总', '13800002001', '互联网/软件开发', '北京市海淀区中关村科技园区8号楼', 1),
(9, '锦程教育科技有限公司', '91310115MA01GHIJKL', '王校长', '13800002002', '教育培训', '上海市浦东新区张江高科技园区', 1),
(10, '鼎盛餐饮管理有限公司', '91440300MA01MNOPQR', '陈经理', '13800002003', '餐饮/酒店', '深圳市南山区科技南路88号', 1);

-- ========== 招聘职位 ==========
INSERT INTO job_posts (enterprise_id, title, salary_range, city, education_req, experience_req, description) VALUES
(1, 'Java高级开发工程师', '20000-30000', '北京', '本科及以上', '3年以上', '负责公司核心业务系统的后端开发与维护，参与系统架构设计和技术选型。要求熟悉Spring Boot、MyBatis、MySQL、Redis等技术栈，有微服务项目经验优先。提供五险一金、带薪年假、弹性工作制等福利。'),
(1, '前端开发工程师', '15000-25000', '北京', '本科及以上', '2年以上', '负责公司Web端产品的前端开发，与产品、设计团队紧密协作。要求精通Vue或React框架，熟悉TypeScript，有良好的代码规范和组件化开发意识。'),
(1, '产品经理', '18000-28000', '北京', '本科及以上', '3年以上', '负责公司SaaS产品的规划与设计，进行需求分析、竞品调研和用户研究。要求有B端产品经验，熟悉Axure或Figma，具备良好的逻辑思维和沟通能力。'),
(2, '少儿编程讲师', '10000-15000', '上海', '本科及以上', '1年以上', '负责6-15岁青少年的编程课程教学，包括Scratch和Python课程。要求热爱教育行业，有耐心，具备良好的表达能力。有教师资格证或编程竞赛获奖经历优先。'),
(2, '课程顾问', '8000-15000', '上海', '大专及以上', '1年以上', '负责课程咨询和学员转化，维护家长关系。底薪加提成，上不封顶。要求形象气质佳，沟通能力强，有教育行业销售经验优先。'),
(3, '门店店长', '8000-12000', '深圳', '大专及以上', '2年以上', '负责门店日常运营管理，包括人员排班、库存管理、营业额目标达成等。要求有餐饮行业管理经验，责任心强，能适应轮班制度。'),
(3, '厨师长', '12000-18000', '深圳', '不限', '5年以上', '负责后厨团队管理和菜品研发，保证出品质量和食品安全。要求有中餐烹饪经验，持有健康证，擅长川菜或粤菜优先。'),
(1, '测试工程师', '12000-20000', '北京', '本科及以上', '2年以上', '负责公司产品的功能测试和自动化测试，编写测试用例和测试报告。要求熟悉Selenium或Cypress等自动化测试工具，有接口测试经验。');

-- ========== 江湖说动态 ==========
INSERT INTO jianghu_posts (user_id, content, like_count, comment_count, created_at) VALUES
(2, '今天面试了一家互联网公司，面试官问了很多关于分布式事务的问题，感觉自己准备得还不够充分。大家有什么好的学习资料推荐吗？', 12, 3, NOW() - INTERVAL 2 HOUR),
(3, '刚拿到数据分析师的offer，薪资比预期高了不少！分享一下我的面试经验：一定要准备好SQL实战题和业务分析案例，面试官很看重实际解决问题的能力。', 28, 5, NOW() - INTERVAL 5 HOUR),
(4, '前端开发者们注意了，Vue 3.5发布了一些很实用的新特性，特别是响应式系统的优化，性能提升明显。建议大家抽空看看官方文档。', 35, 8, NOW() - INTERVAL 1 DAY),
(6, '分享一个求职小技巧：简历上的项目经历一定要用STAR法则来描述（情境-任务-行动-结果），这样面试官能更清楚地了解你的贡献和能力。', 45, 6, NOW() - INTERVAL 1 DAY),
(5, '最近在学习Figma的Auto Layout功能，真的太好用了！设计响应式界面的效率提高了好几倍，强烈推荐给做UI设计的朋友们。', 18, 2, NOW() - INTERVAL 2 DAY),
(7, '成都的互联网氛围越来越好了，最近好几家大厂都在成都设了研发中心。对于不想卷北上广的朋友，成都真的是个不错的选择。', 52, 10, NOW() - INTERVAL 3 DAY);

-- ========== 江湖说评论 ==========
INSERT INTO jianghu_comments (post_id, user_id, content) VALUES
(1, 4, '推荐看看《分布式系统设计模式》这本书，讲得很透彻。'),
(1, 6, '可以去B站搜一下相关的视频教程，有些讲得很好。'),
(1, 3, '加油！多刷几道相关的面试题就好了。'),
(2, 2, '恭喜恭喜！能分享一下具体是哪家公司吗？'),
(2, 5, '太厉害了，向你学习！'),
(2, 4, 'SQL实战题确实很重要，我面试的时候也被问到了。'),
(2, 7, '恭喜拿到心仪的offer！'),
(2, 6, '数据分析这个方向前景很好，恭喜！'),
(3, 2, '感谢分享，马上去看看新特性。'),
(3, 6, 'Vue 3越来越成熟了，生态也很完善。'),
(3, 5, '作为设计师也在关注前端技术的发展，学习了。'),
(4, 3, 'STAR法则真的很有用，我就是用这个方法拿到的offer。'),
(4, 7, '收藏了，正在准备简历，这个建议太及时了。'),
(6, 2, '确实，成都的生活成本比北京低很多，生活质量高。'),
(6, 5, '成都美食也多，工作生活两不误哈哈。');

-- ========== 江湖说点赞 ==========
INSERT INTO jianghu_likes (post_id, user_id) VALUES
(1, 3), (1, 4), (1, 5),
(2, 2), (2, 4), (2, 5), (2, 6), (2, 7),
(3, 2), (3, 5), (3, 6), (3, 7),
(4, 2), (4, 3), (4, 5), (4, 7),
(5, 2), (5, 3), (5, 4),
(6, 2), (6, 3), (6, 4), (6, 5);

-- ========== 好市场商品 ==========
INSERT INTO market_posts (user_id, title, category, price, city, description, contact_info, created_at) VALUES
(8, '中关村奶茶店整体转让', 0, 180000.00, '北京', '位于中关村核心商圈，面积45平米，月租金8000元，设备齐全，品牌授权还有两年。因个人原因无法继续经营，诚意转让。日均营业额3000-5000元，客源稳定。', '李总 13800002001', NOW() - INTERVAL 1 DAY),
(9, '二手办公桌椅一批', 1, 3500.00, '上海', '公司搬迁，低价处理一批办公家具。包括：办公桌10张（1.4米*0.7米），办公椅10把（人体工学椅），文件柜5个。九成新，使用不到一年，可单独购买也可打包带走。', '王校长 13800002002', NOW() - INTERVAL 2 DAY),
(10, '南山区餐饮旺铺转让', 0, 350000.00, '深圳', '位于南山区科技园附近，面积120平米，上下两层。周边写字楼密集，午餐时段客流量大。厨房设备、桌椅、空调等全部包含在内，接手即可营业。', '陈经理 13800002003', NOW() - INTERVAL 3 DAY),
(8, '全新MacBook Pro 16寸 M3芯片', 1, 16800.00, '北京', '公司年会奖品，全新未拆封。MacBook Pro 16寸，M3 Pro芯片，18GB内存，512GB存储。官方售价19999元，现在16800元出，可当面验货。', '李总 13800002001', NOW() - INTERVAL 5 DAY),
(10, '商用冰柜低价出售', 1, 2800.00, '深圳', '海尔商用冰柜，容量518升，双温双控，使用一年半，制冷效果良好。因门店升级换了更大的，这台低价出售。可上门自提，也可协商配送。', '陈经理 13800002003', NOW() - INTERVAL 7 DAY);

-- ========== 聊天联系记录 ==========
INSERT INTO chat_contacts (enterprise_user_id, seeker_user_id, status, contact_date, greeting) VALUES
(8, 2, 1, CURDATE(), '张伟你好，我是华创科技的李总。看了你的简历，你的Java技术栈和我们的岗位需求非常匹配，想和你聊聊我们的高级开发工程师岗位，方便吗？'),
(8, 4, 1, CURDATE(), '王磊你好，我们正在招前端架构师，你的经验很符合我们的要求。我们团队技术氛围很好，期待和你交流。'),
(9, 3, 0, CURDATE(), '李娜你好，我是锦程教育的王校长。我们正在组建数据团队，你的机器学习背景很吸引我们，想了解一下你对教育行业数据分析的看法。'),
(9, 5, 1, CURDATE(), '赵敏你好，我们锦程教育正在找一位UI设计师来负责在线课程平台的视觉设计，看了你的作品集觉得非常不错，想和你聊聊。'),
(10, 6, 1, CURDATE(), '陈晓东你好，我是鼎盛餐饮的陈经理。我们正在做数字化转型，需要一位有DevOps经验的工程师来搭建我们的技术基础设施，你的背景很合适。'),
(10, 7, 1, CURDATE(), '刘芳你好，我们鼎盛餐饮正在招新媒体运营，主要负责我们品牌的抖音和小红书账号运营，看到你之前的运营成绩很亮眼，想详细聊聊。'),
(8, 6, 1, CURDATE() - INTERVAL 1 DAY, '陈晓东你好，华创科技这边有个DevOps架构师的岗位，看了你的简历觉得非常匹配，想了解一下你的求职意向。'),
(9, 7, 0, CURDATE(), '刘芳你好，锦程教育想招一位新媒体运营来负责我们的品牌推广，你的短视频运营经验很吸引我们。');

-- ========== 聊天消息 ==========
INSERT INTO chat_messages (contact_id, sender_id, content, is_read, created_at) VALUES
(1, 8, '张伟你好，看了你的简历觉得很不错，我们这边有个Java高级开发的岗位想和你聊聊。', 1, NOW() - INTERVAL 3 HOUR),
(1, 2, '李总你好，感谢关注！我对这个岗位很感兴趣，能介绍一下具体的技术栈和团队情况吗？', 1, NOW() - INTERVAL 2 HOUR),
(1, 8, '我们主要用Spring Cloud微服务架构，数据库是MySQL和Redis，部署在阿里云上。团队目前有15人，氛围很好。', 1, NOW() - INTERVAL 1 HOUR),
(1, 2, '听起来很不错！请问工作地点在哪里？有弹性工作制吗？', 0, NOW() - INTERVAL 30 MINUTE),
(2, 8, '王磊你好，我们前端团队正在寻找一位架构师来主导技术升级，你的经验很匹配。', 1, NOW() - INTERVAL 5 HOUR),
(2, 4, '你好李总，谢谢认可！能详细说说目前前端的技术栈和要解决的主要问题吗？', 1, NOW() - INTERVAL 4 HOUR),
(2, 8, '目前用的是Vue 2，计划升级到Vue 3 + TypeScript，同时需要搭建组件库和优化构建流程。', 0, NOW() - INTERVAL 3 HOUR),
(4, 9, '赵敏你好，我们的在线教育平台正在改版，需要一位有经验的UI设计师。', 1, NOW() - INTERVAL 6 HOUR),
(4, 5, '王校长好，谢谢关注！我对教育行业的设计很感兴趣，能介绍一下具体的设计需求吗？', 1, NOW() - INTERVAL 5 HOUR),
(4, 9, '主要是K12在线课程平台的全面改版，包括学生端App和教师管理后台，需要从0到1重新设计。', 1, NOW() - INTERVAL 4 HOUR),
(4, 5, '这个项目很有挑战性，我之前做过类似的教育类产品设计。方便约个时间详细聊聊吗？', 0, NOW() - INTERVAL 3 HOUR),
(5, 10, '陈晓东你好，我们餐饮集团正在做数字化升级，需要搭建自己的点餐和供应链系统。', 1, NOW() - INTERVAL 8 HOUR),
(5, 6, '陈经理好，餐饮数字化是个很好的方向。你们目前的技术基础是什么样的？', 1, NOW() - INTERVAL 7 HOUR),
(5, 10, '目前基本是零基础，所有流程都是手工的。我们希望能搭建一套完整的系统，包括点餐、库存、财务。', 1, NOW() - INTERVAL 6 HOUR),
(5, 6, '明白了，这是一个从零搭建的项目，工作量不小但很有意思。我可以先出一个技术方案给你们参考。', 0, NOW() - INTERVAL 5 HOUR),
(6, 10, '刘芳你好，我们鼎盛餐饮想在抖音和小红书上做品牌推广，看到你之前的运营成绩很不错。', 1, NOW() - INTERVAL 4 HOUR),
(6, 7, '陈经理好！餐饮行业做短视频推广效果很好的，我之前帮一家奶茶店做过，效果很明显。', 1, NOW() - INTERVAL 3 HOUR),
(6, 10, '太好了！我们在深圳有8家门店，想通过线上推广带动客流。你觉得应该怎么规划？', 1, NOW() - INTERVAL 2 HOUR),
(6, 7, '建议先从探店视频和美食制作过程入手，这类内容在餐饮行业转化率最高。我可以做一个详细的运营方案。', 0, NOW() - INTERVAL 1 HOUR),
(7, 8, '陈晓东你好，我们华创科技也在招DevOps工程师，主要负责CI/CD和容器化部署。', 1, NOW() - INTERVAL 1 DAY),
(7, 6, '李总好，华创科技的技术氛围我有所耳闻，很感兴趣。能介绍一下目前的基础设施情况吗？', 1, NOW() - INTERVAL 23 HOUR),
(7, 8, '我们目前用的是阿里云ECS，想迁移到Kubernetes。CI用的Jenkins，也想升级到更现代的方案。', 1, NOW() - INTERVAL 22 HOUR),
(7, 6, '这个方向很对，K8s + GitLab CI/ArgoCD 是目前比较主流的方案，我之前做过类似的迁移项目。', 0, NOW() - INTERVAL 21 HOUR);

-- ========== 操作日志 ==========
INSERT INTO operation_logs (user_id, action, target, ip, created_at) VALUES
(1, '管理员登录', 'username=admin', '127.0.0.1', NOW() - INTERVAL 1 DAY),
(1, '审核企业', 'enterprise_id=1 verified=1', '127.0.0.1', NOW() - INTERVAL 1 DAY),
(1, '审核企业', 'enterprise_id=2 verified=1', '127.0.0.1', NOW() - INTERVAL 1 DAY),
(1, '审核企业', 'enterprise_id=3 verified=1', '127.0.0.1', NOW() - INTERVAL 1 DAY),
(2, '用户注册', 'username=张伟', '192.168.1.100', NOW() - INTERVAL 2 DAY),
(3, '用户注册', 'username=李娜', '192.168.1.101', NOW() - INTERVAL 2 DAY),
(8, '用户注册', 'username=华创科技', '192.168.1.200', NOW() - INTERVAL 3 DAY),
(8, '发布职位', 'job_id=1 title=Java高级开发工程师', '192.168.1.200', NOW() - INTERVAL 1 DAY),
(8, '发起联系', 'seeker_user_id=2', '192.168.1.200', NOW() - INTERVAL 3 HOUR);
