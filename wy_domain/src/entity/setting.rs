use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};


// 按公司的品种
#[crud_table(table_name:variety_com)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct VarietyCom {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub variety_name: Option<String>,
    pub variety_code: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(VarietyCom {
    id,
    company_code,
    variety_name,
    variety_code,
    created_at,
    updated_at
});

// 总的品种
#[crud_table(table_name:variety)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct Variety {
    pub id: Option<u64>,
    pub variety_cate: Option<String>,
    pub variety_name: Option<String>,
    pub variety_code: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(Variety {
    id,
    variety_cate,
    variety_name,
    variety_code,
    created_at,
    updated_at
});

// 总的产地
#[crud_table(table_name:origin)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct Origin {
    pub id: Option<u64>,
    pub origin_code: Option<String>,
    pub origin_name: Option<String>,
    pub origin_fullname: Option<String>,
    pub origin_address: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(Origin {
    id,
    origin_code,
    origin_name,
    origin_fullname,
    origin_address,
    created_at,
    updated_at
});

// 按公司的产地
#[crud_table(table_name:origin_com)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct OriginCom {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub origin_code: Option<String>,
    pub origin_name: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(OriginCom {
    id,
    company_code,
    origin_code,
    origin_name,
    created_at,
    updated_at
});

// 质量等级
#[crud_table(table_name:p_quality)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct PQuality {
    pub id: Option<u64>,
    pub lvl_name: Option<String>,
    pub lvl_code: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(PQuality {
    id,
    lvl_name,
    lvl_code,
    created_at,
    updated_at
});

// 质量等级
#[crud_table(table_name:p_type)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct PType {
    pub id: Option<u64>,
    pub p_type: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(PType {
    id,
    p_type,
    created_at,
    updated_at
});

// 支付方式
#[crud_table(table_name:pay_mode)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct PayMode {
    pub id: Option<u64>,
    pub pay_code: Option<String>,
    pub pay_name: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}
impl_field_name_method!(PayMode {
    id,
    pay_code,
    pay_name,
    created_at,
    updated_at
});

// 支付方式公司
#[crud_table(table_name:pay_mode_com)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct PayModeCom {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub pay_code: Option<String>,
    pub pay_name: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}
impl_field_name_method!(PayModeCom {
    id,
    company_code,
    pay_code,
    pay_name,
    created_at,
    updated_at
});
// 角色
#[crud_table(table_name:role)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: Option<u64>,
    pub role_code: Option<String>,
    pub role_name: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}
impl_field_name_method!(Role {
    id,
    role_code,
    role_name,
    created_at,
    updated_at
});

// 仓库
#[crud_table(table_name:storage)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct Storage {
    pub id: Option<u64>,
    pub storage_name: Option<String>,
    pub storage_address: Option<String>,
    pub storage_person: Option<String>,
    pub storage_phone: Option<String>,
    pub storage_code: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}
impl_field_name_method!(Storage {
    id,
    storage_name,
    storage_address,
    storage_person,
    storage_phone,
    storage_code,
    created_at,
    updated_at
});


// 仓库公司
#[crud_table(table_name:storage_com)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct StorageCom {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub storage_name: Option<String>,
    pub storage_address: Option<String>,
    pub storage_person: Option<String>,
    pub storage_phone: Option<String>,
    pub storage_code: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}
impl_field_name_method!(StorageCom {
    id,
    company_code,
    storage_name,
    storage_address,
    storage_person,
    storage_phone,
    storage_code,
    created_at,
    updated_at
});

// 供应商公司
#[crud_table(table_name:supply_com)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct SupplyCom {
   pub id: Option<u64>,
   pub company_code: Option<String>,
   pub supply_name: Option<String>,
   pub supply_cate: Option<String>,
   pub supply_address: Option<String>,
   pub supply_person: Option<String>,
   pub supply_phone: Option<String>,
   pub supply_code: Option<String>,
   pub created_at: Option<DateTimeNative>,
   pub updated_at: Option<DateTimeNative>,
}
impl_field_name_method!(SupplyCom {
    id,
    company_code,
    supply_name,
    supply_cate,
    supply_address,
    supply_person,
    supply_phone,
    supply_code,
    created_at,
    updated_at
});

// 客户公司
#[crud_table(table_name:client_com)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct ClientCom {
   pub id: Option<u64>,
   pub company_code: Option<String>,
   pub client_name: Option<String>,
   pub client_address: Option<String>,
   pub client_person: Option<String>,
   pub client_phone: Option<String>,
   pub created_at: Option<DateTimeNative>,
   pub updated_at: Option<DateTimeNative>,
}
impl_field_name_method!(ClientCom {
    id,
    company_code,
    client_name,
    client_address,
    client_person,
    client_phone,
    created_at,
    updated_at
});
// 公告
#[crud_table(table_name:notice)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct Notice {
   pub id: Option<u64>,
   pub notice: Option<String>,
   pub end_date: Option<DateTimeNative>,
   pub is_effect: Option<i32>,
   pub created_at: Option<DateTimeNative>,
   pub updated_at: Option<DateTimeNative>,
}
impl_field_name_method!(Notice {
    id,
    notice,
    end_date,
    is_effect,
    created_at,
    updated_at,
});


// 反馈信息
#[crud_table(table_name:fackback)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct Fackback {
   pub id: Option<u64>,
   pub category: Option<String>,
   pub  contact: Option<String>,
   pub suggest: Option<String>,
   pub created_at: Option<DateTimeNative>,
   pub updated_at: Option<DateTimeNative>,
}
impl_field_name_method!(Fackback {
    id,
    category,
    contact,
    suggest,
    created_at,
    updated_at,
});



// 规格
#[crud_table(table_name:sepc_com)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct SepcCom {
   pub id: Option<u64>,
   pub company_code: Option<String>,
   pub spec: Option<String>,
   pub created_at: Option<DateTimeNative>,
   pub updated_at: Option<DateTimeNative>,
}
impl_field_name_method!(SepcCom {
    id,
    company_code,
    spec,
    created_at,
    updated_at
});


// 牌号
#[crud_table(table_name:shop_sign_com)]
#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct ShopSignCom {
   pub id: Option<u64>,
   pub company_code: Option<String>,
   pub shop_sign: Option<String>,
   pub created_at: Option<DateTimeNative>,
   pub updated_at: Option<DateTimeNative>,
}
impl_field_name_method!(ShopSignCom {
    id,
    company_code,
    shop_sign,
    created_at,
    updated_at
});