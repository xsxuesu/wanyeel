use crate::APPLICATION_CONTEXT;
use crate::config::config::ApplicationConfig;
use crate::services::finance::InvoiceService;
use crate::services::order::OrderService;
use crate::services::sale::SalePayService;
use crate::services::storage::ProcessSolutionService;
use crate::services::storage::StorageAdjustProductService;
use crate::services::storage::StorageAdjustService;
use crate::services::user::UserService;
use tokio::fs::read_to_string;
use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::ZipPacker;
use std::time::Duration;
use crate::orm::init_rbatis;
use rbatis::rbatis::Rbatis;
use log::info;
use crate::cache::cache::CacheService;
use crate::services::cut_service::CutSolverService;
use crate::services::cut_service_2d::AreaCutSolverService;
use crate::services::setting::*;
use crate::services::agree::*;
use crate::services::order::*;
use crate::services::user::*;
use crate::services::storage::StorageService;
use crate::services::pay::OrderPayService;
use crate::services::sale::SaleInfoService;
use crate::services::sale::SaleProductService;
use crate::services::sale::SaleAfterService;
use crate::services::sale::PreSaleProductService;
use crate::services::storage::{ProcessService,ProcessProductService};
use crate::services::finance::{PayedInfoService,ReceiveInfoService,StaticalService};
//初始化配置信息
pub async fn init_config() {
    let content = read_to_string("application.yaml").await.unwrap();
    let mut config = ApplicationConfig::new(content.as_str());
    let list = config.admin_auth_list_api().clone();
    /*添加需要登录但是不需要权限的路由
     * 如果有额外的可以在application.yml中添加
     * admin_auth_list_api
     *  - XXXXXX
     *  - XXXXX
     * */
    // list.push(Some("/user/info".to_string()));
    // list.push("/dict/type/all".to_string());

    config.set_admin_auth_list_api(list);

    APPLICATION_CONTEXT.set::<ApplicationConfig>(config);
}

pub fn init_log() {
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //create log dir
    std::fs::create_dir_all(&cassie_config.log_dir());
    //initialize fast log
    fast_log::init(
        Config::new()
            .console()
            .file_split(
                &cassie_config.log_dir(),
                str_to_temp_size(&cassie_config.log_temp_size()),
                str_to_rolling(&cassie_config.log_rolling_type()),
                ZipPacker {},
            )
            .level(log::LevelFilter::Info),
    )
    .unwrap();
}


fn str_to_temp_size(arg: &str) -> LogSize {
    match arg {
        arg if arg.ends_with("MB") => {
            let end = arg.find("MB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::MB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("KB") => {
            let end = arg.find("KB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::KB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("GB") => {
            let end = arg.find("GB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::GB(num.parse::<usize>().unwrap())
        }
        _ => LogSize::MB(100),
    }
}

fn str_to_rolling(arg: &str) -> RollingType {
    match arg {
        arg if arg.starts_with("KeepNum(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepNum(".len()..end].to_string();
            RollingType::KeepNum(num.parse::<i64>().unwrap())
        }
        arg if arg.starts_with("KeepTime(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepTime(".len()..end].to_string();
            RollingType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()))
        }
        _ => RollingType::All,
    }
}

fn str_to_log_level(arg: &str) -> log::Level {
    return match arg {
        "warn" => log::Level::Warn,
        "error" => log::Level::Error,
        "trace" => log::Level::Trace,
        "info" => log::Level::Info,
        "debug" => log::Level::Debug,
        _ => log::Level::Info,
    };
}



pub async fn init_database() {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();

    let rbatis = init_rbatis(config).await;
    info!("link database success!{}", config.database_url());
    APPLICATION_CONTEXT.set::<Rbatis>(rbatis);
}



