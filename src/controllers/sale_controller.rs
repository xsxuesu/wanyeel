use crate::crud::crud_service::CrudService;
use crate::services::finance::ReceiveInfoService;
use crate::services::order::OrderTransService;
use crate::services::sale::{PreSaleProductService, SaleAfterService, SaleInfoService, SalePayService, SaleProductService};
use crate::APPLICATION_CONTEXT;
use axum::{extract::Path, response::IntoResponse, Json};
use wy_domain::dto::sale::{
    PreSaleProductDTO, SaleAfterDTO, SaleAfterPageDTO, SaleInfoDTO, SaleInfoPageDTO, SaleInfoRecievedPageDTO, SalePayDTO, SalePayPageDTO, SaleProductDTO, SaleProductFilterDTO,
};
use wy_domain::entity::sale::{PreSaleProduct, SaleAfter, SaleInfo, SalePay, SaleProduct};
use wy_domain::entity::PageData;
use wy_domain::error::Error;
use wy_domain::vo::RespVO;

//保存
pub async fn save_sale_info(Json(arg): Json<SaleInfoDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleInfoService>();
    let mut entity = arg.clone().into();
    let result = sale_service.save(&mut entity).await;

    if result.is_ok() && arg.sale_status.unwrap() == 2 {
        // 更新销售订单相关的产品
        sale_service.trans_sale_relations(entity.id.unwrap()).await;
    }
    if result.is_ok() && arg.sale_status.unwrap() == 1 {
        // 更新库存
        sale_service.trans_sale_out_product(entity.id.unwrap()).await;
    }

    return RespVO::from_result(&result).resp_json();
}
pub async fn update_sale_info(Json(arg): Json<SaleInfoDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleInfoService>();
    let mut entity: SaleInfo = arg.clone().into();
    let result = sale_service.update_by_id(entity.id.unwrap().to_string(), &mut entity).await;
    if result.is_ok() && arg.sale_status.unwrap() == 2 {
        // 更新销售订单相关的产品
        sale_service.trans_sale_relations(entity.id.unwrap()).await;
    }
    if result.is_ok() && arg.sale_status.unwrap() == 1 {
        // 更新库存
        sale_service.trans_sale_out_product(entity.id.unwrap()).await;
    }
    return RespVO::from_result(&result).resp_json();
}
pub async fn get_sale_info(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleInfoService>();
    let result = sale_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn del_sale_info(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleInfoService>();
    let result = sale_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}
// 查询翻页
pub async fn get_sale_info_by_filter(Json(arg): Json<SaleInfoPageDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleInfoService>();
    let page = PageData {
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };
    let result = sale_service.page(&arg, page).await;
    return RespVO::from_result(&result).resp_json();
}
// 应收款过滤
pub async fn get_recieved_info_by_filter(Json(arg): Json<SaleInfoRecievedPageDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleInfoService>();
    let result = sale_service.get_sale_recieved_list(arg).await;
    return RespVO::from_result(&result).resp_json();
}
// 更新库存
pub async fn out_storage_by_sale_id(Path(sale_id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleInfoService>();
    let result = sale_service.trans_sale_out_product(sale_id.parse::<u64>().unwrap()).await;
    return RespVO::from_result(&result).resp_json();
}

//保存销售产品
pub async fn save_sale_product(Json(arg): Json<SaleProductDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleProductService>();
    let mut entity = arg.into();
    let result = sale_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
pub async fn update_sale_product(Json(arg): Json<SaleProductDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleProductService>();
    let mut entity: SaleProduct = arg.into();
    let result = sale_service.update_by_id(entity.id.unwrap().to_string(), &mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
pub async fn get_sale_product(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleProductService>();
    let result = sale_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_sale_product_by_sale_id(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleProductService>();
    let result = sale_service.fetch_list_by_column("sale_id", &vec![id]).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_sale_product_by_after_sale_id(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleProductService>();
    let result = sale_service.fetch_list_by_column("sale_after_id", &vec![id]).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_sale_product_by_page(Json(arg): Json<SaleProductFilterDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleProductService>();
    let page = PageData {
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };
    let result = sale_service.page(&arg, page).await;
    return RespVO::from_result(&result).resp_json();
}

// sale_product_back_storage
pub async fn sale_product_back_storage(Path(sale_after_id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleProductService>();
    let result = sale_service.sale_product_back_storage(sale_after_id.parse::<u64>().unwrap()).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn del_sale_product(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleProductService>();
    let result = sale_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

//保存预售产品
pub async fn save_pre_sale_product(Json(arg): Json<PreSaleProductDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<PreSaleProductService>();
    let mut entity = arg.into();
    let result = sale_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
pub async fn update_pre_sale_product(Json(arg): Json<PreSaleProductDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<PreSaleProductService>();
    let mut entity: PreSaleProduct = arg.into();
    let result = sale_service.update_by_id(entity.id.unwrap().to_string(), &mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
pub async fn get_pre_sale_product(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<PreSaleProductService>();
    let result = sale_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_pre_sale_product_by_sale_id(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<PreSaleProductService>();
    let result = sale_service.fetch_list_by_column("sale_id", &vec![id]).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn del_pre_sale_product(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<PreSaleProductService>();
    let result = sale_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

//销售结算
pub async fn save_sale_pay(Json(arg): Json<SalePayDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SalePayService>();
    let sale_info_service = APPLICATION_CONTEXT.get::<SaleInfoService>();
    let mut entity = arg.clone().into();
    let result = sale_service.save(&mut entity).await;
    let mut id = 0;
    if arg.id.is_some() {
        id = arg.id.unwrap();
    } else {
        if result.is_ok() {
            id = result.clone().unwrap() as u64;
        }
    }
    if result.is_ok() {
        if entity.sale_id.is_some() {
            let res_update_order = sale_info_service.update_sale_key(arg.sale_id.unwrap(), id).await;
            println!("res_update_order:{:?}", res_update_order);
        }
    }

    return RespVO::from_result(&result).resp_json();
}
pub async fn update_sale_pay(Json(arg): Json<SalePayDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SalePayService>();
    let sale_info_service = APPLICATION_CONTEXT.get::<SaleInfoService>();

    let mut entity: SalePay = arg.clone().into();
    let result = sale_service.update_by_id(entity.id.unwrap().to_string(), &mut entity).await;
    let mut id = 0;
    if arg.id.is_some() {
        id = arg.id.unwrap();
    } else {
        if result.is_ok() {
            id = result.clone().unwrap() as u64;
        }
    }
    if result.is_ok() {
        if entity.sale_id.is_some() {
            let res_update_order = sale_info_service.update_sale_key(arg.sale_id.unwrap(), id).await;
            println!("res_update_order:{:?}", res_update_order);
        }
    }
    return RespVO::from_result(&result).resp_json();
}
pub async fn get_sale_pay(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SalePayService>();
    let result = sale_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}
// 查询过滤
pub async fn get_sale_pay_by_filter(Json(arg): Json<SalePayPageDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SalePayService>();
    let page = PageData {
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };
    let result = sale_service.page(&arg, page).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn del_sale_pay(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SalePayService>();
    let result = sale_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn cacl_sale_pay(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SalePayService>();
    let result = sale_service.sale_pay_update_order(id).await;
    return RespVO::from_result(&result).resp_json();
}

// 新建销售结算
// 结算新建
pub async fn get_new_sale_pay(Path(sale_id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleInfoService>();
    let sale_product_service = APPLICATION_CONTEXT.get::<SaleProductService>();
    let sale_trans_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let recieve_service = APPLICATION_CONTEXT.get::<ReceiveInfoService>();
    let sale_after_service = APPLICATION_CONTEXT.get::<SaleAfterService>();

    let result = sale_service.get(sale_id.clone()).await;
    let result_order_product = sale_product_service.fetch_list_by_column("sale_id", &vec![sale_id.clone()]).await;
    let result_order_trans = sale_trans_service.fetch_list_by_column("sale_id", &vec![sale_id.clone()]).await;
    let result_receive = recieve_service.fetch_list_by_column("sale_id", &vec![sale_id.clone()]).await;
    let result_sale_after = sale_after_service.fetch_list_by_column("sale_id", &vec![sale_id.clone()]).await;

    if result.is_ok() && result_order_product.is_ok() && result_order_trans.is_ok() {
        let mut sale_pay = SalePayDTO::default();

        sale_pay.sale_id = Some(sale_id.parse::<u64>().unwrap());
        sale_pay.sale_pay_status = Some(1);
        sale_pay.company_code = result.clone().unwrap().company_code.clone();
        sale_pay.unit = Some("吨".to_string());
        sale_pay.unit_money = Some("元".to_string());
        sale_pay.client_name = result.clone().unwrap().client_name.clone();

        // 销售数量
        let mut sale_number: f64 = 0.0;
        let mut sale_weight: f64 = 0.0;
        let mut sale_amount: f64 = 0.0;
        let mut taxt_rate: f64 = 0.0;
        // 出库数量
        let mut out_number = 0.0;
        // 出库重量
        let mut out_weight = 0.0;

        result_order_product.unwrap().iter().for_each(|order_product| {
            sale_number += order_product.sale_number.unwrap() as f64;
            if order_product.clone().unit.unwrap() == "千克" {
                sale_weight += (order_product.sale_weight.unwrap() as f64) / 1000.0;
            } else {
                sale_weight += order_product.sale_weight.unwrap() as f64;
            }
            sale_amount += (order_product.sale_weight.unwrap() as f64) * (order_product.unit_price.unwrap() as f64);
            taxt_rate += ((order_product.sale_weight.unwrap() as f64) * (order_product.unit_price.unwrap() as f64) * (order_product.tax_rate.unwrap() as f64)) / 100.0;

            if order_product.clone().sale_status.unwrap() > 1 {
                out_number += order_product.sale_number.unwrap() as f64;
                if order_product.clone().unit.unwrap() == "千克" {
                    out_weight += (order_product.sale_weight.unwrap() as f64) / 1000.0;
                } else {
                    out_weight += order_product.sale_weight.unwrap() as f64;
                }
            }
        });
        sale_pay.sale_weight = Some(((sale_weight * 1000.0).round() as f64) / 1000.0 as f64);
        sale_pay.sale_amount = Some(((sale_amount * 1000.0).round() as f64) / 1000.0 as f64);
        sale_pay.sale_number = Some(((sale_number * 1000.0).round() as f64) / 1000.0 as f64);
        sale_pay.out_number = Some(((out_number * 1000.0).round() as f64) / 1000.0 as f64);
        sale_pay.out_weight = Some(((out_weight * 1000.0).round() as f64) / 1000.0 as f64);
        // 运杂费
        let mut trans_amount = 0.0;
        result_order_trans.unwrap().iter().for_each(|order_trans| {
            if order_trans.clone().in_out.unwrap() == 1 {
                //支出
                trans_amount += order_trans.cost_amount.unwrap() as f64;
                taxt_rate += ((order_trans.cost_amount.unwrap() as f64) * (order_trans.cost_rate.unwrap() as f64)) / 100.0;
            }
            if order_trans.clone().in_out.unwrap() == 2 {
                // 收入
                trans_amount += order_trans.cost_amount.unwrap() as f64;
                taxt_rate -= ((order_trans.cost_amount.unwrap() as f64) * (order_trans.cost_rate.unwrap() as f64)) / 100.0;
            }
        });
        sale_pay.trans_amount = Some(((trans_amount * 100.0).round() as f64) / 100.0 as f64);
        sale_pay.pay_amount = sale_pay.sale_amount;
        sale_pay.rate_amount = Some(((taxt_rate * 100.0).round() as f64) / 100.0 as f64);
        println!("sale_pay: {:?}", sale_pay);

        // 已收款
        let mut recieved_amount = 0.0;
        // 已开票
        let mut invoince_amount = 0.0;
        // 税额
        let mut taxt_amount = 0.0;
        result_receive.unwrap().iter().for_each(|receive| {
            recieved_amount += receive.recieve_amount.unwrap() as f64;
            if receive.invoice_amount.is_some() {
                invoince_amount += receive.invoice_amount.unwrap() as f64;
                taxt_amount += (receive.invoice_amount.unwrap() as f64) * (receive.tax_rate.unwrap() as f64) / 100.0;
            }
        });
        sale_pay.recieved_amount = Some(((recieved_amount * 100.0 as f64).round() as f64) / 100.0 as f64);
        sale_pay.invoice_amount = Some(((invoince_amount * 100.0 as f64).round() as f64) / 100.0 as f64);
        sale_pay.rate_amount = Some(((taxt_amount * 100.0 as f64).round() as f64) / 100.0 as f64);

        // 售后费用
        let mut sale_after_amount = 0.0;
        let mut sale_after_ids = result_sale_after
            .unwrap()
            .iter()
            .map(|sale_after| {
                sale_after_amount += sale_after.back_amount.unwrap() as f64;
                sale_after.id.unwrap()
            })
            .collect::<Vec<u64>>();
        sale_pay.after_sale_amount = Some(((sale_after_amount * 100.0).round() as f64) / 100.0 as f64);

        // 获取运杂费
        let mut items = vec![];
        for id in sale_after_ids {
            let trans = sale_trans_service.fetch_list_by_column("buy_after_sale_id", &vec![id.clone().to_string()]).await;
            items.push(trans.unwrap());
        }
        let mut after_sale_trans_amount = 0.0;
        items.iter().for_each(|order_trans_list| {
            order_trans_list.iter().for_each(|order_trans| {
                if order_trans.clone().in_out.unwrap() == 1 {
                    // 支出
                    after_sale_trans_amount += order_trans.cost_amount.unwrap() as f64;
                }

                if order_trans.clone().in_out.unwrap() == 2 {
                    // 收入
                    after_sale_trans_amount -= order_trans.cost_amount.unwrap() as f64;
                }
            });
        });
        sale_pay.after_sale_trans_amount = Some(((after_sale_trans_amount * 100.0).round() as f64) / 100.0 as f64);

        return RespVO::from(&sale_pay).resp_json();
    }
    return RespVO::<i64>::from_error(&Error::E("查询失败".to_string())).resp_json();
}

// 售后
pub async fn save_sale_after(Json(arg): Json<SaleAfterDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleAfterService>();
    let mut entity: SaleAfter = arg.into();

    if entity.sale_after_status.unwrap() == 2 {
        //当确认后先更新
        let update_relations = sale_service.trans_after_sale_relation(entity.id.unwrap()).await;
        if update_relations.is_err() {
            return RespVO::<()>::from_error(&Error::E("更新失败".to_string())).resp_json();
        }
    }
    let result = sale_service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}
pub async fn update_sale_after(Json(arg): Json<SaleAfterDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleAfterService>();
    let mut entity: SaleAfter = arg.into();

    if entity.sale_after_status.unwrap() == 2 {
        //当确认后先更新
        let update_relations = sale_service.trans_after_sale_relation(entity.id.unwrap()).await;
        if update_relations.is_err() {
            return RespVO::<()>::from_error(&Error::E("更新失败".to_string())).resp_json();
        }
    }

    let result = sale_service.update_by_id(entity.id.unwrap().to_string(), &mut entity).await;

    return RespVO::from_result(&result).resp_json();
}
pub async fn get_sale_after(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleAfterService>();
    let result = sale_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn del_sale_after(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleAfterService>();
    let result = sale_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_sale_after_by_filter(Json(arg): Json<SaleAfterPageDTO>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleAfterService>();
    let page = PageData {
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };
    let result = sale_service.page(&arg, page).await;
    return RespVO::from_result(&result).resp_json();
}

// get_sale_after_by_sale_id
pub async fn get_sale_after_by_sale_id(Path(id): Path<String>) -> impl IntoResponse {
    let sale_service = APPLICATION_CONTEXT.get::<SaleAfterService>();
    let result = sale_service.fetch_list_by_column("sale_id", &vec![id]).await;
    return RespVO::from_result(&result).resp_json();
}
