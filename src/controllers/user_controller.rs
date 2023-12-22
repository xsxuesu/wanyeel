use crate::services::user::{UserService, WechatUserService};
use rbatis::DateTimeNative;
use wy_domain::dto::user::{UserDTO, WechatUserDTO};
use wy_domain::entity::user::{User,WechatUser};
use wy_domain::vo::RespVO;
use wy_domain::request::{ByComQuery,ByUIDQuery};
use crate::APPLICATION_CONTEXT;
use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
};
use wy_domain::error::Error;
use crate::crud::crud_service::CrudService;


//保存用户信息
pub async fn save_user(Json(arg): Json<UserDTO>) -> impl IntoResponse {
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
    let result_find= user_service.get_by("phone".to_string(),arg.clone().phone.unwrap()).await;
    if result_find.is_ok(){
        let err = Error::E("用户已存在".to_string());
        return RespVO::<()>::from_error(&err).resp_json();
    }
    let mut entity = arg.into();
    println!("entity:{:?}",entity);
    let result = user_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

//查询用户信息
pub async fn get_user_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
    let result: Result<UserDTO, wy_domain::error::Error> = user_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

//查询用户信息根据手机
pub async fn get_user_by_phone(Path(phone): Path<String>) -> impl IntoResponse {
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
    let result = user_service.get_by("phone".to_string(),phone).await;
    return RespVO::from_result(&result).resp_json();
}

//按照公司查询用户信息
pub async fn get_user_by_com(Path(uid): Path<String>) -> impl IntoResponse {
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
    let mut entity: ByComQuery = ByComQuery{
        company_code: Some(uid),
    };
    let result = user_service.list(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}


//更新用户信息
pub async fn update_user(Json(arg): Json<UserDTO>) -> impl IntoResponse {
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
    let mut entity:User = arg.into();
    entity.lastest_login = Some(DateTimeNative::now());
    let result = user_service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}


//删除用户信息
pub async fn del_user(Path(id): Path<String>) -> impl IntoResponse {
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
    let result = user_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}




//保存weichat用户信息
pub async fn save_weichat_user(Json(arg): Json<WechatUserDTO>) -> impl IntoResponse {
    let user_weichat_service = APPLICATION_CONTEXT.get::<WechatUserService>();
    let mut entity = arg.into();
    let result = user_weichat_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

//查询weichat用户信息
pub async fn get_weichatuser_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let user_weichat_service = APPLICATION_CONTEXT.get::<WechatUserService>();
    let result = user_weichat_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

//更新weichat用户信息
pub async fn update_weichat_user(Json(arg): Json<WechatUserDTO>) -> impl IntoResponse {
    let user_weichat_service = APPLICATION_CONTEXT.get::<WechatUserService>();
    let mut entity:WechatUser = arg.into();
    let result = user_weichat_service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}


//删除weichat用户信息
pub async fn del_weichat_user(Path(id): Path<String>) -> impl IntoResponse {
    let user_weichat_service = APPLICATION_CONTEXT.get::<WechatUserService>();
    let result = user_weichat_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

