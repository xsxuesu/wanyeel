
use wy_domain::dto::pay::{OrderPayDTO,OrderPayFilterParams,OrderPayParams};
use wy_domain::entity::pay::{OrderPay};
use wy_domain::request::{ByComQuery,ByUIDQuery};
use rbatis::crud::{CRUDTable, Skip, CRUD};
use wy_domain::entity::{PageData,CommonField};
use crate::crud::crud_service::CrudService;
use crate::APPLICATION_CONTEXT;
use rbatis::rbatis::Rbatis;
use wy_domain::error::Result as WyResult;
use rbatis::plugin::page::{Page, PageRequest};
// OrderPayService
pub struct OrderPayService;
impl Default for OrderPayService {
    fn default() -> Self {
        OrderPayService {}
    }
}

impl OrderPayService {
    pub async fn save_pay_update_agree(&self, agree_id:String)-> WyResult<()>{
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut tx = rb.acquire_begin().await?.defer_async(|mut tx1| async move {
            if !tx1.is_done() {
                tx1.rollback().await;
                log::error!("tx rollback success!");
            } else {
                println!("don't need rollback!");
            }
        });
        let updata_order = format!("update agree set agree_status = 3 WHERE id = {};",agree_id);
        let res:Result<(i64,usize),rbatis::Error> = tx.fetch(updata_order.as_str()).await;
        let update_trans_order = format!("update order_trans set trans_status = 2 WHERE agree_id = {};",agree_id);
        let res:Result<(i64,usize),rbatis::Error> = tx.fetch(update_trans_order.as_str()).await;
        let update_pay_order = format!("update order_pay set pay_status = 2 WHERE agree_id = {};",agree_id);
        let res:Result<(i64,usize),rbatis::Error> = tx.fetch(update_pay_order.as_str()).await;
        let update_after_sale = format!("update order_after_sale set back_status = 2 WHERE agree_id = {};",agree_id);
        let res:Result<(i64,usize),rbatis::Error> = tx.fetch(update_after_sale.as_str()).await;
        let res = tx.commit().await.unwrap();
        return Ok(())
    }

    pub async fn save_pay_update_order(&self, order_id:String )-> WyResult<()>{
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
        let updata_order = format!("update order_info set order_status = 3 WHERE id = {};",order_id);
        let res:Result<(i64,usize),rbatis::Error> = tx.fetch(updata_order.as_str()).await;
        let update_trans_order = format!("update order_trans set trans_status = 2 WHERE order_id = {};",order_id);
        let res:Result<(i64,usize),rbatis::Error> = tx.fetch(update_trans_order.as_str()).await;
        let update_pay_order = format!("update order_pay set pay_status = 2 WHERE order_id = {};",order_id);
        let res:Result<(i64,usize),rbatis::Error> = tx.fetch(update_pay_order.as_str()).await;
        let update_after_sale = format!("update order_after_sale set back_status = 2 WHERE order_id = {};",order_id);
        let res:Result<(i64,usize),rbatis::Error> = tx.fetch(update_after_sale.as_str()).await;
        let res = tx.commit().await.unwrap();
        return Ok(())
    }
}

impl CrudService<OrderPay, OrderPayDTO, OrderPayParams> for OrderPayService {
    fn get_wrapper(arg: &OrderPayParams) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
        .do_if(arg.company_code().is_some(), |w| w.eq("company_code", arg.company_code().clone().unwrap()))
        .do_if(arg.supply_dept().is_some(), |w| w.eq("supply_dept", arg.supply_dept().clone().unwrap()))
        .do_if(arg.order_pay_start().is_some(), |w| w.ge("pay_date", arg.order_pay_start().clone().unwrap()))
        .do_if(arg.order_pay_end().is_some(), |w| w.le("pay_date", arg.order_pay_end().clone().unwrap()))
        .do_if(arg.buy_mode().is_some(), |w| w.eq("buy_mode", arg.buy_mode().clone().unwrap()))
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut OrderPay) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}
