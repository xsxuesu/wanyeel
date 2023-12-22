use crate::crud::crud_service::CrudService;
use crate::APPLICATION_CONTEXT;
use rbatis::crud::{CRUDTable, Skip, CRUD};
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};
use wy_domain::dto::sale::{SaleInfoPageDTO, SaleAfterDTO,SaleInfoRecievedPageDTO};
use wy_domain::dto::sale::{SaleInfoDTO, SaleProductDTO,PreSaleProductDTO,SalePayDTO,SalePayPageDTO,SaleAfterPageDTO,SaleProductFilterDTO};
use wy_domain::entity::sale::{SaleInfo, SaleProduct,PreSaleProduct, SalePay, SaleAfter};
use wy_domain::entity::{CommonField, PageData};
use wy_domain::error::Result as WyResult;
use wy_domain::request::{ByComQuery, ByUIDQuery};
// SaleService
pub struct SaleInfoService;
impl Default for SaleInfoService {
    fn default() -> Self {
        SaleInfoService {}
    }
}
impl SaleInfoService {
    pub async fn update_sale_key(&self, sale_id:u64, pay_id:u64) -> WyResult<u64>{
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let pay_id = as_bson!(pay_id);
        let sale_id = as_bson!(sale_id);
        let res = rb.exec("update sale_info set sale_pay_id =? where id =?", vec![pay_id,sale_id]).await;
        Ok(res.unwrap().rows_affected)
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct SaleForReduceStorage {
    pub storage_id: u64,
    pub sale_number: f64,
    pub sale_weight: f64,
}

impl SaleInfoService {
    // 应收款
    pub async fn get_sale_recieved_list(&self,arg:SaleInfoRecievedPageDTO) -> WyResult<Vec<SaleInfoDTO>>{
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();

        let mut query = format!(r#"SELECT * from sale_info WHERE  sale_status = 2 and company_code = '{}' "#, arg.company_code.clone().unwrap());

        if arg.sale_start.is_some(){
            let tmp  = format!(" and  sale_date >= '{}' ", arg.sale_start.clone().unwrap());
            query = format!("{} {}",query,tmp)
        }
        if arg.sale_end.is_some(){
            let tmp  = format!(" and  sale_date <= '{}' ", arg.sale_end.clone().unwrap());
            query = format!("{} {}",query,tmp)
        }

        let company_code_arg = as_bson!(arg.company_code.clone().unwrap());
        // let start_arg = as_bson!(arg.sale_start.clone().unwrap());
        // let end_arg = as_bson!(arg.sale_end.clone().unwrap());
        let sale_list: Vec<SaleInfoDTO> = rb.fetch(query.as_str(), vec![]).await.unwrap();

        Ok(sale_list)
    }
    pub async fn trans_sale_out_product(&self, sale_id: u64) -> WyResult<()> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut tx = rb.acquire_begin().await?.defer_async(|mut tx1| async move {
            if !tx1.is_done() {
                tx1.rollback().await;
                log::error!("tx rollback success!");
            } else {
                println!("don't need rollback!");
            }
        });

        // 更新仓库库存
        // 减少现有库存
        let select_sale_product = format!("SELECT storage_id, sale_number,sale_weight FROM sale_product WHERE sale_status = 1 and sale_id = {};", sale_id);
        let sale_id_arg = as_bson!(sale_id);
        let sale_list: Vec<SaleForReduceStorage> = rb.fetch(select_sale_product.as_str(), vec![sale_id_arg]).await.unwrap();

        for item in sale_list {
            let update_storage_sql = format!("UPDATE storage_product SET storage_number = case when storage_number - {} < 0 then 0 else storage_number - {} end , storage_weight = case when storage_weight - {} < 0 then 0 else storage_weight - {} end WHERE id = {} ;",item.sale_number,item.sale_number,item.sale_weight,item.sale_weight,item.storage_id);
            let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_storage_sql.as_str()).await;
        }

        // 更新销售产品表的状态
        let update_sale_product = format!("update sale_product set sale_status = 2 WHERE sale_status = 1 and sale_id = {};", sale_id);
        let res:Result<(i64, usize), rbatis::Error>  = tx.fetch(update_sale_product.as_str()).await;
        // 执行commit
        let res = tx.commit().await.unwrap();

        Ok(())
    }


    pub async fn trans_sale_relations(&self, sale_id: u64) -> WyResult<()> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut tx = rb.acquire_begin().await?.defer_async(|mut tx1| async move {
            if !tx1.is_done() {
                tx1.rollback().await;
                log::error!("tx rollback success!");
            } else {
                println!("don't need rollback!");
            }
        });
        // 更新销售订单相关数据
        // 1、更新运杂费
        let update_trans_order = format!("update order_trans set trans_status = 2 WHERE sale_id = {};", sale_id);
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_trans_order.as_str()).await;

