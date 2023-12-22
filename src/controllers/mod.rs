use axum::{routing::delete, routing::get, routing::post, routing::put, Router};
pub mod agree_controller;
pub mod cut_controller;
pub mod finance_controller;
pub mod order_controller;
pub mod pay_controller;
pub mod sale_controller;
pub mod setting_controller;
pub mod sign_controller;
pub mod storage_controller;
pub mod user_controller;

use agree_controller::*;
use cut_controller::*;
use finance_controller::*;
use order_controller::*;
use pay_controller::*;
use sale_controller::*;
use setting_controller::*;
use sign_controller::*;
use storage_controller::*;
use user_controller::*;

pub fn init_need_auth_router() -> Router {
    Router::new()
    // .route("/stocks_1d_by_weight", post(solve_cut_optimize_by_weight))
    // .route("/stocks_1d_by_len", post(solve_cut_optimize_by_len))
    // .route("/stocks_2d_by_weight", post(solve_cut_optimize_2d_by_weight))
    // .route("/stocks_2d_by_area", post(solve_cut_optimize_2d_by_area))
}

pub fn init_noneed_setting_router() -> Router {
    // setting 设置
    Router::new()
        .route("/varity_company", post(save_company_varity).put(update_company_varity)) // 按公司保存和查询品种
        .route("/varity_by_company/:uid", get(get_company_varity))
        .route("/del_company_varity/:id", get(del_company_varity))
        .route("/get_varity", get(get_varity)) //查询全部品种
        .route("/get_varity/:id", get(get_varity_by_id)) //查询全部品种
        .route("/get_origin", get(get_origin)) //查询全部产地
        .route("/save_company_origin", post(save_company_origin).put(update_company_origin)) //按公司查询产地
        .route("/get_company_origin/:uid", get(get_company_origin)) //按公司查询产地
        .route("/get_company_origin_byid/:id", get(get_company_origin_byid)) //按公司查询产地
        .route("/del_company_origin/:id", get(del_company_origin)) //删除产地
        .route("/get_quality", get(get_quality)) //查询全部质量
        .route("/get_type", get(get_type)) //查询全部类型
        .route("/get_paymode", get(get_paymode)) //查询全部支付方式
        .route("/get_company_paymode", get(get_company_paymode)) //按公司查询支付方式
        .route("/save_company_paymode", post(save_company_paymode))
        .route("/del_company_paymode/:id", get(del_company_paymode)) //删除支付方式
        .route("/get_role", get(get_role)) //查询全部角色
        .route("/get_storage", get(get_storage)) //查询仓库
        .route("/get_storage/:id", get(get_storage_by_id)) //  //查询仓库
        .route("/get_company_storage/:uid", get(get_company_storage)) //按公司查询仓库
        .route("/get_company_storage_by_id/:id", get(get_company_storage_by_id)) //按公司查询仓库
        .route("/del_company_storage/:id", get(del_company_storage)) //删除仓库
        .route("/save_company_storage", post(save_company_storage).put(update_company_storage)) //保存公司仓库
        .route("/save_company_supply", post(save_company_supply).put(update_company_supply)) //保存公司供应商
        .route("/del_company_supply/:id", get(del_company_supply)) //删除公司供应商
        .route("/get_company_supply/:uid", get(get_company_supply)) //查询公司供应商
        .route("/get_company_supply_byid/:id", get(get_company_supply_byid))
        .route("/get_notice", get(get_notice)) //查询公告
        .route("/save_notice", post(save_notice)) //保存公告
        .route("/del_notice/:id", get(del_notice)) //删除公告
        .route("/save_fack", post(save_fack)) //保存反馈
        .route("/company", post(save_company).put(update_company))
        .route("/company/:id", get(get_company))
        .route("/company_client", post(save_company_client).put(update_company_client))
        .route("/company_client/:id", get(get_company_client_byid).delete(del_company_client))
        .route("/company_client_uid/:uid", get(get_company_client))
        .route("/company_spec_uid/:uid", get(get_company_sepc))
        .route("/save_spec", post(save_sepc).put(update_sepc))
        .route("/get_spec/:id", get(get_company_sepc_byid).delete(del_sepc))
        .route("/company_shop_uid/:uid", get(get_company_shop_sign))
        .route("/save_shop", post(save_shop_sign).put(update_shop_sign))
        .route("/get_shop/:id", get(get_company_shop_byid).delete(del_shop_sign))
}

