use crate::cache::cache::CacheService;
use crate::config::config::ApplicationConfig;
use crate::crud::crud_service::CrudService;
use crate::services::sms_services::SmsSendService;
use crate::services::user::UserService;
use crate::APPLICATION_CONTEXT;
use axum::body::Body;
use axum::http::Response;
use axum::{extract::Path, response::IntoResponse, Json};
use captcha::filters::{Dots, Noise, Wave};
use captcha::Captcha;
use log::info;
use rbatis::DateTimeNative;
use rbson::de;
use validator::Validate;
use wy_common::utils::string::random_code;
use wy_domain::dto::user::{SendPhoneDTO, SignInByPhoneDTO, SignInVO, UserDTO};
use wy_domain::error::Error;
use wy_domain::vo::jwt::JWTToken;
use wy_domain::vo::RespVO;

//用户登录手机号
pub async fn user_login_phone(Json(sign): Json<SignInByPhoneDTO>) -> impl IntoResponse {
    let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
    if let Err(e) = sign.validate() {
        return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
    }
    // if let Ok(code) = cache_service.get_string(&format!("_captch:phone_{}", &sign.phone().clone().unwrap())).await {
    //     if !code.eq(&sign.code().clone().unwrap()) {
    //         return RespVO::<()>::from_error(&Error::E("验证码错误".to_string())).resp_json();
    //     }
    // }
    let vo = user_service.get_by("phone".to_string(), sign.phone().clone().unwrap()).await;
    let mut user: Option<UserDTO> = None;
    if vo.is_err() {
        user = Some(UserDTO::new(Some(sign.phone().clone().unwrap())));
        let mut entity = user.clone().unwrap().into();
        let new_user_result = user_service.save(&mut entity).await;
        if new_user_result.is_err() {
            return RespVO::<()>::from_error(&Error::E("用户不存在".to_string())).resp_json();
        }
        let id = Some(new_user_result.unwrap() as u64);
        let phone = sign.phone().clone().unwrap();
        user = Some(UserDTO{
            id:id,
            phone:Some(phone),
            ..Default::default()
        })
    }else {
        user = Some(vo.unwrap());
    }

    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let mut sign_vo = SignInVO::default();
    sign_vo.set_user(Some(user.clone().unwrap().clone()));
    //提前查找所有权限，避免在各个函数方法中重复查找
    let mut jwt_token = JWTToken::default();
    jwt_token.set_id(user.clone().unwrap().id().unwrap());
    jwt_token.set_from(Default::default());
    jwt_token.set_username(user.clone().unwrap().phone().clone().unwrap_or_default());
    jwt_token.set_exp(DateTimeNative::now().timestamp_millis() as usize);
    sign_vo.set_access_token(jwt_token.create_token(config.jwt_secret()).unwrap());

    return RespVO::from_result_ok(sign_vo).resp_json();
}

//手机验证码
pub async fn send_phone_sms(Json(phoneinfo): Json<SendPhoneDTO>) -> impl IntoResponse {
    let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    //验证
    if let Err(e) = phoneinfo.validate() {
        return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
    }
    if let Ok(code) = cache_service.get_string(&format!("_captch:uuid_{}", &phoneinfo.uuid().clone().unwrap())).await {
        if !code.eq(&phoneinfo.vcode().clone().unwrap()) {
            return RespVO::<()>::from_error(&Error::E("验证码错误".to_string())).resp_json();
        }
    }

    let random_number = random_code();
    println!("code : {}", random_number);
    let phone = phoneinfo.phone().as_ref().unwrap();
    let res = cache_service
        .set_string_ex(&format!("_captch:phone_{}", phone.clone()), random_number.as_str(), Some(std::time::Duration::from_secs(60 * 5)))
        .await;
    println!("{:?}", res);
    // send sms code
    let sms_service = APPLICATION_CONTEXT.get::<SmsSendService>();
    let resp = sms_service.send_vcode(phone.to_string(), random_number).await;
    return RespVO::from(&res).resp_json();
}

//captch 图片
pub async fn captcha_png(Path(uuid): Path<String>) -> Response<Body> {
    let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    if uuid.is_empty() {
        return RespVO::<()>::from_error(&Error::from("uuid不能为空!")).resp_json();
    }
    let (captcha_str, png) = {
        let mut captcha = Captcha::new();
        captcha
            .add_chars(4)
            .apply_filter(Noise::new(0.1))
            .apply_filter(Wave::new(1.0, 10.0).horizontal())
            // .apply_filter(Wave::new(2.0, 20.0).vertical())
            .view(160, 60)
            .apply_filter(Dots::new(4));

        let png = captcha.as_png().unwrap();
        (captcha.chars_as_string().to_lowercase(), png)
    };

    let res = cache_service
        .set_string_ex(&format!("_captch:uuid_{}", uuid.clone()), captcha_str.as_str(), Some(std::time::Duration::from_secs(60 * 5)))
        .await;
    println!("{:?}", res);
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Cache-Control", "no-cache")
        .header("Content-Type", "image/png")
        .body(Body::from(png))
        .unwrap()
}
