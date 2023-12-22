use std::collections::HashMap;

use crate::services::agree::{AgreeService, AgreeProductService};
use wy_domain::dto::agree::{AgreeDTO, AgreeProductDTO,AgreeFilterParams,AgreeParams,OrderByFilterDTO};
use wy_domain::entity::PageData;
use wy_domain::entity::agree::{Agree,AgreeProduct};
use wy_domain::vo::RespVO;
use wy_domain::request::{ByComQuery,ByUIDQuery};
use crate::APPLICATION_CONTEXT;
use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
};
use crate::crud::crud_service::CrudService;


//协议保存
pub async fn save_agree(Json(arg): Json<AgreeDTO>) -> impl IntoResponse {
    let agree_service = APPLICATION_CONTEXT.get::<AgreeService>();
    println!("AgreeDTO:{:?}",arg);
    let mut entity = arg.into();
    println!("entity:{:?}",entity);
    let result = agree_service.save(&mut entity).await;

    return RespVO::from_result(&result).resp_json();
}

//协议查询按公司查询
pub async fn get_agree(Path(id): Path<String>) -> impl IntoResponse {
    let agree_service = APPLICATION_CONTEXT.get::<AgreeService>();
    let result = agree_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}
// get_agree_by_uid
pub async fn get_agree_by_uid(Json(arg): Json<AgreeFilterParams>) -> impl IntoResponse {
    let params = AgreeParams {
        company_code:arg.company_code.clone(),
        supply_dept:arg.supply_dept.clone(),
        delivery_dept:arg.delivery_dept.clone(),
        agree_start:arg.agree_start.clone(),
        agree_end:arg.agree_end.clone(),
        agree_status:arg.agree_status.clone(),
    };

    let page = PageData{
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };

    let agree_service = APPLICATION_CONTEXT.get::<AgreeService>();
    let result = agree_service.page(&params,page).await;
    return RespVO::from_result(&result).resp_json();
}
//更新协议
pub async fn update_agree(Json(arg): Json<AgreeDTO>) -> impl IntoResponse {
    let agree_service = APPLICATION_CONTEXT.get::<AgreeService>();
    let mut entity:Agree = arg.into();
    let result = agree_service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

//删除协议
pub async fn del_agree(Path(id): Path<String>) -> impl IntoResponse {
    let agree_service = APPLICATION_CONTEXT.get::<AgreeService>();
    let result = agree_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

// get_agree_by_filter
pub async fn get_agree_by_filter(Json(arg): Json<OrderByFilterDTO>) -> impl IntoResponse {
    let agree_service = APPLICATION_CONTEXT.get::<AgreeService>();
    let result = agree_service.get_agree_payed_list(arg).await;
    return RespVO::from_result(&result).resp_json();
}

//协议产品保存
pub async fn save_agree_product(Json(arg): Json<AgreeProductDTO>) -> impl IntoResponse {
    let agree_product_service = APPLICATION_CONTEXT.get::<AgreeProductService>();
    let mut entity = arg.into();
    let result = agree_product_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

//更新协议产品保存
pub async fn update_agree_product(Json(arg): Json<AgreeProductDTO>) -> impl IntoResponse {
    let agree_product_service = APPLICATION_CONTEXT.get::<AgreeProductService>();
    let mut entity:AgreeProduct = arg.into();
    let result = agree_product_service.update_by_id(entity.id.unwrap().to_string(), &mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

//查询协议产品
pub async fn get_agree_product(Path(id): Path<String>) -> impl IntoResponse {
    let agree_product_service = APPLICATION_CONTEXT.get::<AgreeProductService>();
    let result = agree_product_service.get_by("id".to_string(), id).await;
    return RespVO::from_result(&result).resp_json();
}


//删除协议产品
pub async fn del_agree_product(Path(id): Path<String>) -> impl IntoResponse {
    let agree_product_service = APPLICATION_CONTEXT.get::<AgreeProductService>();
    let result = agree_product_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

//按协议id查询协议产品
pub async fn get_agree_product_byuid(Json(arg): Json<ByUIDQuery>) -> impl IntoResponse {
    let agree_product_service = APPLICATION_CONTEXT.get::<AgreeProductService>();
    let mut entity = arg.into();
    let result = agree_product_service.list(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

//按协议agree_id查询协议产品
pub async fn get_agree_product_by_agreeid(Path(id): Path<String>) -> impl IntoResponse {
    let agree_product_service = APPLICATION_CONTEXT.get::<AgreeProductService>();
    let result = agree_product_service.fetch_list_by_column("agree_id",&vec![id.clone()]).await;
    return RespVO::from_result(&result).resp_json();
}
//按协议sale_id查询协议产品
pub async fn get_agree_product_by_saleid(Path(id): Path<String>) -> impl IntoResponse {
    let agree_product_service = APPLICATION_CONTEXT.get::<AgreeProductService>();
    let result = agree_product_service.fetch_list_by_column("sale_id",&vec![id.clone()]).await;
    return RespVO::from_result(&result).resp_json();
}
