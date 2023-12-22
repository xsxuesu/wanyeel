
use wy_domain::dto::agree::{AgreeProductDTO,AgreeDTO,AgreeParams,OrderByFilterDTO};
use wy_domain::entity::agree::{Agree,AgreeProduct};
use wy_domain::entity::CommonField;
use wy_domain::request::{ByComQuery,ByUIDQuery};
use crate::crud::crud_service::CrudService;
use crate::APPLICATION_CONTEXT;
use rbatis::rbatis::Rbatis;
use wy_domain::error::Result as WyResult;

// AgreeService
pub struct AgreeService;
impl Default for AgreeService {
    fn default() -> Self {
        AgreeService {}
    }
}

impl AgreeService {
    pub async fn update_agree_key(&self, agree_id:u64, pay_id:u64) -> WyResult<u64>{
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        println!("agree_id:{}",agree_id);
        println!("pay_id:{}",pay_id);
        let pay_id = as_bson!(pay_id);
        let agree_id = as_bson!(agree_id);
        let res = rb.exec("update agree set pay_id =? where id =?", vec![pay_id,agree_id]).await;
        Ok(res.unwrap().rows_affected)
    }
    // 应收款
    pub async fn get_agree_payed_list(&self,arg:OrderByFilterDTO) -> WyResult<Vec<AgreeDTO>>{
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();

        let mut query = format!("SELECT * FROM agree WHERE  agree_status = 2 and company_code = '{}' ", arg.company_code.clone().unwrap());

        if arg.order_start.is_some(){
            let tmp  = format!(" and agree_date >= '{}' ", arg.order_start.clone().unwrap());
            query = format!("{} {}",query,tmp)
        }
        if arg.order_end.is_some(){
            let tmp  = format!(" and  agree_date <= '{}' ", arg.order_end.clone().unwrap());
            query = format!("{} {}",query,tmp)
        }

        let company_code_arg = as_bson!(arg.company_code.clone().unwrap());
        // let start_arg = as_bson!(arg.order_start.clone().unwrap());
        // let end_arg = as_bson!(arg.order_end.clone().unwrap());
        let agree_list: Vec<AgreeDTO> = rb.fetch(query.as_str(), vec![company_code_arg]).await.unwrap();

        Ok(agree_list)
    }
}

impl CrudService<Agree, AgreeDTO, AgreeParams> for AgreeService {
    fn get_wrapper(arg: &AgreeParams) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
        .do_if(arg.company_code().is_some(), |w| {
            w.eq("company_code", arg.company_code().clone().unwrap())
        })
        .do_if(arg.supply_dept().is_some(), |w| {
            w.eq("supply_dept", arg.supply_dept().clone().unwrap())
        })
        .do_if(arg.delivery_dept().is_some(), |w| {
            w.eq("delivery_dept", arg.delivery_dept().clone().unwrap())
        })
        .do_if(arg.agree_status().is_some(), |w| {
            w.eq("agree_status", arg.agree_status().clone().unwrap())
        })
        .do_if(arg.agree_start().is_some(), |w| {
            w.ge("agree_date", arg.agree_start().clone().unwrap())
        })
        .do_if(arg.agree_end().is_some(), |w| {
            w.le("agree_date", arg.agree_end().clone().unwrap())
        })
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut Agree) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// AgreeProductService
pub struct AgreeProductService;
impl Default for AgreeProductService {
    fn default() -> Self {
        AgreeProductService {}
    }
}
impl CrudService<AgreeProduct, AgreeProductDTO, ByUIDQuery> for AgreeProductService {
    fn get_wrapper(arg: &ByUIDQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()

    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut AgreeProduct) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}
