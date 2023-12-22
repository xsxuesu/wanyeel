use crate::crud::crud_service::CrudService;
use crate::APPLICATION_CONTEXT;
use rbatis::crud::CRUDMut;
use rbatis::crud::{CRUDTable, Skip, CRUD};
use rbatis::rbatis::Rbatis;
use rbson::Bson;
use wy_domain::dto::finance::{
    BuyStaticalParams, FinanceStaticalParams, PayedInfoDTO, PayedInfoFilterParams, PayedStaticalItem, PayedStatisticItem, ReceivedInfoDTO, ReceivedInfoFilterParams, RecieveStaticalItem,
    SaleStaticalParams, SaleStatisticItem, TaxInfoDTO, TransStaticalItem, TransStaticalParams,
};
use wy_domain::entity::finance::{PayedInfo, ReceivedInfo, TaxInfo};
use wy_domain::entity::CommonField;
use wy_domain::error::Result as WyResult;
use wy_domain::request::{ByComQuery, ByUIDQuery};

// PayedInfoService
pub struct PayedInfoService;
impl Default for PayedInfoService {
    fn default() -> Self {
        PayedInfoService {}
    }
}

impl CrudService<PayedInfo, PayedInfoDTO, PayedInfoFilterParams> for PayedInfoService {
    fn get_wrapper(arg: &PayedInfoFilterParams) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
            .do_if(arg.company_code().is_some(), |w| w.eq("company_code", arg.company_code().clone().unwrap()))
            .do_if(arg.client_name().is_some(), |w| w.eq("recieve_dept", arg.client_name().clone().unwrap()))
            .do_if(arg.payed_date_start().is_some(), |w| w.ge("pay_date", arg.payed_date_start().clone().unwrap()))
            .do_if(arg.payed_date_end().is_some(), |w| w.le("pay_date", arg.payed_date_end().clone().unwrap()))
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut PayedInfo) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// ReceiveInfoService
pub struct ReceiveInfoService;
impl Default for ReceiveInfoService {
    fn default() -> Self {
        ReceiveInfoService {}
    }
}

impl CrudService<ReceivedInfo, ReceivedInfoDTO, ReceivedInfoFilterParams> for ReceiveInfoService {
    fn get_wrapper(arg: &ReceivedInfoFilterParams) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
            .do_if(arg.company_code().is_some(), |w| w.eq("company_code", arg.company_code().clone().unwrap()))
            .do_if(arg.client_name().is_some(), |w| w.eq("client_name", arg.client_name().clone().unwrap()))
            .do_if(arg.receieved_date_start().is_some(), |w| w.ge("recieve_date", arg.receieved_date_start().clone().unwrap()))
            .do_if(arg.receieved_date_end().is_some(), |w| w.le("recieve_date", arg.receieved_date_end().clone().unwrap()))
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut ReceivedInfo) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// 发票
pub struct InvoiceService;
impl Default for InvoiceService {
    fn default() -> Self {
        InvoiceService {}
    }
}
impl CrudService<TaxInfo, TaxInfoDTO, ByComQuery> for InvoiceService {
    fn get_wrapper(arg: &ByComQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut TaxInfo) {
        data.id = common.id;
        data.created_at = common.created_at;
        data.updated_at = common.updated_at;
    }
}

// StaticalService
pub struct StaticalService;
impl Default for StaticalService {
    fn default() -> Self {
        StaticalService {}
    }
}

impl StaticalService {
    pub async fn get_trans_statical(&self, params: TransStaticalParams) -> WyResult<Vec<TransStaticalItem>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();

        let mut query = format!(
            r#"
        SELECT supply_dept,

SUM(CASE WHEN in_out = 1 THEN cost_amount ELSE  0 END) as cost_all_amount,

SUM(CASE WHEN in_out = 2 THEN cost_amount ELSE  0 END) as recieve_all_amount,

SUM(CASE WHEN is_invoice = 1 THEN cost_amount ELSE  0 END) as un_invoice_amount,

SUM(CASE WHEN is_invoice = 2 THEN invoice_amount ELSE  0 END) as invoice_amount

FROM order_trans WHERE trans_status = 2 and company_code = '{}'

        "#,
            params.company_code.clone().unwrap()
        );

