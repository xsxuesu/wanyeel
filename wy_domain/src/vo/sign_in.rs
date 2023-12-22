
use crate::dto::user::UserDTO;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ApiSignInVO {
    user: Option<UserDTO>,
    access_token: String,
}

impl ToString for ApiSignInVO {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}
