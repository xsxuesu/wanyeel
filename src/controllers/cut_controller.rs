
use wy_domain::dto::cut_models::{CutSolver,CutLenSolver};
use wy_domain::dto::cut_models_2d::CutAreaSolver;
use crate::services::cut_service::CutSolverService;
use crate::services::cut_service_2d::AreaCutSolverService;
use wy_domain::vo::RespVO;
use crate::APPLICATION_CONTEXT;
use axum::response::IntoResponse;
use axum::Json;
pub const CODE_PURCHASE: i8 = -2;
//按照重量切割方案
pub async fn solve_cut_optimize_by_weight(Json(solver): Json<CutSolver>) -> impl IntoResponse {
    let cut_service = APPLICATION_CONTEXT.get::<CutSolverService>();
    println!("solver:{:?}",solver);
    let result = cut_service.solve_cut_optimize(solver).await;

    return RespVO::from_result(&result).resp_json();
}

//按照长度切割方案
pub async fn solve_cut_optimize_by_len(Json(solver): Json<CutLenSolver>) -> impl IntoResponse {
    let cut_service = APPLICATION_CONTEXT.get::<CutSolverService>();
    println!("solver:{:?}",solver);
    let result = cut_service.com_optimize_quantity_by_len(&solver).await;
    return RespVO::from_result(&result).resp_json();
}

//按照重量切割方案
pub async fn solve_cut_optimize_2d_by_weight(Json(solver): Json<CutAreaSolver>) -> impl IntoResponse {
    let cut_service_2d = APPLICATION_CONTEXT.get::<AreaCutSolverService>();

    let result = cut_service_2d.solve_cut_optimize(solver).await;
    return RespVO::from_result(&result).resp_json();
}

//按照面积切割方案
pub async fn solve_cut_optimize_2d_by_area(Json(solver): Json<CutAreaSolver>) -> impl IntoResponse {
    let cut_service_2d = APPLICATION_CONTEXT.get::<AreaCutSolverService>();
    let result = cut_service_2d.com_optimize_quantity_by_area_concurrent(&solver).await;
    return RespVO::from_result(&result).resp_json();
}