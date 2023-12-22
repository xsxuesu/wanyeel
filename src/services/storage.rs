use crate::crud::crud_service::CrudService;
use crate::APPLICATION_CONTEXT;
use chrono::NaiveDate;
use rbatis::DateNative;
use rbatis::crud::{CRUDMut, CRUDTable, Skip, CRUD};
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};
use wy_domain::dto::storage::{StorageListDTO, ProcessSolutionDTO, StorageLogDTO,StorageLog};
use wy_domain::dto::storage::{ProcessDTO, ProcessPageDTO, ProcessProductDTO, StorageAdjustDTO, StorageAdjustPageDTO, StorageAdjustProductDTO, StorageFilterParams, StorageProductDTO};
use wy_domain::entity::storage::{Process, ProcessProduct, StorageAdjust, StorageAdjustProduct, StorageProduct, ProcessSolution};
use wy_domain::entity::CommonField;
use wy_domain::error::Result as WyResult;
use wy_domain::request::{ByComQuery, ByProcessProductCateQuery, ByUIDQuery};
use super::order::OrderProductService;
pub struct StorageService;
impl Default for StorageService {
    fn default() -> Self {
        StorageService {}
    }
}


impl StorageService {
    pub async fn get_storage_by_cate(&self, arg: StorageFilterParams) -> WyResult<Vec<StorageListDTO>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut  sql_pre = "SELECT company_code, warehouse, variety , origin , spec, sum(storage_number) as storage_number,  SUM(case when unit = '千克' then storage_weight / 1000 else storage_weight end) as storage_weight, (case when unit = '千克' then '吨' else unit end ) unit  FROM storage_product WHERE company_code = ?  and storage_status =1 and storage_cate = 1 and storage_number > 0 and storage_weight > 0".to_string();
        let company_bson = as_bson!(arg.company_code.clone());
        let mut where_list = vec![company_bson];
        if arg.warehouse.is_some() {
            sql_pre = format!("{} and warehouse =? ", sql_pre);
            where_list.push(as_bson!(arg.warehouse.clone()))
        }
        if arg.variety.is_some() {
            sql_pre = format!("{} and variety =? ", sql_pre);
            where_list.push(as_bson!(arg.variety.clone()));
        }
        sql_pre = format!("{} GROUP BY warehouse,variety,spec; ", sql_pre);

