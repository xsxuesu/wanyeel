
use wy_domain::dto::user::{UserDTO,WechatUserDTO,SignInByPhoneDTO};
use wy_domain::entity::user::{User,WechatUser};
use wy_domain::entity::CommonField;
use wy_domain::request::{ByComQuery,ByUIDQuery};
use crate::crud::crud_service::CrudService;
use crate::APPLICATION_CONTEXT;
use rbatis::rbatis::Rbatis;
use wy_domain::error::Result;

// UserService
pub struct UserService;
impl Default for UserService {
    fn default() -> Self {
        UserService {}
    }
}

impl CrudService<User, UserDTO, ByComQuery> for UserService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper().all_eq(arg)
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut User) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}



// WechatUserService
pub struct WechatUserService;
impl Default for WechatUserService {
    fn default() -> Self {
        WechatUserService {}
    }
}
impl CrudService<WechatUser, WechatUserDTO, ByComQuery> for WechatUserService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut WechatUser) {
        data.id = common.id;
    }
}