        if params.trans_name.is_some() {
            let tmp = format!(" and supply_dept = '{}' ", params.trans_name.clone().unwrap());
            query = format!("{} {}", query, tmp);
        }
        if params.trans_date_start.is_some() {
            let tmp = format!(" and cost_date >= '{}' ", params.trans_date_start.clone().unwrap());
            query = format!("{} {}", query, tmp);
        }
        if params.trans_date_end.is_some() {
            let tmp = format!(" and cost_date <= '{}' ", params.trans_date_end.clone().unwrap());
            query = format!("{} {}", query, tmp);
        }

        let tmp = format!(" GROUP BY supply_dept ");
        query = format!("{} {}", query, tmp);
        //
        let company_code_arg = as_bson!(params.company_code.clone().unwrap());
        // let start_arg = as_bson!(arg.order_start.clone().unwrap());
        // let end_arg = as_bson!(arg.order_end.clone().unwrap());
        let trans_list: Vec<TransStaticalItem> = rb.fetch(query.as_str(), vec![company_code_arg]).await.unwrap();

        Ok(trans_list)
    }

    pub async fn get_payed_statical(&self, params: FinanceStaticalParams) -> WyResult<Vec<PayedStaticalItem>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();

        let mut query = format!(
            r#"
        SELECT recieve_dept, SUM(pay_amount) as pay_amount,
SUM(CASE WHEN is_invoice = 1 THEN pay_amount ELSE  0 END) as un_invoice_amount,
SUM(CASE WHEN is_invoice = 2 THEN invoice_amount ELSE  0 END) as invoice_amount
FROM payed_info WHERE pay_status =2 and company_code = '{}'

        "#,
            params.company_code.clone().unwrap()
        );

        if params.finance_date_start.is_some() {
            let tmp = format!(" and pay_date >= '{}' ", params.finance_date_start.clone().unwrap());
            query = format!("{} {}", query, tmp);
        }
        if params.finance_date_end.is_some() {
            let tmp = format!(" and pay_date <= '{}' ", params.finance_date_end.clone().unwrap());
            query = format!("{} {}", query, tmp);
        }

        let tmp = format!(" GROUP BY  recieve_dept ");
        query = format!("{} {}", query, tmp);
        //
        let company_code_arg = as_bson!(params.company_code.clone().unwrap());
        let trans_list: Vec<PayedStaticalItem> = rb.fetch(query.as_str(), vec![company_code_arg]).await.unwrap();

        Ok(trans_list)
    }

    pub async fn get_recieve_statical(&self, params: FinanceStaticalParams) -> WyResult<Vec<RecieveStaticalItem>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();

        let mut query = format!(
            r#"
        SELECT pay_dept, SUM(recieve_amount) as recieve_amount,
SUM(CASE WHEN is_invoice = 1 THEN recieve_amount ELSE  0 END) as un_invoice_amount,
SUM(CASE WHEN is_invoice = 2 THEN invoice_amount ELSE  0 END) as invoice_amount
FROM recieved_info WHERE recieve_status =2  and company_code = '{}'

        "#,
            params.company_code.clone().unwrap()
        );

        if params.finance_date_start.is_some() {
            let tmp = format!(" and recieve_date >= '{}' ", params.finance_date_start.clone().unwrap());
            query = format!("{} {}", query, tmp);
        }
        if params.finance_date_end.is_some() {
            let tmp = format!(" and recieve_date <= '{}' ", params.finance_date_end.clone().unwrap());
            query = format!("{} {}", query, tmp);
        }

        let tmp = format!(" GROUP BY  pay_dept ");
        query = format!("{} {}", query, tmp);
        //
        let company_code_arg = as_bson!(params.company_code.clone().unwrap());
        let trans_list: Vec<RecieveStaticalItem> = rb.fetch(query.as_str(), vec![company_code_arg]).await.unwrap();

        Ok(trans_list)
    }

    pub async fn get_buy_statical(&self, params: BuyStaticalParams) -> WyResult<Vec<PayedStatisticItem>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        println!("params:{:?}", params);
        let mut date_query = "".to_string();
        if params.payed_date_end.is_some() && params.payed_date_start.is_some() {
            date_query = format!(
                r#" and a.agree_date between '{}' and '{}' "#,
                params.payed_date_start.clone().unwrap(),
                params.payed_date_end.clone().unwrap()
            );
        } else {
            if params.payed_date_end.is_some() {
                date_query = format!(r#" and a.agree_date <= '{}' "#, params.payed_date_end.unwrap());
            }
            if params.payed_date_start.is_some() {
                date_query = format!(r#" and a.agree_date >= '{}' "#, params.payed_date_start.unwrap());
            }
        }

        let query = format!(
            r#"
SELECT supply_dept, sum(agree_amount) as agree_amount,sum(buy_number) as buy_number, sum(buy_weight) as buy_weight,
sum(instorage_number) as instorage_number,sum(instorage_weight) as instorage_weight,sum(instorage_amount) as instorage_amount,
sum(cost_all_amount) as cost_all_amount,sum(recieve_all_amount) as recieve_all_amount,sum(back_number) as back_number,
sum(back_weight) as back_weight,sum(back_money) as back_money,sum(pay_amount) as pay_amount,sum(invoice_amount) as invoice_amount
FROM (

SELECT CASE WHEN a.agree_type = '期货采购' THEN a.delivery_dept ELSE  a.supply_dept END as supply_dept,
CONVERT(SUM(b.agree_amount), DECIMAL(16,6)) as agree_amount ,
CONVERT(SUM(b.buy_number), DECIMAL(16,6)) as buy_number,
CONVERT(SUM(b.buy_weight), DECIMAL(16,6)) as buy_weight,
CONVERT("0.0", DECIMAL(16,6)) as instorage_number,
CONVERT("0.0", DECIMAL(16,6)) as instorage_weight,
CONVERT("0.0", DECIMAL(16,6)) as instorage_amount,
CONVERT("0.0", DECIMAL(16,6)) as cost_all_amount,
CONVERT("0.0", DECIMAL(16,6)) as recieve_all_amount,
CONVERT("0.0", DECIMAL(16,6)) as back_number,
CONVERT("0.0", DECIMAL(16,6)) as back_weight,
CONVERT("0.0", DECIMAL(16,6)) as back_money,
CONVERT("0.0", DECIMAL(16,6)) as pay_amount,
CONVERT("0.0", DECIMAL(16,6)) as invoice_amount
FROM agree as a
left JOIN agree_product as b ON a.id = b.agree_id
WHERE a.company_code = '{}' {} and (a.agree_status = 2 or a.agree_status=3) GROUP BY supply_dept

UNION ALL

SELECT CASE WHEN a.agree_type = '期货采购' THEN a.delivery_dept ELSE  a.supply_dept END as supply_dept,
CONVERT("0.0", DECIMAL(16,6)) as agree_amount,
CONVERT("0.0", DECIMAL(16,6)) as buy_number,
CONVERT("0.0", DECIMAL(16,6)) as buy_weight,
CONVERT(SUM(b.buy_number), DECIMAL(16,6)) as instorage_number,
CONVERT(SUM(b.buy_weight), DECIMAL(16,6))  as instorage_weight,
CONVERT(SUM(buy_amount), DECIMAL(16,6)) as instorage_amount,
CONVERT("0.0", DECIMAL(16,6)) as cost_all_amount,
CONVERT("0.0", DECIMAL(16,6)) as recieve_all_amount,
CONVERT("0.0", DECIMAL(16,6)) as back_number,
CONVERT("0.0", DECIMAL(16,6)) as back_weight,
CONVERT("0.0", DECIMAL(16,6)) as back_money,
CONVERT("0.0", DECIMAL(16,6)) as pay_amount,
CONVERT("0.0", DECIMAL(16,6)) as invoice_amount
FROM agree as a
left JOIN order_product as b ON a.id = b.agree_id
WHERE a.company_code = '{}' {} and (a.agree_status = 2 or a.agree_status=3) and b.storage_status = 2 GROUP BY supply_dept

UNION ALL

SELECT CASE WHEN a.agree_type = '期货采购' THEN a.delivery_dept ELSE  a.supply_dept END as supply_dept,
CONVERT("0.0", DECIMAL(16,6)) as agree_amount,
CONVERT("0.0", DECIMAL(16,6)) as buy_number,
CONVERT("0.0", DECIMAL(16,6)) as buy_weight,
CONVERT("0.0", DECIMAL(16,6)) as instorage_number,
CONVERT("0.0", DECIMAL(16,6)) as instorage_weight,
CONVERT("0.0", DECIMAL(16,6)) as instorage_amount,
CONVERT(SUM(CASE WHEN b.in_out = 1 THEN cost_amount ELSE  0 END), DECIMAL(16,6)) as cost_all_amount,
CONVERT(SUM(CASE WHEN b.in_out = 2 THEN cost_amount ELSE  0 END), DECIMAL(16,6)) as recieve_all_amount,
CONVERT("0.0", DECIMAL(16,6)) as back_number,
CONVERT("0.0", DECIMAL(16,6)) as back_weight,
CONVERT("0.0", DECIMAL(16,6)) as back_money,
CONVERT("0.0", DECIMAL(16,6)) as pay_amount,
CONVERT("0.0", DECIMAL(16,6)) as invoice_amount

FROM agree as a
left JOIN order_trans as b ON a.id = b.agree_id
WHERE a.company_code = '{}' {} and (a.agree_status = 2 or a.agree_status=3) and b.trans_status = 2

UNION ALL

SELECT CASE WHEN a.agree_type = '期货采购' THEN a.delivery_dept ELSE  a.supply_dept END as supply_dept,
CONVERT("0.0", DECIMAL(16,6)) as agree_amount,
CONVERT("0.0", DECIMAL(16,6)) as buy_number,
CONVERT("0.0", DECIMAL(16,6)) as buy_weight,
CONVERT("0.0", DECIMAL(16,6)) as instorage_number,
CONVERT("0.0", DECIMAL(16,6)) as instorage_weight,
CONVERT("0.0", DECIMAL(16,6)) as instorage_amount,
CONVERT("0.0", DECIMAL(16,6)) as cost_all_amount,
CONVERT("0.0", DECIMAL(16,6)) as recieve_all_amount,
CONVERT(SUM(b.back_number), DECIMAL(16,6)) as back_number,
CONVERT(SUM(b.back_weight), DECIMAL(16,6)) as back_weight,
CONVERT(SUM(back_money), DECIMAL(16,6)) as back_money,
CONVERT("0.0", DECIMAL(16,6)) as pay_amount,
CONVERT("0.0", DECIMAL(16,6)) as invoice_amount
FROM agree as a
left JOIN
(SELECT c.agree_id, d.back_number,d.back_weight,c.back_money,c.back_status,c.unit_money FROM order_after_sale as c LEFT JOIN order_after_sale_product as d ON c.id = d.after_sale_id)
as b ON a.id = b.agree_id
WHERE a.company_code = '{}' {} and (a.agree_status = 2 or a.agree_status=3) and b.back_status = 2

UNION ALL

SELECT CASE WHEN a.agree_type = '期货采购' THEN a.delivery_dept ELSE  a.supply_dept END as supply_dept,
CONVERT("0.0", DECIMAL(16,6)) as agree_amount,
CONVERT("0.0", DECIMAL(16,6)) as buy_number,
CONVERT("0.0", DECIMAL(16,6)) as buy_weight,
CONVERT("0.0", DECIMAL(16,6)) as instorage_number,
CONVERT("0.0", DECIMAL(16,6)) as instorage_weight,
CONVERT("0.0", DECIMAL(16,6)) as instorage_amount,
CONVERT("0.0", DECIMAL(16,6)) as cost_all_amount,
CONVERT("0.0", DECIMAL(16,6)) as recieve_all_amount,
CONVERT("0.0", DECIMAL(16,6)) as back_number,
CONVERT("0.0", DECIMAL(16,6)) as back_weight,
CONVERT("0.0", DECIMAL(16,6)) as back_money,
CONVERT(SUM(pay_amount), DECIMAL(16,6)) as pay_amount,
CONVERT(SUM(invoice_amount), DECIMAL(16,6)) as invoice_amount
FROM agree as a
left JOIN payed_info as b ON a.id = b.agree_id
WHERE a.company_code = '{}' {} and (a.agree_status = 2 or a.agree_status=3) and b.pay_status = 2

) as t GROUP BY t.supply_dept
        "#,
            params.company_code.clone().unwrap(),
            date_query,
            params.company_code.clone().unwrap(),
            date_query,
            params.company_code.clone().unwrap(),
            date_query,
            params.company_code.clone().unwrap(),
            date_query,
            params.company_code.clone().unwrap(),
            date_query
        );

        let company_code_arg = as_bson!(params.company_code.clone().unwrap());
        let pay_list: Vec<PayedStatisticItem> = rb.fetch(query.as_str(), vec![company_code_arg]).await.unwrap();

        Ok(pay_list)
    }

    pub async fn get_buy_statical_order(&self, params: BuyStaticalParams) -> WyResult<Vec<PayedStatisticItem>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        println!("params:{:?}", params);
        let mut date_query = "".to_string();
        if params.payed_date_end.is_some() && params.payed_date_start.is_some() {
            date_query = format!(
                r#" and a.buy_date between '{}' and '{}' "#,
                params.payed_date_start.clone().unwrap(),
                params.payed_date_end.clone().unwrap()
            );
        } else {
            if params.payed_date_end.is_some() {
                date_query = format!(r#" and a.buy_date <= '{}' "#, params.payed_date_end.unwrap());
            }
            if params.payed_date_start.is_some() {
                date_query = format!(r#" and a.buy_date >= '{}' "#, params.payed_date_start.unwrap());
            }
        }

        let query = format!(
            r#"

        SELECT supply_dept , SUM(buy_amount) as agree_amount , sum(buy_number) as buy_number, sum(buy_weight) as buy_weight, sum(instorage_number) as instorage_number,
        sum(instorage_weight) as instorage_weight, sum(instorage_amount) as instorage_amount, sum(cost_all_amount) as cost_all_amount, sum(recieve_all_amount) as recieve_all_amount,
        sum(back_number) as back_number, sum(back_weight) as back_weight, sum(back_money) as back_money,sum(pay_amount) as pay_amount, sum(invoice_amount) as invoice_amount
          FROM (

        SELECT a.supply_dept as supply_dept,
        CONVERT(SUM(b.buy_amount), DECIMAL(16,6)) as buy_amount ,
        CONVERT(SUM(b.buy_number), DECIMAL(16,6)) as buy_number,
        CONVERT(SUM(b.buy_weight), DECIMAL(16,6))  as buy_weight,
        CONVERT("0.0", DECIMAL(16,6)) as instorage_number,
        CONVERT("0.0", DECIMAL(16,6)) as instorage_weight,
        CONVERT("0.0", DECIMAL(16,6)) as instorage_amount,
        CONVERT("0.0", DECIMAL(16,6)) as cost_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as recieve_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as back_number,
        CONVERT("0.0", DECIMAL(16,6)) as back_weight,
        CONVERT("0.0", DECIMAL(16,6)) as back_money,
        CONVERT("0.0", DECIMAL(16,6)) as pay_amount,
        CONVERT("0.0", DECIMAL(16,6)) as invoice_amount
        FROM order_info as a
        left JOIN order_product as b ON a.id = b.order_id
        WHERE a.company_code = '{}' {} and (a.order_status = 2 or a.order_status=3)  GROUP BY supply_dept

        UNION ALL

        SELECT a.supply_dept as supply_dept,
        CONVERT("0.0", DECIMAL(16,6)) as buy_amount ,
        CONVERT("0.0", DECIMAL(16,6)) as buy_number,
        CONVERT("0.0", DECIMAL(16,6)) as buy_weight,
        CONVERT(SUM(b.buy_number), DECIMAL(16,6)) as instorage_number,
        CONVERT(SUM(b.buy_weight), DECIMAL(16,6))  as instorage_weight,
        CONVERT(SUM(b.buy_amount), DECIMAL(16,6)) as instorage_amount,
        CONVERT("0.0", DECIMAL(16,6)) as cost_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as recieve_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as back_number,
        CONVERT("0.0", DECIMAL(16,6)) as back_weight,
        CONVERT("0.0", DECIMAL(16,6)) as back_money,
        CONVERT("0.0", DECIMAL(16,6)) as pay_amount,
        CONVERT("0.0", DECIMAL(16,6)) as invoice_amount
        FROM order_info as a
        left JOIN order_product as b ON a.id = b.order_id
        WHERE a.company_code = '{}' {} and (a.order_status = 2 or a.order_status=3)  GROUP BY supply_dept

        UNION ALL

        SELECT a.supply_dept as supply_dept,
        CONVERT("0.0", DECIMAL(16,6)) as buy_amount ,
        CONVERT("0.0", DECIMAL(16,6)) as buy_number,
        CONVERT("0.0", DECIMAL(16,6))  as buy_weight,
        CONVERT("0.0", DECIMAL(16,6)) as instorage_number,
        CONVERT("0.0", DECIMAL(16,6))  as instorage_weight,
        CONVERT("0.0", DECIMAL(16,6)) as instorage_amount,
        CONVERT(SUM(CASE WHEN b.in_out = 1 THEN (b.cost_amount) ELSE  0 END), DECIMAL(16,6)) as cost_all_amount,
        CONVERT(SUM(CASE WHEN b.in_out = 2 THEN (b.cost_amount) ELSE  0 END), DECIMAL(16,6)) as recieve_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as back_number,
        CONVERT("0.0", DECIMAL(16,6)) as back_weight,
        CONVERT("0.0", DECIMAL(16,6)) as back_money,
        CONVERT("0.0", DECIMAL(16,6)) as pay_amount,
        CONVERT("0.0", DECIMAL(16,6)) as invoice_amount
        FROM order_info as a
        left JOIN (select * from order_trans where trans_status = 2 ) as b ON a.id = b.order_id
        WHERE a.company_code = '{}' {} and (a.order_status = 2 or a.order_status=3) GROUP BY supply_dept

        UNION ALL

        SELECT  a.supply_dept as supply_dept,
        CONVERT("0.0", DECIMAL(16,6)) as buy_amount ,
        CONVERT("0.0", DECIMAL(16,6)) as buy_number,
        CONVERT("0.0", DECIMAL(16,6))  as buy_weight,
		CONVERT("0.0", DECIMAL(16,6)) as instorage_number,
        CONVERT("0.0", DECIMAL(16,6))  as instorage_weight,
        CONVERT("0.0", DECIMAL(16,6)) as instorage_amount,
        CONVERT("0.0", DECIMAL(16,6)) as cost_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as recieve_all_amount,
		CONVERT(SUM(b.back_number), DECIMAL(16,6))  as back_number,
        CONVERT(SUM(b.back_weight), DECIMAL(16,6))  as back_weight ,
        CONVERT(SUM(b.back_money), DECIMAL(16,6))  as back_money,
        CONVERT("0.0", DECIMAL(16,6)) as pay_amount,
        CONVERT("0.0", DECIMAL(16,6)) as invoice_amount
        FROM order_info as a
        left JOIN
        (SELECT c.order_id, d.back_number,d.back_weight,c.back_money,c.back_status,c.unit_money FROM order_after_sale as c LEFT JOIN order_after_sale_product as d ON c.id = d.after_sale_id where c.back_status = 2)
        as b ON a.id = b.order_id
        WHERE a.company_code = '{}' {} and (a.order_status = 2 or a.order_status=3) GROUP BY supply_dept

        UNION ALL

        SELECT  a.supply_dept as supply_dept,
        CONVERT("0.0", DECIMAL(16,6)) as buy_amount ,
        CONVERT("0.0", DECIMAL(16,6)) as buy_number,
        CONVERT("0.0", DECIMAL(16,6))  as buy_weight,
		CONVERT("0.0", DECIMAL(16,6)) as instorage_number,
        CONVERT("0.0", DECIMAL(16,6))  as instorage_weight,
        CONVERT("0.0", DECIMAL(16,6)) as instorage_amount,
		CONVERT("0.0", DECIMAL(16,6)) as cost_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as recieve_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as back_number,
        CONVERT("0.0", DECIMAL(16,6)) as back_weight,
        CONVERT("0.0", DECIMAL(16,6)) as back_money,
		CONVERT(SUM(b.pay_amount), DECIMAL(16,6)) as pay_amount,
        CONVERT(SUM(b.invoice_amount), DECIMAL(16,6)) as invoice_amount
        FROM order_info as a
        left JOIN (select * from payed_info where pay_status =  2) as b ON a.id = b.order_id
        WHERE a.company_code = '{}' {} and (a.order_status = 2 or a.order_status=3) GROUP BY supply_dept
        ) as t GROUP BY t.supply_dept
        "#,
            params.company_code.clone().unwrap(),
            date_query,
            params.company_code.clone().unwrap(),
            date_query,
            params.company_code.clone().unwrap(),
            date_query,
            params.company_code.clone().unwrap(),
            date_query,
            params.company_code.clone().unwrap(),
            date_query
        );

        let company_code_arg = as_bson!(params.company_code.clone().unwrap());
        let pay_list: Vec<PayedStatisticItem> = rb.fetch(query.as_str(), vec![company_code_arg]).await.unwrap();

        Ok(pay_list)
    }

    pub async fn get_sale_statical_order(&self, params: SaleStaticalParams) -> WyResult<Vec<SaleStatisticItem>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        println!("params:{:?}", params);
        let mut date_query = "".to_string();
        if params.sale_date_end.is_some() && params.sale_date_start.is_some() {
            date_query = format!(
                r#" and a.sale_date between '{}' and '{}' "#,
                params.sale_date_start.clone().unwrap(),
                params.sale_date_end.clone().unwrap()
            );
        } else {
            if params.sale_date_end.is_some() {
                date_query = format!(r#" and a.sale_date <= '{}' "#, params.sale_date_end.unwrap());
            }
            if params.sale_date_start.is_some() {
                date_query = format!(r#" and a.sale_date >= '{}' "#, params.sale_date_start.unwrap());
            }
        }

        let query = format!(
            r#"

        SELECT client_name,SUM(sale_amount) as sale_amount, SUM(sale_number) as sale_number, SUM(sale_weight) as sale_weight, SUM(outstorage_number) as outstorage_number, SUM(outstorage_weight) as outstorage_weight,
        SUM(cost_all_amount) as cost_all_amount, SUM(recieve_all_amount) as recieve_all_amount, SUM(back_number) as back_number, SUM(back_weight) as back_weight, SUM(back_amount) as back_amount,
        SUM(recieve_amount) as recieve_amount,SUM(invoice_amount) as invoice_amount  FROM (

        SELECT  client_name,
        CONVERT(SUM(b.unit_price*b.sale_weight), DECIMAL(16,6)) as sale_amount,
        CONVERT(SUM(b.sale_number), DECIMAL(16,6)) as sale_number,
        CONVERT(SUM(b.sale_weight), DECIMAL(16,6)) as sale_weight,
        CONVERT("0.0", DECIMAL(16,6)) as outstorage_number,
        CONVERT("0.0", DECIMAL(16,6)) as outstorage_weight,
        CONVERT("0.0", DECIMAL(16,6)) as cost_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as recieve_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as back_number,
        CONVERT("0.0", DECIMAL(16,6)) as back_weight,
        CONVERT("0.0", DECIMAL(16,6)) as back_amount,
        CONVERT("0.0", DECIMAL(16,6)) as recieve_amount,
        CONVERT("0.0", DECIMAL(16,6)) as invoice_amount
        FROM sale_info as a
        left JOIN sale_product as b ON a.id = b.sale_id
        WHERE a.company_code = '{}' {} and (a.sale_status = 2 or a.sale_status=3) GROUP BY client_name

        UNION ALL

        SELECT  client_name,
        CONVERT("0.0", DECIMAL(16,6)) as sale_amount,
        CONVERT("0.0", DECIMAL(16,6)) as sale_number,
        CONVERT("0.0", DECIMAL(16,6)) as sale_weight,
        CONVERT(SUM(b.sale_number), DECIMAL(16,6)) as outstorage_number,
        CONVERT(SUM(b.sale_weight), DECIMAL(16,6)) as outstorage_weight ,
        CONVERT("0.0", DECIMAL(16,6)) as cost_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as recieve_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as back_number,
        CONVERT("0.0", DECIMAL(16,6)) as back_weight,
        CONVERT("0.0", DECIMAL(16,6)) as back_amount,
        CONVERT("0.0", DECIMAL(16,6)) as recieve_amount,
        CONVERT("0.0", DECIMAL(16,6)) as invoice_amount
        FROM sale_info as a
        left JOIN (select * from sale_product where sale_status =2 or sale_status = 3 or sale_status = 4) as b ON a.id = b.sale_id
        WHERE a.company_code = '{}' {} and (a.sale_status = 2 or a.sale_status=3)  GROUP BY client_name

        UNION ALL

        SELECT client_name,
        CONVERT("0.0", DECIMAL(16,6)) as sale_amount,
        CONVERT("0.0", DECIMAL(16,6)) as sale_number,
        CONVERT("0.0", DECIMAL(16,6)) as sale_weight,
        CONVERT("0.0", DECIMAL(16,6)) as outstorage_number,
        CONVERT("0.0", DECIMAL(16,6)) as outstorage_weight,
        CONVERT(SUM(b.cost_amount), DECIMAL(16,6)) as cost_all_amount,
        CONVERT(SUM(b.cost_amount), DECIMAL(16,6)) as recieve_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as back_number,
        CONVERT("0.0", DECIMAL(16,6)) as back_weight,
        CONVERT("0.0", DECIMAL(16,6)) as back_amount,
        CONVERT("0.0", DECIMAL(16,6)) as recieve_amount,
        CONVERT("0.0", DECIMAL(16,6)) as invoice_amount
        FROM sale_info as a
        left JOIN (select * from order_trans where trans_status = 2 ) as b ON a.id = b.sale_id
        WHERE a.company_code = '{}' {} and (a.sale_status = 2 or a.sale_status=3) GROUP BY client_name

        UNION ALL

        SELECT client_name,
        CONVERT("0.0", DECIMAL(16,6)) as sale_amount,
        CONVERT("0.0", DECIMAL(16,6)) as sale_number,
        CONVERT("0.0", DECIMAL(16,6)) as sale_weight,
        CONVERT("0.0", DECIMAL(16,6)) as outstorage_number,
        CONVERT("0.0", DECIMAL(16,6)) as outstorage_weight,
        CONVERT(SUM(b.back_number), DECIMAL(16,6)) as back_number,
        CONVERT(SUM(b.back_weight), DECIMAL(16,6)) as back_weight,
        CONVERT(SUM(b.back_amount), DECIMAL(16,6)) as back_amount ,
        CONVERT("0.0", DECIMAL(16,6)) as cost_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as recieve_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as recieve_amount,
        CONVERT("0.0", DECIMAL(16,6)) as invoice_amount
        FROM sale_info as a
        left JOIN
        (SELECT c.sale_id, d.back_number,d.back_weight,c.back_amount,c.unit_money FROM sale_after as c LEFT JOIN sale_product as d ON c.id = d.sale_after_id)
        as b ON a.id = b.sale_id
        WHERE a.company_code = '{}' {} and (a.sale_status = 2 or a.sale_status=3) GROUP BY client_name

        UNION ALL

        SELECT client_name,
        CONVERT("0.0", DECIMAL(16,6)) as sale_amount,
        CONVERT("0.0", DECIMAL(16,6)) as sale_number,
        CONVERT("0.0", DECIMAL(16,6)) as sale_weight,
        CONVERT("0.0", DECIMAL(16,6)) as outstorage_number,
        CONVERT("0.0", DECIMAL(16,6)) as outstorage_weight,
        CONVERT("0.0", DECIMAL(16,6)) as cost_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as recieve_all_amount,
        CONVERT("0.0", DECIMAL(16,6)) as back_number,
        CONVERT("0.0", DECIMAL(16,6)) as back_weight,
        CONVERT("0.0", DECIMAL(16,6)) as back_amount,
        CONVERT(SUM(b.recieve_amount), DECIMAL(16,6)) as recieve_amount ,
        CONVERT(SUM(b.invoice_amount), DECIMAL(16,6)) as invoice_amount
        FROM sale_info as a
        left JOIN recieved_info as b ON a.id = b.sale_id
        WHERE a.company_code = '{}' {} and (a.sale_status = 2 or a.sale_status=3) and b.recieve_status = 2  GROUP BY client_name

        ) as t GROUP BY t.client_name

        "#,
            params.company_code.clone().unwrap(),
            date_query,
            params.company_code.clone().unwrap(),
            date_query,
            params.company_code.clone().unwrap(),
            date_query,
            params.company_code.clone().unwrap(),
            date_query,
            params.company_code.clone().unwrap(),
            date_query
        );

        let company_code_arg = as_bson!(params.company_code.clone().unwrap());
        let sale_list: Vec<SaleStatisticItem> = rb.fetch(query.as_str(), vec![company_code_arg]).await.unwrap();

        Ok(sale_list)
    }
}