pub fn init_noneed_agree_router() -> Router {
    // 协议
    Router::new()
        .route("/agree", post(save_agree).put(update_agree))
        .route("/agree_by_uid", post(get_agree_by_uid))
        .route("/agree/:id", delete(del_agree).get(get_agree))
        .route("/agree_product", post(save_agree_product).put(update_agree_product))
        .route("/agree_product_agree_id/:id", get(get_agree_product_by_agreeid))
        .route("/agree_product_sale_id/:id", get(get_agree_product_by_saleid))
        .route("/agree_product/:id", delete(del_agree_product).get(get_agree_product))
        .route("/get_agree_by_filter", post(get_agree_by_filter))
}

pub fn init_noneed_order_router() -> Router {
    // 订单
    Router::new()
        // 订单
        .route("/order", post(save_order).put(update_order))
        .route("/order/:id", get(get_order).delete(del_order))
        .route("/order_by_uid", post(get_order_by_uid))
        .route("/order_by_filter", post(get_order_by_filter))
        // 订单产品
        .route("/order_product", post(save_order_product).put(update_order_product))
        .route("/order_product/:id", delete(del_order_product).get(get_order_product_byid))
        .route("/order_product_by_uid/:uid", get(get_order_product))
        .route("/in_storage_by_order/:id", get(in_storage_by_order))
        .route("/in_storage_by_agree/:id", get(in_storage_by_agree))
        .route("/order_product_by_agree_id/:agreeid", get(order_product_by_agree_id))
        .route("/product_by_filter", post(order_product_by_filter))
        .route("/in_storage_by_select_product", post(in_storage_by_select_product))

        // 订单运杂费
        .route("/order_trans", post(save_order_trans).put(update_order_trans))
        .route("/order_trans/:id", delete(del_order_trans).get(get_order_trans_byid))
        .route("/order_trans_by_uid/:uid", get(get_order_trans))
        .route("/order_trans_by_agree_id/:uid", get(get_order_trans_by_agree_id))
        .route("/order_trans_by_adjust_id/:uid", get(get_order_trans_by_adjust_id))
        .route("/order_trans_by_process_id/:uid", get(get_order_trans_by_process_id))
        .route("/order_trans_by_sale_id/:uid", get(get_order_trans_by_sale_id))
        .route("/order_trans_by_after_sale_id/:uid", get(get_order_trans_by_after_sale_id))
        .route("/order_trans_by_buy_after_sale_id/:uid", get(order_trans_by_buy_after_sale_id))
        // 订单结算
        .route("/cacl_order_pay/:order_id", get(cacl_order_pay))
        .route("/cacl_agree_pay/:agree_id", get(cacl_agree_pay))
        .route("/order_pay", post(save_order_pay).put(update_order_pay))
        .route("/order_pay/:id", delete(del_order_pay).get(get_order_pay))
        .route("/new_order_pay/:order_id", get(get_new_order_pay))
        .route("/new_order_pay_by_agree/:agree_id", get(get_new_order_pay_by_agree))
        .route("/order_pay_by_uid", post(get_order_pay_byuid))
        //   售后
        .route("/save_aftersale", post(save_order_aftersale).put(update_order_aftersale))
        .route("/save_aftersale/:id", get(get_order_aftersale).delete(del_order_aftersale))
        .route("/save_aftersale_byuid/:uid", get(get_order_aftersale_byuid))
        .route("/save_aftersale_byuid", post(get_order_aftersale_byfilter))
        .route("/save_aftersale_by_agree_id/:id", get(get_order_aftersale_by_agree_id))
        .route("/save_aftersale_by_order_id/:id", get(get_order_aftersale_by_order_id))
        .route("/save_aftersale_product_byuid/:uid", get(get_order_aftersale_by_pid))
        .route("/save_aftersale_product", post(save_order_aftersale_product).put(update_order_aftersale_product))
        .route("/save_aftersale_product/:id", delete(del_order_aftersale_product))
}

