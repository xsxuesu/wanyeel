use crate::entity::finance::*;
use rbatis::DateNative;
use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};
use rbatis::decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct PayedInfoDTO {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub pay_name: Option<String>,
    pub agree_id: Option<u64>,
    pub order_id: Option<u64>,
    pub recieve_dept: Option<String>,
    pub pay_cate: Option<String>,
    pub pay_use: Option<String>,
    pub pay_way: Option<String>,
    pub pay_date: Option<String>,
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

// Some(DateNative::from_str(self.agree_date.clone().unwrap().as_str()).unwrap()),
impl Into<PayedInfo> for PayedInfoDTO {
    fn into(self) -> PayedInfo {
        PayedInfo {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            pay_name: self.pay_name.clone(),
            agree_id: self.agree_id.clone(),
            order_id: self.order_id.clone(),
            recieve_dept: self.recieve_dept.clone(),
            pay_cate: self.pay_cate.clone(),
            pay_use: self.pay_use.clone(),
            pay_way: self.pay_way.clone(),
            pay_date: Some(DateNative::from_str(self.pay_date.clone().unwrap().as_str()).unwrap()),
            pay_amount: self.pay_amount.clone(),
            unit_money: self.unit_money.clone(),
            is_invoice: self.is_invoice.clone(),
            invoice_amount: self.invoice_amount.clone(),
            tax_rate: self.tax_rate.clone(),
            invoice_weight: self.invoice_weight.clone(),
            unit: self.unit.clone(),
            mome: self.mome.clone(),
            pay_person: self.pay_person.clone(),
            pay_status: self.pay_status.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}
// Some(arg.after_sale_date.unwrap().to_string()),
impl From<PayedInfo> for PayedInfoDTO {
    fn from(arg: PayedInfo) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            pay_name: arg.pay_name,
            agree_id: arg.agree_id,
            order_id: arg.order_id,
            recieve_dept: arg.recieve_dept,
            pay_cate: arg.pay_cate,
            pay_use: arg.pay_use,
            pay_way: arg.pay_way,
            pay_date: Some(arg.pay_date.unwrap().to_string()),
            pay_amount: arg.pay_amount,
            unit_money: arg.unit_money,
            is_invoice: arg.is_invoice,
            invoice_amount: arg.invoice_amount,
            tax_rate: arg.tax_rate,
            invoice_weight: arg.invoice_weight,
            unit: arg.unit,
            mome: arg.mome,
            pay_person: arg.pay_person,
            pay_status: arg.pay_status,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ReceivedInfoDTO {
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
    pub recieve_date: Option<String>,
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

// Some(DateNative::from_str(self.agree_date.clone().unwrap().as_str()).unwrap()),
impl Into<ReceivedInfo> for ReceivedInfoDTO {
    fn into(self) -> ReceivedInfo {
        ReceivedInfo {
            id: self.id.clone(),
            recieve_name: self.recieve_name.clone(),
            sale_id: self.sale_id.clone(),
            company_code: self.company_code.clone(),
            pay_dept: self.pay_dept.clone(),
            recieve_cate: self.recieve_cate.clone(),
            recieve_use: self.recieve_use.clone(),
            recieve_way: self.recieve_way.clone(),
            recieve_amount: self.recieve_amount.clone(),
            unit_money: self.unit_money.clone(),
            recieve_person: self.recieve_person.clone(),
            recieve_date: Some(DateNative::from_str(self.recieve_date.clone().unwrap().as_str()).unwrap()),
            is_invoice: self.is_invoice.clone(),
            invoice_amount: self.invoice_amount.clone(),
            tax_rate: self.tax_rate.clone(),
            invoice_weight: self.invoice_weight.clone(),
            unit: self.unit.clone(),
            mome: self.mome.clone(),
            recieve_status: self.recieve_status.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}
// Some(arg.after_sale_date.unwrap().to_string()),
impl From<ReceivedInfo> for ReceivedInfoDTO {
    fn from(arg: ReceivedInfo) -> Self {
        Self {
            id: arg.id,
            recieve_name: arg.recieve_name,
            sale_id: arg.sale_id,
            company_code: arg.company_code,
            pay_dept: arg.pay_dept,
            recieve_cate: arg.recieve_cate,
            recieve_use: arg.recieve_use,
            recieve_way: arg.recieve_way,
            recieve_amount: arg.recieve_amount,
            unit_money: arg.unit_money,
            recieve_person: arg.recieve_person,
            recieve_date: Some(arg.recieve_date.unwrap().to_string()),
            is_invoice: arg.is_invoice,
            invoice_amount: arg.invoice_amount,
            tax_rate: arg.tax_rate,
            invoice_weight: arg.invoice_weight,
            unit: arg.unit,
            mome: arg.mome,
            recieve_status: arg.recieve_status,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct PayedInfoFilterParams {
    pub payed_date_start: Option<String>,
    pub payed_date_end: Option<String>,
    pub company_code: Option<String>,
    pub client_name: Option<String>,
    pub page_size: Option<u64>,
    pub page_no: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ReceivedInfoFilterParams {
    pub receieved_date_start: Option<String>,
    pub receieved_date_end: Option<String>,
    pub company_code: Option<String>,
    pub client_name: Option<String>,
    pub page_size: Option<u64>,
    pub page_no: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct TransStaticalParams {
    pub company_code: Option<String>,
    pub trans_date_start: Option<String>,
    pub trans_date_end: Option<String>,
    pub trans_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct TransStaticalItem {
    pub supply_dept: Option<String>,
    pub cost_all_amount: Option<f64>,
    pub recieve_all_amount: Option<f64>,
    pub un_invoice_amount: Option<f64>,
    pub invoice_amount: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct FinanceStaticalParams {
    pub company_code: Option<String>,
    pub finance_date_start: Option<String>,
    pub finance_date_end: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct PayedStaticalItem {
    pub recieve_dept: Option<String>,
    pub pay_amount: Option<f64>,
    pub un_invoice_amount: Option<f64>,
    pub invoice_amount: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct RecieveStaticalItem {
    pub pay_dept: Option<String>,
    pub recieve_amount: Option<f64>,
    pub un_invoice_amount: Option<f64>,
    pub invoice_amount: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct BuyStaticalParams {
    pub company_code: Option<String>,
    pub payed_date_start: Option<String>,
    pub payed_date_end: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SaleStaticalParams {
    pub company_code: Option<String>,
    pub sale_date_start: Option<String>,
    pub sale_date_end: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct PayedStatisticItem {
    pub supply_dept: Option<String>,
    pub agree_amount: Option<String>,
    pub buy_number: Option<String>,
    pub buy_weight: Option<String>,
    pub instorage_number: Option<String>,
    pub instorage_weight: Option<String>,
    pub instorage_amount: Option<String>,
    pub cost_all_amount: Option<String>,
    pub recieve_all_amount: Option<String>,
    pub back_number: Option<String>,
    pub back_weight: Option<String>,
    pub back_money: Option<String>,
    pub pay_amount: Option<String>,
    pub invoice_amount: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SaleStatisticItem {
    pub client_name: Option<String>,
    pub sale_amount: Option<String>,
    pub sale_number: Option<String>,
    pub sale_weight: Option<String>,
    pub outstorage_number: Option<String>,
    pub outstorage_weight: Option<String>,
    pub cost_all_amount: Option<String>,
    pub recieve_all_amount: Option<String>,
    pub back_number: Option<String>,
    pub back_weight: Option<String>,
    pub back_amount: Option<String>,
    pub recieve_amount: Option<String>,
    pub invoice_amount: Option<String>,
}



#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct TaxInfoDTO {
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

// Some(DateNative::from_str(self.agree_date.clone().unwrap().as_str()).unwrap()),
impl Into<TaxInfo> for TaxInfoDTO {
    fn into(self) -> TaxInfo {
        TaxInfo {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            trans_id: self.trans_id.clone(),
            payed_id: self.payed_id.clone(),
            recieved_id: self.recieved_id.clone(),
            tax_rate: self.tax_rate.clone(),
            invoice_amount: self.invoice_amount.clone(),
            invoice_weight: self.invoice_weight.clone(),
            unit: self.unit.clone(),
            unit_money: self.unit_money.clone(),
            tax_person: self.tax_person.clone(),
            tax_status: self.tax_status.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}
// Some(arg.after_sale_date.unwrap().to_string()),
impl From<TaxInfo> for TaxInfoDTO {
    fn from(arg: TaxInfo) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            trans_id: arg.trans_id,
            payed_id: arg.payed_id,
            recieved_id: arg.recieved_id,
            tax_rate: arg.tax_rate,
            invoice_amount: arg.invoice_amount,
            invoice_weight: arg.invoice_weight,
            unit: arg.unit,
            unit_money: arg.unit_money,
            tax_person: arg.tax_person,
            tax_status: arg.tax_status,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}