        let res: Vec<StorageListDTO> = rb.fetch(sql_pre.as_str(), where_list).await.unwrap();
        return Ok(res);
    }

    pub async fn get_storage_list_by_cate(&self, arg: StorageListDTO) -> WyResult<Vec<StorageProduct>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let company_bson = as_bson!(arg.company_code.clone());
        let variety_bson = as_bson!(arg.variety.clone());
        let warehouse_bson = as_bson!(arg.warehouse.clone());
        let origin_bson = as_bson!(arg.origin.clone());
        let res: Vec<StorageProduct> = rb
            .fetch(
                "SELECT * FROM storage_product WHERE company_code = ?  and storage_status =1 and storage_cate = 1 and variety = ? and warehouse=? and origin=? ;",
                vec![company_bson, variety_bson, warehouse_bson, origin_bson],
            )
            .await
            .unwrap();
        return Ok(res);
    }


    // 库存流水
    pub async fn search_storage_log(&self,arg: StorageLogDTO) -> WyResult<Vec<StorageLog>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();

        let mut warehouse_query = "".to_string();

        if arg.warehouse.is_some() {
            warehouse_query = format!(" and warehouse = '{}' ", arg.warehouse.clone().unwrap());
        }

        let mut warehouse_query_a = "".to_string();

        if arg.warehouse.is_some() {
            warehouse_query_a = format!(" and (b.in_warehouse = '{}' or b.out_warehouse = '{}')", arg.warehouse.clone().unwrap(),arg.warehouse.clone().unwrap());
        }


        let mut date_query = "".to_string();
        if arg.search_start.is_some() && arg.search_end.is_some() {
            date_query = format!(
                r#" and created_at between '{}' and '{}' "#,
                arg.search_start.clone().unwrap(),
                arg.search_end.clone().unwrap()
            );
        } else {
            if arg.search_end.is_some() {
                date_query = format!(r#" and created_at <= '{}' "#, arg.search_end.clone().unwrap());
            }
            if arg.search_start.is_some() {
                date_query = format!(r#" and created_at >= '{}' "#, arg.search_start.clone().unwrap());
            }
        }

        let mut date_query_a = "".to_string();
        if arg.search_start.is_some() && arg.search_end.is_some() {
            date_query_a = format!(
                r#" and a.created_at between '{}' and '{}' "#,
                arg.search_start.clone().unwrap(),
                arg.search_end.clone().unwrap()
            );
        } else {
            if arg.search_end.is_some() {
                date_query_a = format!(r#" and a.created_at <= '{}' "#, arg.search_end.unwrap());
            }
            if arg.search_start.is_some() {
                date_query_a = format!(r#" and a.created_at >= '{}' "#, arg.search_start.unwrap());
            }
        }

        let query = format!(
            r#"

            SELECT '采购入库' as operation, warehouse,variety,shop_sign,spec,buy_number as op_number,buy_weight as op_weight,created_at
            FROM order_product WHERE storage_status =  2 {} {}  and company_code = '{}'

            UNION ALL

            SELECT '销售出库' as operation, warehouse,variety,shop_sign,spec,sale_number as op_number,sale_weight as op_weight,created_at
            FROM sale_product WHERE sale_status =  2 {} {} and company_code = '{}'

            UNION ALL

            SELECT
            CASE WHEN b.adjust_cate = '出库' THEN '库存调整出库' ELSE CASE WHEN b.adjust_cate = '入库' THEN '库存调整入库' ELSE '库存调整调拨' END END as operation,
            CASE WHEN b.adjust_cate = '出库' THEN b.out_warehouse ELSE CASE WHEN b.adjust_cate = '入库' THEN b.in_warehouse ELSE CONCAT(b.out_warehouse , '->' ,b.in_warehouse) END END as warehouse,
            a.variety as variety ,a.shop_sign as shop_sign,a.spec as spec,a.adjust_number as op_number,a.adjust_weight as op_weight,a.created_at
            FROM storage_adjust_product as a  LEFT JOIN storage_adjust as b on a.adjust_id = b.id  WHERE a.change_status =  2  {} {} and a.company_code = '{}'

            UNION ALL

            SELECT '退货入库'  as operation, warehouse,variety,shop_sign,spec,back_number as op_number,back_weight as op_weight,created_at
            FROM sale_product WHERE back_status =  2 {} {} and company_code = '{}'

            UNION ALL

            SELECT '加工出库' as operation, warehouse,variety,shop_sign,spec,process_number as op_number,process_weight as op_weight,created_at
            FROM process_product WHERE process_cate =  2 {} {} and company_code = '{}'

            UNION ALL

            SELECT '退货出库' as operation, warehouse,variety,shop_sign,spec,back_number as op_number,back_weight as op_weight,created_at
            FROM order_after_sale_product WHERE after_sale_status =  2 {} {} and company_code = '{}'

        "#,
            date_query,
            warehouse_query,
            arg.company_code.clone().unwrap(),
            date_query,
            warehouse_query,
            arg.company_code.clone().unwrap(),
            date_query_a,
            warehouse_query_a,
            arg.company_code.clone().unwrap(),
            date_query,
            warehouse_query,
            arg.company_code.clone().unwrap(),
            date_query,
            warehouse_query,
            arg.company_code.clone().unwrap(),
            date_query,
            warehouse_query,
            arg.company_code.clone().unwrap()
        );
        let company_code_arg = as_bson!(arg.company_code.unwrap().to_string());
        let order_list: Vec<StorageLog> = rb.fetch(query.as_str(), vec![company_code_arg]).await.unwrap();

        Ok(order_list)
    }
}

impl CrudService<StorageProduct, StorageProductDTO, StorageListDTO> for StorageService {
    fn get_wrapper(arg: &StorageListDTO) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
            .gt("storage_weight", 0)
            .do_if(arg.company_code().is_some(), |w| w.eq("company_code", arg.company_code().clone().unwrap()))
            .do_if(arg.warehouse().is_some(), |w| w.eq("warehouse", arg.warehouse().clone().unwrap()))
            .do_if(arg.variety().is_some(), |w| w.eq("variety", arg.variety().clone().unwrap()))
            .do_if(arg.origin().is_some(), |w| w.eq("origin", arg.origin().clone().unwrap()))
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut StorageProduct) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

pub struct StorageAdjustService;
impl Default for StorageAdjustService {
    fn default() -> Self {
        StorageAdjustService {}
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StorageForReduce {
    pub storage_id: u64,
    pub adjust_number: f64,
    pub adjust_weight: f64,
}

impl StorageAdjustService {
    pub async fn delete_adjust_relations(&self, adjust_id: i64) -> WyResult<()> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut tx = rb.acquire_begin().await?.defer_async(|mut tx1| async move {
            if !tx1.is_done() {
                tx1.rollback().await;
                log::error!("tx rollback success!");
            } else {
                println!("don't need rollback!");
            }
        });
        // 删除状态
        let delete_trans_order = format!("delete from order_trans  WHERE adjust_id = {};", adjust_id);
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(delete_trans_order.as_str()).await;
        let delete_order_product = format!("delete from order_product  WHERE adjust_id = {};", adjust_id);
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(delete_order_product.as_str()).await;
        let delete_adjust_product = format!("delete from storage_adjust_product WHERE adjust_id = {};", adjust_id);
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(delete_adjust_product.as_str()).await;
        // 执行commit
        let res = tx.commit().await.unwrap();
        return Ok(());
    }

    pub async fn trans_adjust_relations(&self, adjust: StorageAdjust) -> WyResult<()> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut tx = rb.acquire_begin().await?.defer_async(|mut tx1| async move {
            if !tx1.is_done() {
                tx1.rollback().await;
                log::error!("tx rollback success!");
            } else {
                println!("don't need rollback!");
            }
        });
        // 修改状态
        let update_trans_order = format!("update order_trans set trans_status = 2 WHERE adjust_id = {};", adjust.id.unwrap());
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_trans_order.as_str()).await;
        let update_pay_order = format!("update order_product set storage_status = 2 WHERE adjust_id = {};", adjust.id.unwrap());
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_pay_order.as_str()).await;
        let update_adjust_product = format!("update storage_adjust_product set change_status = 2 WHERE adjust_id = {};", adjust.id.unwrap());
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_adjust_product.as_str()).await;

        // 将调整的产品调入仓库
        if adjust.adjust_cate.clone().unwrap() == "入库".to_string() {
            // 保存库存产品
            let order_product_service = APPLICATION_CONTEXT.get::<OrderProductService>();
            let result = order_product_service.fetch_list_by_column("adjust_id", &vec![adjust.id.unwrap().to_string()]).await;
            // 将查询到的订单产品转为入库产品 并根据入库状态 storage_status = 1
            let mut storage_product_list = vec![];
            let mut order_product_list = vec![];
            for item in result.unwrap().into_iter() {
                if item.storage_status.unwrap() != 1 {
                    continue;
                }
                order_product_list.push(item.clone()); //订单产品列表
                let mut storage_product: StorageProduct = item.clone().into();
                storage_product.adjust_id = adjust.id;
                storage_product.product_id = item.id;
                storage_product_list.push(storage_product);
            }
            let res = tx
                .save_batch(&mut storage_product_list, &[Skip::Column("id"), Skip::Column("created_at"), Skip::Column("updated_at")])
                .await;
        }

        // 将调整的产品调出仓库
        if adjust.adjust_cate.clone().unwrap() == "出库".to_string() {
            let select_storage_product = format!("SELECT storage_id, adjust_number,adjust_weight FROM storage_adjust_product WHERE adjust_id = {};", adjust.id.unwrap());
            let adjust_id_arg = as_bson!(adjust.id.unwrap().to_string());
            let storage_list: Vec<StorageForReduce> = rb.fetch(select_storage_product.as_str(), vec![adjust_id_arg]).await.unwrap();

            for item in storage_list {
                let update_storage_sql = format!("UPDATE storage_product SET storage_number = case when storage_number - {} < 0 then 0 else storage_number - {} end , storage_weight = case when storage_weight - {} < 0 then 0 else storage_weight - {} end WHERE id = {} ;",item.adjust_number,item.adjust_number,item.adjust_weight,item.adjust_weight,item.storage_id);
                let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_storage_sql.as_str()).await;
            }
        }
        // 将仓库的产品更改仓库名称
        if adjust.adjust_cate.clone().unwrap() == "调拨".to_string() {
            // 调整入库
            let select_storage_product = format!("SELECT storage_id, adjust_number,adjust_weight FROM storage_adjust_product WHERE adjust_id = {};", adjust.id.unwrap());
            let adjust_id_arg = as_bson!(adjust.id.unwrap().to_string());
            let storage_list: Vec<StorageForReduce> = rb.fetch(select_storage_product.as_str(), vec![adjust_id_arg.clone()]).await.unwrap();
            for item in storage_list {
                let get_storage_sql = format!("SELECT * from storage_product where id = {} ;", item.storage_id);
                let res: Vec<StorageProduct> = rb.fetch(get_storage_sql.as_str(), vec![adjust_id_arg.clone()]).await.unwrap();
                if res.len() > 0 {
                    let mut for_in_storage = res.get(0).unwrap().to_owned();
                    for_in_storage.id = None;
                    for_in_storage.order_id = None;
                    for_in_storage.adjust_id = adjust.id;
                    for_in_storage.storage_number = Some(item.adjust_number);
                    for_in_storage.storage_weight = Some(item.adjust_weight);
                    for_in_storage.warehouse = adjust.in_warehouse.clone();
                    for_in_storage.instorage_date = Some(DateNative::now());
                    let res = tx.save(&mut for_in_storage, &[Skip::Column("id"), Skip::Column("created_at"), Skip::Column("updated_at")]).await;
                }
            }
            // 调整出库
            let select_storage_product = format!("SELECT storage_id, adjust_number,adjust_weight FROM storage_adjust_product WHERE adjust_id = {};", adjust.id.unwrap());
            let adjust_id_arg = as_bson!(adjust.id.unwrap().to_string());

            let storage_list: Vec<StorageForReduce> = rb.fetch(select_storage_product.as_str(), vec![adjust_id_arg.clone()]).await.unwrap();
            for item in storage_list {
                let update_storage_sql = format!("UPDATE storage_product SET storage_number = case when storage_number - {} < 0 then 0 else storage_number - {} end , storage_weight = case when storage_weight - {} < 0 then 0 else storage_weight - {} end WHERE id = {} ;",item.adjust_number,item.adjust_number,item.adjust_weight,item.adjust_weight,item.storage_id);
                let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_storage_sql.as_str()).await;
            }
        }

        // 执行commit
        let res = tx.commit().await.unwrap();
        return Ok(());
    }
}

