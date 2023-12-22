pub mod agree;
pub mod order;
pub mod setting;
pub mod storage;
pub mod user;
pub mod pay;
pub mod sale;
pub mod finance;

use rbatis::DateTimeNative;
/**
*struct:CommonField
*desc:所有表的公共字段 CRUD_SERVICE使用
*author:String
*email:249608904@qq.com
*/
#[derive(Clone, Debug)]
pub struct CommonField {
    pub id: Option<u64>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

#[derive(Clone, Debug)]
pub struct PageData {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
}
