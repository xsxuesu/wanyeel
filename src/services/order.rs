use crate::crud::crud_service::CrudService;
use crate::APPLICATION_CONTEXT;
use rbatis::crud::CRUDMut;
use rbatis::crud::{CRUDTable, Skip, CRUD};
use rbatis::rbatis::Rbatis;
use rbson::Bson;
use wy_domain::dto::agree::OrderByFilterDTO;
use wy_domain::dto::order::{
    OrderAfterSaleDTO, OrderAfterSaleParams, OrderAfterSaleProductDTO, OrderDTO, OrderParams, OrderProductDTO, OrderProductForSelectFilterParams, OrderTransDTO, TransCodeFilterParams,
};
use wy_domain::dto::sale::SaleInfoDTO;
use wy_domain::entity::order::{Order, OrderAfterSale, OrderAfterSaleProduct, OrderProduct, OrderTrans};
use wy_domain::entity::storage::StorageProduct;
use wy_domain::entity::CommonField;
use wy_domain::error::Result as WyResult;
use wy_domain::request::{ByComQuery, ByUIDQuery};
// OrderService
pub struct OrderService;
impl Default for OrderService {
    fn default() -> Self {
        OrderService {}
    }
}

impl OrderService {
    pub async fn update_order_key(&self, order_id: u64, pay_id: u64) -> WyResult<u64> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();

        let pay_id = as_bson!(pay_id);
        let order_id = as_bson!(order_id);
        let res = rb.exec("update order_info set pay_id =? where id =?", vec![pay_id, order_id]).await;
        Ok(res.unwrap().rows_affected)
    }

    pub async fn get_order_by_filter(&self, arg: OrderByFilterDTO) -> WyResult<Vec<OrderDTO>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();

        let mut query = format!("SELECT * FROM order_info WHERE order_status = 2 and company_code = '{}' ", arg.company_code.clone().unwrap());

        if arg.order_start.is_some() {
            let tmp = format!(" and pay_date >= '{}' ", arg.order_start.clone().unwrap());
            query = format!("{} {}", query, tmp)
        }
        if arg.order_end.is_some() {
            let tmp = format!(" and pay_date <= '{}' ", arg.order_end.clone().unwrap());
            query = format!("{} {}", query, tmp)
        }

        let company_code_arg = as_bson!(arg.company_code.clone().unwrap());
        // let start_arg = as_bson!(arg.order_start.clone().unwrap());
        // let end_arg = as_bson!(arg.order_end.clone().unwrap());
        let agree_list: Vec<OrderDTO> = rb.fetch(query.as_str(), vec![company_code_arg]).await.unwrap();

        Ok(agree_list)
    }
    // 协议入库
    pub async fn agree_trans_to_storage_product(&self, agree_id: String) -> WyResult<()> {
        let order_product_service = APPLICATION_CONTEXT.get::<OrderProductService>();
        let result = order_product_service.fetch_list_by_column("agree_id", &vec![agree_id.clone()]).await;
        // 将查询到的订单产品转为入库产品 并根据入库状态 storage_status = 1
        let mut storage_product_list = vec![];
        let mut order_product_list = vec![];
        for item in result.unwrap().into_iter() {
            if item.storage_status.unwrap() != 1 {
                continue;
            }
            order_product_list.push(item.clone()); //订单产品列表
            let mut storage_product: StorageProduct = item.clone().into();
            match agree_id.clone().parse::<u64>() {
                Ok(v) => {
                    storage_product.agree_id = Some(v);
                }
                Err(e) => {
                    println!("订单转换err:{:?}", e);
                }
            }
            storage_product.product_id = item.id;
            storage_product_list.push(storage_product);
        }

        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut tx = rb.acquire_begin().await?.defer_async(|mut tx1| async move {
            if !tx1.is_done() {
                tx1.rollback().await;
                println!("tx rollback success!");
                log::error!("tx rollback success!");
            } else {
                println!("don't need rollback!");
            }
        });
        // 批量保存入库产品

        let res = tx
            .save_batch(&mut storage_product_list, &[Skip::Column("id"), Skip::Column("created_at"), Skip::Column("updated_at")])
            .await;
        // 更新订单的产品
        let updata_product_order = format!("update order_product set storage_status = 2 WHERE agree_id = {};", agree_id.clone());
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(updata_product_order.as_str()).await;

        // 更新订单状态
        // let updata_order = format!("update order_info set order_status = 2 WHERE id = {};", agree_id.clone());
        // let res: Result<(i64, usize), rbatis::Error> = tx.fetch(updata_order.as_str()).await;

        let res = tx.commit().await.unwrap();
        return Ok(());
    }
    // 订单入库
    pub async fn trans_to_storage_product(&self, order_id: String) -> WyResult<()> {
        let order_product_service = APPLICATION_CONTEXT.get::<OrderProductService>();
        let result = order_product_service.fetch_list_by_column("order_id", &vec![order_id.clone()]).await;
        // 将查询到的订单产品转为入库产品 并根据入库状态 storage_status = 1
        let mut storage_product_list = vec![];
        let mut order_product_list = vec![];
        for item in result.unwrap().into_iter() {
            if item.storage_status.unwrap() != 1 {
                continue;
            }
            order_product_list.push(item.clone()); //订单产品列表
            let mut storage_product: StorageProduct = item.clone().into();
            match order_id.clone().parse::<u64>() {
                Ok(v) => {
                    storage_product.order_id = Some(v);
                }
                Err(e) => {
                    println!("订单转换err:{:?}", e);
                }
            }
            storage_product.product_id = item.id;
            storage_product_list.push(storage_product);
        }

        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut tx = rb.acquire_begin().await?.defer_async(|mut tx1| async move {
            if !tx1.is_done() {
                tx1.rollback().await;
                println!("tx rollback success!");
                log::error!("tx rollback success!");
            } else {
                println!("don't need rollback!");
            }
        });
        // 批量保存入库产品

        let res = tx
            .save_batch(&mut storage_product_list, &[Skip::Column("id"), Skip::Column("created_at"), Skip::Column("updated_at")])
            .await;
        // 更新订单的产品
        let updata_product_order = format!("update order_product set storage_status = 2 WHERE order_id = {};", order_id.clone());
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(updata_product_order.as_str()).await;

        // 更新订单状态
        let updata_order = format!("update order_info set order_status = 2 WHERE id = {};", order_id.clone());
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(updata_order.as_str()).await;

        let res = tx.commit().await.unwrap();
        return Ok(());
    }
    // 按照选择入库
    pub async fn trans_to_storage_product_by_select(&self, arg_list:Vec<OrderProductDTO>)->WyResult<()>{

        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut tx = rb.acquire_begin().await?.defer_async(|mut tx1| async move {
            if !tx1.is_done() {
                tx1.rollback().await;
                println!("tx rollback success!");
                log::error!("tx rollback success!");
            } else {
                println!("don't need rollback!");
            }
        });

        // 将查询到的订单产品转为入库产品 并根据入库状态 storage_status = 1
        let mut storage_product_list = vec![];
        for item in arg_list.into_iter() {
            if item.storage_status.unwrap() != 1 {
                continue;
            }
            let mut storage_product: StorageProduct = item.clone().into();
            storage_product.order_id = item.order_id;
            storage_product.agree_id = item.agree_id;
            storage_product.product_id = item.id;
            storage_product_list.push(storage_product);
            // // 更新订单的产品
        let updata_product_order = format!("update order_product set storage_status = 2 WHERE id = {};", item.id.unwrap());
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(updata_product_order.as_str()).await;
        }
        // 批量保存入库产品
        let res = tx
            .save_batch(&mut storage_product_list, &[Skip::Column("id"), Skip::Column("created_at"), Skip::Column("updated_at")])
            .await;

        let res = tx.commit().await.unwrap();
        return Ok(());
    }
}

