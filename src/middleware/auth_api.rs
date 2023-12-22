use crate::APPLICATION_CONTEXT;
use axum::http::{HeaderValue};
use axum::http::request::Parts;
use axum::{
    async_trait,
    extract::FromRequestParts,
};
use wy_domain::error::Error;
use crate::config::config::ApplicationConfig;
use wy_domain::vo::RespVO;
use super::{checked_token, set_local};

/**
 *struct:Auth
 *desc:权限验证中间件 初始化 REQUEST_CONTEXT
 *author:String
 *email:249608904@qq.com
 */
pub struct Auth;


#[async_trait]
impl<S> FromRequestParts<S> for Auth
    where
        S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(req: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();

        /*获取method path */
        let action = req.method.clone().to_string();
        let path = req.uri.path().to_string();

        /*获取token*/
        let value = HeaderValue::from_str("").unwrap();
        let headers = req.headers.clone();
        let token = headers.get("access_token").unwrap_or(&value);
        let resp: RespVO<String> = RespVO {
            code: Some(-1),
            msg: Some(format!("请登录")),
            data: None,
        };
        let token_value = token.to_str().unwrap_or("");
        /*token为空提示登录*/
        if token_value.is_empty() {
            return Err(Error::E(serde_json::json!(&resp).to_string()));
        }
        /*验证token有效性*/
        match checked_token(token_value) {
            Ok(data) => {
                //登录了但是不需要权限
                let data1 = data.clone();
                set_local(data1, path.clone());
                return Ok(Self {});
            }
            Err(e) => {
                println!("error:{}", e);
                //401 http状态码会强制前端退出当前登陆状态
                return Err(Error::from(serde_json::json!(&resp).to_string()));
            }
        }
    }
}