impl CrudService<StorageAdjust, StorageAdjustDTO, StorageAdjustPageDTO> for StorageAdjustService {
    fn get_wrapper(arg: &StorageAdjustPageDTO) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
            .do_if(arg.adjust_end().is_some(), |w| w.le("adjust_date", arg.adjust_end().clone().unwrap()))
            .do_if(arg.adjust_start().is_some(), |w| w.ge("adjust_date", arg.adjust_start().clone().unwrap()))
            .do_if(arg.company_code().is_some(), |w| w.eq("company_code", arg.company_code().clone().unwrap()))
            .do_if(arg.warehouse().is_some(), |w| w.eq("in_warehouse", arg.warehouse().clone().unwrap()))
            .or()
            .do_if(arg.warehouse().is_some(), |w| w.eq("out_warehouse", arg.warehouse().clone().unwrap()))
    }

    fn set_save_common_fields(&self, common: CommonField, data: &mut StorageAdjust) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

pub struct StorageAdjustProductService;
impl Default for StorageAdjustProductService {
    fn default() -> Self {
        StorageAdjustProductService {}
    }
}

impl CrudService<StorageAdjustProduct, StorageAdjustProductDTO, ByComQuery> for StorageAdjustProductService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut StorageAdjustProduct) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

pub struct ProcessService;
impl Default for ProcessService {
    fn default() -> Self {
        ProcessService {}
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ProcessStorageForReduce {
    pub storage_id: u64,
    pub process_number: f64,
    pub process_weight: f64,
}

impl ProcessService {
    pub async fn delete_process_relations(&self, process_id: u64) -> WyResult<()> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut tx = rb.acquire_begin().await?.defer_async(|mut tx1| async move {
            if !tx1.is_done() {
                tx1.rollback().await;
                log::error!("tx rollback success!");
            } else {
                println!("don't need rollback!");
            }
        });
        // 删除process_product表中数据
        let delete_process_product = format!("delete from process_product WHERE process_id = {};", process_id.clone());
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(delete_process_product.as_str()).await;
        // 删除 order_trans
        let delete_order_trans = format!("delete from order_trans WHERE process_id = {};", process_id.clone());
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(delete_order_trans.as_str()).await;
        // 执行commit
        let res = tx.commit().await.unwrap();
        return Ok(());
    }

    pub async fn trans_process_relations(&self, process_id: u64) -> WyResult<()> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut tx = rb.acquire_begin().await?.defer_async(|mut tx1| async move {
            if !tx1.is_done() {
                tx1.rollback().await;
                log::error!("tx rollback success!");
            } else {
                println!("don't need rollback!");
            }
        });
        // 修改加工的产品和加工的后的产品状态
        let update_process_product = format!("update process_product set storage_status = 2 WHERE process_id = {};", process_id.clone());
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_process_product.as_str()).await;

