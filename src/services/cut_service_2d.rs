use wy_domain::dto::cut_models_2d::{CutAreaSolver,error,ResultSolver,SubResultSolver,PieceSolver,ChildSolverList,RuleSolutions,SubChildAreaSolver};
use wy_domain::error::Result;
use cut_optimizer_2d::Optimizer;
use cut_optimizer_2d::{StockPiece,CutPiece,PatternDirection};
use std::rc::Rc;
use std::cell::RefCell;
use random_number::random;
use http::StatusCode;
use wy_common::utils::cut::solve_cut_2d_optimize_for_service;
use std::sync::{Arc, Mutex};
use std::thread;
use chrono::Local;
const MAX_SOLUTION : usize = 8;

pub struct AreaCutSolverService;
impl Default for AreaCutSolverService {
    fn default() -> Self {
        AreaCutSolverService {}
    }
}

#[warn(unused_variables)]
impl AreaCutSolverService {
    // 计算最优方案
    pub async fn solve_cut_optimize(&self, slover: CutAreaSolver) -> Result<ResultSolver> {
        let mut solution = Optimizer::new();
        let mut total_weight = 0;
        let mut total_area = 0;
        let mut area_list = vec![];
        let mut weight_list = vec![];
        // 添加母卷的信息
        for ele in slover.parent_areas().iter() {
            solution.add_stock_piece(StockPiece{
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
            if ele.quantity.is_some() && ele.quantity > Some(0) {
                solution.add_cut_piece(CutPiece{
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

                let mut result_info = ResultSolver::default();
                let mut result_cut_pieces = vec![];
                let mut used_stock = 0;


                // 设置子卷长度数组
                for _ in slover.child_areas().iter() {
                    result_info.sub_weights.push(0);
                }

                // 循环计算切割捆包的值
                for ele in solution.stock_pieces {

                    let mut e_cut_piece = SubResultSolver::default();
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
                let mut result_info = ResultSolver::default();
                result_info.set_status_name("Error".to_string());
                return Ok(result_info);
            },
        }
    }

    pub async fn com_optimize_quantity_by_area(&self,solver: &CutAreaSolver) -> Result<ChildSolverList> {

        let fmt = "%Y年%m月%d日 %H:%M:%S";
        let now = Local::now().format(fmt);
        println!("{}", now);

        let parent_all_valid_area = solver.parent_areas().iter().map(|parent|{
            parent.width() *parent.length()* parent.quantity()
        }).sum::<usize>();
        // 获得计算最小承受期望值
        let percent =solver.percent / 100.0;

        let parent_all_valid_area_min = ((parent_all_valid_area as f32) * percent) as usize;

        // 计算子卷的最大数量
        let mut childs_quantity = vec![];

        let  childs_quantity_zero = Rc::new(RefCell::new(vec![0;solver.child_areas().len()]));
        let mut con_childs_quantity = vec![];
        for ele in solver.child_areas().iter(){
            childs_quantity.push( (parent_all_valid_area as f32 / (ele.width() * ele.length()) as f32) as usize );
        }
        // 最多循环100000次
        for _ in 0..10000{
            for index in 0..childs_quantity.len(){
                let mut rng = random_number::rand::thread_rng();
                let n: usize = random!(0..childs_quantity[index], rng);
                childs_quantity_zero.borrow_mut()[index]=n;
            }
            // 计算子卷宽度 的长度 不小于 差额的5mm
            let mut com_childs_area_all = 0;
            for (index,quantity) in childs_quantity_zero.borrow_mut().iter().enumerate(){
                com_childs_area_all += solver.child_areas()[index].width() * solver.child_areas()[index].length() * quantity;
            }
            // 在有效的长度内计入有效的列表
            // println!("parent_all_valid_area_min:{}",parent_all_valid_area_min);
            // println!("com_childs_area_all:{}",com_childs_area_all);
            // println!("parent_all_valid_area:{}",parent_all_valid_area);
            if  parent_all_valid_area_min  < com_childs_area_all &&  com_childs_area_all <= parent_all_valid_area {
                if !con_childs_quantity.contains(childs_quantity_zero.borrow_mut().to_vec().as_ref()){
                    let mut has_zero = false;  // 如果有值为0 的情况，不记入列表
                    for (_,item) in childs_quantity_zero.borrow_mut().iter().enumerate() {
                        if *item == 0 {
                            has_zero = true;
                        }
                    }
                    if has_zero == false {
                        con_childs_quantity.push(childs_quantity_zero.borrow_mut().to_vec());
                    }

                }
            }
        }
        let mut result_list = vec![];
        let mut result_child_solvers = ChildSolverList::default();
        if con_childs_quantity.len() > 0 {
            for ele in con_childs_quantity.iter() {
                let  mut childs = vec![];
                for (index,quantity) in ele.iter().enumerate() {
                    let mut sub_child = SubChildAreaSolver::default();
                    sub_child.quantity = Some(*quantity);
                    sub_child.width = solver.child_areas()[index].width().to_owned();
                    sub_child.length = solver.child_areas()[index].length().to_owned();
                    childs.push(sub_child);
                }
                result_list.push(childs);
            }
        }
        println!("result_list:{:?}",result_list);
        println!("result_list len:{:?}",result_list.len());
        let mut valid_result_list:Vec<RuleSolutions> = vec![];
        // 检查计算结果是否有可行解
        if result_list.len() > 0 {
            for ele in result_list.iter() {
                let mut cut_solver_tmp = CutAreaSolver::default();
                let mut rule_solutions = RuleSolutions::default();
                cut_solver_tmp.set_child_areas(ele.to_vec());
                cut_solver_tmp.set_parent_areas(solver.parent_areas().to_vec());
                cut_solver_tmp.set_side(*(solver.side()));
                cut_solver_tmp.set_seed(*(solver.seed()));
                let cut_solver_result_tmp = solve_cut_2d_optimize_for_service(cut_solver_tmp);
                println!("cut_solver_result_tmp:{:?}",cut_solver_result_tmp);
                match cut_solver_result_tmp {
                    Ok(result) => {
                        // println!("result.status_name():{:?}",result.status_name());
                        if result.status_name() != "Error" {
                            rule_solutions.set_sub_child_solver(ele.to_vec());
                            rule_solutions.set_solutions(result);
                            valid_result_list.push(rule_solutions);
                        }
                    },
                    Err(err) =>{
                        println!("error:{:?}",err)
                    },
                }

            }
        }

        // 排序
        valid_result_list.sort_by(|a, b| {
           return a.solutions().area_used().cmp(&b.solutions().area_used());
        });

        result_child_solvers.set_solutions(valid_result_list);

        let fmt = "%Y年%m月%d日 %H:%M:%S";
        let now = Local::now().format(fmt);
        println!("{}", now);

        return Ok(result_child_solvers);
    }


    pub async fn com_optimize_quantity_by_area_concurrent(&self,solver: &CutAreaSolver) -> Result<ChildSolverList> {

        let fmt = "%Y年%m月%d日 %H:%M:%S";
        let now = Local::now().format(fmt);
        println!("{}", now);


        let parent_all_valid_area = solver.parent_areas().iter().map(|parent|{
            parent.width() *parent.length()* parent.quantity()
        }).sum::<usize>();

       // 获得计算最小承受期望值
       let percent =solver.percent / 100.0;

       let parent_all_valid_area_min = ((parent_all_valid_area as f32) * percent) as usize;

        // 计算子卷的最大数量
        let mut childs_quantity = vec![];

        let  childs_quantity_zero = Rc::new(RefCell::new(vec![0;solver.child_areas().len()]));
        let mut con_childs_quantity = vec![];
        for ele in solver.child_areas().iter(){
            childs_quantity.push( (parent_all_valid_area as f32 / (ele.width() * ele.length()) as f32) as usize );
        }
        // 最多循环100000次
        for _ in 0..10000{
            for index in 0..childs_quantity.len(){
                let mut rng = random_number::rand::thread_rng();
                let n: usize = random!(0..childs_quantity[index], rng);
                childs_quantity_zero.borrow_mut()[index]=n;
            }
            // 计算子卷宽度 的长度 不小于 差额的5mm
            let mut com_childs_area_all = 0;
            for (index,quantity) in childs_quantity_zero.borrow_mut().iter().enumerate(){
                com_childs_area_all += solver.child_areas()[index].width() * solver.child_areas()[index].length() * quantity;
            }
            // 在有效的长度内计入有效的列表
            // println!("parent_all_valid_area_min:{}",parent_all_valid_area_min);
            // println!("com_childs_area_all:{}",com_childs_area_all);
            // println!("parent_all_valid_area:{}",parent_all_valid_area);
            if  parent_all_valid_area_min  < com_childs_area_all &&  com_childs_area_all <= parent_all_valid_area {
                if !con_childs_quantity.contains(childs_quantity_zero.borrow_mut().to_vec().as_ref()){
                    let mut has_zero = false;  // 如果有值为0 的情况，不记入列表
                    for (_,item) in childs_quantity_zero.borrow_mut().iter().enumerate() {
                        if *item == 0 {
                            has_zero = true;
                        }
                    }
                    if has_zero == false {
                        con_childs_quantity.push(childs_quantity_zero.borrow_mut().to_vec());
                    }

                }
            }
        }
        let mut result_list = vec![];
        let mut result_child_solvers = ChildSolverList::default();
        if con_childs_quantity.len() > 0 {
            for ele in con_childs_quantity.iter() {
                let  mut childs = vec![];
                for (index,quantity) in ele.iter().enumerate() {
                    let mut sub_child = SubChildAreaSolver::default();
                    sub_child.quantity = Some(*quantity);
                    sub_child.width = solver.child_areas()[index].width().to_owned();
                    sub_child.length = solver.child_areas()[index].length().to_owned();
                    childs.push(sub_child);
                }

                // 判断是否重复，重复就不添加到待计算列表中
                let mut has_child = false;
                for items in result_list.clone() {
                    if items == childs {
                        has_child = true;
                    }
                }
                if !has_child {
                    result_list.push(childs);
                }
            }
        }

        let valid_result_list = Arc::new(Mutex::new(vec![]));
        println!("thread id : {:?} ,list:{:?}",thread::current().id(),result_list.len());
        // 检查计算结果是否有可行解
        if result_list.len() > 0 {

            let  r_l = result_list.split_at(result_list.len()/2);
            let  l_2 = r_l.0.split_at(r_l.0.len()/2);
            let  r_2 = r_l.1.split_at(r_l.1.len()/2);

            let l_2_0 = l_2.0.split_at(l_2.0.len()/2);
            let l_2_1 = l_2.1.split_at(l_2.1.len()/2);

            let r_2_0 = r_2.0.split_at(r_2.0.len()/2);
            let r_2_1 = r_2.1.split_at(r_2.1.len()/2);

            let mut handlers = vec![];

            for i in 0..8 {
                let mut list = vec![];
                match i {
                    0 => list = l_2_0.0.to_vec(),
                    1 => list = l_2_0.1.to_vec(),
                    2 => list = l_2_1.0.to_vec(),
                    3 => list = l_2_1.1.to_vec(),
                    4 => list = r_2_0.0.to_vec(),
                    5 => list = r_2_0.1.to_vec(),
                    6 => list = r_2_1.0.to_vec(),
                    _ => list = r_2_1.1.to_vec(),
                }

                let valid_result_list_clone = Arc::clone(&valid_result_list);
                let solver_clone = solver.clone();
                let handle = thread::spawn(move || {

                    let mut result_v = valid_result_list_clone.lock().unwrap();
                    println!("thread id : {:?} ,list:{:?}",thread::current().id(),list.len());
                    for ele in list.iter() {
                        let mut cut_solver_tmp = CutAreaSolver::default();
                        let mut rule_solutions = RuleSolutions::default();
                        cut_solver_tmp.set_child_areas(ele.to_vec());
                        cut_solver_tmp.set_parent_areas(solver_clone.parent_areas().to_vec());
                        cut_solver_tmp.set_side(*(solver_clone.side()));
                        cut_solver_tmp.set_seed(*(solver_clone.seed()));
                        let cut_solver_result_tmp = solve_cut_2d_optimize_for_service(cut_solver_tmp);
                        // println!("thread id : {:?} ,cut_solver_result_tmp:{:?}",thread::current().id(),cut_solver_result_tmp);
                        match cut_solver_result_tmp {
                            Ok(result) => {
                                // println!("result.status_name():{:?}",result.status_name());
                                if result.status_name() != "Error" {
                                    rule_solutions.set_sub_child_solver(ele.to_vec());
                                    rule_solutions.set_solutions(result);
                                    (*result_v).push(rule_solutions);
                                    // 为了减少计算耗时，超过8个就退出
                                    if (*result_v).len() >= MAX_SOLUTION {
                                        break;
                                    }
                                }
                            },
                            Err(err) =>{
                                println!("error:{:?}",err)
                            },
                        }

                    }
                });
                handlers.push(handle);
            }
            // 处理handlers
            for handle in handlers {
                handle.join().unwrap();
            }
        }
        // 排序
        let mut return_result = valid_result_list.lock().unwrap();
        (*return_result).sort_by(|a, b| {
           return a.solutions().area_used().cmp(&b.solutions().area_used());
        });
        result_child_solvers.set_solutions(return_result.to_vec());


        let fmt = "%Y年%m月%d日 %H:%M:%S";
        let now = Local::now().format(fmt);
        println!("{}", now);

        return Ok(result_child_solvers);
    }

}