use serde::{Deserialize, Serialize};
use crate::entity::user::{User,WechatUser,Company};
use rbatis::DateTimeNative;
use validator_derive::Validate;


#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct UserDTO {
   pub id: Option<u64>,
   pub  phone: Option<String>,
   pub  company_code: Option<String>,
   pub  avator: Option<String>,
   pub  nickname: Option<String>,
   pub  sex: Option<String>,
   pub  role: Option<String>,
   pub  address: Option<String>,
   pub  title: Option<String>,
   pub  lastest_login: Option<DateTimeNative>,
   pub  created_at: Option<DateTimeNative>,
   pub  updated_at: Option<DateTimeNative>,
}
impl UserDTO {
    pub fn new(phone: Option<String>) -> Self {
        return Self {
            phone,
           ..Default::default()
        }
    }
}

impl Into<User> for UserDTO {
    fn into(self) -> User {
        User {
            id: self.id().clone(),
            phone: self.phone.clone(),
            company_code: self.company_code.clone(),
            avator: self.avator.clone(),
            nickname: self.nickname.clone(),
            sex: self.sex.clone(),
            role: self.role.clone(),
            address: self.address.clone(),
            title: self.title.clone(),
            lastest_login: self.lastest_login.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<User> for UserDTO {
    fn from(arg: User) -> Self {
        Self {
            id: arg.id,
            phone: arg.phone,
            company_code: arg.company_code,
            avator: arg.avator,
            nickname: arg.nickname,
            sex: arg.sex,
            role: arg.role,
            address: arg.address,
            title: arg.title,
            lastest_login: arg.lastest_login,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct WechatUserDTO {
    id: Option<u64>,
    unionid: Option<String>,
    openid: Option<String>,
    routine_openid: Option<String>,
    nickname: Option<String>,
    headimgurl: Option<String>,
    sex: Option<u8>,
    city: Option<String>,
    language: Option<String>,
    province: Option<String>,
    country: Option<String>,
    remark: Option<i32>,
    groupid: Option<i32>,
    user_type: Option<String>,
    session_key: Option<String>,
}

impl Into<WechatUser> for WechatUserDTO {
    fn into(self) -> WechatUser {
        WechatUser {
            id: self.id().clone(),
            unionid: self.unionid.clone(),
            openid: self.openid.clone(),
            routine_openid: self.routine_openid.clone(),
            nickname: self.nickname.clone(),
            headimgurl: self.headimgurl.clone(),
            sex: self.sex.clone(),
            city: self.city.clone(),
            language: self.language.clone(),
            province: self.province.clone(),
            country: self.country.clone(),
            remark: self.remark.clone(),
            groupid: self.groupid.clone(),
            user_type: self.user_type.clone(),
            session_key: self.session_key.clone(),
        }
    }
}

impl From<WechatUser> for WechatUserDTO {
    fn from(arg: WechatUser) -> Self {
        Self {
            id: arg.id,
            unionid: arg.unionid,
            openid: arg.openid,
            routine_openid: arg.routine_openid,
            nickname: arg.nickname,
            headimgurl: arg.headimgurl,
            sex: arg.sex,
            city: arg.city,
            language: arg.language,
            province: arg.province,
            country: arg.country,
            remark: arg.remark,
            groupid: arg.groupid,
            user_type: arg.user_type,
            session_key: arg.session_key,
        }
    }
}




// 公司

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct CompanyDTO {
    pub id: Option<u64>,
    pub company_name: Option<String>,
    pub company_code: Option<String>,
    pub company_address: Option<String>,
    pub company_phone: Option<String>,
    pub company_email: Option<String>,
    pub register_at: Option<DateTimeNative>,
    pub lastest_at: Option<DateTimeNative>,
    pub level: Option<String>,
    pub tax_number: Option<String>,
    pub tax_com_name: Option<String>,
    pub company_person: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}


impl Into<Company> for CompanyDTO {
    fn into(self) -> Company {
        Company {
            id: self.id.clone(),
            company_name: self.company_name.clone(),
            company_code: self.company_code.clone(),
            company_address: self.company_address.clone(),
            company_phone: self.company_phone.clone(),
            company_email: self.company_email.clone(),
            register_at: self.register_at.clone(),
            lastest_at: self.lastest_at.clone(),
            level: self.level.clone(),
            tax_number: self.tax_number.clone(),
            tax_com_name: self.tax_com_name.clone(),
            company_person: self.company_person.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<Company> for CompanyDTO {
    fn from(arg: Company) -> Self {
        Self {
            id: arg.id,
            company_name: arg.company_name,
            company_code: arg.company_code,
            company_address: arg.company_address,
            company_phone: arg.company_phone,
            company_email: arg.company_email,
            register_at: arg.register_at,
            lastest_at: arg.lastest_at,
            level: arg.level,
            tax_number: arg.tax_number,
            tax_com_name: arg.tax_com_name,
            company_person: arg.company_person,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


#[derive(Serialize, Deserialize, Validate, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SignInByPhoneDTO {
    #[validate(required)]
    #[validate(length(min = 11, message = "账号最少11个字符"))]
    phone: Option<String>,
    #[validate(required)]
    #[validate(length(equal = 4, message = "验证码必须为4位"))]
    code: Option<String>,
}


#[derive(Serialize, Deserialize, Validate, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SendPhoneDTO {
    #[validate(required)]
    #[validate(length(min = 11, message = "账号最少11个字符"))]
    phone: Option<String>,
    //验证码，可用是短信验证码，图片验证码,二维码验证码...
    #[validate(required)]
    #[validate(length(equal = 4, message = "验证码必须为4位"))]
    vcode: Option<String>,
    #[validate(required)]
    uuid: Option<String>,
}

/// 验证码
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CatpchaDTO {
    pub uuid: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SignInVO {
    user: Option<UserDTO>,
    access_token: String,
}