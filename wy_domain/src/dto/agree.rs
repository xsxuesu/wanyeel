use crate::entity::agree::*;
use rbatis::DateTimeNative;
use rbatis::DateNative;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct AgreeDTO {
   pub id: Option<u64>,
   pub pay_id: Option<u64>,
   pub uid: Option<String>,
   pub company_code: Option<String>,
   pub contract_no: Option<String>,
   pub agree_type: Option<String>,
   pub supply_dept: Option<String>,
   pub delivery_dept: Option<String>,
   pub agree_person: Option<String>,
   pub agree_date: Option<String>,
   pub deposit_amount: Option<f64>,
   pub unit_money: Option<String>,
   pub agree_amount: Option<f64>,
   pub interest_rate: Option<f64>,
   pub interest_start: Option<String>,
   pub limit_days: Option<i32>,
   pub agree_status: Option<i32>,
   pub all_amount: Option<f64>,
   pub payed_amount: Option<f64>,
   pub after_sale_amount: Option<f64>,
   pub trans_in_amount: Option<f64>,
   pub trans_out_amount: Option<f64>,
   pub mome: Option<String>,
   pub  created_at: Option<DateTimeNative>,
   pub  updated_at: Option<DateTimeNative>,
}

impl Into<Agree> for AgreeDTO {
    fn into(self) -> Agree {
        Agree {
            id: self.id.clone(),
            pay_id: self.pay_id.clone(),
            uid: self.uid.clone(),
            contract_no: self.contract_no.clone(),
            company_code: self.company_code.clone(),
            agree_type: self.agree_type.clone(),
            supply_dept: self.supply_dept.clone(),
            delivery_dept: self.delivery_dept.clone(),
            agree_person: self.agree_person.clone(),
            agree_date: Some(DateNative::from_str(self.agree_date.clone().unwrap().as_str()).unwrap()),
            deposit_amount: self.deposit_amount.clone(),
            unit_money: self.unit_money.clone(),
            agree_amount: self.agree_amount.clone(),
            interest_rate: self.interest_rate.clone(),
            limit_days: self.limit_days.clone(),
            interest_start: Some(DateNative::from_str(self.interest_start.clone().unwrap().as_str()).unwrap()),
            agree_status: self.agree_status.clone(),
            mome: self.mome.clone(),
            all_amount: self.all_amount.clone(),
            payed_amount: self.payed_amount.clone(),
            after_sale_amount: self.after_sale_amount.clone(),
            trans_in_amount: self.trans_in_amount.clone(),
            trans_out_amount: self.trans_out_amount.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<Agree> for AgreeDTO {
    fn from(arg: Agree) -> Self {
        Self {
            id: arg.id,
            pay_id: arg.pay_id,
            uid: arg.uid,
            contract_no: arg.contract_no,
            company_code: arg.company_code,
            agree_type: arg.agree_type,
            supply_dept: arg.supply_dept,
            delivery_dept: arg.delivery_dept,
            agree_person: arg.agree_person,
            agree_date:  Some(arg.agree_date.unwrap().to_string()),
            deposit_amount: arg.deposit_amount,
            unit_money: arg.unit_money,
            agree_amount: arg.agree_amount,
            interest_rate: arg.interest_rate,
            limit_days: arg.limit_days,
            interest_start: Some(arg.interest_start.unwrap().to_string()),
            agree_status: arg.agree_status,
            all_amount: arg.all_amount,
            payed_amount: arg.payed_amount,
            after_sale_amount: arg.after_sale_amount,
            trans_in_amount:arg.trans_in_amount,
            trans_out_amount:arg.trans_out_amount,
            mome: arg.mome,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}



#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct AgreeProductDTO {
    id: Option<u64>,
    sale_id: Option<u64>,
    agree_id: Option<u64>,
    company_code: Option<String>,
    variety: Option<String>,
    origin: Option<String>,
    shop_sign: Option<String>,
    spec: Option<String>,
    cacl_mode: Option<String>,
    unit: Option<String>,
    unit_money: Option<String>,
    buy_number: Option<f64>,
    one_weight: Option<f64>,
    buy_weight: Option<f64>,
    agree_unitprice: Option<f64>,
    agree_amount: Option<f64>,
    tax_rate: Option<f64>,
    warehouse: Option<String>,
    created_at: Option<DateTimeNative>,
    updated_at: Option<DateTimeNative>,
}



impl Into<AgreeProduct> for AgreeProductDTO {
    fn into(self) -> AgreeProduct {
        AgreeProduct {
            id: self.id.clone(),
            sale_id: self.sale_id.clone(),
            agree_id: self.agree_id.clone(),
            company_code: self.company_code.clone(),
            variety: self.variety.clone(),
            origin: self.origin.clone(),
            shop_sign: self.shop_sign.clone(),
            spec: self.spec.clone(),
            cacl_mode: self.cacl_mode.clone(),
            unit: self.unit.clone(),
            unit_money: self.unit_money.clone(),
            buy_number: self.buy_number.clone(),
            one_weight: self.one_weight.clone(),
            buy_weight: self.buy_weight.clone(),
            agree_unitprice: self.agree_unitprice.clone(),
            agree_amount: self.agree_amount.clone(),
            tax_rate: self.tax_rate.clone(),
            warehouse: self.warehouse.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<AgreeProduct> for AgreeProductDTO {
    fn from(arg: AgreeProduct) -> Self {
        Self {
            id: arg.id,
            sale_id: arg.sale_id,
            agree_id: arg.agree_id,
            company_code: arg.company_code,
            variety: arg.variety,
            origin: arg.origin,
            shop_sign: arg.shop_sign,
            spec: arg.spec,
            cacl_mode: arg.cacl_mode,
            unit: arg.unit,
            unit_money: arg.unit_money,
            buy_number: arg.buy_number,
            one_weight: arg.one_weight,
            buy_weight: arg.buy_weight,
            agree_unitprice: arg.agree_unitprice,
            agree_amount: arg.agree_amount,
            tax_rate: arg.tax_rate,
            warehouse: arg.warehouse,
            created_at: arg.created_at,
            updated_at: arg.updated_at,

        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct AgreeFilterParams{
    pub company_code: Option<String>,
    pub  agree_start: Option<DateNative>,
    pub  agree_end: Option<DateNative>,
    pub  supply_dept: Option<String>,
    pub  delivery_dept: Option<String>,
    pub  agree_status: Option<i32>,
    pub  page_size: Option<u64>,
    pub page_no: Option<u64>,
  }

  #[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct AgreeParams{
    pub company_code: Option<String>,
    pub agree_start: Option<DateNative>,
    pub agree_end: Option<DateNative>,
    pub supply_dept: Option<String>,
    pub delivery_dept: Option<String>,
    pub  agree_status: Option<i32>,
  }


  #[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
  #[getset(get = "pub", set = "pub")]
  pub struct OrderByFilterDTO{
      pub company_code: Option<String>,
      pub order_start: Option<String>,
      pub order_end: Option<String>,
}