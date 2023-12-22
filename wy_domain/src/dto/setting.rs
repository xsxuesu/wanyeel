
use crate::entity::setting::*;
use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct VarietyComDTO {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub variety_name: Option<String>,
    pub variety_code: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<VarietyCom> for VarietyComDTO {
    fn into(self) -> VarietyCom {
        VarietyCom {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            variety_name: self.variety_name.clone(),
            variety_code: self.variety_code.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<VarietyCom> for VarietyComDTO {
    fn from(arg: VarietyCom) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            variety_name: arg.variety_name,
            variety_code: arg.variety_code,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct VarietyDTO {
    pub id: Option<u64>,
    pub variety_name: Option<String>,
    pub variety_code: Option<String>,
    pub variety_cate: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<Variety> for VarietyDTO {
    fn into(self) -> Variety {
        Variety {
            id: self.id.clone(),
            variety_name: self.variety_name.clone(),
            variety_code: self.variety_code.clone(),
            variety_cate: self.variety_cate.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<Variety> for VarietyDTO {
    fn from(arg: Variety) -> Self {
        Self {
            id: arg.id,
            variety_cate: arg.variety_cate,
            variety_name: arg.variety_name,
            variety_code: arg.variety_code,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct VarietyTreeDTO {
    pub variety_name: Option<String>,
    pub variety_code: Option<String>,
    pub children: Option<Vec<VarietyDTO>>,
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OriginDTO {
    pub id: Option<u64>,
    pub origin_code: Option<String>,
    pub origin_name: Option<String>,
    pub origin_fullname: Option<String>,
    pub origin_address: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<Origin> for OriginDTO {
    fn into(self) -> Origin {
        Origin {
            id: self.id.clone(),
            origin_code: self.origin_code.clone(),
            origin_name: self.origin_name.clone(),
            origin_fullname: self.origin_fullname.clone(),
            origin_address: self.origin_address.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<Origin> for OriginDTO {
    fn from(arg: Origin) -> Self {
        Self {
            id: arg.id,
            origin_code: arg.origin_code,
            origin_name: arg.origin_name,
            origin_fullname: arg.origin_fullname,
            origin_address: arg.origin_address,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct OriginComDTO {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub origin_code: Option<String>,
    pub origin_name: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<OriginCom> for OriginComDTO {
    fn into(self) -> OriginCom {
        OriginCom {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            origin_code: self.origin_code.clone(),
            origin_name: self.origin_name.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<OriginCom> for OriginComDTO {
    fn from(arg: OriginCom) -> Self {
        Self {
            id: arg.id,
            origin_code: arg.origin_code,
            origin_name: arg.origin_name,
            company_code: arg.company_code,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct PQualityDTO {
    pub id: Option<u64>,
    pub lvl_name: Option<String>,
    pub lvl_code: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<PQuality> for PQualityDTO {
    fn into(self) -> PQuality {
        PQuality {
            id: self.id.clone(),
            lvl_name: self.lvl_name.clone(),
            lvl_code: self.lvl_code.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<PQuality> for PQualityDTO {
    fn from(arg: PQuality) -> Self {
        Self {
            id: arg.id,
            lvl_name: arg.lvl_name,
            lvl_code: arg.lvl_code,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct PTypeDTO {
    pub id: Option<u64>,
    pub p_type: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<PType> for PTypeDTO {
    fn into(self) -> PType {
        PType {
            id: self.id.clone(),
            p_type: self.p_type.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<PType> for PTypeDTO {
    fn from(arg: PType) -> Self {
        Self {
            id: arg.id,
            p_type: arg.p_type,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct PayModeDTO {
    pub id: Option<u64>,
    pub pay_code: Option<String>,
    pub pay_name: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<PayMode> for PayModeDTO {
    fn into(self) -> PayMode {
        PayMode {
            id: self.id.clone(),
            pay_code: self.pay_code.clone(),
            pay_name: self.pay_name.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<PayMode> for PayModeDTO {
    fn from(arg: PayMode) -> Self {
        Self {
            id: arg.id,
            pay_code: arg.pay_code,
            pay_name: arg.pay_name,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}



#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct PayModeComDTO {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub pay_code: Option<String>,
    pub pay_name: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<PayModeCom> for PayModeComDTO {
    fn into(self) -> PayModeCom {
        PayModeCom {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            pay_code: self.pay_code.clone(),
            pay_name: self.pay_name.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<PayModeCom> for PayModeComDTO {
    fn from(arg: PayModeCom) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            pay_code: arg.pay_code,
            pay_name: arg.pay_name,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct RoleDTO {
    pub id: Option<u64>,
    pub role_code: Option<String>,
    pub role_name: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<Role> for RoleDTO {
    fn into(self) -> Role {
        Role {
            id: self.id.clone(),
            role_code: self.role_code.clone(),
            role_name: self.role_name.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<Role> for RoleDTO {
    fn from(arg: Role) -> Self {
        Self {
            id: arg.id,
            role_code: arg.role_code,
            role_name: arg.role_name,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct StorageDTO {
    pub id: Option<u64>,
    pub storage_name: Option<String>,
    pub storage_address: Option<String>,
    pub storage_person: Option<String>,
    pub storage_phone: Option<String>,
    pub storage_code: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<Storage> for StorageDTO {
    fn into(self) -> Storage {
        Storage {
            id: self.id.clone(),
            storage_name: self.storage_name.clone(),
            storage_address: self.storage_address.clone(),
            storage_person: self.storage_person.clone(),
            storage_phone: self.storage_phone.clone(),
            storage_code: self.storage_code.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<Storage> for StorageDTO {
    fn from(arg: Storage) -> Self {
        Self {
            id: arg.id,
            storage_name: arg.storage_name,
            storage_address: arg.storage_address,
            storage_person: arg.storage_person,
            storage_phone: arg.storage_phone,
            storage_code: arg.storage_code,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct StorageComDTO {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub storage_name: Option<String>,
    pub storage_address: Option<String>,
    pub storage_person: Option<String>,
    pub storage_phone: Option<String>,
    pub storage_code: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<StorageCom> for StorageComDTO {
    fn into(self) -> StorageCom {
        StorageCom {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            storage_name: self.storage_name.clone(),
            storage_address: self.storage_address.clone(),
            storage_person: self.storage_person.clone(),
            storage_phone: self.storage_phone.clone(),
            storage_code: self.storage_code.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<StorageCom> for StorageComDTO {
    fn from(arg: StorageCom) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            storage_name: arg.storage_name,
            storage_address: arg.storage_address,
            storage_person: arg.storage_person,
            storage_phone: arg.storage_phone,
            storage_code: arg.storage_code,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SupplyComDTO {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub supply_name: Option<String>,
    pub supply_cate: Option<String>,
    pub supply_address: Option<String>,
    pub supply_person: Option<String>,
    pub supply_phone: Option<String>,
    pub supply_code: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<SupplyCom> for SupplyComDTO {
    fn into(self) -> SupplyCom {
        SupplyCom {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            supply_name: self.supply_name.clone(),
            supply_cate: self.supply_cate.clone(),
            supply_address: self.supply_address.clone(),
            supply_person: self.supply_person.clone(),
            supply_phone: self.supply_phone.clone(),
            supply_code: self.supply_code.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<SupplyCom> for SupplyComDTO {
    fn from(arg: SupplyCom) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            supply_name: arg.supply_name,
            supply_cate: arg.supply_cate,
            supply_address: arg.supply_address,
            supply_person: arg.supply_person,
            supply_phone: arg.supply_phone,
            supply_code: arg.supply_code,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}




#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct NoticeDTO {
    pub id: Option<u64>,
    pub notice: Option<String>,
    pub end_date: Option<DateTimeNative>,
    pub is_effect: Option<i32>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<Notice> for NoticeDTO {
    fn into(self) -> Notice {
        Notice {
            id: self.id.clone(),
            notice: self.notice.clone(),
            end_date: self.end_date.clone(),
            is_effect: self.is_effect.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<Notice> for NoticeDTO {
    fn from(arg: Notice) -> Self {
        Self {
            id: arg.id,
            notice: arg.notice,
            end_date: arg.end_date,
            is_effect: arg.is_effect,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct FackbackDTO {
    pub id: Option<u64>,
    pub category: Option<String>,
    pub contact: Option<String>,
    pub suggest: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<Fackback> for FackbackDTO {
    fn into(self) -> Fackback {
        Fackback {
            id: self.id.clone(),
            category: self.category.clone(),
            contact: self.contact.clone(),
            suggest: self.suggest.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<Fackback> for FackbackDTO {
    fn from(arg: Fackback) -> Self {
        Self {
            id: arg.id,
            category: arg.category,
            contact: arg.contact,
            suggest: arg.suggest,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ClientComDTO {
    pub id: Option<u64>,
    pub company_code: Option<String>,
    pub client_name: Option<String>,
    pub client_address: Option<String>,
    pub client_person: Option<String>,
    pub client_phone: Option<String>,
    pub created_at: Option<DateTimeNative>,
    pub updated_at: Option<DateTimeNative>,
}

impl Into<ClientCom> for ClientComDTO {
    fn into(self) -> ClientCom {
        ClientCom {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            client_name: self.client_name.clone(),
            client_address: self.client_address.clone(),
            client_person: self.client_person.clone(),
            client_phone: self.client_phone.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<ClientCom> for ClientComDTO {
    fn from(arg: ClientCom) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            client_name: arg.client_name,
            client_address: arg.client_address,
            client_person: arg.client_person,
            client_phone: arg.client_phone,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}


// 规格
#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SepcComDTO {
   pub id: Option<u64>,
   pub company_code: Option<String>,
   pub spec: Option<String>,
   pub created_at: Option<DateTimeNative>,
   pub updated_at: Option<DateTimeNative>,
}


impl Into<SepcCom> for SepcComDTO {
    fn into(self) -> SepcCom {
        SepcCom {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            spec: self.spec.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<SepcCom> for SepcComDTO {
    fn from(arg: SepcCom) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            spec: arg.spec,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}

// 牌号
#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ShopSignComDTO {
   pub id: Option<u64>,
   pub company_code: Option<String>,
   pub shop_sign: Option<String>,
   pub created_at: Option<DateTimeNative>,
   pub updated_at: Option<DateTimeNative>,
}


impl Into<ShopSignCom> for ShopSignComDTO {
    fn into(self) -> ShopSignCom {
        ShopSignCom {
            id: self.id.clone(),
            company_code: self.company_code.clone(),
            shop_sign: self.shop_sign.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

impl From<ShopSignCom> for ShopSignComDTO {
    fn from(arg: ShopSignCom) -> Self {
        Self {
            id: arg.id,
            company_code: arg.company_code,
            shop_sign: arg.shop_sign,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}