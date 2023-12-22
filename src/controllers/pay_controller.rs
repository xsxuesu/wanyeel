use crate::crud::crud_service::CrudService;
use crate::services::agree::AgreeService;
use crate::services::order::{AfterSaleService, OrderProductService, OrderService, OrderTransService};
use crate::services::pay::OrderPayService;
use crate::services::finance::PayedInfoService;
use wy_domain::dto::finance::PayedInfoDTO;
use crate::APPLICATION_CONTEXT;
use axum::{extract::Path, response::IntoResponse, Json};
use wy_domain::dto::order;
use wy_domain::dto::pay::{OrderPayDTO, OrderPayFilterParams, OrderPayParams};
use wy_domain::entity::agree::Agree;
use wy_domain::entity::pay::OrderPay;
use wy_domain::entity::PageData;
use wy_domain::error::Error;
use wy_domain::request::{ByComQuery, ByUIDQuery};
use wy_domain::vo::RespVO;

//保存
pub async fn save_order_pay(Json(arg): Json<OrderPayDTO>) -> impl IntoResponse {
    let order_pay_service = APPLICATION_CONTEXT.get::<OrderPayService>();
    let order_service = APPLICATION_CONTEXT.get::<OrderService>();
    let agree_service = APPLICATION_CONTEXT.get::<AgreeService>();
    let mut entity = arg.clone().into();
    let result = order_pay_service.save(&mut entity).await;
    let mut id = 0;
    if arg.id.is_some() {
        id = arg.id.unwrap();
    } else {
        if result.is_ok() {
            id = result.clone().unwrap() as u64;
        }
    }
    if result.is_ok() {
        println!("result:{:?}", result);
        if entity.order_id.is_some() {
            let res_update_order = order_service.update_order_key(arg.order_id.unwrap(), id).await;
            println!("res_update_order:{:?}", res_update_order);
        }
        if entity.agree_id.is_some() {
            let res_update_agree = agree_service.update_agree_key(arg.agree_id.unwrap(), id).await;
            println!("res_update_agree:{:?}", res_update_agree);
        }
    }

    return RespVO::from_result(&result).resp_json();
}
//结算
pub async fn cacl_order_pay(Path(id): Path<String>) -> impl IntoResponse {
    let order_pay_service = APPLICATION_CONTEXT.get::<OrderPayService>();
    let result = order_pay_service.save_pay_update_order(id).await;
    return RespVO::from_result(&result).resp_json();
}
// 结算协议
pub async fn cacl_agree_pay(Path(id): Path<String>) -> impl IntoResponse {
    let order_pay_service = APPLICATION_CONTEXT.get::<OrderPayService>();
    let result = order_pay_service.save_pay_update_agree(id).await;
    return RespVO::from_result(&result).resp_json();
}
//更新结算
pub async fn update_order_pay(Json(arg): Json<OrderPayDTO>) -> impl IntoResponse {
    let order_pay_service = APPLICATION_CONTEXT.get::<OrderPayService>();
    let order_service = APPLICATION_CONTEXT.get::<OrderService>();
    let agree_service = APPLICATION_CONTEXT.get::<AgreeService>();
    let mut entity: OrderPay = arg.clone().into();
    let result = order_pay_service.update_by_id(entity.id.unwrap().to_string(), &mut entity).await;
    if result.is_ok() {
        if entity.order_id.is_some() {
            let res_update_order = order_service.update_order_key(arg.order_id.unwrap(), arg.id.unwrap()).await;
        }
        if entity.agree_id.is_some() {
            let res_update_agree = agree_service.update_agree_key(arg.agree_id.unwrap(), arg.id.unwrap()).await;
        }
    }
    return RespVO::from_result(&result).resp_json();
}

//查询结算
pub async fn get_order_pay(Path(id): Path<String>) -> impl IntoResponse {
    let order_pay_service = APPLICATION_CONTEXT.get::<OrderPayService>();
    let result = order_pay_service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}
