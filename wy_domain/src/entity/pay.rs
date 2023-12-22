use rbatis::DateNative;
use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};

// 订单结算
#[crud_table(table_name:order_pay)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderPay {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub agree_id: Option<u64>,
    pub order_id: Option<u64>,
    pub agree_type: Option<String>,
    pub supply_dept: Option<String>,
    pub delivery_dept: Option<String>,
    pub buy_mode: Option<String>,
    pub buy_number: Option<f64>,
    pub buy_weight: Option<f64>,
    pub buy_amount: Option<f64>,
    pub storage_number: Option<f64>,
    pub storage_weight: Option<f64>,
    pub unit: Option<String>,
    pub agree_rate: Option<f64>,
    pub rate_amount: Option<f64>,
    pub pay_number: Option<f64>,
    pub pay_weight: Option<f64>,
    pub unit_money: Option<String>,
    pub product_amount: Option<f64>,
    pub trans_amount: Option<f64>,
    pub pay_amount: Option<f64>,
    pub payed_amount: Option<f64>,
    pub back_amount: Option<f64>,
    pub after_sale_amount: Option<f64>,
    pub after_sale_trans_amount: Option<f64>,
    pub money_way: Option<i32>,
    pub pay_date: Option<DateNative>,
    pub pay_person: Option<String>,
    pub mome: Option<String>,
    pub pay_status: Option<u8>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl_field_name_method!(OrderPay {
    id,
    order_id,
    agree_id,
    buy_number,
    buy_weight,
    buy_amount,
    storage_number,
    storage_weight,
    unit,
    unit_money,
    agree_rate,
    pay_number,
    pay_weight,
    product_amount,
    trans_amount,
    pay_amount,
    rate_amount,
    payed_amount,
    back_amount,
    after_sale_amount,
    after_sale_trans_amount,
    money_way,
    pay_date,
    pay_person,
    mome,
    created_at,
    updated_at,
});
