pub mod auth;
use crate::sdk::aes_crypt::AesCrypt;
use crate::utils::error::Result;
use serde::{Deserialize, Serialize};


pub fn resolve_data(session_key: String, iv: String, encrypted_data: String) -> Result<WxUserInfo> {
    let key = base64::decode(session_key).unwrap();
    let iv = base64::decode(iv).unwrap();
    let aes = AesCrypt::new(key, iv);
    let data = aes.decrypt(encrypted_data);
    let info: WxUserInfo = serde_json::from_str(&data).unwrap();
    Ok(info)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WxUserInfo {
    pub openid: Option<String>,
    pub nick_name: Option<String>,
    pub gender: Option<u8>,
    pub language: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub avatar_url: Option<String>,
    pub pure_phone_number: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WxappSessionKey {
    pub session_key: String,
    pub expires_in: i64,
    pub openid: String,
    pub unionid: Option<String>,
}