// 协议结算
pub async fn get_new_order_pay_by_agree(Path(agree_id): Path<String>) -> impl IntoResponse {
    let agree_service = APPLICATION_CONTEXT.get::<AgreeService>();
    let order_product_service = APPLICATION_CONTEXT.get::<OrderProductService>();
    let order_trans_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let order_pay_service = APPLICATION_CONTEXT.get::<PayedInfoService>();
    let after_sale_service = APPLICATION_CONTEXT.get::<AfterSaleService>();

    let result = agree_service.get(agree_id.clone()).await;
    let result_order_product = order_product_service.fetch_list_by_column("agree_id", &vec![agree_id.clone()]).await;
    let result_order_trans = order_trans_service.fetch_list_by_column("agree_id", &vec![agree_id.clone()]).await;
    let result_order_pay = order_pay_service.fetch_list_by_column("agree_id", &vec![agree_id.clone()]).await;
    let result_after_sale = after_sale_service.fetch_list_by_column("agree_id", &vec![agree_id.clone()]).await;

    if result.is_ok() && result_order_product.is_ok() && result_order_trans.is_ok() {
        let mut order_pay = OrderPayDTO::default();

        order_pay.agree_id = Some(agree_id.parse::<u64>().unwrap());
        order_pay.pay_status = Some(1);
        order_pay.company_code = result.clone().unwrap().company_code.clone();
        order_pay.unit = Some("吨".to_string());
        order_pay.unit_money = Some("元".to_string());
        order_pay.agree_type = result.clone().unwrap().agree_type.clone();
        order_pay.supply_dept = result.clone().unwrap().supply_dept.clone();
        order_pay.delivery_dept = result.clone().unwrap().delivery_dept.clone();
        // 采购数量
        let mut buy_number: f64 = 0.0;
        let mut buy_weight: f64 = 0.0;
        let mut buy_amount: f64 = 0.0;
        let mut storage_number: f64 = 0.0;
        let mut storage_weight: f64 = 0.0;


        result_order_product.unwrap().iter().for_each(|order_product| {
            buy_number += order_product.buy_number.unwrap() as f64;
            if order_product.clone().unit.unwrap() == "千克" {
                buy_weight += (order_product.buy_weight.unwrap() as f64) / 1000.0;
            } else {
                buy_weight += order_product.buy_weight.unwrap() as f64;
            }
            buy_amount += order_product.buy_amount.unwrap() as f64;

            if order_product.storage_status.unwrap() == 2 {
                // 已入库
                if order_product.clone().unit.unwrap() == "千克" {
                    storage_weight += (order_product.buy_weight.unwrap() as f64) / 1000.0;
                } else {
                    storage_weight += order_product.buy_weight.unwrap() as f64;
                }
                storage_number += order_product.buy_number.unwrap() as f64;
            }
        });
        order_pay.buy_number = Some(buy_number);
        order_pay.buy_weight = Some(((buy_weight * 1000.0).round() as f64) / 1000.0 as f64);

        order_pay.storage_number = Some(storage_number);
        order_pay.storage_weight = Some(((storage_weight * 1000.0).round() as f64) / 1000.0 as f64);
        order_pay.product_amount = Some(((buy_amount * 100.0).round() as f64) / 100.0 as f64);
        // 运杂费
        let mut trans_amount = 0.0;
        result_order_trans.unwrap().iter().for_each(|order_trans| {
            if  order_trans.in_out.unwrap() == 1 { // 支出
                trans_amount += order_trans.cost_amount.unwrap() as f64;
            }
            if  order_trans.in_out.unwrap() == 2 { // 收入
                trans_amount -= order_trans.cost_amount.unwrap() as f64;
            }
        });
        order_pay.trans_amount = Some(((trans_amount * 100.0).round() as f64) / 100.0 as f64);
        // 采购的总金额 = 销售的总金额 + 运费
        order_pay.buy_amount = Some(order_pay.trans_amount.unwrap() + order_pay.product_amount.unwrap());

        // 已支付金额
        let mut pay_amount = 0.0;
        // 已开票金额
        let mut payed_amount = 0.0;
        // 支付税额
        let mut pay_tax_amount = 0.0;
        result_order_pay.unwrap().iter().for_each(|order_pay| {
            pay_amount += order_pay.pay_amount.unwrap() as f64;
            payed_amount += order_pay.invoice_amount.unwrap() as f64;
            pay_tax_amount += (order_pay.invoice_amount.unwrap() as f64) * (order_pay.tax_rate.unwrap() as f64) /100.0;
        });
        // 已支付金额
        order_pay.pay_amount = Some(((pay_amount * 100.0).round() as f64) / 100.0 as f64);
        // 已开票金额
        order_pay.payed_amount = Some(((payed_amount * 100.0).round() as f64) / 100.0 as f64);
        // 支付税额
        order_pay.rate_amount = Some(((pay_tax_amount * 100.0).round() as f64) / 100.0 as f64);

        // 已退款费用
        let mut after_sale_amount = 0.0;
        // 退费的运杂费
        let mut after_sale_trans_amount = 0.0;
        let after_sale_ids: Vec<_> = result_after_sale
            .unwrap()
            .iter()
            .map(|after_sale| {
                after_sale_amount += after_sale.back_money.unwrap() as f64;
                after_sale.id.unwrap()
            })
            .collect();
        // 获取运杂费
        println!("after_sale_ids: {:?}",after_sale_ids);
        let mut items = vec![];
        for id in after_sale_ids {
            let trans = order_trans_service.fetch_list_by_column("after_sale_id", &vec![id.clone().to_string()]).await;
            items.push(trans.unwrap());
        }

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

        order_pay.after_sale_amount = Some(((after_sale_amount * 100.0).round() as f64) / 100.0 as f64);
        order_pay.after_sale_trans_amount = Some(((after_sale_trans_amount * 100.0).round() as f64) / 100.0 as f64);
        println!("order_pay: {}", serde_json::to_string_pretty(&order_pay).unwrap());
        return RespVO::from(&order_pay).resp_json();
    }
    return RespVO::<i64>::from_error(&Error::E("查询失败".to_string())).resp_json();
}
// 结算新建
pub async fn get_new_order_pay(Path(order_id): Path<String>) -> impl IntoResponse {
    let order_service = APPLICATION_CONTEXT.get::<OrderService>();
    let order_product_service = APPLICATION_CONTEXT.get::<OrderProductService>();
    let order_trans_service = APPLICATION_CONTEXT.get::<OrderTransService>();
    let payed_service = APPLICATION_CONTEXT.get::<PayedInfoService>();
    let after_sale_service = APPLICATION_CONTEXT.get::<AfterSaleService>();

    let result = order_service.get(order_id.clone()).await;
    let result_order_product = order_product_service.fetch_list_by_column("order_id", &vec![order_id.clone()]).await;
    let result_order_trans = order_trans_service.fetch_list_by_column("order_id", &vec![order_id.clone()]).await;
    let result_order_pay = payed_service.fetch_list_by_column("order_id", &vec![order_id.clone()]).await;
    let result_after_sale = after_sale_service.fetch_list_by_column("order_id", &vec![order_id.clone()]).await;

    if result.is_ok() && result_order_product.is_ok() && result_order_trans.is_ok() {
        let mut order_pay = OrderPayDTO::default();

        order_pay.order_id = Some(order_id.parse::<u64>().unwrap());
        order_pay.pay_status = Some(1);
        order_pay.company_code = result.clone().unwrap().company_code.clone();
        order_pay.unit = Some("吨".to_string());
        order_pay.unit_money = Some("元".to_string());
        order_pay.buy_mode = result.clone().unwrap().buy_mode.clone();
        order_pay.supply_dept = result.clone().unwrap().supply_dept.clone();
        order_pay.delivery_dept = result.clone().unwrap().delivery_dept.clone();
        // 采购数量
        let mut buy_number: f64 = 0.0;
        let mut buy_weight: f64 = 0.0;
        let mut buy_amount: f64 = 0.0;
        let mut storage_number: f64 = 0.0;
        let mut storage_weight: f64 = 0.0;
        let mut taxt_rate: f64 = 0.0;

        result_order_product.unwrap().iter().for_each(|order_product| {
            buy_number += order_product.buy_number.unwrap() as f64;
            if order_product.clone().unit.unwrap() == "千克" {
                buy_weight += (order_product.buy_weight.unwrap() as f64) / 1000.0;
            } else {
                buy_weight += order_product.buy_weight.unwrap() as f64;
            }
            buy_amount += order_product.buy_amount.unwrap() as f64;
                taxt_rate += ((order_product.buy_amount.unwrap() as f64) * (order_product.tax_rate.unwrap() as f64)) / 100.0;

            if order_product.storage_status.unwrap() == 2 {
                // 已入库
                if order_product.clone().unit.unwrap() == "千克" {
                    storage_weight += (order_product.buy_weight.unwrap() as f64) / 1000.0;
                } else {
                    storage_weight += order_product.buy_weight.unwrap() as f64;
                }
                storage_number += order_product.buy_number.unwrap() as f64;
            }
        });
        order_pay.buy_number = Some(((buy_number * 1000.0).round() as f64) / 1000.0 as f64);
        order_pay.buy_weight = Some(((buy_weight * 1000.0).round() as f64) / 1000.0 as f64);
        order_pay.storage_number = Some(((storage_number * 1000.0).round() as f64) / 1000.0 as f64);
        order_pay.storage_weight = Some(((storage_weight * 1000.0).round() as f64) / 1000.0 as f64);
        order_pay.product_amount = Some(((buy_amount * 100.0).round() as f64) / 100.0 as f64);
        // 运杂费
        let mut trans_amount = 0.0;
        result_order_trans.unwrap().iter().for_each(|order_trans| {
            if order_trans.clone().in_out.unwrap() == 1 {
                // 支出
                trans_amount += order_trans.cost_amount.unwrap() as f64;
            }

            if order_trans.clone().in_out.unwrap() == 2 {
                // 收入
                trans_amount -= order_trans.cost_amount.unwrap() as f64;
            }
        });
        order_pay.trans_amount = Some(((trans_amount * 100.0).round() as f64) / 100.0 as f64);
        order_pay.rate_amount = Some((taxt_rate * 100.0 as f64).round() / 100.0 as f64);
        order_pay.buy_amount = Some(((order_pay.trans_amount.unwrap() + order_pay.product_amount.unwrap()) * 100.0 as f64).round() / 100.0 as f64);

        // 已支付金额
        let mut pay_amount = 0.0;
        let mut payed_amount = 0.0;
        // 支付税额
        let mut pay_tax_amount = 0.0;
        result_order_pay.unwrap().iter().for_each(|order_pay:&PayedInfoDTO | {
            pay_amount += order_pay.pay_amount.unwrap() as f64;
                payed_amount += order_pay.invoice_amount.unwrap() as f64;
                pay_tax_amount += (order_pay.invoice_amount.unwrap() as f64) * (order_pay.tax_rate.unwrap() as f64)  /100.0;
        });
        // 已支付金额
        order_pay.pay_amount = Some(((pay_amount * 100.0).round() as f64) / 100.0 as f64);
        // 已开票金额
        order_pay.payed_amount = Some(((payed_amount * 100.0).round() as f64) / 100.0 as f64);
        // 税额
        order_pay.rate_amount = Some(((pay_tax_amount * 100.0).round() as f64) / 100.0 as f64);

        // 已退款费用
        let mut after_sale_amount = 0.0;
        // 退费的运杂费
        let mut after_sale_trans_amount = 0.0;
        let after_sale_ids: Vec<_> = result_after_sale
            .unwrap()
            .iter()
            .map(|after_sale| {
                if after_sale.back_money.is_some() {
                    after_sale_amount += after_sale.back_money.unwrap() as f64;
                }
                after_sale.id.unwrap()
            })
            .collect();
        // 获取运杂费
        let mut items = vec![];
        for id in after_sale_ids {
            let trans = order_trans_service.fetch_list_by_column("after_sale_id", &vec![id.clone().to_string()]).await;
            items.push(trans.unwrap());
        }

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

        order_pay.after_sale_amount = Some(((after_sale_amount * 100.0).round() as f64) / 100.0 as f64);
        order_pay.after_sale_trans_amount = Some(((after_sale_trans_amount * 100.0).round() as f64) / 100.0 as f64);

        return RespVO::from(&order_pay).resp_json();
    }
    return RespVO::<i64>::from_error(&Error::E("查询失败".to_string())).resp_json();
}

