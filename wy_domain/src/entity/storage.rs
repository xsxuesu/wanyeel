use rbatis::{DateNative, DateTimeNative};
use serde::{Deserialize, Serialize};

#[crud_table(table_name:storage_adjust_product)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageAdjustProduct {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub adjust_id: Option<u64>,
    pub storage_id: Option<u64>,
    pub variety: Option<String>,
    pub origin: Option<String>,
    pub warehouse: Option<String>,
    pub shop_sign: Option<String>,
    pub spec: Option<String>,
    pub adjust_number: Option<f64>,
    pub adjust_weight: Option<f64>,
    pub unit: Option<String>,
    pub storage_number: Option<f64>,
    pub storage_weight: Option<f64>,
    pub resource_number: Option<String>,
    pub contract_number: Option<String>,
    pub vechel_number: Option<String>,
    pub pack_number: Option<String>,
    pub adjust_date: Option<DateNative>,
    pub adjust_person: Option<String>,
    pub change_status: Option<i32>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(StorageAdjustProduct {
    id,
    company_code,
    adjust_id,
    storage_id,
    variety,
    origin,
    warehouse,
    shop_sign,
    spec,
    adjust_number,
    adjust_weight,
    unit,
    storage_number,
    storage_weight,
    resource_number,
    contract_number,
    vechel_number,
    pack_number,
    adjust_date,
    adjust_person,
    change_status,
    created_at,
    updated_at,
});


// 仓库公司
#[crud_table(table_name:storage_adjust)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageAdjust {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub in_warehouse: Option<String>,
    pub out_warehouse: Option<String>,
    pub adjust_cate: Option<String>,
    pub adjust_why: Option<String>,
    pub adjust_number: Option<f64>,
    pub adjust_weight: Option<f64>,
    pub variety: Option<String>,
    pub origin: Option<String>,
    pub spec: Option<String>,
    pub shop_sign: Option<String>,
    pub adjust_date: Option<DateNative>,
    pub adjust_person: Option<String>,
    pub storage_cate: Option<i32>,
    pub trans_in_amount: Option<f64>,
    pub trans_out_amount: Option<f64>,
    pub storage_id: Option<u64>,
    pub adjust_status: Option<i32>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(StorageAdjust {
    id,
    company_code,
    in_warehouse,
    out_warehouse,
    adjust_cate,
    adjust_why,
    adjust_number,
    adjust_weight,
    variety,
    origin,
    spec,
    shop_sign,
    adjust_date,
    adjust_person,
    storage_cate,
    storage_id,
    trans_in_amount,
    trans_out_amount,
    adjust_status,
    created_at,
    updated_at
});

// 仓库管理
#[crud_table(table_name:storage_manage)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageManage {
    pub id: Option<u64>,
    pub uid: Option<String>,
    pub warehouse: Option<String>,
    pub variety: Option<String>,
    pub origin: Option<String>,
    pub shop_sign: Option<String>,
    pub spec: Option<String>,
    pub storage_number: Option<f64>,
    pub storage_weight: Option<f64>,
    pub can_sale_number: Option<f64>,
    pub can_sale_weight: Option<f64>,
    pub lock_number: Option<f64>,
    pub unit_price: Option<f64>,
    pub actual_price: Option<f64>,
    pub extra_price: Option<f64>,
    pub cacl_mode: Option<String>,
    pub resource_number: Option<String>,
    pub contract_number: Option<String>,
    pub vechel_number: Option<String>,
    pub storage_id: Option<String>,
    pub storage_owner: Option<String>,
    pub agree_uid: Option<String>,
    pub order_uid: Option<String>,
    pub instorage_date: Option<DateTimeNative>,
    pub check_date: Option<DateTimeNative>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(StorageManage {
    id,
    uid,
    warehouse,
    variety,
    origin,
    shop_sign,
    spec,
    storage_number,
    storage_weight,
    can_sale_number,
    can_sale_weight,
    lock_number,
    unit_price,
    actual_price,
    extra_price,
    cacl_mode,
    resource_number,
    contract_number,
    vechel_number,
    storage_id,
    storage_owner,
    agree_uid,
    order_uid,
    instorage_date,
    check_date,
    created_at,
    updated_at,
});