impl CrudService<Order, OrderDTO, OrderParams> for OrderService {
    fn get_wrapper(arg: &OrderParams) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
            .do_if(arg.company_code().is_some(), |w| w.eq("company_code", arg.company_code().clone().unwrap()))
            .do_if(arg.supply_dept().is_some(), |w| w.eq("supply_dept", arg.supply_dept().clone().unwrap()))
            .do_if(arg.delivery_dept().is_some(), |w| w.eq("delivery_dept", arg.delivery_dept().clone().unwrap()))
            .do_if(arg.order_status().is_some(), |w| w.eq("order_status", arg.order_status().clone().unwrap()))
            .do_if(arg.order_start().is_some(), |w| w.ge("buy_date", arg.order_start().clone().unwrap()))
            .do_if(arg.order_end().is_some(), |w| w.le("buy_date", arg.order_end().clone().unwrap()))
            .do_if(arg.buy_mode().is_some(), |w| w.eq("buy_mode", arg.buy_mode().clone().unwrap()))
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut Order) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// OrderProductService
pub struct OrderProductService;
impl Default for OrderProductService {
    fn default() -> Self {
        OrderProductService {}
    }
}

impl OrderProductService {
    pub async fn update_storage_by_id(&self, ids: Vec<u64>) -> WyResult<u64> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut ids_boson = vec![];
        for id in ids.iter() {
            ids_boson.push(as_bson!(id));
        }
        let res = rb.exec("update order_product set storage_status = 2 WHERE id = ?;", ids_boson).await;
        return Ok(res.unwrap().rows_affected);
    }
}

