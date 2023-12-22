use crate::services::storage::{StorageService,StorageAdjustService,StorageAdjustProductService, ProcessService,ProcessProductService, ProcessSolutionService};
use crate::services::order::OrderProductService;
use wy_domain::dto::storage::{StorageProductDTO,StorageListDTO,StorageFilterParams,StorageAdjustDTO,StorageProductPageFilterParams,StorageAdjustProductDTO,StorageAdjustPageDTO,ProcessPageDTO, ProcessDTO,ProcessProductDTO,ProcessSolutionDTO,StorageLogDTO};
use wy_domain::entity::storage::{StorageProduct,StorageAdjust,StorageAdjustProduct,Process,ProcessProduct,ProcessSolution};
use wy_domain::vo::RespVO;
use wy_domain::entity::PageData;
use wy_domain::request::{ByComQuery,ByUIDQuery,ByProcessProductCateQuery};
use crate::APPLICATION_CONTEXT;
use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
};
use crate::crud::crud_service::CrudService;


pub async fn save_storage_product(Json(arg): Json<StorageProductDTO>) -> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageService>();
    let result = service.save(&mut arg.into()).await;
    return RespVO::from_result(&result).resp_json();
}


pub async fn update_storage_product(Json(arg): Json<StorageProductDTO>) -> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageService>();
    let mut enetity:StorageProduct = arg.into();
    let result = service.update_by_id(enetity.id.unwrap().to_string(),&mut enetity).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_storage_product_by_warehouse(Json(arg): Json<StorageFilterParams>) -> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageService>();
    let result = service.get_storage_by_cate(arg).await;
    return RespVO::from_result(&result).resp_json();
}
pub async fn get_storage_product_by_variety(Json(arg): Json<StorageFilterParams>) -> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageService>();
    let result = service.get_storage_by_cate(arg).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_storage_product_list_by_cate(Json(arg):Json<StorageListDTO>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageService>();
    let result = service.list(&arg).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_storage_product_list_by_page(Json(arg):Json<StorageProductPageFilterParams>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageService>();
    let params = StorageListDTO {
        company_code:arg.company_code,
        warehouse:arg.warehouse,
        variety:None,
        origin:None,
        shop_sign:None,
        spec:None,
        unit:None,
        storage_number:Some(0.0),
        storage_weight:Some(0.0),
    };

    let page = PageData{
        page_no: arg.page_no,
        page_size: arg.page_size,
    };
    let result = service.page(&params,page).await;
    return RespVO::from_result(&result).resp_json();
}

// 库存调整
pub async fn save_storage_adjust(Json(arg):Json<StorageAdjustDTO>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageAdjustService>();
    let mut entity = arg.clone().into();
    let result = service.save(&mut entity).await;
// 调用更新
    if arg.adjust_status.is_some() && arg.adjust_status.unwrap() == 2 {
        if result.is_ok(){
            entity.id = Some(result.clone().unwrap() as u64);
            service.trans_adjust_relations(entity).await;
        }
    }

    return RespVO::from_result(&result).resp_json();
}

pub async fn update_storage_adjust(Json(arg):Json<StorageAdjustDTO>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageAdjustService>();
    let mut entity:StorageAdjust = arg.clone().into();
    let result = service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
    // 调用更新
    if arg.adjust_status.is_some() && arg.adjust_status.unwrap() == 2 {
        if result.is_ok(){
            service.trans_adjust_relations(entity).await;
        }
    }
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_storage_adjust(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageAdjustService>();
    let result = service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_storage_adjust_by_uid(Path(uid):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageAdjustService>();
    let result = service.fetch_list_by_column("company_code",&vec![uid]).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn del_storage_adjust(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageAdjustService>();
    let result = service.del(&id).await;
    if result.is_ok(){
        // 调用删除关联的表内容
        service.delete_adjust_relations(id.parse::<i64>().unwrap()).await;
    }
    return RespVO::from_result(&result).resp_json();
}


// 库存调整产品
pub async fn save_storage_adjust_product(Json(arg):Json<StorageAdjustProductDTO>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageAdjustProductService>();
    let mut entity = arg.into();
    let result = service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn update_storage_adjust_product(Json(arg):Json<StorageAdjustProductDTO>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageAdjustProductService>();
    let mut entity:StorageAdjustProduct = arg.into();
    let result = service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_storage_adjust_product(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageAdjustProductService>();
    let result = service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_storage_adjust_product_by_uid(Path(uid):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageAdjustProductService>();
    let result = service.fetch_list_by_column("company_code",&vec![uid]).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn del_storage_adjust_product(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageAdjustProductService>();
    let result = service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_storage_adjust_product_by_adjust_id(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageAdjustProductService>();
    let result = service.fetch_list_by_column("adjust_id",&vec![id]).await;
    return RespVO::from_result(&result).resp_json();
}
// 查询调入库产品
pub async fn get_product_by_adjust_id(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<OrderProductService>();
    let result = service.fetch_list_by_column("adjust_id",&vec![id]).await;
    return RespVO::from_result(&result).resp_json();
}

// 查询调入库产品
pub async fn get_storage_product_adjust_by_page(Json(arg):Json<StorageAdjustPageDTO>)-> impl IntoResponse {
    println!("arg: {:?}", arg);
    let service: &StorageAdjustService = APPLICATION_CONTEXT.get::<StorageAdjustService>();
    let page = PageData{
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };
    let result = service.page(&arg,page).await;
    return RespVO::from_result(&result).resp_json();
}
/****************************************************************
 * 加工
 *
 */

// 查询加工
pub async fn get_process_by_page(Json(arg):Json<ProcessPageDTO>)-> impl IntoResponse {
    let service: &ProcessService = APPLICATION_CONTEXT.get::<ProcessService>();
    let page = PageData{
        page_no: Some(arg.page_no().unwrap().into()),
        page_size: Some(arg.page_size().unwrap().into()),
    };
    let result = service.page(&arg,page).await;
    return RespVO::from_result(&result).resp_json();
}


pub async fn save_process(Json(arg):Json<ProcessDTO>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessService>();
    let mut entity:Process = arg.into();
    if entity.process_status.unwrap() == 2 { //加工后确认
        let result_trans = service.trans_process_relations(entity.id.unwrap()).await;
        if result_trans.is_ok() {
         let result = service.save( &mut entity).await;
         return RespVO::from_result(&result).resp_json();
        }else{
         return RespVO::from_result(&result_trans).resp_json();
        }
     }else{
         let result = service.save(&mut entity).await;
         return RespVO::from_result(&result).resp_json();
     }
}

pub async fn update_process(Json(arg):Json<ProcessDTO>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessService>();
    let mut entity:Process = arg.into();
    if entity.process_status.unwrap() == 2 { //加工后确认
       let result_trans = service.trans_process_relations(entity.id.unwrap()).await;
       if result_trans.is_ok() {
        let result = service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
        return RespVO::from_result(&result).resp_json();
       }else{
        return RespVO::from_result(&result_trans).resp_json();
       }
    }else{
        let result = service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
        return RespVO::from_result(&result).resp_json();
    }
}

pub async fn del_process(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessService>();
    let result = service.del(&id).await;
    if result.is_ok() {
        service.delete_process_relations(id.parse::<u64>().unwrap()).await;
    }
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_process(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessService>();
    let result = service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

/****************************************************************
 * 加工库存
 */



pub async fn save_process_product(Json(arg):Json<ProcessProductDTO>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessProductService>();
    let mut entity:ProcessProduct = arg.into();
    let result = service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn update_process_product(Json(arg):Json<ProcessProductDTO>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessProductService>();
    let mut entity:ProcessProduct = arg.into();
    let result = service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn del_process_product(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessProductService>();
    let result = service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_process_product(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessProductService>();
    let result = service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_process_product_by_process_id(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessProductService>();
    let param = ByProcessProductCateQuery{
        process_id:Some(id.parse::<u64>().unwrap()),
        process_cate:Some(1),
    };
    let result = service.list(&param).await;
    return RespVO::from_result(&result).resp_json();
}
pub async fn get_processd_product_by_process_id(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessProductService>();
    let param = ByProcessProductCateQuery{
        process_id:Some(id.parse::<u64>().unwrap()),
        process_cate:Some(2),
    };
    let result = service.list(&param).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_process_product_by_storage_id(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessProductService>();
    let result = service.fetch_list_by_column("storage_id",&vec![id]).await;
    return RespVO::from_result(&result).resp_json();
}

/********
 * 库存流水
 */

 pub async fn search_storage_log(Json(arg):Json<StorageLogDTO>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<StorageService>();
    let result = service.search_storage_log(arg).await;
    return RespVO::from_result(&result).resp_json();
}


/****************************************************************
 * 加工方案
 */



 pub async fn save_process_solution(Json(arg):Json<ProcessSolutionDTO>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessSolutionService>();
    let mut entity = arg.into();
    let result = service.save(&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn update_process_solution(Json(arg):Json<ProcessSolutionDTO>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessSolutionService>();
    let mut entity: ProcessSolution = arg.into();
    let result = service.update_by_id(entity.id.unwrap().to_string(),&mut entity).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn del_process_solution(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessSolutionService>();
    let result = service.del(&id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_process_solution(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessSolutionService>();
    let result = service.get(id).await;
    return RespVO::from_result(&result).resp_json();
}

pub async fn get_process_solution_by_process_id(Path(id):Path<String>)-> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<ProcessSolutionService>();
    let result = service.get_by("process_id".to_string(),id).await;
    return RespVO::from_result(&result).resp_json();
}