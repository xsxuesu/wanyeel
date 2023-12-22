use rbatis::{DateNative, DateTimeNative};
use serde::{Deserialize, Serialize};

// 采购支付
#[crud_table(table_name:payed_info)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PayedInfo {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub pay_name: Option<String>,
    pub agree_id: Option<u64>,
    pub order_id: Option<u64>,
    pub recieve_dept: Option<String>,
    pub pay_cate: Option<String>,
    pub pay_use: Option<String>,
    pub pay_way: Option<String>,
    pub pay_date: Option<DateNative>,
    pub pay_amount: Option<f64>,
    pub unit_money: Option<String>,
    pub is_invoice: Option<i32>,
    pub invoice_amount: Option<f64>,
    pub tax_rate: Option<f64>,
    pub invoice_weight: Option<f64>,
    pub unit: Option<String>,
    pub mome: Option<String>,
    pub pay_person: Option<String>,
    pub pay_status: Option<i32>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(PayedInfo {
    id,
    company_code,
    pay_name,
    agree_id,
    order_id,
    recieve_dept,
    pay_cate,
    pay_use,
    pay_way,
    pay_date,
    pay_amount,
    unit_money,
    is_invoice,
    invoice_amount,
    tax_rate,
    invoice_weight,
    unit,
    mome,
    pay_person,
    pay_status,
    created_at,
    updated_at,
});

// 采购支付
#[crud_table(table_name:recieved_info)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReceivedInfo {
    pub id: Option<u64>,
    pub recieve_name: Option<String>,
    pub sale_id: Option<u64>,
    pub company_code: Option<String>,
    pub pay_dept: Option<String>,
    pub recieve_cate: Option<String>,
    pub recieve_use: Option<String>,
    pub recieve_way: Option<String>,
    pub recieve_amount: Option<f64>,
    pub unit_money: Option<String>,
    pub recieve_person: Option<String>,
    pub recieve_date: Option<DateNative>,
    pub is_invoice: Option<i32>,
    pub invoice_amount: Option<f64>,
    pub tax_rate: Option<f64>,
    pub invoice_weight: Option<f64>,
    pub unit: Option<String>,
    pub mome: Option<String>,
    pub recieve_status: Option<i32>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}


impl_field_name_method!(ReceivedInfo {
    id,
    recieve_name,
    sale_id,
    company_code,
    pay_dept,
    recieve_cate,
    recieve_use,
    recieve_way,
    recieve_amount,
    unit_money,
    recieve_person,
    recieve_date,
    is_invoice,
    invoice_amount,
    tax_rate,
    invoice_weight,
    unit,
    mome,
    recieve_status,
    created_at,
    updated_at,
});

// 发票信息
#[crud_table(table_name:tax_info)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaxInfo {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub trans_id: Option<u64>,
    pub payed_id: Option<u64>,
    pub recieved_id: Option<u64>,
    pub tax_rate: Option<f64>,
    pub invoice_amount: Option<f64>,
    pub invoice_weight: Option<f64>,
    pub unit: Option<String>,
    pub unit_money: Option<String>,
    pub tax_person: Option<String>,
    pub tax_status: Option<u8>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(TaxInfo {
    id,
    company_code,
    trans_id,
    payed_id,
    recieved_id,
    tax_rate,
    invoice_amount,
    invoice_weight,
    unit,
    unit_money,
    tax_person,
    tax_status,
    created_at,
    updated_at,
});