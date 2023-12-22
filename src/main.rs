use axum::{http::Uri,http::Request,response::{IntoResponse,Response}, Router, Server,middleware::{self,Next}};
use log::warn;
use wy_domain::vo::RespVO;
use wanyeel::{
    init_context,
    APPLICATION_CONTEXT,
    config::config::ApplicationConfig,
    apis,
};


async fn fallback(uri: Uri) -> impl IntoResponse {
    let msg = format!("地址不存在：{}", uri);
    warn!("{}", msg.clone());
    RespVO::<String> {
        code: Some(-1),
        msg: Some(msg),
        data: None,
    }
    .resp_json()
}


#[tokio::main]
async fn main() {
    //初始化上环境下文
    init_context().await;
    // 启动服务
    let commerce_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let server = format!("{}:{}", commerce_config.server().host(), commerce_config.server().port());
    let cors = middleware::from_fn(my_middleware);
    //绑定端口初始化路由
    let app = Router::new()
        // .nest("/admin", apis::admin::routers())
        .nest("/api/setting", apis::setting::routers())
        .nest("/api/agree", apis::agree::routers())
        .nest("/api/order", apis::order::routers())
        .nest("/api/tool", apis::tool::routers())
        .nest("/api/user", apis::user::routers())
        .nest("/api/auth", apis::login::routers())
        .nest("/api/storage", apis::storage::routers())
        .nest("/api/sale", apis::sale::routers())
        .nest("/api/finance", apis::finance::routers())
        .layer(cors)
        .fallback(fallback);
    // 启动服务
    Server::bind(&server.parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}

async fn my_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // do something with `request`...

    let response = next.run(request).await;

    // do something with `response`...

    response
}