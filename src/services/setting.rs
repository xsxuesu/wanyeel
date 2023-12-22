
use std::collections::HashMap;
use wy_domain::dto::setting::{VarietyComDTO,VarietyDTO,OriginDTO,OriginComDTO,PQualityDTO,PTypeDTO,PayModeDTO,PayModeComDTO,RoleDTO,StorageDTO,StorageComDTO,SupplyComDTO,NoticeDTO,FackbackDTO,VarietyTreeDTO,ClientComDTO,ShopSignComDTO,SepcComDTO};
use wy_domain::entity::setting::{VarietyCom,Variety,Origin,OriginCom,PQuality,PType,PayMode,PayModeCom,Role,Storage,StorageCom,SupplyCom,Notice,Fackback,ClientCom,SepcCom,ShopSignCom};
use wy_domain::entity::CommonField;
use wy_domain::entity::user::Company;
use wy_domain::dto::user::CompanyDTO;
use wy_domain::request::{ByComQuery, AllQuery,StorageQuery,NoticeQuery};
use crate::crud::crud_service::CrudService;
use crate::APPLICATION_CONTEXT;
use rbatis::rbatis::Rbatis;
use wy_domain::error::Result;

// VarietyCom
pub struct VarietyComService;
impl Default for VarietyComService {
    fn default() -> Self {
        VarietyComService {}
    }
}
impl CrudService<VarietyCom, VarietyComDTO, ByComQuery> for VarietyComService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper().all_eq(arg)
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut VarietyCom) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}


// Variety
pub struct VarietyService;
impl Default for VarietyService {
    fn default() -> Self {
        VarietyService {}
    }
}

impl VarietyService {
   pub async fn get_all_by_cate(&self)->Result<Vec<VarietyTreeDTO>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        //执行查询 exec("select distinct variety_cate from variety")
        let table:Vec<HashMap<String,String>> = rb
        .fetch("select distinct variety_cate from variety",vec![])
        .await
        .unwrap();
        let mut resull_result_solver:Vec<VarietyTreeDTO> = vec![];
        for e in table {
            let value = e.get("variety_cate").unwrap();

           let sub_table = self.fetch_list_by_column("variety_cate",&vec![value.to_owned()]).await.unwrap();

            let tmp_variety_tree = VarietyTreeDTO {
                variety_name: Some(value.to_string()),
                variety_code: Some(value.to_string()),
                children: Some(sub_table),
            };
            resull_result_solver.push(tmp_variety_tree)
        }

        Ok(resull_result_solver)
    }
}

impl CrudService<Variety, VarietyDTO, AllQuery> for VarietyService {
    fn get_wrapper(arg: &AllQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut Variety) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// Origin
pub struct OriginService;
impl Default for OriginService {
    fn default() -> Self {
        OriginService {}
    }
}
impl CrudService<Origin, OriginDTO, AllQuery> for OriginService {
    fn get_wrapper(arg: &AllQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut Origin) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}


// OriginCom
pub struct OriginComService;
impl Default for OriginComService {
    fn default() -> Self {
        OriginComService {}
    }
}
impl CrudService<OriginCom, OriginComDTO, ByComQuery> for OriginComService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut OriginCom) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}


// PQuality
pub struct PQualityService;
impl Default for PQualityService {
    fn default() -> Self {
        PQualityService {}
    }
}
impl CrudService<PQuality, PQualityDTO, AllQuery> for PQualityService {
    fn get_wrapper(arg: &AllQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut PQuality) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// PType

pub struct PTypeService;
impl Default for PTypeService {
    fn default() -> Self {
        PTypeService {}
    }
}
impl CrudService<PType, PTypeDTO, AllQuery> for PTypeService {
    fn get_wrapper(arg: &AllQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut PType) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// PayMode
pub struct PayModeService;
impl Default for PayModeService {
    fn default() -> Self {
        PayModeService {}
    }
}
impl CrudService<PayMode, PayModeDTO, AllQuery> for PayModeService {
    fn get_wrapper(arg: &AllQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut PayMode) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// PayModeCom
pub struct PayModeComService;
impl Default for PayModeComService {
    fn default() -> Self {
        PayModeComService {}
    }
}
impl CrudService<PayModeCom, PayModeComDTO, ByComQuery> for PayModeComService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut PayModeCom) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}



// ROLE
pub struct RoleService;
impl Default for RoleService {
    fn default() -> Self {
        RoleService {}
    }
}
impl CrudService<Role, RoleDTO, AllQuery> for RoleService {
    fn get_wrapper(arg: &AllQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut Role) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// StorageDTO
pub struct StorageService;
impl Default for StorageService {
    fn default() -> Self {
        StorageService {}
    }
}
impl CrudService<Storage, StorageDTO, StorageQuery> for StorageService {
    fn get_wrapper(arg: &StorageQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut Storage) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}


// StorageComDTO

pub struct StorageComService;
impl Default for StorageComService {
    fn default() -> Self {
        StorageComService {}
    }
}
impl CrudService<StorageCom, StorageComDTO, ByComQuery> for StorageComService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper().all_eq(arg)
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut StorageCom) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// SupplyComDTO
pub struct SupplyComService;
impl Default for SupplyComService {
    fn default() -> Self {
        SupplyComService {}
    }
}
impl CrudService<SupplyCom, SupplyComDTO, ByComQuery> for SupplyComService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SupplyCom) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// ClientComDTO
pub struct ClientComService;
impl Default for ClientComService {
    fn default() -> Self {
        ClientComService {}
    }
}
impl CrudService<ClientCom, ClientComDTO, ByComQuery> for ClientComService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut ClientCom) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}


// NoticeDTO
pub struct NoticeService;
impl Default for NoticeService {
    fn default() -> Self {
        NoticeService {}
    }
}

impl CrudService<Notice, NoticeDTO, NoticeQuery> for NoticeService {
    fn get_wrapper(arg: &NoticeQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut Notice) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// FackdbackDTO
pub struct FackbackService;
impl Default for FackbackService {
    fn default() -> Self {
        FackbackService {}
    }
}
impl CrudService<Fackback, FackbackDTO, ByComQuery> for FackbackService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut Fackback) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}


// CompanyDTO
pub struct CompanyService;
impl Default for CompanyService {
    fn default() -> Self {
        CompanyService {}
    }
}
impl CrudService<Company, CompanyDTO, ByComQuery> for CompanyService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut Company) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}



// SecService
pub struct SepcService;
impl Default for SepcService {
    fn default() -> Self {
        SepcService {}
    }
}
impl CrudService<SepcCom, SepcComDTO, ByComQuery> for SepcService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SepcCom) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}



// 牌号
pub struct ShopSignService;
impl Default for ShopSignService {
    fn default() -> Self {
        ShopSignService {}
    }
}
impl CrudService<ShopSignCom, ShopSignComDTO, ByComQuery> for ShopSignService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut ShopSignCom) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}
