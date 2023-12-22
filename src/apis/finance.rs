use axum::{middleware::from_extractor, Router};
use crate::controllers::{init_need_auth_router,init_noneed_finance_router};
use crate::middleware::auth_api;
//order
pub fn routers() -> Router {
    need_auth_routers()
    .merge(init_noneed_finance_router())
}
//需要权限认证的路由
pub fn need_auth_routers() -> Router {
    Router::new()
        .merge(init_need_auth_router())
        .layer(from_extractor::<auth_api::Auth>())
}
