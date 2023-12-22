use crate::entity::sale::*;
use rbatis::DateNative;
use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SaleInfoDTO {
    pub id: Option<u64>,
    pub sale_pay_id: Option<u64>,
    pub company_code: Option<String>,
    pub client_name: Option<String>,
    pub sale_mode: Option<i32>,
    pub sale_date: Option<String>,
    pub delivery_cate: Option<i32>,
    pub latest_delivery_date: Option<String>,
    pub sale_tax: Option<f64>,
    pub charge_mode: Option<i32>,
    pub charge_pre_scale: Option<f64>,
    pub charge_pre_amount: Option<f64>,
    pub pre_sale_all_amount: Option<f64>,
    pub sale_all_amount: Option<f64>,
    pub recieved_amount: Option<f64>,
    pub trans_in_amount: Option<f64>,
    pub trans_out_amount: Option<f64>,
    pub after_sale_amount: Option<f64>,
    pub unit_money: Option<String>,
    pub charge_date: Option<String>,
    pub charge_way: Option<String>,
    pub sale_person: Option<String>,
    pub sale_status: Option<i32>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

// Some(DateNative::from_str(self.agree_date.clone().unwrap().as_str()).unwrap()),
impl Into<SaleInfo> for SaleInfoDTO {
    fn into(self) -> SaleInfo {
        SaleInfo {
            id: self.id.clone(),
            sale_pay_id: self.sale_pay_id.clone(),
            company_code: self.company_code.clone(),
            client_name: self.client_name.clone(),
            sale_mode: self.sale_mode.clone(),
            sale_date: Some(DateNative::from_str(self.sale_date.clone().unwrap().as_str()).unwrap()),
            delivery_cate: self.delivery_cate.clone(),
            latest_delivery_date: Some(DateNative::from_str(self.latest_delivery_date.clone().unwrap().as_str()).unwrap()),
            sale_tax: self.sale_tax.clone(),
            pre_sale_all_amount: self.pre_sale_all_amount.clone(),
            sale_all_amount: self.sale_all_amount.clone(),
            recieved_amount: self.recieved_amount.clone(),
            trans_in_amount: self.trans_in_amount.clone(),
            trans_out_amount: self.trans_out_amount.clone(),
            charge_mode: self.charge_mode.clone(),
            charge_pre_scale: self.charge_pre_scale.clone(),
            charge_pre_amount: self.charge_pre_amount.clone(),
            after_sale_amount: self.after_sale_amount.clone(),
            unit_money: self.unit_money.clone(),
            charge_date: Some(DateNative::from_str(self.charge_date.clone().unwrap().as_str()).unwrap()),
            charge_way: self.charge_way.clone(),
            sale_person: self.sale_person.clone(),
            sale_status: self.sale_status.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<SaleInfo> for SaleInfoDTO {
    fn from(arg: SaleInfo) -> Self {
        Self {
            id: arg.id,
            sale_pay_id: arg.sale_pay_id,
            company_code: arg.company_code,
            client_name: arg.client_name,
            sale_mode: arg.sale_mode,
            sale_date: Some(arg.sale_date.unwrap().to_string()),
            delivery_cate: arg.delivery_cate,
            latest_delivery_date: Some(arg.latest_delivery_date.unwrap().to_string()),
            sale_tax: arg.sale_tax,
            charge_mode: arg.charge_mode,
            charge_pre_scale: arg.charge_pre_scale,
            charge_pre_amount: arg.charge_pre_amount,
            unit_money: arg.unit_money,
            pre_sale_all_amount: arg.pre_sale_all_amount,
            sale_all_amount: arg.sale_all_amount,
            recieved_amount: arg.recieved_amount,
            trans_in_amount: arg.trans_in_amount,
            trans_out_amount: arg.trans_out_amount,
            after_sale_amount: arg.after_sale_amount,
            charge_date: Some(arg.charge_date.unwrap().to_string()),
            charge_way: arg.charge_way,
            sale_person: arg.sale_person,
            sale_status: arg.sale_status,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SaleInfoPageDTO {
    pub sale_mode: Option<i32>,
    pub sale_start: Option<String>,
    pub sale_end: Option<String>,
    pub company_code: Option<String>,
    pub client_name: Option<String>,
    pub page_size: Option<u64>,
    pub page_no: Option<u64>,
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SaleProductDTO {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub warehouse: Option<String>,
    pub variety: Option<String>,
    pub origin: Option<String>,
    pub shop_sign: Option<String>,
    pub spec: Option<String>,
    pub sale_number: Option<f64>,
    pub sale_weight: Option<f64>,
    pub unit_price: Option<f64>,
    pub cacl_mode: Option<String>,
    pub way_weight: Option<String>,
    pub one_weight: Option<f64>,
    pub tax_rate: Option<f64>,
    pub unit: Option<String>,
    pub unit_money: Option<String>,
    pub resource_number: Option<String>,
    pub contract_number: Option<String>,
    pub vechel_number: Option<String>,
    pub package_number: Option<String>,
    pub storage_id: Option<u64>,
    pub sale_id: Option<u64>,
    pub storage_cate: Option<i32>,
    pub instorage_date: Option<String>,
    pub outstorage_date: Option<String>,
    pub sale_status: Option<i32>,
    pub back_number: Option<f64>,
    pub back_weight: Option<f64>,
    pub back_amount: Option<f64>,
    pub back_status: Option<i32>,
    pub sale_after_id: Option<u64>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<SaleProduct> for SaleProductDTO {
    fn into(self) -> SaleProduct {
        SaleProduct {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            warehouse: self.warehouse.clone(),
            variety: self.variety.clone(),
            origin: self.origin.clone(),
            shop_sign: self.shop_sign.clone(),
            spec: self.spec.clone(),
            sale_number: self.sale_number.clone(),
            sale_weight: self.sale_weight.clone(),
            unit_price: self.unit_price.clone(),
            cacl_mode: self.cacl_mode.clone(),
            way_weight: self.way_weight.clone(),
            one_weight: self.one_weight.clone(),
            tax_rate: self.tax_rate.clone(),
            unit: self.unit.clone(),
            unit_money: self.unit_money.clone(),
            resource_number: self.resource_number.clone(),
            contract_number: self.contract_number.clone(),
            vechel_number: self.vechel_number.clone(),
            package_number: self.package_number.clone(),
            storage_id: self.storage_id.clone(),
            sale_id: self.sale_id.clone(),
            storage_cate: self.storage_cate.clone(),
            instorage_date: Some(DateNative::from_str(self.instorage_date.clone().unwrap().as_str()).unwrap()),
            outstorage_date: Some(DateNative::from_str(self.outstorage_date.clone().unwrap().as_str()).unwrap()),
            sale_status: self.sale_status.clone(),
            back_number: self.back_number.clone(),
            back_weight: self.back_weight.clone(),
            back_amount: self.back_amount.clone(),
            back_status: self.back_status.clone(),
            sale_after_id: self.sale_after_id.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}


impl From<SaleProduct> for SaleProductDTO {
    fn from(arg: SaleProduct) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            warehouse: arg.warehouse,
            variety: arg.variety,
            origin: arg.origin,
            shop_sign: arg.shop_sign,
            spec: arg.spec,
            sale_number: arg.sale_number,
            sale_weight: arg.sale_weight,
            unit_price: arg.unit_price,
            cacl_mode: arg.cacl_mode,
            way_weight: arg.way_weight,
            one_weight: arg.one_weight,
            tax_rate: arg.tax_rate,
            unit: arg.unit,
            unit_money: arg.unit_money,
            resource_number: arg.resource_number,
            contract_number: arg.contract_number,
            vechel_number: arg.vechel_number,
            package_number: arg.package_number,
            storage_id: arg.storage_id,
            sale_id: arg.sale_id,
            storage_cate: arg.storage_cate,
            instorage_date: Some(arg.instorage_date.unwrap().to_string()),
            outstorage_date: Some(arg.outstorage_date.unwrap().to_string()),
            sale_status: arg.sale_status,
            back_number: arg.back_number,
            back_amount: arg.back_amount,
            back_weight: arg.back_weight,
            back_status: arg.back_status,
            sale_after_id: arg.sale_after_id,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


  #[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
  #[getset(get = "pub", set = "pub")]
  pub struct SaleProductFilterDTO {
      pub sale_id: Option<u64>,
      pub company_code: Option<String>,
    //   pub client_name: Option<String>,
      pub page_size: Option<u64>,
      pub page_no: Option<u64>,
  }


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct PreSaleProductDTO {
    pub id: Option<u64>,
    pub sale_id: Option<u64>,
    pub company_code: Option<String>,
    pub variety: Option<String>,
    pub origin: Option<String>,
    pub shop_sign: Option<String>,
    pub spec: Option<String>,
    pub cacl_mode: Option<String>,
    pub unit: Option<String>,
    pub unit_money: Option<String>,
    pub presale_number: Option<f64>,
    pub presale_weight: Option<f64>,
    pub presale_all_weight: Option<f64>,
    pub presale_unit_price: Option<f64>,
    pub presale_amount: Option<f64>,
    pub tax_rate: Option<f64>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}


// Some(DateNative::from_str(self.agree_date.clone().unwrap().as_str()).unwrap()),
impl Into<PreSaleProduct> for PreSaleProductDTO {
    fn into(self) -> PreSaleProduct {
        PreSaleProduct {
            id: self.id.clone(),
            sale_id: self.sale_id.clone(),
            company_code: self.company_code.clone(),
            variety: self.variety.clone(),
            origin: self.origin.clone(),
            shop_sign: self.shop_sign.clone(),
            spec: self.spec.clone(),
            cacl_mode: self.cacl_mode.clone(),
            unit: self.unit.clone(),
            unit_money: self.unit_money.clone(),
            presale_number: self.presale_number.clone(),
            presale_weight: self.presale_weight.clone(),
            presale_all_weight: self.presale_all_weight.clone(),
            presale_unit_price: self.presale_unit_price.clone(),
            presale_amount: self.presale_amount.clone(),
            tax_rate: self.tax_rate.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<PreSaleProduct> for PreSaleProductDTO {
    fn from(arg: PreSaleProduct) -> Self {
        Self {
            id: arg.id,
            sale_id: arg.sale_id,
            company_code: arg.company_code,
            variety: arg.variety,
            origin: arg.origin,
            shop_sign: arg.shop_sign,
            spec: arg.spec,
            cacl_mode: arg.cacl_mode,
            unit: arg.unit,
            unit_money: arg.unit_money,
            presale_number: arg.presale_number,
            presale_weight: arg.presale_weight,
            presale_all_weight: arg.presale_all_weight,
            presale_unit_price: arg.presale_unit_price,
            presale_amount: arg.presale_amount,
            tax_rate: arg.tax_rate,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SalePayDTO {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub client_name: Option<String>,
    pub sale_id: Option<u64>,
    pub sale_type: Option<i32>,
    pub sale_number: Option<f64>,
    pub sale_weight: Option<f64>,
    pub out_number: Option<f64>,
    pub out_weight: Option<f64>,
    pub sale_unit_price: Option<f64>,
    pub sale_amount: Option<f64>,
    pub pay_weight: Option<f64>,
    pub tax_rate: Option<f64>,
    pub pay_amount: Option<f64>,
    pub recieved_amount: Option<f64>,
    pub invoice_amount: Option<f64>,
    pub after_sale_amount: Option<f64>,
    pub after_sale_trans_amount: Option<f64>,
    pub pay_diff_amount: Option<f64>,
    pub money_way: Option<i32>,
    pub pay_date: Option<String>,
    pub pay_person: Option<String>,
    pub unit: Option<String>,
    pub unit_money: Option<String>,
    pub sale_pay_status: Option<i32>,
    pub trans_amount: Option<f64>,
    pub rate_amount: Option<f64>,
    pub mome: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}


// Some(DateNative::from_str(self.agree_date.clone().unwrap().as_str()).unwrap()),
impl Into<SalePay> for SalePayDTO {
    fn into(self) -> SalePay {
        SalePay {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            client_name: self.client_name.clone(),
            sale_id: self.sale_id.clone(),
            sale_type: self.sale_type.clone(),
            sale_number: self.sale_number.clone(),
            sale_weight: self.sale_weight.clone(),
            out_number: self.out_number.clone(),
            out_weight: self.out_weight.clone(),
            sale_unit_price: self.sale_unit_price.clone(),
            sale_amount: self.sale_amount.clone(),
            pay_weight: self.pay_weight.clone(),
            tax_rate: self.tax_rate.clone(),
            pay_amount: self.pay_amount.clone(),
            recieved_amount: self.recieved_amount.clone(),
            invoice_amount: self.invoice_amount.clone(),
            after_sale_amount: self.after_sale_amount.clone(),
            after_sale_trans_amount: self.after_sale_trans_amount.clone(),
            pay_diff_amount: self.pay_diff_amount.clone(),
            money_way: self.money_way.clone(),
            pay_date: Some(DateNative::from_str(self.pay_date.clone().unwrap().as_str()).unwrap()),
            pay_person: self.pay_person.clone(),
            unit: self.unit.clone(),
            unit_money: self.unit_money.clone(),
            sale_pay_status: self.sale_pay_status.clone(),
            trans_amount: self.trans_amount.clone(),
            rate_amount: self.rate_amount.clone(),
            mome:self.mome.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),

        }
    }
}

impl From<SalePay> for SalePayDTO {
    fn from(arg: SalePay) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            client_name: arg.client_name,
            sale_id: arg.sale_id,
            sale_type: arg.sale_type,
            sale_number: arg.sale_number,
            sale_weight: arg.sale_weight,
            out_number: arg.out_number,
            out_weight: arg.out_weight,
            sale_unit_price: arg.sale_unit_price,
            sale_amount: arg.sale_amount,
            pay_weight: arg.pay_weight,
            tax_rate: arg.tax_rate,
            pay_amount: arg.pay_amount,
            recieved_amount: arg.recieved_amount,
            invoice_amount: arg.invoice_amount,
            after_sale_amount: arg.after_sale_amount,
            after_sale_trans_amount: arg.after_sale_trans_amount,
            pay_diff_amount: arg.pay_diff_amount,
            money_way: arg.money_way,
            pay_date: Some(arg.pay_date.unwrap().to_string()),
            pay_person: arg.pay_person,
            unit: arg.unit,
            unit_money: arg.unit_money,
            trans_amount: arg.trans_amount,
            sale_pay_status: arg.sale_pay_status,
            rate_amount:arg.rate_amount,
            mome: arg.mome,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SalePayPageDTO {
    pub sale_pay_start: Option<String>,
    pub sale_pay_end: Option<String>,
    pub company_code: Option<String>,
    pub client_name: Option<String>,
    pub page_size: Option<u64>,
    pub page_no: Option<u64>,
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SaleAfterDTO {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub after_sale_cate: Option<String>,
    pub client_name: Option<String>,
    pub sale_id: Option<u64>,
    pub sale_name: Option<String>,
    pub after_sale_date: Option<String>,
    pub back_mode: Option<String>,
    pub back_date: Option<String>,
    pub back_money_way: Option<String>,
    pub back_amount: Option<f64>,
    pub unit_money: Option<String>,
    pub trans_in_amount: Option<f64>,
    pub trans_out_amount: Option<f64>,
    pub back_person: Option<String>,
    pub mome: Option<String>,
    pub sale_after_status: Option<i32>,
    pub created_at: Option<DateTimeNative>, // You may want to use a proper date type here
    pub updated_at: Option<DateTimeNative>, // You may want to use a proper date type here
}

// Some(DateNative::from_str(self.agree_date.clone().unwrap().as_str()).unwrap()),
impl Into<SaleAfter> for SaleAfterDTO {
    fn into(self) -> SaleAfter {
        SaleAfter {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            client_name: self.client_name.clone(),
            sale_id: self.sale_id.clone(),
            sale_name: self.sale_name.clone(),
            after_sale_cate: self.after_sale_cate.clone(),
            after_sale_date: Some(DateNative::from_str(self.after_sale_date.clone().unwrap().as_str()).unwrap()),
            back_mode: self.back_mode.clone(),
            back_date: Some(DateNative::from_str(self.back_date.clone().unwrap().as_str()).unwrap()),
            back_money_way: self.back_money_way.clone(),
            back_amount: self.back_amount.clone(),
            unit_money: self.unit_money.clone(),
            trans_in_amount: self.trans_in_amount.clone(),
            trans_out_amount: self.trans_out_amount.clone(),
            back_person: self.back_person.clone(),
            sale_after_status: self.sale_after_status.clone(),
            mome: self.mome.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),

        }
    }
}

impl From<SaleAfter> for SaleAfterDTO {
    fn from(arg: SaleAfter) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            client_name: arg.client_name,
            sale_id: arg.sale_id,
            sale_name: arg.sale_name,
            after_sale_cate: arg.after_sale_cate,
            after_sale_date: Some(arg.after_sale_date.unwrap().to_string()),
            back_mode: arg.back_mode,
            back_date: Some(arg.back_date.unwrap().to_string()),
            back_money_way: arg.back_money_way,
            back_amount: arg.back_amount,
            unit_money: arg.unit_money,
            trans_in_amount: arg.trans_in_amount,
            trans_out_amount: arg.trans_out_amount,
            back_person: arg.back_person,
            sale_after_status: arg.sale_after_status,
            mome: arg.mome,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SaleAfterPageDTO {
    pub sale_after_start: Option<String>,
    pub sale_after_end: Option<String>,
    pub company_code: Option<String>,
    pub client_name: Option<String>,
    pub page_size: Option<u64>,
    pub page_no: Option<u64>,
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SaleInfoRecievedPageDTO {
    pub sale_start: Option<String>,
    pub sale_end: Option<String>,
    pub company_code: Option<String>,
}