// 仓库管理
#[crud_table(table_name:storage_product)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageProduct {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub warehouse: Option<String>,
    pub variety: Option<String>,
    pub origin: Option<String>,
    pub shop_sign: Option<String>,
    pub spec: Option<String>,
    pub storage_number: Option<f64>,
    pub storage_weight: Option<f64>,
    pub lock_number: Option<f64>,
    pub lock_weight: Option<f64>,
    pub cacl_mode: Option<String>,
    pub way_weight: Option<String>,
    pub one_weight: Option<f64>,
    pub unit_price: Option<f64>,
    pub tax_rate: Option<f64>,
    pub unit: Option<String>,
    pub unit_money: Option<String>,
    pub resource_number: Option<String>,
    pub contract_number: Option<String>,
    pub vechel_number: Option<String>,
    pub package_number: Option<String>,
    pub order_id: Option<u64>,
    pub agree_id: Option<u64>,
    pub adjust_id: Option<u64>,
    pub product_id: Option<u64>,
    pub process_id: Option<u64>,
    pub in_storage_mode: Option<u8>, //入库方式 1 订单 2直接入库
    pub storage_cate: Option<u8>,    //库存类型 1 成品 2 废品 3 余料
    pub instorage_date: Option<DateNative>,
    pub storage_status: Option<u8>, //库存状态 1 入库在库 2 加工 3 已出库
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(StorageProduct {
    id,
    company_code,
    warehouse,
    variety,
    origin,
    shop_sign,
    spec,
    storage_number,
    storage_weight,
    lock_number,
    lock_weight,
    cacl_mode,
    way_weight,
    one_weight,
    unit_price,
    tax_rate,
    unit,
    unit_money,
    resource_number,
    contract_number,
    vechel_number,
    package_number,
    order_id,
    agree_id,
    product_id,
    process_id,
    in_storage_mode,
    storage_cate,
    instorage_date,
    storage_status
    created_at,
    updated_at,
});


// 仓库加工管理
#[crud_table(table_name:process)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Process {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub warehouse: Option<String>,
    pub process_number: Option<f64>,
    pub process_weight: Option<f64>,
    pub process_cate: Option<i32>,
    pub process_date: Option<DateNative>,
    pub process_status: Option<i32>,
    pub process_person: Option<String>,
    pub trans_in_amount: Option<f64>,
    pub trans_out_amount: Option<f64>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(Process {
    id,
    warehouse,
    process_number,
    process_weight,
    process_cate,
    process_date,
    process_status,
    process_person,
    trans_in_amount,
    trans_out_amount,
    created_at,
    updated_at,
});


#[crud_table(table_name:process_product)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessProduct {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub warehouse: Option<String>,
    pub variety: Option<String>,
    pub origin: Option<String>,
    pub shop_sign: Option<String>,
    pub spec: Option<String>,
    pub process_number: Option<f64>,
    pub process_weight: Option<f64>,
    pub cacl_mode: Option<String>,
    pub way_weight: Option<String>,
    pub one_weight: Option<f64>,
    pub unit: Option<String>,
    pub resource_number: Option<String>,
    pub contract_number: Option<String>,
    pub vechel_number: Option<String>,
    pub package_number: Option<String>,
    pub process_id: Option<u32>,
    pub storage_id: Option<u32>,
    pub process_date: Option<DateNative>,
    pub storage_status: Option<i32>,
    pub process_cate: Option<i32>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(ProcessProduct {
    id,
    company_code,
    warehouse,
    variety,
    origin,
    shop_sign,
    spec,
    process_number,
    process_weight,
    cacl_mode,
    way_weight,
    one_weight,
    unit,
    resource_number,
    contract_number,
    vechel_number,
    package_number,
    process_id,
    storage_id,
    process_date,
    storage_status,
    process_cate,
    created_at,
    updated_at,
});




// 仓库加工管理
#[crud_table(table_name:process_solution)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessSolution {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub process_id: Option<u64>,
    pub parent_list: Option<String>,
    pub sub_list: Option<String>,
    pub process_solution: Option<String>,
    pub process_person: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(ProcessSolution {
    id,
    company_code,
    process_id,
    parent_list,
    sub_list,
    process_solution,
    process_person,
    created_at,
    updated_at,
});