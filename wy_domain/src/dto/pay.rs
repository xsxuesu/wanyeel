use crate::entity::pay::*;
use rbatis::DateNative;
use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OrderPayDTO {
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
    pub agree_rate: Option<f64>,
    pub unit: Option<String>,
    pub unit_money: Option<String>,
    pub product_amount: Option<f64>,
    pub trans_amount: Option<f64>,
    pub rate_amount: Option<f64>,
    pub pay_number: Option<f64>,
    pub pay_weight: Option<f64>,
    pub pay_amount: Option<f64>,
    pub payed_amount: Option<f64>,
    pub back_amount: Option<f64>,
    pub after_sale_amount: Option<f64>,
    pub after_sale_trans_amount: Option<f64>,
    pub money_way: Option<i32>,
    pub pay_date: Option<String>,
    pub pay_person: Option<String>,
    pub mome: Option<String>,
    pub pay_status: Option<u8>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<OrderPay> for OrderPayDTO {
    fn into(self) -> OrderPay {
        OrderPay {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            agree_id: self.agree_id.clone(),
            order_id: self.order_id.clone(),
            agree_type: self.agree_type.clone(),
            supply_dept: self.supply_dept.clone(),
            delivery_dept: self.delivery_dept.clone(),
            buy_mode: self.buy_mode.clone(),
            buy_number: self.buy_number.clone(),
            buy_weight: self.buy_weight.clone(),
            buy_amount: self.buy_amount.clone(),
            storage_number: self.storage_number.clone(),
            storage_weight: self.storage_weight.clone(),
            agree_rate: self.agree_rate.clone(),
            rate_amount: self.rate_amount.clone(),
            unit: self.unit.clone(),
            unit_money: self.unit_money.clone(),
            product_amount: self.product_amount.clone(),
            trans_amount: self.trans_amount.clone(),
            pay_number: self.pay_number.clone(),
            pay_weight: self.pay_weight.clone(),
            pay_amount: self.pay_amount.clone(),
            payed_amount: self.payed_amount.clone(),
            back_amount: self.back_amount.clone(),
            after_sale_amount: self.after_sale_amount.clone(),
            after_sale_trans_amount: self.after_sale_trans_amount.clone(),
            money_way: self.money_way.clone(),
            pay_date: Some(DateNative::from_str(self.pay_date.clone().unwrap().as_str()).unwrap()),
            pay_person: self.pay_person.clone(),
            mome: self.mome.clone(),
            pay_status: self.pay_status.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<OrderPay> for OrderPayDTO {
    fn from(arg: OrderPay) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            order_id: arg.order_id,
            agree_id: arg.agree_id,
            agree_type: arg.agree_type,
            supply_dept: arg.supply_dept,
            delivery_dept: arg.delivery_dept,
            buy_mode: arg.buy_mode,
            buy_number: arg.buy_number,
            buy_weight: arg.buy_weight,
            buy_amount: arg.buy_amount,
            storage_number: arg.storage_number,
            storage_weight: arg.storage_weight,
            agree_rate: arg.agree_rate,
            rate_amount: arg.rate_amount,
            unit: arg.unit,
            unit_money: arg.unit_money,
            product_amount:arg.product_amount,
            trans_amount: arg.trans_amount,
            pay_number: arg.pay_number,
            pay_weight: arg.pay_weight,
            pay_amount: arg.pay_amount,
            payed_amount: arg.payed_amount,
            back_amount: arg.back_amount,
            after_sale_amount: arg.after_sale_amount,
            after_sale_trans_amount: arg.after_sale_trans_amount,
            money_way: arg.money_way,
            pay_date: Some(arg.pay_date.unwrap().to_string()),
            pay_person: arg.pay_person,
            mome: arg.mome,
            pay_status: arg.pay_status,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OrderPayFilterParams {
    pub company_code: Option<String>,
    pub order_pay_start: Option<DateNative>,
    pub order_pay_end: Option<DateNative>,
    pub supply_dept: Option<String>,
    pub buy_mode: Option<String>,
    pub page_size: Option<u64>,
    pub page_no: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OrderPayParams {
    pub company_code: Option<String>,
    pub order_pay_start: Option<DateNative>,
    pub order_pay_end: Option<DateNative>,
    pub supply_dept: Option<String>,
    pub buy_mode: Option<String>,
}