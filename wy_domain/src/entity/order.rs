use rbatis::DateNative;
use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};

// 采购订单
#[crud_table(table_name:order_info)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: Option<u64>,
    pub pay_id: Option<u64>,
    pub uid: Option<String>,
    pub company_code: Option<String>,
    pub buy_mode: Option<String>,
    pub buy_code: Option<i32>,
    pub supply_dept: Option<String>,
    pub buy_date: Option<DateNative>,
    pub delivery_dept: Option<String>,
    pub buy_person: Option<String>,
    pub storage_mode: Option<String>,
    pub pay_mode: Option<i32>,
    pub prepay_scale: Option<f64>,
    pub pay_date: Option<DateNative>,
    pub pay_amount: Option<f64>,
    pub all_pay_amount: Option<f64>,
    pub trans_in_amount: Option<f64>,
    pub trans_out_amount: Option<f64>,
    pub payed_amount: Option<f64>,
    pub after_sale_amount: Option<f64>,
    pub unit_money: Option<String>,
    pub order_status: Option<i32>,
    pub mome: Option<String>,
    pub from_type: Option<String>,
    pub from_id: Option<u64>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(Order {
    id,
    pay_id,
    uid,
    company_code,
    buy_mode,
    buy_code,
    supply_dept,
    buy_date,
    delivery_dept,
    buy_person,
    storage_mode,
    pay_mode,
    prepay_scale,
    pay_date,
    pay_amount,
    all_pay_amount,
    trans_in_amount,
    trans_out_amount,
    payed_amount,
    after_sale_amount,
    unit_money,
    order_status,
    mome,
    from_type,
    from_id,
    created_at,
    updated_at,
});

// 采购订单产品
#[crud_table(table_name:order_product)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderProduct {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub uid: Option<String>,
    pub order_id: Option<u64>,
    pub agree_id: Option<u64>,
    pub adjust_id: Option<u64>,
    pub variety: Option<String>,
    pub origin: Option<String>,
    pub warehouse: Option<String>,
    pub shop_sign: Option<String>,
    pub spec: Option<String>,
    pub cacl_mode: Option<String>,
    pub buy_number: Option<f64>,
    pub one_weight: Option<f64>,
    pub buy_weight: Option<f64>,
    pub unit_price: Option<f64>,
    pub buy_amount: Option<f64>,
    pub tax_rate: Option<f64>,
    pub unit: Option<String>,
    pub unit_money: Option<String>,
    pub way_weight: Option<String>,
    pub resource_number: Option<String>,
    pub contract_number: Option<String>,
    pub vechel_number: Option<String>,
    pub pack_number: Option<String>,
    pub mome: Option<String>,
    pub storage_status: Option<i32>,
    pub created_person: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(OrderProduct {
    id,
    company_code,
    uid,
    order_id,
    agree_id,
    adjust_id,
    variety,
    origin,
    warehouse
    shop_sign,
    spec,
    cacl_mode,
    buy_number,
    one_weight,
    buy_weight,
    unit_price,
    buy_amount,
    tax_rate,
    unit,
    unit_money,
    way_weight,
    resource_number,
    contract_number,
    vechel_number,
    pack_number,
    mome,
    storage_status,
    created_person,
    created_at,
    updated_at,
});

