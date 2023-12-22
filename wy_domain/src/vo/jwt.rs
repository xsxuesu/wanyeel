use std::default;
use crate::error::Error;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header,Algorithm, Validation};
use serde::{Deserialize, Serialize};

/**
*struct:JWTToken
*desc:JWT 鉴权 Token结构
*author:String
*email:249608904@qq.com
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Source {
    Admin, //管理端
    Wxapp, //微信小程序
    Pc,    //pc端
    Mp,    //微信公众号
}
impl default::Default for Source {
    fn default() -> Self {
        Source::Admin
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct JWTToken {
    //账号id
    id: u64,
    //是否超级管理员
    super_admin: i32,
    //账号
    username: String,
    //来源
    from: Source,
    //租户编码
    agency_code: String,
    //过期时间
    exp: usize,
}

impl JWTToken {
    /**
     *method:create_token
     *desc:create token
     *author:String
     *email:249608904@qq.com
     */
    pub fn create_token(&self, secret: &str) -> Result<String, Error> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::from("JWTToken encode fail!")), // in practice you would return the error
        };
    }

    /**
     *method:verify
     *desc:验证并返回JWTToken
     *author:String
     *email:249608904@qq.com
     */
    pub fn verify(secret: &str, token: &str) -> Result<JWTToken, Error> {
        let validation = Validation::new(Algorithm::HS256);
        return match decode::<JWTToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => return Err(Error::from("Token失效")), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => return Err(Error::from("InvalidIssuer")), // Example on how to handle a specific error
                _ => return Err(Error::from("InvalidToken other errors")),
            },
        };
    }
}
