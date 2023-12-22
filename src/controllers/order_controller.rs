use crate::crud::crud_service::CrudService;
use crate::services::order::{AfterSaleProductService, AfterSaleService, OrderProductService, OrderService, OrderTransService};
use crate::APPLICATION_CONTEXT;
use axum::{extract::Path, response::IntoResponse, Json};
use rbatis::DateNative;
use wy_domain::dto::agree::OrderByFilterDTO;
use wy_domain::dto::order::{
    OrderAfterSaleDTO, OrderAfterSaleFilterParams, OrderAfterSaleParams, OrderAfterSaleProductDTO, OrderDTO, OrderFilterParams, OrderParams, OrderProductDTO, OrderProductForSelectFilterParams,
    OrderTransDTO, TransCodeFilterParams,
};
use wy_domain::entity::order::{Order, OrderAfterSale, OrderAfterSaleProduct, OrderProduct, OrderTrans};
use wy_domain::entity::PageData;
use wy_domain::vo::RespVO;

pub async fn order_product_by_agree_id(Path(agree_id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderProductService>();
    let result = order_service.fetch_list_by_column("agree_id", &vec![agree_id]).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn order_product_by_filter(Json(arg): Json<OrderProductForSelectFilterParams>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderProductService>();
    let page = PageData {
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };
    let result = order_service.page(&arg, page).await;
    return RespVO::from_result(&result).resp_json();
}

//保存采购订单 并 入库
pub async fn in_storage_by_agree(Path(agree_id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderService>();
    // 根据订单的ID 查询订单产品
    let trans_to_storage_res = order_service.agree_trans_to_storage_product(agree_id).await;
    return RespVO::from_result(&trans_to_storage_res).resp_json();
}

//保存采购订单 并 入库
pub async fn in_storage_by_order(Path(order_id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderService>();
    // 根据订单的ID 查询订单产品
    let trans_to_storage_res = order_service.trans_to_storage_product(order_id).await;
    return RespVO::from_result(&trans_to_storage_res).resp_json();
}

// 按照选择入库 in_storage_by_select_product
pub async fn in_storage_by_select_product(Json(arg): Json<Vec<OrderProductDTO>>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderService>();
    // 根据订单的ID 查询订单产品
    let trans_to_storage_res = order_service.trans_to_storage_product_by_select(arg).await;
    return RespVO::from_result(&trans_to_storage_res).resp_json();
}

//保存采购订单
pub async fn save_order(Json(arg): Json<OrderDTO>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderService>();
    let mut entity = arg.into();
    let result = order_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

//查询采购订单
pub async fn get_order(Path(id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderService>();
    let result = order_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

// get_order_by_uid
pub async fn get_order_by_uid(Json(arg): Json<OrderFilterParams>) -> impl IntoResponse {
    let params = OrderParams {
        company_code: arg.company_code.clone(),
        supply_dept: arg.supply_dept.clone(),
        delivery_dept: arg.delivery_dept.clone(),
        buy_mode: arg.buy_mode.clone(),
        order_start: if arg.order_start.is_some() {
            Some(DateNative::from_str(arg.order_start.clone().unwrap().as_str()).unwrap())
        } else {
            None
        },
        order_end: if arg.order_end.is_some() {
            Some(DateNative::from_str(arg.order_end.clone().unwrap().as_str()).unwrap())
        } else {
            None
        },
        order_status: arg.order_status.clone(),
    };

    let page = PageData {
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };

    println!("OrderParams:{:?}", params);
    println!("page:{:?}", page);

    let agree_service = APPLICATION_CONTEXT.get::<OrderService>();
    let result = agree_service.page(&params, page).await;
    println!("result:{:?}", result);
    return RespVO::from_result(&result).resp_json();
}

//更新采购订单
pub async fn update_order(Json(arg): Json<OrderDTO>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderService>();
    let mut entity: Order = arg.into();
    let result = order_service.update_by_id(entity.id.unwrap().to_string(), &mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

//删除采购订单
pub async fn del_order(Path(id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderService>();
    let result = order_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}
// 查询应付款
pub async fn get_order_by_filter(Json(arg): Json<OrderByFilterDTO>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderService>();
    let result = order_service.get_order_by_filter(arg).await;
    return RespVO::from_result(&result).resp_json();
}

//保存采购订单的产品
pub async fn save_order_product(Json(arg): Json<OrderProductDTO>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderProductService>();
    let mut entity = arg.into();
    let result = order_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

//查询采购订单的产品
pub async fn get_order_product(Path(uid): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderProductService>();
    let result = order_service.fetch_list_by_column("order_id", &vec![uid]).await;
    return RespVO::from_result(&result).resp_json();
}

//更新采购订单的产品
pub async fn update_order_product(Json(arg): Json<OrderProductDTO>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderProductService>();
    let mut entity: OrderProduct = arg.into();
    let result = order_service.update_by_id(entity.id.unwrap().to_string(), &mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

//删除采购订单
pub async fn del_order_product(Path(id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderProductService>();
    let result = order_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

//按照id查询采购订单
pub async fn get_order_product_byid(Path(id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderProductService>();
    let result = order_service.get_by("id".to_string(), id).await;
    return RespVO::from_result(&result).resp_json();
}

//按照id查询采购订单
pub async fn get_order_product_byuid(Path(id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderProductService>();
    let result = order_service.get_by("order_id".to_string(), id).await;
    return RespVO::from_result(&result).resp_json();
}

//保存采购订单的运杂费
pub async fn save_order_trans(Json(arg): Json<OrderTransDTO>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let mut entity = arg.into();
    let result = order_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_order_trans_by_agree_id(Path(agree_id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let result = order_service.fetch_list_by_column("agree_id", &vec![agree_id]).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_order_trans_by_adjust_id(Path(adjust_id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let result = order_service.fetch_list_by_column("adjust_id", &vec![adjust_id]).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_order_trans_by_process_id(Path(process_id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let result = order_service.fetch_list_by_column("process_id", &vec![process_id]).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_order_trans_by_sale_id(Path(sale_id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let result = order_service.fetch_list_by_column("sale_id", &vec![sale_id]).await;
    return RespVO::from_result(&result).resp_json();
}

// get_order_trans_by_after_sale_id
pub async fn get_order_trans_by_after_sale_id(Path(after_sale_id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let result = order_service.fetch_list_by_column("after_sale_id", &vec![after_sale_id]).await;
    return RespVO::from_result(&result).resp_json();
}
// order_trans_by_buy_after_sale_id
pub async fn order_trans_by_buy_after_sale_id(Path(after_sale_id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let result = order_service.fetch_list_by_column("buy_after_sale_id", &vec![after_sale_id]).await;
    return RespVO::from_result(&result).resp_json();
}
//查询采购订单的运杂费
pub async fn get_order_trans(Path(uid): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let result = order_service.fetch_list_by_column("order_id", &vec![uid]).await;
    return RespVO::from_result(&result).resp_json();
}

//更新采购订单的运杂费
pub async fn update_order_trans(Json(arg): Json<OrderTransDTO>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let mut entity: OrderTrans = arg.into();
    let result = order_service.update_by_id(entity.id.unwrap().to_string(), &mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

//删除采购订单的运杂费
pub async fn del_order_trans(Path(id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let result = order_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

//按照id查询采购订单
pub async fn get_order_trans_byid(Path(id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let result = order_service.get_by("id".to_string(), id).await;
    return RespVO::from_result(&result).resp_json();
}

//售后
pub async fn save_order_aftersale(Json(arg): Json<OrderAfterSaleDTO>) -> impl IntoResponse {
    let after_sale_service = APPLICATION_CONTEXT.get::<AfterSaleService>();
    let mut entity: OrderAfterSale = arg.into();
    let result = after_sale_service.save(&mut entity).await;
    if result.is_err() {
        return RespVO::from_result(&result.clone()).resp_json();
    } else {
        let id = result.clone().unwrap();
        let result_udpate = after_sale_service.after_sale_update_storage(id as u64).await;
        if result_udpate.is_err() {
            return RespVO::from_result(&result_udpate).resp_json();
        }
    }
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_order_aftersale(Path(id): Path<String>) -> impl IntoResponse {
    let after_sale_service = APPLICATION_CONTEXT.get::<AfterSaleService>();
    let result = after_sale_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

//更新采购订单的运杂费
pub async fn update_order_aftersale(Json(arg): Json<OrderAfterSaleDTO>) -> impl IntoResponse {
    let after_sale_service = APPLICATION_CONTEXT.get::<AfterSaleService>();
    let mut entity: OrderAfterSale = arg.into();
    println!("entity:{:?}", entity);
    if entity.back_status.unwrap() == 2 {
        let result_update = after_sale_service.after_sale_update_storage(entity.id.unwrap()).await;
        if result_update.is_ok() {
            let result = after_sale_service.update_by_id(entity.id.unwrap().to_string(), &mut entity).await;
            return RespVO::from_result(&result).resp_json();
        }
    }
    let result = after_sale_service.update_by_id(entity.id.unwrap().to_string(), &mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

//删除
pub async fn del_order_aftersale(Path(id): Path<String>) -> impl IntoResponse {
    let after_sale_service = APPLICATION_CONTEXT.get::<AfterSaleService>();
    let result = after_sale_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

//按照id查询采购订单
pub async fn get_order_aftersale_byuid(Path(uid): Path<String>) -> impl IntoResponse {
    let after_sale_service = APPLICATION_CONTEXT.get::<AfterSaleService>();
    let result = after_sale_service.get_by("company_code".to_string(), uid).await;
    return RespVO::from_result(&result).resp_json();
}
// get_order_aftersale_by_agree_id
pub async fn get_order_aftersale_by_agree_id(Path(agree_id): Path<String>) -> impl IntoResponse {
    let after_sale_service = APPLICATION_CONTEXT.get::<AfterSaleService>();
    let result = after_sale_service.fetch_list_by_column("agree_id", &vec![agree_id]).await;
    return RespVO::from_result(&result).resp_json();
}
// get_order_aftersale_by_order_id
pub async fn get_order_aftersale_by_order_id(Path(order_id): Path<String>) -> impl IntoResponse {
    let after_sale_service = APPLICATION_CONTEXT.get::<AfterSaleService>();
    let result = after_sale_service.fetch_list_by_column("order_id", &vec![order_id]).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_order_aftersale_byfilter(Json(arg): Json<OrderAfterSaleFilterParams>) -> impl IntoResponse {
    let after_sale_service = APPLICATION_CONTEXT.get::<AfterSaleService>();

    println!("params: {:?}", arg);

    let params: OrderAfterSaleParams = OrderAfterSaleParams {
        company_code: arg.company_code.clone(),
        supply_dept: arg.supply_dept.clone(),
        after_sale_start: arg.after_sale_start.clone(),
        after_sale_end: arg.after_sale_end.clone(),
    };

    let page = PageData {
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };
    let result = after_sale_service.page(&params, page).await;
    return RespVO::from_result(&result).resp_json();
}

//售后产品
pub async fn save_order_aftersale_product(Json(arg): Json<OrderAfterSaleProductDTO>) -> impl IntoResponse {
    let after_sale_product_service = APPLICATION_CONTEXT.get::<AfterSaleProductService>();
    let mut entity = arg.into();
    let result = after_sale_product_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn update_order_aftersale_product(Json(arg): Json<OrderAfterSaleProductDTO>) -> impl IntoResponse {
    let after_sale_product_service = APPLICATION_CONTEXT.get::<AfterSaleProductService>();
    let mut entity: OrderAfterSaleProduct = arg.into();
    let result = after_sale_product_service.update_by_id(entity.id.unwrap().to_string(), &mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_order_aftersale_product(Path(id): Path<String>) -> impl IntoResponse {
    let after_sale_service = APPLICATION_CONTEXT.get::<AfterSaleProductService>();
    let result = after_sale_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

//删除
pub async fn del_order_aftersale_product(Path(id): Path<String>) -> impl IntoResponse {
    let after_sale_service = APPLICATION_CONTEXT.get::<AfterSaleProductService>();
    let result = after_sale_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

//按照id查询采购订单
pub async fn get_order_aftersale_by_pid(Path(uid): Path<String>) -> impl IntoResponse {
    let after_sale_service = APPLICATION_CONTEXT.get::<AfterSaleProductService>();
    let result = after_sale_service.fetch_list_by_column("after_sale_id", &vec![uid]).await;
    return RespVO::from_result(&result).resp_json();
}

// 费用管理
// OrderTransService
pub async fn get_transcode_by_filter(Json(arg): Json<TransCodeFilterParams>) -> impl IntoResponse {
    let trans_cost_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let page = PageData {
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };
    let result = trans_cost_service.page(&arg, page).await;
    return RespVO::from_result(&result).resp_json();
}
