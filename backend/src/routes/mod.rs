mod auth;
mod seeker;
mod enterprise;
mod job;
mod chat;
mod jianghu;
mod market;
mod admin;
mod upload;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/auth")
                .route("/register", web::post().to(auth::register))
                .route("/login", web::post().to(auth::login))
                .route("/me", web::get().to(auth::me))
                .route("/refresh", web::post().to(auth::refresh))
            )
            .service(web::scope("/seeker")
                .route("/profile", web::post().to(seeker::upsert_seeker_profile))
                .route("/profile", web::get().to(seeker::get_seeker_profile))
                .route("/list", web::get().to(seeker::list_seekers))
                .route("/{id}", web::get().to(seeker::get_seeker_detail))
            )
            .service(web::scope("/enterprise")
                .route("/profile", web::post().to(enterprise::upsert_enterprise_profile))
                .route("/profile", web::get().to(enterprise::get_enterprise_profile))
                .route("/job", web::post().to(job::create_job))
                .route("/job/{id}", web::put().to(job::update_job))
                .route("/job/{id}", web::delete().to(job::delete_job))
                .route("/jobs", web::get().to(job::list_my_jobs))
            )
            .service(web::scope("/jobs")
                .route("", web::get().to(job::list_jobs))
                .route("/{id}", web::get().to(job::get_job_detail))
            )
            .service(web::scope("/chat")
                .route("/contact", web::post().to(chat::initiate_contact))
                .route("/contact/{id}/reply", web::put().to(chat::reply_contact))
                .route("/contacts", web::get().to(chat::list_contacts))
                .route("/message", web::post().to(chat::send_message))
                .route("/messages/{contact_id}", web::get().to(chat::list_messages))
                .route("/messages/{contact_id}/read", web::put().to(chat::mark_read))
            )
            .service(web::scope("/jianghu")
                .route("/post", web::post().to(jianghu::create_jianghu_post))
                .route("/posts", web::get().to(jianghu::list_jianghu_posts))
                .route("/post/{id}/like", web::post().to(jianghu::toggle_like))
                .route("/post/{id}/comment", web::post().to(jianghu::add_comment))
                .route("/post/{id}/comments", web::get().to(jianghu::list_comments))
                .route("/post/{id}", web::delete().to(jianghu::delete_jianghu_post))
            )
            .service(web::scope("/market")
                .route("/post", web::post().to(market::create_market_post))
                .route("/posts", web::get().to(market::list_market_posts))
                .route("/post/{id}", web::get().to(market::get_market_detail))
                .route("/post/{id}", web::put().to(market::update_market_post))
                .route("/post/{id}", web::delete().to(market::delete_market_post))
            )
            .service(web::scope("/admin")
                .route("/users", web::get().to(admin::admin_list_users))
                .route("/user/{id}/status", web::put().to(admin::admin_toggle_user_status))
                .route("/enterprises", web::get().to(admin::admin_list_enterprises))
                .route("/enterprise/{id}/verify", web::put().to(admin::admin_verify_enterprise))
                .route("/logs", web::get().to(admin::admin_list_logs))
                .route("/stats", web::get().to(admin::admin_stats))
                .route("/market", web::get().to(admin::admin_list_market))
                .route("/market/{id}", web::delete().to(admin::admin_delete_market))
                .route("/jianghu", web::get().to(admin::admin_list_jianghu))
                .route("/jianghu/{id}", web::delete().to(admin::admin_delete_jianghu))
                .route("/jobs", web::get().to(admin::admin_list_jobs))
                .route("/job/{id}", web::delete().to(admin::admin_delete_job))
            )
            .route("/upload", web::post().to(upload::upload_file))
    );
}

use actix_web::HttpRequest;

pub(crate) fn get_ip(req: &HttpRequest) -> String {
    req.connection_info().realip_remote_addr().unwrap_or("unknown").to_string()
}