        // 保存加工后的产品入库
        let process_product_service = APPLICATION_CONTEXT.get::<ProcessProductService>();
        let param = ByProcessProductCateQuery {
            process_id: Some(process_id as u64),
            process_cate: Some(2),
        };
        let result = process_product_service.list(&param).await;
        // 将查询到的订单产品转为入库产品 并根据入库状态 storage_status = 1
        let mut storage_product_list = vec![];
        let mut process_product_list = vec![];
        for item in result.unwrap().into_iter() {
            if item.storage_status.unwrap() == 2 {
                process_product_list.push(item.clone()); //订单产品列表
                let mut storage_product: StorageProduct = item.clone().into();
                storage_product.process_id = item.id;
                storage_product_list.push(storage_product);
            }
        }
        let res = tx
            .save_batch(&mut storage_product_list, &[Skip::Column("id"), Skip::Column("created_at"), Skip::Column("updated_at")])
            .await;

        // 减少现有库存
        let select_storage_product = format!("SELECT storage_id, process_number,process_weight FROM process_product WHERE process_id = {};", process_id.clone());
        let adjust_id_arg = as_bson!(process_id);
        let storage_list: Vec<ProcessStorageForReduce> = rb.fetch(select_storage_product.as_str(), vec![adjust_id_arg]).await.unwrap();