pub async fn init_service() {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    APPLICATION_CONTEXT.set::<CacheService>(CacheService::new().unwrap());
    info!("CacheService init success!");
    APPLICATION_CONTEXT.set::<CutSolverService>(CutSolverService::default());
    info!("CutSolverService init success!");
    APPLICATION_CONTEXT.set::<AreaCutSolverService>(AreaCutSolverService::default());
    info!("AreaCutSolverService init success!");
    APPLICATION_CONTEXT.set::<VarietyComService>(VarietyComService::default());
    info!("VarietyComService init success!");
    APPLICATION_CONTEXT.set::<SepcService>(SepcService::default());
    info!("SepcService init success!");
    APPLICATION_CONTEXT.set::<ShopSignService>(ShopSignService::default());
    info!("ShopSignService init success!");
    APPLICATION_CONTEXT.set::<VarietyService>(VarietyService::default());
    info!("VarietyService init success!");
    APPLICATION_CONTEXT.set::<OriginService>(OriginService::default());
    info!("OriginService init success!");
    APPLICATION_CONTEXT.set::<OriginComService>(OriginComService::default());
    info!("OriginComService init success!");
    APPLICATION_CONTEXT.set::<PQualityService>(PQualityService::default());
    info!("PQualityService init success!");
    APPLICATION_CONTEXT.set::<PTypeService>(PTypeService::default());
    info!("PTypeService init success!");
    APPLICATION_CONTEXT.set::<PayModeService>(PayModeService::default());
    info!("PayModeService init success!");
    APPLICATION_CONTEXT.set::<PayModeComService>(PayModeComService::default());
    info!("VarietyComService init success!");
    APPLICATION_CONTEXT.set::<RoleService>(RoleService::default());
    info!("RoleService init success!");
    APPLICATION_CONTEXT.set::<StorageService>(StorageService::default());
    info!("StorageService init success!");
    APPLICATION_CONTEXT.set::<StorageComService>(StorageComService::default());
    info!("StorageComService init success!");
    APPLICATION_CONTEXT.set::<SupplyComService>(SupplyComService::default());
    info!("SupplyComService init success!");
    APPLICATION_CONTEXT.set::<NoticeService>(NoticeService::default());
    info!("NoticeService init success!");
    APPLICATION_CONTEXT.set::<FackbackService>(FackbackService::default());
    info!("FackdbackService init success!");
    APPLICATION_CONTEXT.set::<UserService>(UserService::default());
    info!("UserService init success!");
    APPLICATION_CONTEXT.set::<CompanyService>(CompanyService::default());
    info!("CompanyService init success!");
    APPLICATION_CONTEXT.set::<ClientComService>(ClientComService::default());
    info!("CompanyService init success!");
    APPLICATION_CONTEXT.set::<AgreeService>(AgreeService::default());
    info!("AgreeService init success!");
    APPLICATION_CONTEXT.set::<AgreeProductService>(AgreeProductService::default());
    info!("AgreeProductService init success!");
    APPLICATION_CONTEXT.set::<OrderService>(OrderService::default());
    info!("OrderService init success!");
    APPLICATION_CONTEXT.set::<OrderProductService>(OrderProductService::default());
    info!("OrderProductService init success!");
    APPLICATION_CONTEXT.set::<OrderTransService>(OrderTransService::default());
    info!("OrderTransService init success!");
    APPLICATION_CONTEXT.set::<StorageService>(StorageService::default());
    info!("StorageService init success!");
    APPLICATION_CONTEXT.set::<OrderPayService>(OrderPayService::default());
    info!("OrderPayService init success!");
    APPLICATION_CONTEXT.set::<AfterSaleService>(AfterSaleService::default());
    info!("AfterSaleService init success!");
    APPLICATION_CONTEXT.set::<AfterSaleProductService>(AfterSaleProductService::default());
    info!("AfterSaleProductService init success!");
    APPLICATION_CONTEXT.set::<StorageAdjustService>(StorageAdjustService::default());
    info!("StorageAdjustService init success!");
    APPLICATION_CONTEXT.set::<StorageAdjustProductService>(StorageAdjustProductService::default());
    info!("StorageAdjustProductService init success!");
    APPLICATION_CONTEXT.set::<ProcessService>(ProcessService::default());
    info!("ProcessService init success!");
    APPLICATION_CONTEXT.set::<ProcessProductService>(ProcessProductService::default());
    info!("ProcessProductService init success!");
    APPLICATION_CONTEXT.set::<SaleInfoService>(SaleInfoService::default());
    info!("SaleInfoService init success!");
    APPLICATION_CONTEXT.set::<SaleProductService>(SaleProductService::default());
    info!("SaleProductService init success!");
    APPLICATION_CONTEXT.set::<PreSaleProductService>(PreSaleProductService::default());
    info!("PreSaleProductService init success!");
    APPLICATION_CONTEXT.set::<SalePayService>(SalePayService::default());
    info!("SalePayService init success!");
    APPLICATION_CONTEXT.set::<SaleAfterService>(SaleAfterService::default());
    info!("SaleAfterService init success!");
    APPLICATION_CONTEXT.set::<PayedInfoService>(PayedInfoService::default());
    info!("PayedInfoService init success!");
    APPLICATION_CONTEXT.set::<ReceiveInfoService>(ReceiveInfoService::default());
    info!("ReceiveInfoService init success!");
    APPLICATION_CONTEXT.set::<ProcessSolutionService>(ProcessSolutionService::default());
    info!("ProcessSolutionService init success!");
    APPLICATION_CONTEXT.set::<StaticalService>(StaticalService::default());
    info!("StaticalService init success!");
    APPLICATION_CONTEXT.set::<InvoiceService>(InvoiceService::default());
    info!("InvoiceService init success!");
}