// 采购运杂费
#[crud_table(table_name:order_trans)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderTrans {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub uid: Option<String>,
    pub order_id: Option<u64>,
    pub agree_id: Option<u64>,
    pub adjust_id: Option<u64>,
    pub process_id: Option<u64>,
    pub sale_id: Option<u64>,
    pub after_sale_id: Option<u64>,
    pub buy_after_sale_id: Option<u64>,
    pub supply_dept: Option<String>,
    pub in_out: Option<i32>,
    pub cost_way: Option<String>,
    pub cost_code: Option<i32>,
    pub cost_date: Option<DateNative>,
    pub cost_category: Option<String>,
    pub unit_category: Option<String>,
    pub unit_number: Option<f64>,
    pub unit_price: Option<f64>,
    pub cost_rate: Option<f64>,
    pub cost_amount: Option<f64>,
    pub is_invoice: Option<i32>,
    pub invoice_amount: Option<f64>,
    pub create_person: Option<String>,
    pub unit_money: Option<String>,
    pub mome: Option<String>,
    pub trans_status: Option<i32>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(OrderTrans {
    id,
    company_code,
    uid,
    order_id,
    agree_id,
    adjust_id,
    process_id,
    sale_id,
    after_sale_id,
    buy_after_sale_id,
    supply_dept,
    in_out,
    cost_way,
    cost_code,
    cost_date,
    cost_category,
    unit_category,
    unit_number,
    unit_price,
    cost_rate,
    cost_amount,
    is_invoice,
    invoice_amount,
    create_person,
    unit_money,
    mome,
    trans_status,
    created_at,
    updated_at,
});

// 采购运杂费
#[crud_table(table_name:order_after_sale)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderAfterSale {
    pub id: Option<u64>,
    pub storage_id: Option<u64>,
    pub company_code: Option<String>,
    pub after_sale_name: Option<String>,
    pub order_id: Option<u64>,
    pub agree_id: Option<u64>,
    pub supply_dept: Option<String>,
    pub back_mode: Option<String>,
    pub back_money_way: Option<String>,
    pub back_money: Option<f64>,
    pub unit_money: Option<String>,
    pub trans_in_amount: Option<f64>,
    pub trans_out_amount: Option<f64>,
    pub back_date: Option<DateNative>,
    pub back_person: Option<String>,
    pub back_status: Option<i32>,
    pub mome: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}


impl_field_name_method!(OrderAfterSale {
    id,
    storage_id,
    company_code,
    after_sale_name,
    order_id,
    agree_id,
    supply_dept,
    back_mode,
    back_money_way,
    back_money,
    trans_in_amount,
    trans_out_amount,
    back_date,
    back_person,
    back_status,
    mome,
    created_at,
    updated_at,
});

// 售后产品表
#[crud_table(table_name:order_after_sale_product)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderAfterSaleProduct {
    pub id: Option<u64>,
    pub after_sale_id: Option<u64>,
    pub order_prodct_id: Option<u64>,
    pub company_code: Option<String>,
    pub uid: Option<String>,
    pub order_id: Option<u64>,
    pub agree_id: Option<u64>,
    pub variety: Option<String>,
    pub origin: Option<String>,
    pub warehouse: Option<String>,
    pub shop_sign: Option<String>,
    pub spec: Option<String>,
    pub cacl_mode: Option<String>,
    pub back_weight: Option<f64>,
    pub back_number: Option<f64>,
    pub buy_number: Option<f64>,
    pub one_weight: Option<f64>,
    pub buy_weight: Option<f64>,
    pub way_weight: Option<String>,
    pub unit_price: Option<f64>,
    pub buy_amount: Option<f64>,
    pub tax_rate: Option<f64>,
    pub unit: Option<String>,
    pub unit_money: Option<String>,
    pub resource_number: Option<String>,
    pub contract_number: Option<String>,
    pub vechel_number: Option<String>,
    pub pack_number: Option<String>,
    pub created_person: Option<String>,
    pub after_sale_status: Option<i32>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(OrderAfterSaleProduct {
    id,
    after_sale_id,
    order_prodct_id,
    company_code,
    uid,
    order_id,
    agree_id,
    variety,
    origin,
    warehouse,
    shop_sign,
    spec,
    cacl_mode,
    buy_number,
    one_weight,
    buy_weight,
    way_weight,
    unit_price,
    buy_amount,
    tax_rate,
    unit,
    unit_money,
    resource_number,
    contract_number,
    vechel_number,
    pack_number,
    created_person,
    after_sale_status,
    created_at,
    updated_at,
});