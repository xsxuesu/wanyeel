use getset::*;
use serde::{Deserialize, Serialize};
use http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize,  Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct CutAreaSolver {
    pub child_areas: Vec<SubChildAreaSolver>,
    pub parent_areas: Vec<SubParentAreaSolver>,
    pub side: usize,
    pub seed: usize,
    pub percent: f32,
}

#[derive(Debug, Serialize, Deserialize,Copy, Clone, Getters, Setters, Default,PartialEq)]
#[getset(get = "pub", set = "pub")]
pub struct SubChildAreaSolver {
   pub width: usize,
   pub length: usize,
   pub quantity: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ChildSolverList {
   pub solutions: Vec<RuleSolutions> ,
}

#[derive(Debug, Serialize, Deserialize,Copy, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SubChildSolver {
   pub quantity: Option<usize>,
   pub width: usize,
   pub length: usize,
}

#[derive(Debug, Serialize, Deserialize, Copy,Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SubParentAreaSolver {
   pub quantity: usize,
   pub width: usize,
   pub length: usize,
   pub weight: usize,
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ResultSolver {
    pub status_name: String,
    pub num_solutions: String,
    pub num_unique_solutions: String,
    pub num_rolls_used: usize,
    pub weight_used: usize,
    pub area_used: usize,
    pub solutions: Vec<SubResultSolver>,
    pub sub_weights: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SubResultSolver {
    used_area: usize,
    unused_area: usize,
    used_weight: f64,
    unused_weight: f64,
    subs: Vec<PieceSolver>,
}
#[derive(Debug, Serialize, Deserialize,Copy, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct PieceSolver {
    pub x: usize,
    pub y: usize,
    pub length: usize,
    pub width: usize,
    pub weight: f64
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct ResultSolverList {
    solutions:Vec<ResultSolver>
}


#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct RuleSolutions {
    sub_child_solver: Vec<SubChildAreaSolver>,
    solutions:ResultSolver,
}

type OptimizeError = (StatusCode, Json<Value>);

pub fn error(status_code: StatusCode, message: &str) -> OptimizeError {
    (status_code, Json(json!({ "message": message })))
}

pub fn error_with_data<T: Serialize>(status_code: StatusCode, message: &str, data: T) -> OptimizeError {
    (
        status_code,
        Json(json!({ "message": message, "data": data })),
    )
}