pub fn init_noneed_tool_router() -> Router {
    Router::new()
        // 工具
        .route("/stocks_1d_by_weight", post(solve_cut_optimize_by_weight))
        .route("/stocks_1d_by_len", post(solve_cut_optimize_by_len))
        .route("/stocks_2d_by_weight", post(solve_cut_optimize_2d_by_weight))
        .route("/stocks_2d_by_area", post(solve_cut_optimize_2d_by_area))
}

pub fn init_noneed_user_router() -> Router {
    Router::new()
        // 人员
        .route("/user", post(save_user).put(update_user))
        .route("/user/:id", get(get_user_by_id).delete(del_user))
        .route("/user_by_com/:uid", get(get_user_by_com))
        .route("/user_by_phone/:phone", get(get_user_by_phone))
        .route("/weichat_user", post(save_weichat_user).put(update_weichat_user))
        .route("/weichat_user/:id", get(get_weichatuser_by_id).delete(del_weichat_user))
}

pub fn init_noneed_login_router() -> Router {
    Router::new()
        // 人员
        .route("/login", post(user_login_phone))
}

pub fn init_noneed_storage_router() -> Router {
    Router::new()
        .route("/storage_product", post(save_storage_product).put(update_storage_product))
        .route("/get_storage_product_by_variety", post(get_storage_product_by_variety))
        .route("/get_storage_product_by_warehouse", post(get_storage_product_by_variety))
        .route("/get_storage_product_list_by_cate", post(get_storage_product_list_by_cate))
        .route("/get_storage_product_list_by_page", post(get_storage_product_list_by_page))
        // 库存调整
        .route("/storage_adjust", post(save_storage_adjust).put(update_storage_adjust))
        .route("/storage_adjust/:id", get(get_storage_adjust).delete(del_storage_adjust))
        .route("/storage_adjust_by_uid/:id", get(get_storage_adjust_by_uid))
        // 库存调整
        .route("/storage_adjust_product", post(save_storage_adjust_product).put(update_storage_adjust_product))
        .route("/storage_adjust_product/:id", get(get_storage_adjust_product).delete(del_storage_adjust_product))
        .route("/storage_adjust_product_by_uid/:id", get(get_storage_adjust_product_by_uid))
        .route("/storage_adjust_product_by_adjust_id/:id", get(get_storage_adjust_product_by_adjust_id))
        .route("/adjust_product_by_adjust_id/:id", get(get_product_by_adjust_id))
        .route("/get_storage_product_adjust_by_page", post(get_storage_product_adjust_by_page))
        // 加工
        .route("/get_process_by_page", post(get_process_by_page))
        .route("/process", post(save_process).put(update_process))
        .route("/process/:id", get(get_process).delete(del_process))
        // 加工产品
        .route("/get_process_product_by_process_id/:id", get(get_process_product_by_process_id))
        .route("/get_processd_product_by_process_id/:id", get(get_processd_product_by_process_id))
        .route("/get_process_product_by_storage_id/:id", get(get_process_product_by_storage_id))
        .route("/process_product", post(save_process_product).put(update_process_product))
        .route("/process_product/:id", get(get_process_product).delete(del_process_product))
        // 切割
        .route("/solver_1d_number", post(solve_cut_optimize_by_len))
        .route("/solver_1d_weight", post(solve_cut_optimize_by_weight))
        .route("/solver_2d_weight", post(solve_cut_optimize_2d_by_weight))
        .route("/solver_2d_area", post(solve_cut_optimize_2d_by_area))
        .route("/process_solution", post(save_process_solution).put(update_process_solution))
        .route("/process_solution/:id", get(get_process_solution).delete(del_process_solution))
        .route("/process_solution_by_process_id/:id", get(get_process_solution_by_process_id))
        // 库存日志
        .route("/search_storage_log", post(search_storage_log))
}

