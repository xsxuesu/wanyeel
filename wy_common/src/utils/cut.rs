use wy_domain::dto::cut_models::{CutSolver,ResultSolver,SubResultSolver};
use wy_domain::dto::cut_models_2d::{CutAreaSolver,ResultSolver as ResultSolver2d,error,PieceSolver,SubResultSolver as SubResultSolver2d};
use std::result::Result;
use cut_optimizer_1d::Optimizer as Optimizer1d;
use cut_optimizer_1d::{StockPiece,CutPiece};
use cut_optimizer_2d::Optimizer as Optimizer2d;
use cut_optimizer_2d::{StockPiece as StockPiece2d,CutPiece as CutPiece2d,PatternDirection};
use http::StatusCode;
// 计算最优方案
#[warn(unused_variables)]
pub fn solve_cut_optimize_for_service( slover: CutSolver) -> Result<ResultSolver,cut_optimizer_1d::Error> {
    let mut result_info = ResultSolver::default();
    let mut solution = Optimizer1d::new();
    // 添加母卷的信息
    for ele in slover.parent_rolls().iter() {
        solution.add_stock_piece(StockPiece{
            length:ele.width-slover.out_side,
            weight:ele.weight,
            quantity:Some(ele.quantity),
        });
    }
    // 添加子卷的信息
    for (index,ele)  in slover.child_rolls().iter().enumerate() {
        solution.add_cut_piece(CutPiece{
            length:ele.width,
            external_id:Some(index),
            quantity:ele.quantity,
        });
    }
    // 计算切割方案
   let optimizer = solution
    .set_cut_width(slover.side)
    .set_random_seed(slover.seed as u64)
    .allow_mixed_stock_sizes(true)
    .optimize(|_| {});

    match optimizer  {
        Ok(optimizer) => {
            let mut result_cut_pieces = vec![];
            let mut used_stock = 0;
            // 设置子卷长度数组
            for _ in slover.child_rolls().iter() {
                result_info.sub_weights.push(0);
            }
            // 循环计算切割捆包的值
            for ele in optimizer.stock_pieces {
                used_stock += 1;
                let mut e_cut_piece = SubResultSolver::default();
                let mut remain_length = ele.length;
                let mut all_sub_len = vec![];
                let mut all_sub_weight = vec![];
                for e in ele.cut_pieces {
                    all_sub_len.push(e.length);
                    all_sub_weight.push(e.weight);
                    result_info.sub_weights[e.external_id.unwrap()] += e.weight;
                    remain_length -= e.length;
                }
                e_cut_piece.set_un_used(remain_length);
                let un_used_weight =  ((remain_length as f32 / ele.length as f32) * ele.weight as f32) as usize;
                e_cut_piece.set_un_used_weight(un_used_weight);
                e_cut_piece.set_subs(all_sub_len);
                e_cut_piece.set_sub_weights(all_sub_weight);
                e_cut_piece.set_parent_length(ele.length);
                e_cut_piece.set_parent_weight(ele.weight);
                result_cut_pieces.push(e_cut_piece);
            }
            // 赋值 返回
            result_info.set_status_name("OPTIMAL".to_string());
            result_info.set_num_unique_solutions("1".to_string());
            result_info.set_num_solutions("1".to_string());
            result_info.set_num_rolls_used(used_stock);
            result_info.set_solutions(result_cut_pieces);
            return Ok(result_info);
        },
        Err(err) => {
            return Err(err);
        }
    }
}

#[warn(unused_variables)]
pub fn solve_cut_2d_optimize_for_service(slover: CutAreaSolver) -> Result<ResultSolver2d,cut_optimizer_2d::Error>{
    let mut solution = Optimizer2d::new();
    let mut total_weight = 0;
    let mut total_area = 0;
    let mut area_list = vec![];
    let mut weight_list = vec![];
    // 添加母卷的信息
    for ele in slover.parent_areas().iter() {
        solution.add_stock_piece(StockPiece2d{
            length:ele.length,
            width:ele.width,
            weight:ele.weight,
            quantity:Some(ele.quantity),
            pattern_direction:PatternDirection::None
        });
        for _ in 0..ele.quantity().to_owned() {
            area_list.push(ele.length*ele.width);
            weight_list.push(ele.weight as f64);
        }
        total_weight += ele.weight * ele.quantity;
        total_area += ele.width * ele.length * ele.quantity;
    }
    // 添加子卷的信息

    for (index,ele)  in slover.child_areas().iter().enumerate() {
        if ele.quantity > Some(0) {
            solution.add_cut_piece(CutPiece2d{
                length:ele.length,
                width:ele.width,
                quantity:ele.quantity.unwrap(),
                external_id:Some(index),
                pattern_direction:PatternDirection::None,
                can_rotate:true,
            });
        }
    }
    // 计算切割方案
   let optimizer = solution
    .set_cut_width(slover.side)
    .set_random_seed(slover.seed as u64)
    .optimize_guillotine(|_| {})
    .map_err(|e| match e {
        cut_optimizer_2d::Error::NoFitForCutPiece(_) => error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Cut piece doesn't fit in any stock pieces",
        ),
    });

    match optimizer {
        Ok(solution) => {

            let mut result_info = ResultSolver2d::default();
            let mut result_cut_pieces = vec![];
            let mut used_stock = 0;


            // 设置子卷长度数组
            for _ in slover.child_areas().iter() {
                result_info.sub_weights.push(0);
            }

            // 循环计算切割捆包的值
            for ele in solution.stock_pieces {

                let mut e_cut_piece = SubResultSolver2d::default();
                let mut all_subs = vec![];
                let mut used_area = 0;
                let mut used_weight:f64 = 0.0;

                for e in ele.cut_pieces {
                    let mut piece = PieceSolver::default();
                    piece.x = e.x;
                    piece.y = e.y;
                    piece.length = e.length;
                    piece.width = e.width;
                    piece.weight = (e.length as f64)*(e.width as f64)/(total_area as f64) * (total_weight as f64);
                    used_weight += piece.weight;
                    used_area += e.length * e.width;

                    all_subs.push(piece);
                }
                e_cut_piece.set_subs(all_subs);
                e_cut_piece.set_used_area(used_area);
                e_cut_piece.set_unused_area(area_list[used_stock] - used_area);
                e_cut_piece.set_unused_weight(weight_list[used_stock] - used_weight);
                e_cut_piece.set_used_weight(used_weight);
                result_cut_pieces.push(e_cut_piece);
                used_stock += 1;
            }

            // 赋值 返回
            result_info.set_status_name("OPTIMAL".to_string());
            result_info.set_num_unique_solutions("1".to_string());
            result_info.set_num_solutions("1".to_string());
            result_info.set_num_rolls_used(used_stock);
            result_info.set_weight_used(solution.weight);
            result_info.set_solutions(result_cut_pieces);
            return Ok(result_info);
        },
        Err(_) => {
            let mut result_info = ResultSolver2d::default();
            result_info.set_status_name("Error".to_string());
            return Ok(result_info);
        },
    }
}