use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};

#[crud_table(table_name:user)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<u64>,
    pub phone: Option<String>,
    pub company_code: Option<String>,
    pub avator: Option<String>,
    pub nickname: Option<String>,
    pub sex: Option<String>,
    pub role: Option<String>,
    pub address: Option<String>,
    pub title: Option<String>,
    pub lastest_login: Option<DateTimeNative>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(User {
    id,
    phone,
    company_code,
    avator,
    nickname,
    sex,
    role,
    address,
    title,
    lastest_login,
    created_at,
    updated_at
});

#[crud_table(table_name:company)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Company {
   pub id: Option<u64>,
   pub  company_name: Option<String>,
   pub  company_code: Option<String>,
   pub  company_address: Option<String>,
   pub  company_phone: Option<String>,
   pub  company_email: Option<String>,
   pub  register_at: Option<DateTimeNative>,
   pub  lastest_at: Option<DateTimeNative>,
   pub  level: Option<String>,
   pub  tax_number: Option<String>,
   pub  tax_com_name: Option<String>,
   pub  company_person: Option<String>,
   pub  created_at: Option<DateTimeNative>,
   pub  updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(Company {
    id,
    company_name,
    company_code,
    company_address,
    company_phone,
    company_email,
    register_at,
    lastest_at,
    level,
    tax_number,
    tax_com_name,
    company_person,
    created_at,
    updated_at,
});

#[crud_table(table_name:wechat_user)]
#[derive(Clone, Debug)]
pub struct WechatUser {
    pub id: Option<u64>,
    pub unionid: Option<String>,
    pub openid: Option<String>,
    pub routine_openid: Option<String>,
    pub nickname: Option<String>,
    pub headimgurl: Option<String>,
    pub sex: Option<u8>,
    pub city: Option<String>,
    pub language: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub remark: Option<i32>,
    pub groupid: Option<i32>,
    pub user_type: Option<String>,
    pub session_key: Option<String>,
}
impl_field_name_method!(WechatUser {
    id,
    unionid,
    openid,
    routine_openid,
    nickname,
    headimgurl,
    sex,
    city,
    language,
    province,
    country,
    remark,
    groupid,
    user_type,
});