        for item in storage_list {
            let update_storage_sql = format!("UPDATE storage_product SET storage_number = case when storage_number - {} < 0 then 0 else storage_number - {} end , storage_weight = case when storage_weight - {} < 0 then 0 else storage_weight - {} end WHERE id = {} ;",item.process_number,item.process_number,item.process_weight,item.process_weight,item.storage_id);
            let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_storage_sql.as_str()).await;
        }
        // 执行commit
        let res = tx.commit().await.unwrap();
        return Ok(());
    }
}

impl CrudService<Process, ProcessDTO, ProcessPageDTO> for ProcessService {
    fn get_wrapper(arg: &ProcessPageDTO) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
            .do_if(arg.process_end().is_some(), |w| w.le("process_date", arg.process_end().clone().unwrap()))
            .do_if(arg.process_start().is_some(), |w| w.ge("process_date", arg.process_start().clone().unwrap()))
            .do_if(arg.company_code().is_some(), |w| w.eq("company_code", arg.company_code().clone().unwrap()))
            .do_if(arg.warehouse().is_some(), |w| w.eq("warehouse", arg.warehouse().clone().unwrap()))
    }

    fn set_save_common_fields(&self, common: CommonField, data: &mut Process) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

pub struct ProcessProductService;
impl Default for ProcessProductService {
    fn default() -> Self {
        ProcessProductService {}
    }
}

impl CrudService<ProcessProduct, ProcessProductDTO, ByProcessProductCateQuery> for ProcessProductService {
    fn get_wrapper(arg: &ByProcessProductCateQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
            .do_if(arg.process_id().is_some(), |w| w.eq("process_id", arg.process_id().clone().unwrap()))
            .do_if(arg.process_cate().is_some(), |w| w.eq("process_cate", arg.process_cate().clone().unwrap()))
    }

    fn set_save_common_fields(&self, common: CommonField, data: &mut ProcessProduct) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}


pub struct ProcessSolutionService;
impl Default for ProcessSolutionService {
    fn default() -> Self {
        ProcessSolutionService {}
    }
}


impl CrudService<ProcessSolution, ProcessSolutionDTO, ByProcessProductCateQuery> for ProcessSolutionService {
    fn get_wrapper(arg: &ByProcessProductCateQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }

    fn set_save_common_fields(&self, common: CommonField, data: &mut ProcessSolution) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}