//查询结算
pub async fn del_order_pay(Path(id): Path<String>) -> impl IntoResponse {
    let order_pay_service = APPLICATION_CONTEXT.get::<OrderPayService>();
    let result = order_pay_service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

//查询结算byUid
// pub async fn get_order_pay_byuid(Path(uid): Path<String>) -> impl IntoResponse {
//     let order_pay_service = APPLICATION_CONTEXT.get::<OrderPayService>();
//     let result = order_pay_service.fetch_list_by_column("company_code", &vec![uid]).await;
//     return RespVO::from_result(&result).resp_json();
// }

//查询结算byUid
pub async fn get_order_pay_byuid(Json(arg): Json<OrderPayFilterParams>) -> impl IntoResponse {
    let params = OrderPayParams {
        company_code: arg.company_code.clone(),
        supply_dept: arg.supply_dept.clone(),
        order_pay_start: arg.order_pay_start().clone(),
        order_pay_end: arg.order_pay_end().clone(),
        buy_mode: arg.buy_mode.clone(),
    };

    let page = PageData {
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };
    let order_pay_service = APPLICATION_CONTEXT.get::<OrderPayService>();
    let result = order_pay_service.page(&params, page).await;
    return RespVO::from_result(&result).resp_json();
}

//查询结算byUid
pub async fn get_order_pay_by_search(Json(arg): Json<OrderPayFilterParams>) -> impl IntoResponse {
    let params = OrderPayParams {
        company_code: arg.company_code.clone(),
        supply_dept: arg.supply_dept.clone(),
        order_pay_start: arg.order_pay_start.clone(),
        order_pay_end: arg.order_pay_end.clone(),
        buy_mode: arg.buy_mode.clone(),
    };

    let page = PageData {
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };
    let order_pay_service = APPLICATION_CONTEXT.get::<OrderPayService>();
    let result = order_pay_service.page(&params, page).await;
    return RespVO::from_result(&result).resp_json();
}
