use serde::{Deserialize, Serialize};

/**
*struct:ByComQuery
*desc:根据公司查询
*author:String
*email:249608904@qq.com
*/
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ByComQuery {
    pub company_code: Option<String>,
}

/**
*struct:ByIDQuery
*desc:根据公司查询
*author:String
*email:249608904@qq.com
*/
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ByIDQuery {
    pub id: Option<u64>,
}

/**
*struct:ByUIDQuery
*desc:根据公司查询
*author:String
*email:249608904@qq.com
*/
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ByUIDQuery {
    pub uid: Option<String>,
}


/**
*struct:ByUIDQuery
*desc:根据公司查询
*author:String
*email:249608904@qq.com
*/
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct AllQuery {
}


/**
*struct:StorageQuery
*desc:根据公司查询
*author:String
*email:249608904@qq.com
*/
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct StorageQuery {
    pub name: Option<String>,
}


/**
*struct:NOticeQuery
*desc:根据公司查询
*author:String
*email:249608904@qq.com
*/
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct NoticeQuery {
    pub is_effect: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ByProcessProductCateQuery {
    pub process_id: Option<u64>,
    pub process_cate: Option<u64>,
}