impl CrudService<OrderProduct, OrderProductDTO, OrderProductForSelectFilterParams> for OrderProductService {
    fn get_wrapper(arg: &OrderProductForSelectFilterParams) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        println!("OrderProductForSelectFilterParams: {:?}", arg);
        rb.new_wrapper()
            .do_if(arg.company_code().is_some(), |w| w.eq("company_code", arg.company_code().clone().unwrap()))
            .do_if(arg.agree_id().unwrap() > 0, |w| w.eq("agree_id", arg.agree_id().clone().unwrap()))
            .do_if(arg.order_id().unwrap() > 0, |w| w.eq("order_id", arg.order_id().clone().unwrap()))
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut OrderProduct) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// OrderTransService
pub struct OrderTransService;
impl Default for OrderTransService {
    fn default() -> Self {
        OrderTransService {}
    }
}
// pub payed_date_start: Option<String>,
// pub payed_date_end: Option<String>,
// pub company_code: Option<String>,
// pub client_name: Option<String>,
impl CrudService<OrderTrans, OrderTransDTO, TransCodeFilterParams> for OrderTransService {
    fn get_wrapper(arg: &TransCodeFilterParams) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
            .do_if(arg.company_code().is_some(), |w| w.eq("company_code", arg.company_code().clone().unwrap()))
            .do_if(arg.client_name().is_some(), |w| w.eq("supply_dept", arg.client_name().clone().unwrap()))
            .do_if(arg.payed_date_start().is_some(), |w| w.ge("cost_date", arg.payed_date_start().clone().unwrap()))
            .do_if(arg.payed_date_end().is_some(), |w| w.le("cost_date", arg.payed_date_end().clone().unwrap()))
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut OrderTrans) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// AfterSaleService
pub struct AfterSaleService;
impl Default for AfterSaleService {
    fn default() -> Self {
        AfterSaleService {}
    }
}

struct TmpItem {
    id: u64,
    back_number: f64,
    back_weight: f64,
}

impl AfterSaleService {
    pub async fn after_sale_update_storage(&self, after_sale_id: u64) -> WyResult<()> {
         // 查询售后产品
         let after_sale_product_service = APPLICATION_CONTEXT.get::<AfterSaleProductService>();
         let res = after_sale_product_service.fetch_list_by_column("after_sale_id", &vec![after_sale_id.clone().to_string()]).await?;
         if res.is_empty() {
             return Ok(());
         }

        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut tx = rb.acquire_begin().await?.defer_async(|mut tx1| async move {
            if !tx1.is_done() {
                tx1.rollback().await;
                log::error!("tx rollback success!");
            } else {
                println!("don't need rollback!");
            }
        });


        let mut ids = vec![];
        let mut back_items = vec![];
        for item in res {
            ids.push(item.id.clone().unwrap().to_string());
            let item_tmp = TmpItem {
                id: item.order_prodct_id.clone().unwrap(),
                back_number: item.back_number.clone().unwrap(),
                back_weight: item.back_weight.clone().unwrap(),
            };
            back_items.push(item_tmp);
        }

        // 更新售后服务的产品状态
        let updata_after_sale_products_sql = format!("update order_after_sale_product set after_sale_status = 2 WHERE id in ({});", ids.join(","));
        println!("updata_after_sale_products_sql: {}", updata_after_sale_products_sql);
        let res = tx.exec_sql(updata_after_sale_products_sql.as_str()).await;

        //修改库存
        for back_item in back_items {
            let updata_after_sale_products = format!(
                "update storage_product set storage_number = storage_number - {}, storage_weight = storage_weight - {} where product_id = {};",
                back_item.back_number, back_item.back_weight, back_item.id
            );
            tx.exec_sql(updata_after_sale_products.as_str()).await.unwrap();
        }
        let res = tx.commit().await.unwrap();

        Ok(())
    }
}

impl CrudService<OrderAfterSale, OrderAfterSaleDTO, OrderAfterSaleParams> for AfterSaleService {
    fn get_wrapper(arg: &OrderAfterSaleParams) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
            .do_if(arg.company_code().is_some(), |w| w.eq("company_code", arg.company_code().clone().unwrap()))
            .do_if(arg.supply_dept().is_some(), |w| w.eq("supply_dept", arg.supply_dept().clone().unwrap()))
            .do_if(arg.after_sale_start().is_some(), |w| w.ge("back_date", arg.after_sale_start().clone().unwrap()))
            .do_if(arg.after_sale_end().is_some(), |w| w.le("back_date", arg.after_sale_end().clone().unwrap()))
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut OrderAfterSale) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// AfterSaleProductService
pub struct AfterSaleProductService;
impl Default for AfterSaleProductService {
    fn default() -> Self {
        AfterSaleProductService {}
    }
}
impl CrudService<OrderAfterSaleProduct, OrderAfterSaleProductDTO, ByUIDQuery> for AfterSaleProductService {
    fn get_wrapper(arg: &ByUIDQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut OrderAfterSaleProduct) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}
