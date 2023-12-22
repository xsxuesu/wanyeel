use crate::entity::order::*;
use crate::entity::storage::*;
use rbatis::DateNative;
use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OrderDTO {
    pub id: Option<u64>,
    pub pay_id: Option<u64>,
    pub uid: Option<String>,
    pub company_code: Option<String>,
    pub buy_mode: Option<String>,
    pub buy_code: Option<i32>,
    pub supply_dept: Option<String>,
    pub buy_date: Option<String>,
    pub delivery_dept: Option<String>,
    pub buy_person: Option<String>,
    pub storage_mode: Option<String>,
    pub pay_mode: Option<i32>,
    pub prepay_scale: Option<f64>,
    pub pay_date: Option<String>,
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

impl Into<Order> for OrderDTO {
    fn into(self) -> Order {
        Order {
            id: self.id.clone(),
            pay_id: self.pay_id.clone(),
            uid: self.uid.clone(),
            company_code: self.company_code.clone(),
            buy_mode: self.buy_mode.clone(),
            buy_code: self.buy_code.clone(),
            supply_dept: self.supply_dept.clone(),
            buy_date: Some(DateNative::from_str(self.buy_date.clone().unwrap().as_str()).unwrap()),
            delivery_dept: self.delivery_dept.clone(),
            buy_person: self.buy_person.clone(),
            storage_mode: self.storage_mode.clone(),
            pay_mode: self.pay_mode.clone(),
            prepay_scale: self.prepay_scale.clone(),
            pay_date: Some(DateNative::from_str(self.pay_date.clone().unwrap().as_str()).unwrap()),
            pay_amount: self.pay_amount.clone(),
            all_pay_amount: self.all_pay_amount.clone(),
            trans_in_amount: self.trans_in_amount.clone(),
            trans_out_amount: self.trans_out_amount.clone(),
            payed_amount: self.payed_amount.clone(),
            after_sale_amount: self.after_sale_amount.clone(),
            unit_money: self.unit_money.clone(),
            order_status: self.order_status.clone(),
            mome: self.mome.clone(),
            from_type: self.from_type.clone(),
            from_id: self.from_id.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<Order> for OrderDTO {
    fn from(arg: Order) -> Self {
        Self {
            id: arg.id,
            pay_id: arg.pay_id,
            uid: arg.uid,
            company_code: arg.company_code,
            buy_mode: arg.buy_mode,
            buy_code: arg.buy_code,
            supply_dept: arg.supply_dept,
            buy_date: Some(arg.buy_date.unwrap().to_string()),
            delivery_dept: arg.delivery_dept,
            buy_person: arg.buy_person,
            storage_mode: arg.storage_mode,
            pay_mode: arg.pay_mode,
            prepay_scale: arg.prepay_scale,
            pay_date: Some(arg.pay_date.unwrap().to_string()),
            pay_amount: arg.pay_amount,
            all_pay_amount: arg.all_pay_amount,
            trans_in_amount: arg.trans_in_amount,
            trans_out_amount: arg.trans_out_amount,
            payed_amount: arg.payed_amount,
            after_sale_amount: arg.after_sale_amount,
            unit_money: arg.unit_money,
            order_status: arg.order_status,
            mome: arg.mome,
            from_type: arg.from_type,
            from_id: arg.from_id,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OrderProductDTO {
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
    pub unit: Option<String>,
    pub unit_money: Option<String>,
    pub way_weight: Option<String>,
    pub tax_rate: Option<f64>,
    pub resource_number: Option<String>,
    pub contract_number: Option<String>,
    pub vechel_number: Option<String>,
    pub pack_number: Option<String>,
    pub storage_status: Option<i32>,
    pub mome: Option<String>,
    pub created_person: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<StorageProduct> for OrderProductDTO {
    fn into(self) -> StorageProduct {
        StorageProduct {
            id: Some(0),
            company_code: self.company_code.clone(),
            warehouse: self.warehouse.clone(),
            variety: self.variety.clone(),
            origin: self.origin.clone(),
            shop_sign: self.shop_sign.clone(),
            spec: self.spec.clone(),
            storage_number: self.buy_number.clone(),
            storage_weight: self.buy_weight.clone(),
            lock_number: None,
            lock_weight: None,
            cacl_mode:self.cacl_mode.clone(),
            way_weight: self.way_weight.clone(),
            one_weight: self.one_weight.clone(),
            unit_price: self.unit_price.clone(),
            tax_rate: self.tax_rate.clone(),
            unit: self.unit.clone(),
            unit_money: self.unit_money.clone(),
            resource_number: self.resource_number.clone(),
            contract_number: self.contract_number.clone(),
            vechel_number: self.vechel_number.clone(),
            package_number: self.pack_number.clone(),
            order_id: self.order_id.clone(),
            agree_id: self.agree_id.clone(),
            adjust_id: self.adjust_id.clone(),
            product_id: self.id.clone(),
            process_id: None,
            in_storage_mode: Some(1),  //入库方式 1 订单 2直接入库
            storage_cate: Some(1),  //库存类型 1 成品 2 废品 3 余料
            storage_status:Some(1),  //库存状态 1 入库在库 2 加工 3 已出库
            instorage_date: Some(DateNative::now()),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
        }
    }

impl Into<OrderProduct> for OrderProductDTO {
    fn into(self) -> OrderProduct {
        OrderProduct {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            uid: self.uid.clone(),
            order_id: self.order_id.clone(),
            agree_id: self.agree_id.clone(),
            adjust_id: self.adjust_id.clone(),
            variety: self.variety.clone(),
            origin: self.origin.clone(),
            warehouse:self.warehouse.clone(),
            shop_sign: self.shop_sign.clone(),
            spec: self.spec.clone(),
            cacl_mode: self.cacl_mode.clone(),
            buy_number: self.buy_number.clone(),
            one_weight: self.one_weight.clone(),
            buy_weight: self.buy_weight.clone(),
            unit_price: self.unit_price.clone(),
            buy_amount: self.buy_amount.clone(),
            way_weight: self.way_weight.clone(),
            tax_rate: self.tax_rate.clone(),
            unit: self.unit.clone(),
            unit_money: self.unit_money.clone(),
            resource_number: self.resource_number.clone(),
            contract_number: self.contract_number.clone(),
            vechel_number: self.vechel_number.clone(),
            pack_number: self.pack_number.clone(),
            mome: self.mome.clone(),
            storage_status: self.storage_status.clone(),
            created_person: self.created_person.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<OrderProduct> for OrderProductDTO {
    fn from(arg: OrderProduct) -> Self {
        Self {
            id: arg.id,
            uid: arg.uid,
            company_code: arg.company_code,
            order_id: arg.order_id,
            agree_id: arg.agree_id,
            adjust_id: arg.adjust_id,
            variety: arg.variety,
            origin: arg.origin,
            warehouse: arg.warehouse,
            shop_sign: arg.shop_sign,
            spec: arg.spec,
            cacl_mode: arg.cacl_mode,
            buy_number: arg.buy_number,
            one_weight: arg.one_weight,
            buy_weight: arg.buy_weight,
            unit_price: arg.unit_price,
            buy_amount: arg.buy_amount,
            tax_rate: arg.tax_rate,
            unit: arg.unit,
            unit_money: arg.unit_money,
            way_weight: arg.way_weight,
            resource_number: arg.resource_number,
            contract_number: arg.contract_number,
            vechel_number: arg.vechel_number,
            pack_number: arg.pack_number,
            mome: arg.mome,
            storage_status: arg.storage_status,
            created_person: arg.created_person,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OrderTransDTO {
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
    pub cost_date: Option<String>,
    pub cost_category: Option<String>,
    pub unit_category: Option<String>,
    pub unit_number: Option<f64>,
    pub unit_price: Option<f64>,
    pub cost_rate: Option<f64>,
    pub cost_amount: Option<f64>,
    pub is_invoice: Option<i32>,
    pub invoice_amount: Option<f64>,
    pub unit_money: Option<String>,
    pub create_person: Option<String>,
    pub mome: Option<String>,
    pub trans_status : Option<i32>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<OrderTrans> for OrderTransDTO {
    fn into(self) -> OrderTrans {
        OrderTrans {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            uid: self.uid.clone(),
            order_id: self.order_id.clone(),
            agree_id: self.agree_id.clone(),
            adjust_id: self.adjust_id.clone(),
            process_id: self.process_id.clone(),
            sale_id: self.sale_id.clone(),
            after_sale_id: self.after_sale_id.clone(),
            buy_after_sale_id: self.buy_after_sale_id.clone(),
            supply_dept: self.supply_dept.clone(),
            in_out: self.in_out.clone(),
            cost_way: self.cost_way.clone(),
            cost_code: self.cost_code.clone(),
            cost_date: Some(DateNative::from_str(self.cost_date.clone().unwrap().as_str()).unwrap()),
            cost_category: self.cost_category.clone(),
            unit_category: self.unit_category.clone(),
            unit_number: self.unit_number.clone(),
            unit_price: self.unit_price.clone(),
            cost_rate: self.cost_rate.clone(),
            cost_amount: self.cost_amount.clone(),
            unit_money: self.unit_money.clone(),
            is_invoice: self.is_invoice.clone(),
            invoice_amount: self.invoice_amount.clone(),
            create_person: self.create_person.clone(),
            mome: self.mome.clone(),
            trans_status:self.trans_status.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<OrderTrans> for OrderTransDTO {
    fn from(arg: OrderTrans) -> Self {
        Self {
            id: arg.id,
            uid: arg.uid,
            company_code: arg.company_code,
            order_id: arg.order_id,
            agree_id: arg.agree_id,
            adjust_id: arg.adjust_id,
            process_id: arg.process_id,
            sale_id: arg.sale_id,
            after_sale_id: arg.after_sale_id,
            buy_after_sale_id: arg.buy_after_sale_id,
            supply_dept: arg.supply_dept,
            in_out: arg.in_out,
            cost_way: arg.cost_way,
            cost_code: arg.cost_code,
            cost_date: Some(arg.cost_date.unwrap().to_string()),
            cost_category: arg.cost_category,
            unit_category: arg.unit_category,
            unit_number: arg.unit_number,
            unit_price: arg.unit_price,
            cost_rate: arg.cost_rate,
            cost_amount: arg.cost_amount,
            unit_money: arg.unit_money,
            is_invoice: arg.is_invoice,
            invoice_amount: arg.invoice_amount,
            create_person: arg.create_person,
            mome: arg.mome,
            trans_status: arg.trans_status,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OrderFilterParams {
    pub company_code: Option<String>,
    pub order_start: Option<String>,
    pub order_end: Option<String>,
    pub supply_dept: Option<String>,
    pub delivery_dept: Option<String>,
    pub buy_mode: Option<String>,
    pub order_status: Option<i32>,
    pub page_size: Option<u64>,
    pub page_no: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OrderParams {
    pub company_code: Option<String>,
    pub order_start: Option<DateNative>,
    pub order_end: Option<DateNative>,
    pub supply_dept: Option<String>,
    pub buy_mode: Option<String>,
    pub delivery_dept: Option<String>,
    pub order_status: Option<i32>,
}



#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OrderAfterSaleDTO {
   pub id: Option<u64>,
    pub   storage_id: Option<u64>,
    pub  company_code: Option<String>,
    pub  after_sale_name: Option<String>,
    pub  order_id: Option<u64>,
    pub  agree_id: Option<u64>,
    pub  supply_dept: Option<String>,
    pub  back_mode: Option<String>,
    pub  back_money_way: Option<String>,
    pub  back_money: Option<f64>,
    pub  unit_money: Option<String>,
    pub  back_date: Option<String>,
    pub   back_person: Option<String>,
    pub  back_status: Option<i32>,
    pub  trans_in_amount: Option<f64>,
    pub  trans_out_amount: Option<f64>,
    pub  mome: Option<String>,
    pub  created_at: Option<DateTimeNative>,
    pub  updated_at: Option<DateTimeNative>,
}


impl Into<OrderAfterSale> for OrderAfterSaleDTO {
    fn into(self) -> OrderAfterSale {
        OrderAfterSale {
            id: self.id.clone(),
            storage_id: self.storage_id.clone(),
            company_code: self.company_code.clone(),
            after_sale_name: self.after_sale_name.clone(),
            order_id: self.order_id.clone(),
            agree_id: self.agree_id.clone(),
            supply_dept: self.supply_dept.clone(),
            back_mode: self.back_mode.clone(),
            back_money_way: self.back_money_way.clone(),
            back_money: self.back_money.clone(),
            unit_money: self.unit_money.clone(),
            back_date: Some(DateNative::from_str(self.back_date.clone().unwrap().as_str()).unwrap()),
            back_person: self.back_person.clone(),
            back_status: self.back_status.clone(),
            trans_in_amount: self.trans_in_amount.clone(),
            trans_out_amount: self.trans_out_amount.clone(),
            mome: self.mome.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<OrderAfterSale> for OrderAfterSaleDTO {
    fn from(arg: OrderAfterSale) -> Self {
        Self {
            id: arg.id,
            storage_id: arg.storage_id,
            company_code: arg.company_code,
            after_sale_name:arg.after_sale_name,
            order_id: arg.order_id,
            agree_id: arg.agree_id,
            supply_dept: arg.supply_dept,
            back_mode: arg.back_mode,
            back_money_way: arg.back_money_way,
            back_money: arg.back_money,
            unit_money: arg.unit_money,
            back_date: Some(arg.back_date.unwrap().to_string()),
            back_person: arg.back_person,
            back_status: arg.back_status,
            trans_in_amount: arg.trans_in_amount,
            trans_out_amount: arg.trans_out_amount,
            mome: arg.mome,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OrderAfterSaleProductDTO {
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


impl Into<OrderAfterSaleProduct> for OrderAfterSaleProductDTO {
    fn into(self) -> OrderAfterSaleProduct {
        OrderAfterSaleProduct {
            id: self.id.clone(),
            after_sale_id: self.after_sale_id.clone(),
            order_prodct_id: self.order_prodct_id.clone(),
            company_code: self.company_code.clone(),
            uid: self.uid.clone(),
            order_id: self.order_id.clone(),
            agree_id: self.agree_id.clone(),
            variety: self.variety.clone(),
            origin: self.origin.clone(),
            warehouse: self.warehouse.clone(),
            shop_sign: self.shop_sign.clone(),
            spec: self.spec.clone(),
            cacl_mode: self.cacl_mode.clone(),
            back_number:self.back_number.clone(),
            back_weight: self.back_weight.clone(),
            buy_number: self.buy_number.clone(),
            one_weight: self.one_weight.clone(),
            buy_weight: self.buy_weight.clone(),
            way_weight: self.way_weight.clone(),
            unit_price: self.unit_price.clone(),
            buy_amount: self.buy_amount.clone(),
            tax_rate: self.tax_rate.clone(),
            unit: self.unit.clone(),
            unit_money: self.unit_money.clone(),
            resource_number: self.resource_number.clone(),
            contract_number: self.contract_number.clone(),
            vechel_number: self.vechel_number.clone(),
            pack_number: self.pack_number.clone(),
            created_person: self.created_person.clone(),
            after_sale_status: self.after_sale_status.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<OrderAfterSaleProduct> for OrderAfterSaleProductDTO {
    fn from(arg: OrderAfterSaleProduct) -> Self {
        Self {
            id: arg.id,
            after_sale_id: arg.after_sale_id,
            order_prodct_id: arg.order_prodct_id,
            company_code: arg.company_code,
            uid: arg.uid,
            order_id: arg.order_id,
            agree_id: arg.agree_id,
            variety: arg.variety,
            origin: arg.origin,
            warehouse: arg.warehouse,
            shop_sign: arg.shop_sign,
            spec: arg.spec,
            cacl_mode: arg.cacl_mode,
            back_number: arg.back_number,
            back_weight: arg.back_weight,
            buy_number: arg.buy_number,
            one_weight: arg.one_weight,
            buy_weight: arg.buy_weight,
            way_weight: arg.way_weight,
            unit_price: arg.unit_price,
            buy_amount: arg.buy_amount,
            tax_rate: arg.tax_rate,
            unit: arg.unit,
            unit_money: arg.unit_money,
            resource_number: arg.resource_number,
            contract_number: arg.contract_number,
            vechel_number: arg.vechel_number,
            pack_number: arg.pack_number,
            created_person: arg.created_person,
            after_sale_status: arg.after_sale_status,
            created_at: arg.created_at,
            updated_at: arg.updated_at,

        }
    }
}



#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OrderAfterSaleFilterParams {
    pub company_code: Option<String>,
    pub after_sale_start: Option<DateNative>,
    pub after_sale_end: Option<DateNative>,
    pub supply_dept: Option<String>,
    pub page_size: Option<u64>,
    pub page_no: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OrderAfterSaleParams {
    pub company_code: Option<String>,
    pub after_sale_start: Option<DateNative>,
    pub after_sale_end: Option<DateNative>,
    pub supply_dept: Option<String>,
}





#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OrderProductForSelectFilterParams {
    pub company_code: Option<String>,
    pub order_id: Option<i32>,
    pub agree_id: Option<i32>,
    pub page_size: Option<u64>,
    pub page_no: Option<u64>,
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct TransCodeFilterParams {
    pub payed_date_start: Option<String>,
    pub payed_date_end: Option<String>,
    pub company_code: Option<String>,
    pub client_name: Option<String>,
    pub page_size: Option<u64>,
    pub page_no: Option<u64>,
}
