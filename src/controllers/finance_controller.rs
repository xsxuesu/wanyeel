use crate::services::finance::{PayedInfoService,ReceiveInfoService,StaticalService,InvoiceService};
use wy_domain::dto::finance::{PayedInfoDTO,ReceivedInfoDTO,PayedInfoFilterParams,ReceivedInfoFilterParams,TransStaticalParams,FinanceStaticalParams,BuyStaticalParams, SaleStaticalParams, TaxInfoDTO};
use wy_domain::entity::finance::{PayedInfo,ReceivedInfo,TaxInfo};
use wy_domain::vo::RespVO;
use wy_domain::entity::PageData;
use rbatis::DateNative;
use crate::APPLICATION_CONTEXT;
use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
};
use crate::crud::crud_service::CrudService;



//付款
pub async fn save_payed_info(Json(arg): Json<PayedInfoDTO>) -> impl IntoResponse {
    let payed_service = APPLICATION_CONTEXT.get::<PayedInfoService>();
    let mut entity = arg.into();
    let result = payed_service.save( &mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
pub async fn update_payed_info(Json(arg): Json<PayedInfoDTO>) -> impl IntoResponse {
    let payed_service = APPLICATION_CONTEXT.get::<PayedInfoService>();
    let mut entity:PayedInfo = arg.into();
    let result = payed_service.update_by_id( entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
pub async fn get_payed_info(Path(id): Path<String>) -> impl IntoResponse {
    let payed_service = APPLICATION_CONTEXT.get::<PayedInfoService>();
    let result = payed_service.get( id).await;
    return RespVO::from_result(&result).resp_json();
}
// 查询过滤
pub async fn get_payed_info_by_filter(Json(arg): Json<PayedInfoFilterParams>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<PayedInfoService>();
    let page = PageData{
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };
    let result = sale_service.page(&arg,page).await;
    return RespVO::from_result(&result).resp_json();
}

// get_payed_info_by_agree_id
pub async fn get_payed_info_by_agree_id(Path(id): Path<String>) -> impl IntoResponse {
    let payed_service = APPLICATION_CONTEXT.get::<PayedInfoService>();
    let result = payed_service.fetch_list_by_column( "agree_id",&vec![id]).await;
    return RespVO::from_result(&result).resp_json();
}
// get_payed_info_by_sale_id
pub async fn get_payed_info_by_order_id(Path(id): Path<String>) -> impl IntoResponse {
    let payed_service = APPLICATION_CONTEXT.get::<PayedInfoService>();
    let result = payed_service.fetch_list_by_column( "order_id",&vec![id]).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn del_payed_info(Path(id): Path<String>) -> impl IntoResponse {
    let payed_service = APPLICATION_CONTEXT.get::<PayedInfoService>();
    let result = payed_service.del( &id).await;
    return RespVO::from_result(&result).resp_json();
}



//收款
pub async fn save_receieved_info(Json(arg): Json<ReceivedInfoDTO>) -> impl IntoResponse {
    let received_service = APPLICATION_CONTEXT.get::<ReceiveInfoService>();
    let mut entity = arg.into();
    let result = received_service.save( &mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
pub async fn update_receieved_info(Json(arg): Json<ReceivedInfoDTO>) -> impl IntoResponse {
    let received_service = APPLICATION_CONTEXT.get::<ReceiveInfoService>();
    let mut entity:ReceivedInfo = arg.into();
    let result = received_service.update_by_id( entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
pub async fn get_receieved_info(Path(id): Path<String>) -> impl IntoResponse {
    let received_service = APPLICATION_CONTEXT.get::<ReceiveInfoService>();
    let result = received_service.get( id).await;
    return RespVO::from_result(&result).resp_json();
}
// 查询过滤
pub async fn get_receieved_info_by_filter(Json(arg): Json<ReceivedInfoFilterParams>) -> impl IntoResponse {
    let receieved_service = APPLICATION_CONTEXT.get::<ReceiveInfoService>();
    let page = PageData{
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };
    let result = receieved_service.page(&arg,page).await;
    return RespVO::from_result(&result).resp_json();
}
// 根据销售id 查询收款get_recieved_by_sale_id
pub async fn get_recieved_by_sale_id(Path(id): Path<String>) -> impl IntoResponse {
    let receieved_service = APPLICATION_CONTEXT.get::<ReceiveInfoService>();
    let result = receieved_service.fetch_list_by_column("sale_id",&vec![id]).await;
    return RespVO::from_result(&result).resp_json();
}


pub async fn del_receieved_info(Path(id): Path<String>) -> impl IntoResponse {
    let received_service = APPLICATION_CONTEXT.get::<ReceiveInfoService>();
    let result = received_service.del( &id).await;
    return RespVO::from_result(&result).resp_json();
}


/************************************************************************************************
 *
 * 财务统计
 *
 */

 pub async fn statistics_trans(Json(arg): Json<TransStaticalParams>) -> impl IntoResponse {
    let statistics_service = APPLICATION_CONTEXT.get::<StaticalService>();
    let result = statistics_service.get_trans_statical( arg).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn statistics_payed(Json(arg): Json<FinanceStaticalParams>) -> impl IntoResponse {
    let statistics_service = APPLICATION_CONTEXT.get::<StaticalService>();
    let result = statistics_service.get_payed_statical( arg).await;
    return RespVO::from_result(&result).resp_json();
}


pub async fn statistics_recieve(Json(arg): Json<FinanceStaticalParams>) -> impl IntoResponse {
    let statistics_service = APPLICATION_CONTEXT.get::<StaticalService>();
    let result = statistics_service.get_recieve_statical( arg).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn statistics_buyed(Json(arg): Json<BuyStaticalParams>) -> impl IntoResponse {
    let statistics_service = APPLICATION_CONTEXT.get::<StaticalService>();
    let result = statistics_service.get_buy_statical( arg).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn statistics_buyed_order(Json(arg): Json<BuyStaticalParams>) -> impl IntoResponse {
    let statistics_service = APPLICATION_CONTEXT.get::<StaticalService>();
    let result = statistics_service.get_buy_statical_order( arg).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn statistics_sale_order(Json(arg): Json<SaleStaticalParams>) -> impl IntoResponse {
    let statistics_service = APPLICATION_CONTEXT.get::<StaticalService>();
    let result = statistics_service.get_sale_statical_order( arg).await;
    return RespVO::from_result(&result).resp_json();
}

/************************************************************************************************
 * 发票
 */


pub async fn save_tax_info(Json(arg): Json<TaxInfoDTO>) -> impl IntoResponse {
    let tax_service = APPLICATION_CONTEXT.get::<InvoiceService>();
    let mut entity = arg.into();
    let result = tax_service.save( &mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
pub async fn update_tax_info(Json(arg): Json<TaxInfoDTO>) -> impl IntoResponse {
    let tax_service = APPLICATION_CONTEXT.get::<InvoiceService>();
    let mut entity:TaxInfo = arg.into();
    let result = tax_service.update_by_id( entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
pub async fn get_tax_info(Path(id): Path<String>) -> impl IntoResponse {
    let tax_service = APPLICATION_CONTEXT.get::<InvoiceService>();
    let result = tax_service.get( id).await;
    return RespVO::from_result(&result).resp_json();
}
// 查询过滤
pub async fn get_tax_by_payed_id(Path(id): Path<String>) -> impl IntoResponse {
    let tax_service = APPLICATION_CONTEXT.get::<InvoiceService>();
    let result = tax_service.fetch_list_by_column("payed_id", &vec![id]).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_tax_by_recieved_id(Path(id): Path<String>) -> impl IntoResponse {
    let tax_service = APPLICATION_CONTEXT.get::<InvoiceService>();
    let result = tax_service.fetch_list_by_column("recieved_id",&vec![id]).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_tax_by_trans_id(Path(id): Path<String>) -> impl IntoResponse {
    let tax_service = APPLICATION_CONTEXT.get::<InvoiceService>();
    let result = tax_service.fetch_list_by_column("trans_id",&vec![id]).await;
    return RespVO::from_result(&result).resp_json();
}


pub async fn del_tax_info(Path(id): Path<String>) -> impl IntoResponse {
    let tax_service = APPLICATION_CONTEXT.get::<InvoiceService>();
    let result = tax_service.del( &id).await;
    return RespVO::from_result(&result).resp_json();
}
