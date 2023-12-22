use crate::entity::storage::{Process, ProcessProduct, ProcessSolution, StorageAdjust, StorageAdjustProduct, StorageManage, StorageProduct};
use rbatis::DateNative;
use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct StorageAdjustProductDTO {
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
    pub adjust_date: Option<String>,
    pub adjust_person: Option<String>,
    pub change_status: Option<i32>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}
impl Into<StorageAdjustProduct> for StorageAdjustProductDTO {
    fn into(self) -> StorageAdjustProduct {
        StorageAdjustProduct {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            adjust_id: self.adjust_id.clone(),
            storage_id: self.storage_id.clone(),
            variety: self.variety.clone(),
            origin: self.origin.clone(),
            warehouse: self.warehouse.clone(),
            shop_sign: self.shop_sign.clone(),
            spec: self.spec.clone(),
            adjust_number: self.adjust_number.clone(),
            adjust_weight: self.adjust_weight.clone(),
            unit: self.unit.clone(),
            storage_number: self.storage_number.clone(),
            storage_weight: self.storage_weight.clone(),
            resource_number: self.resource_number.clone(),
            contract_number: self.contract_number.clone(),
            vechel_number: self.vechel_number.clone(),
            pack_number: self.pack_number.clone(),
            adjust_date: Some(DateNative::from_str(self.adjust_date.clone().unwrap().as_str()).unwrap()),
            adjust_person: self.adjust_person.clone(),
            change_status: self.change_status.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<StorageAdjustProduct> for StorageAdjustProductDTO {
    fn from(arg: StorageAdjustProduct) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            adjust_id: arg.adjust_id,
            storage_id: arg.storage_id,
            variety: arg.variety,
            origin: arg.origin,
            warehouse: arg.warehouse,
            shop_sign: arg.shop_sign,
            spec: arg.spec,
            adjust_number: arg.adjust_number,
            adjust_weight: arg.adjust_weight,
            unit: arg.unit,
            storage_number: arg.storage_number,
            storage_weight: arg.storage_weight,
            resource_number: arg.resource_number,
            contract_number: arg.contract_number,
            vechel_number: arg.vechel_number,
            pack_number: arg.pack_number,
            adjust_date: Some(arg.adjust_date.unwrap().to_string()),
            adjust_person: arg.adjust_person,
            change_status: arg.change_status,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct StorageAdjustDTO {
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
    pub adjust_date: Option<String>,
    pub storage_cate: Option<i32>,
    pub adjust_person: Option<String>,
    pub storage_id: Option<u64>,
    pub trans_in_amount: Option<f64>,
    pub trans_out_amount: Option<f64>,
    pub adjust_status: Option<i32>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<StorageAdjust> for StorageAdjustDTO {
    fn into(self) -> StorageAdjust {
        StorageAdjust {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            in_warehouse: self.in_warehouse.clone(),
            out_warehouse: self.out_warehouse.clone(),
            adjust_cate: self.adjust_cate.clone(),
            adjust_why: self.adjust_why.clone(),
            adjust_number: self.adjust_number.clone(),
            adjust_weight: self.adjust_weight.clone(),
            variety: self.variety.clone(),
            origin: self.origin.clone(),
            spec: self.spec.clone(),
            shop_sign: self.shop_sign.clone(),
            adjust_date: Some(DateNative::from_str(self.adjust_date.clone().unwrap().as_str()).unwrap()),
            adjust_person: self.adjust_person.clone(),
            storage_cate: self.storage_cate.clone(),
            storage_id: self.storage_id.clone(),
            trans_in_amount: self.trans_in_amount.clone(),
            trans_out_amount: self.trans_out_amount.clone(),
            adjust_status: self.adjust_status.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<StorageAdjust> for StorageAdjustDTO {
    fn from(arg: StorageAdjust) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            in_warehouse: arg.in_warehouse,
            out_warehouse: arg.out_warehouse,
            adjust_cate: arg.adjust_cate,
            adjust_why: arg.adjust_why,
            adjust_number: arg.adjust_number,
            adjust_weight: arg.adjust_weight,
            variety: arg.variety,
            origin: arg.origin,
            spec: arg.spec,
            shop_sign: arg.shop_sign,
            adjust_date: Some(arg.adjust_date.unwrap().to_string()),
            adjust_person: arg.adjust_person,
            storage_cate: arg.storage_cate,
            storage_id: arg.storage_id,
            trans_in_amount: arg.trans_in_amount,
            trans_out_amount: arg.trans_out_amount,
            adjust_status: arg.adjust_status,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct StorageManageDTO {
    id: Option<u64>,
    uid: Option<String>,
    warehouse: Option<String>,
    variety: Option<String>,
    origin: Option<String>,
    shop_sign: Option<String>,
    spec: Option<String>,
    storage_number: Option<f64>,
    storage_weight: Option<f64>,
    can_sale_number: Option<f64>,
    can_sale_weight: Option<f64>,
    lock_number: Option<f64>,
    unit_price: Option<f64>,
    actual_price: Option<f64>,
    extra_price: Option<f64>,
    cacl_mode: Option<String>,
    resource_number: Option<String>,
    contract_number: Option<String>,
    vechel_number: Option<String>,
    storage_id: Option<String>,
    storage_owner: Option<String>,
    agree_uid: Option<String>,
    order_uid: Option<String>,
    instorage_date: Option<DateTimeNative>,
    check_date: Option<DateTimeNative>,
    created_at: Option<DateTimeNative>,
    updated_at: Option<DateTimeNative>,
}

impl Into<StorageManage> for StorageManageDTO {
    fn into(self) -> StorageManage {
        StorageManage {
            id: self.id.clone(),
            uid: self.uid.clone(),
            warehouse: self.warehouse.clone(),
            variety: self.variety.clone(),
            origin: self.origin.clone(),
            shop_sign: self.shop_sign.clone(),
            spec: self.spec.clone(),
            storage_number: self.storage_number.clone(),
            storage_weight: self.storage_weight.clone(),
            can_sale_number: self.can_sale_number.clone(),
            can_sale_weight: self.can_sale_weight.clone(),
            lock_number: self.lock_number.clone(),
            unit_price: self.unit_price.clone(),
            actual_price: self.actual_price.clone(),
            extra_price: self.extra_price.clone(),
            cacl_mode: self.cacl_mode.clone(),
            resource_number: self.resource_number.clone(),
            contract_number: self.contract_number.clone(),
            vechel_number: self.vechel_number.clone(),
            storage_id: self.storage_id.clone(),
            storage_owner: self.storage_owner.clone(),
            agree_uid: self.agree_uid.clone(),
            order_uid: self.order_uid.clone(),
            instorage_date: self.instorage_date.clone(),
            check_date: self.check_date.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<StorageManage> for StorageManageDTO {
    fn from(arg: StorageManage) -> Self {
        Self {
            id: arg.id,
            uid: arg.uid,
            warehouse: arg.warehouse,
            variety: arg.variety,
            origin: arg.origin,
            shop_sign: arg.shop_sign,
            spec: arg.spec,
            storage_number: arg.storage_number,
            storage_weight: arg.storage_weight,
            can_sale_number: arg.can_sale_number,
            can_sale_weight: arg.can_sale_weight,
            lock_number: arg.lock_number,
            unit_price: arg.unit_price,
            actual_price: arg.actual_price,
            extra_price: arg.extra_price,
            cacl_mode: arg.cacl_mode,
            resource_number: arg.resource_number,
            contract_number: arg.contract_number,
            vechel_number: arg.vechel_number,
            storage_id: arg.storage_id,
            storage_owner: arg.storage_owner,
            agree_uid: arg.agree_uid,
            order_uid: arg.order_uid,
            instorage_date: arg.instorage_date,
            check_date: arg.check_date,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct StorageProductDTO {
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
    pub way_weight: Option<String>,
    pub cacl_mode: Option<String>,
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
    pub in_storage_mode: Option<u8>,
    pub storage_cate: Option<u8>,
    pub storage_status: Option<u8>,
    pub instorage_date: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<StorageProduct> for StorageProductDTO {
    fn into(self) -> StorageProduct {
        StorageProduct {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            warehouse: self.warehouse.clone(),
            variety: self.variety.clone(),
            origin: self.origin.clone(),
            shop_sign: self.shop_sign.clone(),
            spec: self.spec.clone(),
            storage_number: self.storage_number.clone(),
            storage_weight: self.storage_weight.clone(),
            lock_number: self.lock_number.clone(),
            lock_weight: self.lock_weight.clone(),
            cacl_mode: self.cacl_mode.clone(),
            way_weight: self.way_weight.clone(),
            one_weight: self.one_weight.clone(),
            unit_price: self.unit_price.clone(),
            tax_rate: self.tax_rate.clone(),
            unit: self.unit.clone(),
            unit_money: self.unit_money.clone(),
            resource_number: self.resource_number.clone(),
            contract_number: self.contract_number.clone(),
            vechel_number: self.vechel_number.clone(),
            package_number: self.package_number.clone(),
            order_id: self.order_id.clone(),
            agree_id: self.agree_id.clone(),
            adjust_id: self.adjust_id.clone(),
            product_id: self.product_id.clone(),
            process_id: self.process_id.clone(),
            in_storage_mode: self.in_storage_mode.clone(),
            storage_cate: self.storage_cate.clone(),
            storage_status: self.storage_status.clone(),
            instorage_date: Some(DateNative::from_str(self.instorage_date.clone().unwrap().as_str()).unwrap()),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<StorageProduct> for StorageProductDTO {
    fn from(arg: StorageProduct) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            warehouse: arg.warehouse,
            variety: arg.variety,
            origin: arg.origin,
            shop_sign: arg.shop_sign,
            spec: arg.spec,
            storage_number: arg.storage_number,
            storage_weight: arg.storage_weight,
            lock_number: arg.lock_number,
            lock_weight: arg.lock_weight,
            cacl_mode: arg.cacl_mode,
            way_weight: arg.way_weight,
            one_weight: arg.one_weight,
            unit_price: arg.unit_price,
            tax_rate: arg.tax_rate,
            unit: arg.unit,
            unit_money: arg.unit_money,
            resource_number: arg.resource_number,
            contract_number: arg.contract_number,
            vechel_number: arg.vechel_number,
            package_number: arg.package_number,
            order_id: arg.order_id,
            agree_id: arg.agree_id,
            adjust_id: arg.adjust_id,
            product_id: arg.product_id,
            process_id: arg.process_id,
            in_storage_mode: arg.in_storage_mode,
            storage_cate: arg.storage_cate,
            instorage_date: Some(arg.instorage_date.unwrap().to_string()),
            storage_status: arg.storage_status,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

// 加工
#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ProcessDTO {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub warehouse: Option<String>,
    pub process_number: Option<f64>,
    pub process_weight: Option<f64>,
    pub process_cate: Option<i32>,
    pub process_date: Option<String>,
    pub process_status: Option<i32>,
    pub trans_in_amount: Option<f64>,
    pub trans_out_amount: Option<f64>,
    pub process_person: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}
// Some(DateNative::from_str(self.instorage_date.clone().unwrap().as_str()).unwrap()),
impl Into<Process> for ProcessDTO {
    fn into(self) -> Process {
        Process {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            warehouse: self.warehouse.clone(),
            process_number: self.process_number.clone(),
            process_weight: self.process_weight.clone(),
            process_cate: self.process_cate.clone(),
            process_date: Some(DateNative::from_str(self.process_date.clone().unwrap().as_str()).unwrap()),
            process_status: self.process_status.clone(),
            process_person: self.process_person.clone(),
            trans_in_amount: self.trans_in_amount.clone(),
            trans_out_amount: self.trans_out_amount.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<Process> for ProcessDTO {
    fn from(arg: Process) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            warehouse: arg.warehouse,
            process_number: arg.process_number,
            process_weight: arg.process_weight,
            process_cate: arg.process_cate,
            process_date: Some(arg.process_date.unwrap().to_string()),
            process_status: arg.process_status,
            process_person: arg.process_person,
            trans_in_amount: arg.trans_in_amount,
            trans_out_amount: arg.trans_out_amount,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct StorageListDTO {
    pub company_code: Option<String>,
    pub warehouse: Option<String>,
    pub variety: Option<String>,
    pub origin: Option<String>,
    pub shop_sign: Option<String>,
    pub spec: Option<String>,
    pub unit: Option<String>,
    pub storage_number: Option<f64>,
    pub storage_weight: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct StorageFilterParams {
    pub company_code: Option<String>,
    pub warehouse: Option<String>,
    pub variety: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct StorageProductPageFilterParams {
    pub company_code: Option<String>,
    pub warehouse: Option<String>,
    pub page_size: Option<u64>,
    pub page_no: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct StorageAdjustPageDTO {
    pub adjust_start: Option<String>,
    pub adjust_end: Option<String>,
    pub company_code: Option<String>,
    pub warehouse: Option<String>,
    pub page_size: Option<u64>,
    pub page_no: Option<u64>,
}

/****************************************************************
 * 加工页面查询过滤
 */

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ProcessPageFilterParams {
    pub company_code: Option<String>,
    pub warehouse: Option<String>,
    pub page_size: Option<u64>,
    pub page_no: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ProcessPageDTO {
    pub process_start: Option<String>,
    pub process_end: Option<String>,
    pub company_code: Option<String>,
    pub warehouse: Option<String>,
    pub page_size: Option<u64>,
    pub page_no: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]

pub struct ProcessProductDTO {
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
    pub process_date: Option<String>,
    pub storage_status: Option<i32>,
    pub process_cate: Option<i32>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}
// Some(DateNative::from_str(self.process_date.clone().unwrap().as_str()).unwrap()),
impl Into<ProcessProduct> for ProcessProductDTO {
    fn into(self) -> ProcessProduct {
        ProcessProduct {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            warehouse: self.warehouse.clone(),
            variety: self.variety.clone(),
            origin: self.origin.clone(),
            shop_sign: self.shop_sign.clone(),
            spec: self.spec.clone(),
            process_number: self.process_number.clone(),
            process_weight: self.process_weight.clone(),
            process_date: Some(DateNative::from_str(self.process_date.clone().unwrap().as_str()).unwrap()),
            cacl_mode: self.cacl_mode.clone(),
            way_weight: self.way_weight.clone(),
            one_weight: self.one_weight.clone(),
            unit: self.unit.clone(),
            resource_number: self.resource_number.clone(),
            contract_number: self.contract_number.clone(),
            vechel_number: self.vechel_number.clone(),
            package_number: self.package_number.clone(),
            process_id: self.process_id.clone(),
            storage_id: self.storage_id.clone(),
            storage_status: self.storage_status.clone(),
            process_cate: self.process_cate.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl Into<StorageProduct> for ProcessProductDTO {
    fn into(self) -> StorageProduct {
        StorageProduct {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            warehouse: self.warehouse.clone(),
            variety: self.variety.clone(),
            origin: self.origin.clone(),
            shop_sign: self.shop_sign.clone(),
            spec: self.spec.clone(),
            storage_number: Some(self.process_number.clone().unwrap() as f64),
            storage_weight: Some(self.process_weight.clone().unwrap() as f64),
            lock_number: None,
            lock_weight: None,
            cacl_mode: self.cacl_mode.clone(),
            way_weight: self.way_weight.clone(),
            one_weight: Some(self.one_weight.clone().unwrap() as f64),
            unit: self.unit.clone(),
            resource_number: self.resource_number.clone(),
            contract_number: self.contract_number.clone(),
            vechel_number: self.vechel_number.clone(),
            package_number: self.package_number.clone(),
            process_id: Some(self.process_id.clone().unwrap() as u64),
            storage_status: Some(1),
            unit_money: None,
            unit_price: None,
            tax_rate: None,
            order_id: None,
            agree_id: None,
            adjust_id: None,
            product_id: None,
            in_storage_mode: Some(2), // in_storage_mode 直接入库
            storage_cate: Some(4),    //加工
            instorage_date: Some(DateNative::now()),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

// Some(arg.process_date.unwrap().to_string()),
impl From<ProcessProduct> for ProcessProductDTO {
    fn from(arg: ProcessProduct) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            warehouse: arg.warehouse,
            variety: arg.variety,
            origin: arg.origin,
            shop_sign: arg.shop_sign,
            spec: arg.spec,
            process_number: arg.process_number,
            process_weight: arg.process_weight,
            process_date: Some(arg.process_date.unwrap().to_string()),
            cacl_mode: arg.cacl_mode,
            way_weight: arg.way_weight,
            one_weight: arg.one_weight,
            unit: arg.unit,
            resource_number: arg.resource_number,
            contract_number: arg.contract_number,
            vechel_number: arg.vechel_number,
            package_number: arg.package_number,
            storage_status: arg.storage_status,
            process_cate: arg.process_cate,
            process_id: arg.process_id,
            storage_id: arg.storage_id,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ProcessSolutionDTO {
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

impl Into<ProcessSolution> for ProcessSolutionDTO {
    fn into(self) -> ProcessSolution {
        ProcessSolution {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            process_id: self.process_id.clone(),
            parent_list: self.parent_list.clone(),
            sub_list: self.sub_list.clone(),
            process_solution: self.process_solution.clone(),
            process_person: self.process_person.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

// Some(arg.process_date.unwrap().to_string()),
impl From<ProcessSolution> for ProcessSolutionDTO {
    fn from(arg: ProcessSolution) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            process_id: arg.process_id,
            parent_list: arg.parent_list,
            sub_list: arg.sub_list,
            process_solution: arg.process_solution,
            process_person: arg.process_person,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}



#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct StorageLogDTO {
    pub company_code: Option<String>,
    pub search_start: Option<String>,
    pub search_end: Option<String>,
    pub warehouse: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageLog {
    pub operation: String,
    pub warehouse: Option<String>,
    pub variety: Option<String>,
    pub shop_sign: Option<String>,
    pub spec: Option<String>,
    pub op_number: Option<f64>,
    pub op_weight: Option<f64>,
    pub created_at: Option<DateTimeNative>,
}