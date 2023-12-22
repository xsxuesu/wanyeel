use std::collections::HashMap;
use crate::crud::crud_service::CrudService;
use crate::services::setting::VarietyService;
use crate::services::setting::*;
use crate::APPLICATION_CONTEXT;
use axum::{extract::Path, response::IntoResponse, Json};
use wy_common::utils::code_generate;
use wy_domain::dto::setting::{FackbackDTO, NoticeDTO, PayModeComDTO, StorageComDTO, SupplyComDTO, VarietyComDTO,ClientComDTO,OriginComDTO, SepcComDTO, ShopSignComDTO};
use wy_domain::dto::user::CompanyDTO;
use wy_domain::entity::user::Company;
use wy_domain::entity::setting::{VarietyCom,SupplyCom,StorageCom,ClientCom,OriginCom, SepcCom, ShopSignCom};
use wy_domain::request::{AllQuery, ByComQuery, NoticeQuery};
use wy_domain::vo::RespVO;
use rbatis::DateTimeNative;
use chrono::{Months,Local,DateTime};
//保存公司的品种
pub async fn save_company_varity(Json(varity): Json<VarietyComDTO>) -> impl IntoResponse {
    let setting_service = APPLICATION_CONTEXT.get::<VarietyComService>();
    let mut entity = varity.into();
    let result = setting_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
//更新公司的品种
pub async fn update_company_varity(Json(varity): Json<VarietyComDTO>) -> impl IntoResponse {
    let setting_service = APPLICATION_CONTEXT.get::<VarietyComService>();
    let mut entity:VarietyCom = varity.into();
    let result = setting_service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

//查询公司的品种
pub async fn get_company_varity(Path(uid): Path<String>) -> impl IntoResponse {
    let variety_com_service = APPLICATION_CONTEXT.get::<VarietyComService>();
    let result = variety_com_service.fetch_list_by_column("company_code",&vec![uid]).await;
    return RespVO::from_result(&result).resp_json();
}

//删除公司的品种
pub async fn del_company_varity(Path(id): Path<String>) -> impl IntoResponse {
    let setting_service = APPLICATION_CONTEXT.get::<VarietyComService>();
    let result = setting_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

//查询全部的品种
pub async fn get_varity() -> impl IntoResponse {
    let varity_service = APPLICATION_CONTEXT.get::<VarietyService>();
    let result = varity_service.get_all_by_cate().await;
    return RespVO::from_result(&result).resp_json();
}
// 查询品种byid

pub async fn get_varity_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let varity_service = APPLICATION_CONTEXT.get::<VarietyComService>();
    let result = varity_service.get_by("id".to_string(),id).await;
    return RespVO::from_result(&result).resp_json();
}

//查询全部的产地
pub async fn get_origin(Json(query): Json<AllQuery>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<OriginService>();
    let result = origin_service.list(&query).await;
    return RespVO::from_result(&result).resp_json();
}


//查询公司的产地
pub async fn get_company_origin_byid(Path(id): Path<String>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<OriginComService>();
    let result = origin_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}
//查询公司的产地
pub async fn get_company_origin(Path(uid): Path<String>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<OriginComService>();
    let result = origin_service.fetch_list_by_column("company_code",&vec![uid]).await;
    return RespVO::from_result(&result).resp_json();
}
//保存公司的产地
pub async fn save_company_origin(Json(arg): Json<OriginComDTO>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<OriginComService>();
    let mut entity = arg.into();
    let result = origin_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
// 更新公司产地
pub async fn update_company_origin(Json(arg): Json<OriginComDTO>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<OriginComService>();
    let mut entity:OriginCom = arg.into();
    let result = origin_service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
//删除公司的产地
pub async fn del_company_origin(Path(id): Path<String>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<OriginComService>();
    let result = origin_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

// PQualityService
// 查询质量等级
pub async fn get_quality(Json(query): Json<AllQuery>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<PQualityService>();
    let result = origin_service.list(&query).await;
    return RespVO::from_result(&result).resp_json();
}

// PTypeService
// 查询库存产品类型
pub async fn get_type(Json(query): Json<AllQuery>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<PTypeService>();
    let result = origin_service.list(&query).await;
    return RespVO::from_result(&result).resp_json();
}

// PayModeService
// 查询支付方式
pub async fn get_paymode(Json(query): Json<AllQuery>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<PayModeService>();
    let result = origin_service.list(&query).await;
    return RespVO::from_result(&result).resp_json();
}

// PayModeComService
// 按公司查询支付方式
pub async fn get_company_paymode(Json(query): Json<ByComQuery>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<PayModeComService>();
    let result = origin_service.list(&query).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn save_company_paymode(Json(arg): Json<PayModeComDTO>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<PayModeComService>();
    let mut entity = arg.into();
    let result = origin_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn del_company_paymode(Path(id): Path<String>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<PayModeComService>();
    let result = origin_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

// RoleService
// 按公司查询支付方式
pub async fn get_role() -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<RoleService>();
    let query = AllQuery{};
    let result = origin_service.list(&query).await;
    return RespVO::from_result(&result).resp_json();
}

// StorageService
// 按公司查询仓库
pub async fn get_storage(Json(query): Json<HashMap<String, String>>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<StorageService>();
    let result = origin_service.search(&query).await;
    return RespVO::from_result(&result).resp_json();
}

// get_storage_by_id
pub async fn get_storage_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<StorageService>();
    let result = origin_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}
// StorageComService
// 按公司查询仓库
pub async fn get_company_storage(Path(uid): Path<String>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<StorageComService>();
    let result = origin_service.fetch_list_by_column("company_code",&vec![uid]).await;
    return RespVO::from_result(&result).resp_json();
}

// 按公司查询仓库
pub async fn get_company_storage_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<StorageComService>();
    let result = origin_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn del_company_storage(Path(id): Path<String>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<StorageComService>();
    let result = origin_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn save_company_storage(Json(arg): Json<StorageComDTO>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<StorageComService>();
    let mut entity = arg.into();
    let result = origin_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
// update_company_storage
pub async fn update_company_storage(Json(arg): Json<StorageComDTO>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<StorageComService>();
    let mut entity:StorageCom = arg.into();
    let result = origin_service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
// SupplyComService
// 按公司保存供应商
pub async fn save_company_supply(Json(varity): Json<SupplyComDTO>) -> impl IntoResponse {
    let setting_service = APPLICATION_CONTEXT.get::<SupplyComService>();
    let mut entity = varity.into();
    let result = setting_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

// 更新供应商保存公司
pub async fn update_company_supply(Json(varity): Json<SupplyComDTO>) -> impl IntoResponse {
    let setting_service = APPLICATION_CONTEXT.get::<SupplyComService>();
    let mut entity:SupplyCom = varity.into();
    let result = setting_service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
// 删除供应商
pub async fn del_company_supply(Path(id): Path<String>) -> impl IntoResponse {
    let setting_service = APPLICATION_CONTEXT.get::<SupplyComService>();
    let result = setting_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

// 按公司供应商
pub async fn get_company_supply(Path(uid): Path<String>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<SupplyComService>();
    let result = origin_service.fetch_list_by_column("company_code",&vec![uid]).await;
    return RespVO::from_result(&result).resp_json();
}

// 按id查询供应商
pub async fn get_company_supply_byid(Path(id): Path<String>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<SupplyComService>();
    let result = origin_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

// NoticeService
// 查询通知
pub async fn get_notice(Json(query): Json<NoticeQuery>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<NoticeService>();
    let result = origin_service.list(&query).await;
    return RespVO::from_result(&result).resp_json();
}
// 发布通知
pub async fn save_notice(Json(arg): Json<NoticeDTO>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<NoticeService>();
    let mut entity = arg.into();
    let result = origin_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
// 删除通知
pub async fn del_notice(Path(id): Path<String>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<NoticeService>();
    let result = origin_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

// FackdbackService
pub async fn save_fack(Json(arg): Json<FackbackDTO>) -> impl IntoResponse {
    let origin_service = APPLICATION_CONTEXT.get::<FackbackService>();
    let mut entity = arg.into();
    let result = origin_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

// 保存公司信息
pub async fn save_company(Json(arg): Json<CompanyDTO>) -> impl IntoResponse {
    let company_service = APPLICATION_CONTEXT.get::<CompanyService>();

    let mut entity: Company = arg.into();
    // 生成公司代码
    if entity.company_code.is_none() {
        loop {
            let company_code = code_generate::generate_company_code();
            let result = company_service.get_by("company_code".to_string(), company_code.clone()).await;
            if result.is_err() {
                entity.company_code = Some(company_code.to_string());
                break;
            }
        }
    }
    // 赋值注册时间
    if entity.register_at.is_none() {
        entity.register_at = Some(DateTimeNative::now());
    }
    // 最后可用时间
    if entity.lastest_at.is_none() {
        let now: DateTime<Local> = Local::now();
        let latest_date = now.checked_add_months(Months::new(3));
        println!("latest_date:{}",latest_date.unwrap().format("%Y-%m-%d %H:%M:%S").to_string());
        let latest_at = DateTimeNative::from(latest_date.unwrap());
        entity.lastest_at =Some(latest_at);
    }
    let result = company_service.save(&mut entity).await;

    if result.is_err() {
        return RespVO::from_result(&result).resp_json();
    }
    entity.id = Some(result.unwrap().try_into().unwrap());  //设置id 返回
    return RespVO::from_result_ok(entity).resp_json();
}

// 查询公司信息
pub async fn get_company(Path(id): Path<String>) -> impl IntoResponse {
    let company_service = APPLICATION_CONTEXT.get::<CompanyService>();
    let result = company_service.get_by("company_code".to_string(), id).await;
    return RespVO::from_result(&result).resp_json();
}

// 更新公司信息
pub async fn update_company(Json(arg): Json<CompanyDTO>) -> impl IntoResponse {
    let company_service = APPLICATION_CONTEXT.get::<CompanyService>();
    let mut entity: Company = arg.into();
    let result = company_service.update_by_id(entity.id.unwrap().to_string(), &mut entity).await;
    return RespVO::from_result(&result).resp_json();
}


// ClientComService
// 按公司保存客户
pub async fn save_company_client(Json(client): Json<ClientComDTO>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<ClientComService>();
    let mut entity = client.into();
    println!("client: {:#?}", entity);
    let result = client_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

// 更新客户
pub async fn update_company_client(Json(varity): Json<ClientComDTO>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<ClientComService>();
    let mut entity:ClientCom = varity.into();
    let result = client_service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
// 删除客户
pub async fn del_company_client(Path(id): Path<String>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<ClientComService>();
    let result = client_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

// 按公司客户
pub async fn get_company_client(Path(uid): Path<String>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<ClientComService>();
    let result = client_service.fetch_list_by_column("company_code",&vec![uid]).await;
    return RespVO::from_result(&result).resp_json();
}

// 按id查询客户
pub async fn get_company_client_byid(Path(id): Path<String>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<ClientComService>();
    let result = client_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}




// SepcService
// 按公司保存客户
pub async fn save_sepc(Json(sepc): Json<SepcComDTO>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<SepcService>();
    let mut entity = sepc.into();
    let result = client_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

// 更新客户
pub async fn update_sepc(Json(sepc): Json<SepcComDTO>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<SepcService>();
    let mut entity:SepcCom = sepc.into();
    let result = client_service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
// 删除客户
pub async fn del_sepc(Path(id): Path<String>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<SepcService>();
    let result = client_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

// 按公司客户
pub async fn get_company_sepc(Path(uid): Path<String>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<SepcService>();
    let result = client_service.fetch_list_by_column("company_code",&vec![uid]).await;
    return RespVO::from_result(&result).resp_json();
}

// 按id查询客户
pub async fn get_company_sepc_byid(Path(id): Path<String>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<SepcService>();
    let result = client_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}



// ShopSignService
// 按公司保存客户
pub async fn save_shop_sign(Json(shop): Json<ShopSignComDTO>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<ShopSignService>();
    let mut entity = shop.into();
    let result = client_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

// 更新客户
pub async fn update_shop_sign(Json(shop): Json<ShopSignComDTO>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<ShopSignService>();
    let mut entity:ShopSignCom = shop.into();
    let result = client_service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
// 删除客户
pub async fn del_shop_sign(Path(id): Path<String>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<ShopSignService>();
    let result = client_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

// 按公司客户
pub async fn get_company_shop_sign(Path(uid): Path<String>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<ShopSignService>();
    let result = client_service.fetch_list_by_column("company_code",&vec![uid]).await;
    return RespVO::from_result(&result).resp_json();
}

// 按id查询客户
pub async fn get_company_shop_byid(Path(id): Path<String>) -> impl IntoResponse {
    let client_service = APPLICATION_CONTEXT.get::<ShopSignService>();
    let result = client_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}