pub fn init_noneed_sale_router() -> Router {
    Router::new()
        // 销售
        .route("/sale_info", post(save_sale_info).put(update_sale_info))
        .route("/sale_info/:id", get(get_sale_info).delete(del_sale_info))
        .route("/get_sale_info_by_filter", post(get_sale_info_by_filter))
        .route("/get_recieved_info_by_filter", post(get_recieved_info_by_filter))
        .route("/out_storage_by_sale_id/:id", get(out_storage_by_sale_id))
        // 销售产品
        .route("/sale_product", post(save_sale_product).put(update_sale_product))
        .route("/sale_product/:id", get(get_sale_product).delete(del_sale_product))
        .route("/get_sale_product_by_sale_id/:id", get(get_sale_product_by_sale_id))
        .route("/get_sale_product_by_after_sale_id/:id", get(get_sale_product_by_after_sale_id))
        .route("/get_sale_product_by_page", post(get_sale_product_by_page))
        .route("/sale_product_back_storage/:id", get(sale_product_back_storage))
        // 预售产品
        .route("/pre_sale_product", post(save_pre_sale_product).put(update_pre_sale_product))
        .route("/pre_sale_product/:id", get(get_pre_sale_product).delete(del_pre_sale_product))
        .route("/get_pre_sale_product_by_sale_id/:id", get(get_pre_sale_product_by_sale_id))
        // 销售结算
        .route("/save_pay", post(save_sale_pay).put(update_sale_pay))
        .route("/save_pay/:id", get(get_sale_pay).delete(del_sale_pay))
        .route("/get_sale_pay_by_filter", post(get_sale_pay_by_filter))
        .route("/get_new_sale_pay/:id", get(get_new_sale_pay))
        .route("/cacl_sale_pay/:id", get(cacl_sale_pay))
        // 销售售后
        .route("/sale_after", post(save_sale_after).put(update_sale_after))
        .route("/sale_after/:id", get(get_sale_after).delete(del_sale_after))
        .route("/get_sale_after_by_filter", post(get_sale_after_by_filter))
        .route("/get_sale_after_by_sale_id/:id", get(get_sale_after_by_sale_id))
}

pub fn init_noneed_finance_router() -> Router {
    Router::new()
        // 财务
        .route("/payed_info", post(save_payed_info).put(update_payed_info))
        .route("/payed_info/:id", get(get_payed_info).delete(del_payed_info))
        .route("/get_payed_info_by_filter", post(get_payed_info_by_filter))
        .route("/get_payed_info_by_agree_id/:id", get(get_payed_info_by_agree_id))
        .route("/get_payed_info_by_order_id/:id", get(get_payed_info_by_order_id))
        // 收款
        .route("/received_info", post(save_receieved_info).put(update_receieved_info))
        .route("/received_info/:id", get(get_receieved_info).delete(del_receieved_info))
        .route("/get_receieved_info_by_filter", post(get_receieved_info_by_filter))
        .route("/get_recieved_by_sale_id/:id", get(get_recieved_by_sale_id))

        // 费用
        .route("/get_transcost_by_filter", post(get_transcode_by_filter))

        // 统计
        .route("/get_trans_by_staticstic", post(statistics_trans))
        .route("/get_payed_by_staticstic", post(statistics_payed))
        .route("/get_recieve_by_staticstic", post(statistics_recieve))
        .route("/get_buyed_by_staticstic", post(statistics_buyed))
        .route("/get_buyed_by_staticstic_order", post(statistics_buyed_order))
        .route("/get_sale_by_staticstic", post(statistics_sale_order))

        // 发票
        .route("/tax_info", post(save_tax_info).put(update_tax_info))
        .route("/tax_info/:id", get(get_tax_info).delete(del_tax_info))
        .route("/get_tax_by_payed_id/:id", get(get_tax_by_payed_id))
        .route("/get_tax_by_recieved_id/:id", get(get_tax_by_recieved_id))
        .route("/get_tax_by_trans_id/:id", get(get_tax_by_trans_id))

}