        let update_sale_product = format!("update sale_product set sale_status = 2 WHERE sale_id = {};", sale_id);
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_sale_product.as_str()).await;

        // 2、更新仓库库存
        // 减少现有库存
        let select_sale_product = format!("SELECT storage_id, sale_number,sale_weight FROM sale_product WHERE sale_id = {};", sale_id);
        let sale_id_arg = as_bson!(sale_id);
        let sale_list: Vec<SaleForReduceStorage> = rb.fetch(select_sale_product.as_str(), vec![sale_id_arg]).await.unwrap();

        for item in sale_list {
            let update_storage_sql = format!("UPDATE storage_product SET storage_number = case when storage_number - {} < 0 then 0 else storage_number - {} end , storage_weight = case when storage_weight - {} < 0 then 0 else storage_weight - {} end WHERE id = {} ;",item.sale_number,item.sale_number,item.sale_weight,item.sale_weight,item.storage_id);
            let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_storage_sql.as_str()).await;
        }
        // 执行commit
        let res = tx.commit().await.unwrap();

        Ok(())
    }
}

impl CrudService<SaleInfo, SaleInfoDTO, SaleInfoPageDTO> for SaleInfoService {
    fn get_wrapper(arg: &SaleInfoPageDTO) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
            .do_if(arg.company_code().is_some(), |w| w.eq("company_code", arg.company_code().clone().unwrap()))
            .do_if(arg.client_name().is_some(), |w| w.eq("client_name", arg.client_name().clone().unwrap()))
            .do_if(arg.sale_mode.is_some(), |w| w.eq("sale_mode", arg.sale_mode().clone().unwrap()))
            .do_if(arg.sale_start().is_some(), |w| w.ge("sale_date", arg.sale_start().clone().unwrap()))
            .do_if(arg.sale_end().is_some(), |w| w.le("sale_date", arg.sale_end().clone().unwrap()))
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SaleInfo) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// SaleProductService
pub struct SaleProductService;
impl Default for SaleProductService {
    fn default() -> Self {
        SaleProductService {}
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct SaleProductForBakcStorage {
    pub storage_id: u64,
    pub back_number: f64,
    pub back_weight: f64,
}

impl SaleProductService {
    pub async fn sale_product_back_storage(&self,sale_after_id:u64)-> WyResult<()>{
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();

        let select_sale_product_for_back = format!("SELECT storage_id, back_number,back_weight FROM sale_product WHERE sale_after_id = {} and back_status = 1;", sale_after_id);
        let sale_after_id_arg = as_bson!(sale_after_id);
        let sale_product_back_list: Vec<SaleProductForBakcStorage> = rb.fetch(select_sale_product_for_back.as_str(), vec![sale_after_id_arg]).await.unwrap();

        let mut tx = rb.acquire_begin().await?.defer_async(|mut tx1| async move {
            if !tx1.is_done() {
                tx1.rollback().await;
                log::error!("tx rollback success!");
            } else {
                println!("don't need rollback!");
            }
        });
        // 1.更新销售产品表的状态
        let update_sale_product_status = format!("update sale_product set back_status = 2 WHERE sale_after_id = {};", sale_after_id);
        let res:Result<(i64, usize), rbatis::Error>  = tx.fetch(update_sale_product_status.as_str()).await;
        // 2、更新仓库库存
        for item in sale_product_back_list {
            let update_storage_sql = format!("UPDATE storage_product SET storage_number = storage_number + {} , storage_weight = storage_weight + {} WHERE id = {} ;",item.back_number,item.back_weight,item.storage_id);
            let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_storage_sql.as_str()).await;
        }
        // 执行commit
        let res = tx.commit().await.unwrap();
        Ok(())
    }
}


impl CrudService<SaleProduct, SaleProductDTO, SaleProductFilterDTO> for SaleProductService {
    fn get_wrapper(arg: &SaleProductFilterDTO) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
        .do_if(arg.company_code().is_some(), |w| w.eq("company_code", arg.company_code().clone().unwrap()))
        // .do_if(arg.client_name().is_some(), |w| w.eq("client_name", arg.client_name().clone().unwrap()))
        .do_if(arg.sale_id().is_some(), |w| w.eq("sale_id", arg.sale_id().clone().unwrap()))

    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SaleProduct) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}



// PreSaleProductService
pub struct PreSaleProductService;
impl Default for PreSaleProductService {
    fn default() -> Self {
        PreSaleProductService {}
    }
}
impl CrudService<PreSaleProduct, PreSaleProductDTO, ByComQuery> for PreSaleProductService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut PreSaleProduct) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}




// SalePayService
pub struct SalePayService;
impl Default for SalePayService {
    fn default() -> Self {
        SalePayService {}
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SaleAfterForTransCost {
    pub id: u64,
}

impl SalePayService {
    pub async fn sale_pay_update_order(&self, sale_id:String )-> WyResult<()>{
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
        // 更新销售订单状态
        let updata_order = format!("update sale_info set sale_status = 3 WHERE id = {};",sale_id);
        let res:Result<(i64,usize),rbatis::Error> = tx.fetch(updata_order.as_str()).await;
// 更新运杂费
        let update_trans_order = format!("update order_trans set trans_status = 2 WHERE sale_id = {};",sale_id);
        let res:Result<(i64,usize),rbatis::Error> = tx.fetch(update_trans_order.as_str()).await;
// 更新收款信息
        let update_pay_order = format!("update sale_pay set sale_pay_status = 2 WHERE sale_id = {};",sale_id);
        let res:Result<(i64,usize),rbatis::Error> = tx.fetch(update_pay_order.as_str()).await;
// 更新售后信息
        let update_sale_after = format!("update sale_after set sale_after_status = 2 WHERE sale_id = {};",sale_id);
        let res:Result<(i64,usize),rbatis::Error> = tx.fetch(update_sale_after.as_str()).await;
        let res = tx.commit().await.unwrap();
// 查询售后id
        let select_sale_after_id = format!("SELECT id FROM sale_after WHERE sale_id = {};", sale_id);
        let sale_id_arg = as_bson!(sale_id);
        let sale_after_list: Vec<SaleAfterForTransCost> = rb.fetch(select_sale_after_id.as_str(), vec![sale_id_arg]).await.unwrap();
        // 更新运杂费状态
        for item in sale_after_list {
            let update_transcost_sql = format!("UPDATE order_trans SET trans_status = 2  WHERE buy_after_sale_id = {} ;",item.id);
            let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_transcost_sql.as_str()).await;
        }
        return Ok(())
    }
}
impl CrudService<SalePay, SalePayDTO, SalePayPageDTO> for SalePayService {
    fn get_wrapper(arg: &SalePayPageDTO) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
            .do_if(arg.company_code().is_some(), |w| w.eq("company_code", arg.company_code().clone().unwrap()))
            .do_if(arg.client_name().is_some(), |w| w.eq("client_name", arg.client_name().clone().unwrap()))
            .do_if(arg.sale_pay_start().is_some(), |w| w.ge("pay_date", arg.sale_pay_start().clone().unwrap()))
            .do_if(arg.sale_pay_end().is_some(), |w| w.le("pay_date", arg.sale_pay_end().clone().unwrap()))
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SalePay) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}




// SaleAfterService
pub struct SaleAfterService;
impl Default for SaleAfterService {
    fn default() -> Self {
        SaleAfterService {}
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SaleAfterForRaiseStorage {
    pub storage_id: u64,
    pub back_number: f64,
    pub back_weight: f64,
}


impl SaleAfterService {
    pub async fn trans_after_sale_relation(&self,after_sale_id:u64) -> WyResult<()> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut tx = rb.acquire_begin().await?.defer_async(|mut tx1| async move {
            if tx1.is_done() {
                println!("don't need rollback!");
                log::info!("don't need rollback!");
            } else {
                tx1.rollback().await;
                log::error!("tx rollback success!");
            }
        });
        // 更新销售订单相关数据
        // 1、更新运杂费
        let update_trans_order = format!("update order_trans set trans_status = 2 WHERE buy_after_sale_id = {};", after_sale_id);
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_trans_order.as_str()).await;
        // 2、更新售后产品为退货
        let update_sale_product = format!("update sale_product set sale_status = 4 WHERE sale_after_id = {};", after_sale_id);
        let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_sale_product.as_str()).await;
        // 3、退货入库
        // SaleAfterForRaiseStorage
        // 3、1 查找退货售后产品
        let select_sale_product = format!("SELECT storage_id, back_number,back_weight FROM sale_product WHERE sale_after_id = {};", after_sale_id);
        let sale_after_id_arg = as_bson!(after_sale_id);
        let sale_after_product_list: Vec<SaleAfterForRaiseStorage> = rb.fetch(select_sale_product.as_str(), vec![sale_after_id_arg]).await.unwrap();
        // 3、2 增加库存
        for product_tmpe in sale_after_product_list {
            let update_sale_product = format!("update storage_product set storage_status = 1 , storage_number =  storage_number + {}, storage_weight = storage_weight + {}  WHERE id = {};", product_tmpe.back_number,product_tmpe.back_weight,product_tmpe.storage_id);
            let res: Result<(i64, usize), rbatis::Error> = tx.fetch(update_sale_product.as_str()).await;
        }
        // 执行commit
        let res = tx.commit().await.unwrap();
        Ok(())
    }
}

impl CrudService<SaleAfter, SaleAfterDTO, SaleAfterPageDTO> for SaleAfterService {
    fn get_wrapper(arg: &SaleAfterPageDTO) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
            .do_if(arg.company_code().is_some(), |w| w.eq("company_code", arg.company_code().clone().unwrap()))
            .do_if(arg.client_name().is_some(), |w| w.eq("client_name", arg.client_name().clone().unwrap()))
            .do_if(arg.sale_after_start().is_some(), |w| w.ge("back_date", arg.sale_after_start().clone().unwrap()))
            .do_if(arg.sale_after_end().is_some(), |w| w.le("back_date", arg.sale_after_end().clone().unwrap()))
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SaleAfter) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}