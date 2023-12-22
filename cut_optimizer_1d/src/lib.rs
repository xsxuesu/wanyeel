//! 试图找出的最佳切割
//! 从线材长度到线材长度的线性切割长度，浪费最少。
//! 它使用遗传算法和多重启发式算法来解决问题。

#![deny(missing_docs)]

mod basic;
mod genetic;

#[cfg(test)]
mod tests;

use basic::BasicBin;
use fnv::FnvHashSet;
use genetic::population::Population;
use genetic::unit::Unit;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::borrow::Borrow;
use std::cmp;
use std::hash::{Hash, Hasher};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// A rectangular piece that needs to be cut from a stock piece.
#[cfg_attr(feature = "serialize", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug)]
pub struct CutPiece {
    /// Quantity of this cut piece.
    pub quantity: usize,

    /// ID to be used by the caller to match up result cut pieces
    /// with the original cut piece. This ID has no meaning to the
    /// optimizer so it can be set to `None` if not needed.
    pub external_id: Option<usize>,

    /// Length of this cut piece.
    pub length: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct CutPieceWithId {
    pub(crate) id: usize,
    pub(crate) external_id: Option<usize>,
    pub(crate) length: usize,
}

impl Hash for CutPieceWithId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl PartialEq for CutPieceWithId {
    fn eq(&self, other: &CutPieceWithId) -> bool {
        self.id == other.id
    }
}
impl Eq for CutPieceWithId {}

#[derive(Clone, Debug)]
pub(crate) struct UsedCutPiece {
    pub(crate) id: usize,
    pub(crate) external_id: Option<usize>,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) weight: usize,
}

impl UsedCutPiece {
    pub(crate) fn length(&self) -> usize {
        self.end - self.start
    }
}

impl PartialEq for UsedCutPiece {
    fn eq(&self, other: &UsedCutPiece) -> bool {
        self.id == other.id
    }
}
impl Eq for UsedCutPiece {}

impl From<&UsedCutPiece> for CutPieceWithId {
    fn from(used_cut_piece: &UsedCutPiece) -> Self {
        Self {
            id: used_cut_piece.id,
            external_id: used_cut_piece.external_id,
            length: used_cut_piece.end - used_cut_piece.start,
        }
    }
}

impl From<&UsedCutPiece> for ResultCutPiece {
    fn from(used_cut_piece: &UsedCutPiece) -> Self {
        Self {
            external_id: used_cut_piece.external_id,
            start: used_cut_piece.start,
            end: used_cut_piece.end,
            length : used_cut_piece.end - used_cut_piece.start,
            weight: used_cut_piece.weight,
        }
    }
}

///被优化器放置中的切割
#[cfg_attr(feature = "serialize", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResultCutPiece {
    /// ID that matches the one on the cut piece that was passed to the optimizer.
    pub external_id: Option<usize>,

    /// Start location of this cut piece within the stock piece.
    pub start: usize,

    /// End location of this cut piece within the stock piece.
    pub end: usize,
    /// this cut piece weight.
    pub weight: usize,
    /// cut length
    pub length: usize,
}

///可切割一个或多个的卷
#[cfg_attr(feature = "serialize", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub struct StockPiece {
    /// Length of rectangular stock piece.
    pub length: usize,

    /// weight to use to optimize for weight when not all stock pieces are the same weight per unit
    /// length. If optimizing for less waste instead, weight can be set to 0 for all stock pieces.
    pub weight: usize,

    /// Quantity of this stock piece available for optimization. `None` means infinite quantity.
    pub quantity: Option<usize>,
}

impl StockPiece {
    /// Decrement the quantity of this stock piece. If quantity is `None` it will remain `None`.
    fn dec_quantity(&mut self) {
        if let Some(ref mut quantity) = self.quantity {
            *quantity -= 1;
        }
    }

    /// Increment the quantity of this stock piece. If quantity is `None` it will remain `None`.
    fn inc_quantity(&mut self) {
        if let Some(ref mut quantity) = self.quantity {
            *quantity += 1;
        }
    }
}


///优化器用来获取一个或多个切割件
#[cfg_attr(feature = "serialize", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug)]
pub struct ResultStockPiece {
    /// Length of this stock piece.
    pub length: usize,

    /// Cut pieces to cut from this stock piece.
    pub cut_pieces: Vec<ResultCutPiece>,

    /// weight of stock piece.
    pub weight: usize,
}

///打包的容器
trait Bin {
    /// new
    fn new(length: usize, blade_width: usize, weight: usize) -> Self;

    ///在0.0到1.0的范围内计算的适合度，其中1.0是最适合的。
    fn fitness(&self) -> f64;

    fn weight(&self) -> usize;

    fn length(&self) -> usize;

    ///从中移除 UsedCutPiece并返回移除的数量
    fn remove_cut_pieces<I>(&mut self, cut_pieces: I) -> usize
    where
        I: Iterator,
        I::Item: Borrow<UsedCutPiece>;

    ///返回一个遍历' UsedCutPiece的迭代器。
    fn cut_pieces(&self) -> std::slice::Iter<'_, UsedCutPiece>;

    ///插入' CutPieceWithId
    fn insert_cut_piece(&mut self, cut_piece: &CutPieceWithId) -> bool;

    ///返回' StockPiece '是否等 bin
    fn matches_stock_piece(&self, stock_piece: &StockPiece) -> bool;
}

struct OptimizerUnit<'a, B>
where
    B: Bin,
{
    bins: Vec<B>,

    possible_stock_pieces: &'a [StockPiece],

    //当前可用于新卷的库存件
    available_stock_pieces: Vec<StockPiece>,

    //切割不能添加到卷的碎片
    unused_cut_pieces: FnvHashSet<CutPieceWithId>,

    blade_width: usize,
}

impl<'a, B> Clone for OptimizerUnit<'a, B>
where
    B: Bin + Clone,
{
    fn clone(&self) -> Self {
        Self {
            bins: self.bins.clone(),
            possible_stock_pieces: self.possible_stock_pieces,
            available_stock_pieces: self.available_stock_pieces.to_vec(),
            unused_cut_pieces: self.unused_cut_pieces.clone(),
            blade_width: self.blade_width,
        }
    }
}

impl<'a, B> OptimizerUnit<'a, B>
where
    B: Bin,
{
    fn new<R>(
        possible_stock_pieces: &'a [StockPiece],
        cut_pieces: &[&CutPieceWithId],
        blade_width: usize,
        rng: &mut R,
    ) -> Result<OptimizerUnit<'a, B>>
    where
        R: Rng + ?Sized,
    {
        let mut unit = OptimizerUnit {
            bins: Vec::new(),
            possible_stock_pieces,
            available_stock_pieces: possible_stock_pieces.to_vec(),
            unused_cut_pieces: Default::default(),
            blade_width,
        };

        for cut_piece in cut_pieces {
            if !unit.insert_cut_piece(cut_piece, rng) {
                unit.unused_cut_pieces.insert((*cut_piece).clone());
            }
        }

        Ok(unit)
    }

    pub(crate) fn generate_initial_units(
        possible_stock_pieces: &'a [StockPiece],
        mut cut_pieces: Vec<&CutPieceWithId>,
        blade_width: usize,
        random_seed: u64,
    ) -> Result<Vec<OptimizerUnit<'a, B>>> {
        let length_set: FnvHashSet<usize> = cut_pieces.iter().map(|p| p.length).collect();
        let unique_cut_lengths = length_set.len();

        let num_units = {
            let denom = if cut_pieces.len() > 1 {
                (cut_pieces.len() as f64).log10()
            } else {
                1.0
            };

            cmp::max(
                10,
                (cut_pieces.len() as f64 / denom + ((unique_cut_lengths - 1) * 10) as f64) as usize,
            )
        };
        let mut units = Vec::with_capacity(num_units);
        let mut rng: StdRng = SeedableRng::seed_from_u64(random_seed);

        //
        cut_pieces.sort_by_key(|p| p.length);
        units.push(OptimizerUnit::new(
            possible_stock_pieces,
            &cut_pieces,
            blade_width,
            &mut rng,
        )?);

        // 根据长度倒序
        cut_pieces.sort_by_key(|p| cmp::Reverse(p.length));
        units.push(OptimizerUnit::new(
            possible_stock_pieces,
            &cut_pieces,
            blade_width,
            &mut rng,
        )?);

        // 随机排序
        for _ in 0..num_units - units.len() {
            cut_pieces.shuffle(&mut rng);
            units.push(OptimizerUnit::new(
                possible_stock_pieces,
                &cut_pieces,
                blade_width,
                &mut rng,
            )?);
        }

        Ok(units)
    }

    fn insert_cut_piece<R>(&mut self, cut_piece: &CutPieceWithId, rng: &mut R) -> bool
    where
        R: Rng + ?Sized,
    {
        for bin in self.bins.iter_mut() {
            if bin.insert_cut_piece(cut_piece) {
                return true;
            }
        }

        self.add_to_new_bin(cut_piece, rng)
    }

    fn add_to_new_bin<R>(&mut self, cut_piece: &CutPieceWithId, rng: &mut R) -> bool
    where
        R: Rng + ?Sized,
    {
        let stock_pieces = self
            .available_stock_pieces
            .iter_mut()
            .filter(|stock_piece| {
                stock_piece.quantity != Some(0) && cut_piece.length <= stock_piece.length
            });

        match stock_pieces.choose(rng) {
            Some(stock_piece) => {
                stock_piece.dec_quantity();

                let mut bin = B::new(stock_piece.length, self.blade_width, stock_piece.weight);
                if !bin.insert_cut_piece(cut_piece) {
                    return false;
                }
                self.bins.push(bin);
                true
            }
            None => false,
        }
    }

    fn crossover<R>(&self, other: &OptimizerUnit<'a, B>, rng: &mut R) -> OptimizerUnit<'a, B>
    where
        R: Rng + ?Sized,
        B: Clone,
    {
        //如果没有多个卷，我们就不能进行交叉，
        if self.bins.len() < 2 && other.bins.len() < 2 {
            return self.clone();
        }

        //随机选择插入点和卷的范围注入到新单元。
        let cross_dest = rng.gen_range(0..=self.bins.len());
        let cross_src_start = rng.gen_range(0..other.bins.len());
        let cross_src_end = rng.gen_range(cross_src_start + 1..=other.bins.len());

        //创建一个新单元，并注入self的bin和other的一些bin。
        let mut new_unit = OptimizerUnit {
            bins: (&self.bins[..cross_dest])
                .iter()
                .chain((&other.bins[cross_src_start..cross_src_end]).iter())
                .chain((&self.bins[cross_dest..]).iter())
                .cloned()
                .collect(),
            possible_stock_pieces: self.possible_stock_pieces,
            //使所有可能的库存件初始可用。数量将更新如下。
            available_stock_pieces: self.possible_stock_pieces.to_vec(),
            //浪费的块。
            unused_cut_pieces: Default::default(),
            blade_width: self.blade_width,
        };

        let mut unused_cut_pieces = self.unused_cut_pieces.clone();

        //根据注入卷更新可用库存件数量。
        other.bins[cross_src_start..cross_src_end]
            .iter()
            .for_each(|bin| {
                if let Some(ref mut stock_piece) = new_unit
                    .available_stock_pieces
                    .iter_mut()
                    .find(|sp| bin.matches_stock_piece(sp))
                {
                    // 从未使用的套装中取出
                    for cut_piece in bin.cut_pieces() {
                        unused_cut_pieces.remove(&cut_piece.into());
                    }
                    stock_piece.dec_quantity();
                } else {
                    panic!("Attempt to inject invalid bin in crossover operation. This shouldn't happen, and means there is a bug in the code.");
                }
            });

       //从所有未注入的容器中移除注入的切块
        //并更新可用的库存件数量。
        for i in (0..cross_dest)
            .chain((cross_dest + cross_src_end - cross_src_start)..new_unit.bins.len())
            .rev()
        {
            let bin = &mut new_unit.bins[i];
            let stock_piece = new_unit
                .available_stock_pieces
                .iter_mut()
                .find(|sp| sp.quantity != Some(0) && bin.matches_stock_piece(sp));
            let injected_cut_pieces = (&other.bins[cross_src_start..cross_src_end])
                .iter()
                .flat_map(Bin::cut_pieces);
            let num_removed_cut_pieces = bin.remove_cut_pieces(injected_cut_pieces);

            if let (0, Some(stock_piece)) = (num_removed_cut_pieces, stock_piece) {
                //我们没有从这个卷里取出任何碎片，所以我们将保留它
                //减少可用库存件的数量
                stock_piece.dec_quantity();
            } else {
                //要么没有可用的库存件，要么有
                //我们从这个卷里取出了一些碎片，所以我们将它移除
                //将其切割件添加到未使用的集合中
                for cut_piece in bin.cut_pieces() {
                    unused_cut_pieces.insert(cut_piece.into());
                }
                new_unit.bins.remove(i);
            }
        }

        //尝试添加所有未使用的切片。
        unused_cut_pieces.retain(|cut_piece| !new_unit.insert_cut_piece(cut_piece, rng));
        new_unit.unused_cut_pieces = unused_cut_pieces;

        //移除没有切的卷
        for i in (0..new_unit.bins.len()).rev() {
            if new_unit.bins[i].cut_pieces().next().is_none() {
                let bin = &mut new_unit.bins[i];
                if let Some(ref mut stock_piece) = new_unit
                    .available_stock_pieces
                    .iter_mut()
                    .find(|sp| sp.quantity != Some(0) && bin.matches_stock_piece(sp))
                {
                    //使库存件可再次使用
                    stock_piece.inc_quantity();
                }
                new_unit.bins.remove(i);
            }
        }

        new_unit
    }

  //随机应用一个变异到这个单位。
    fn mutate<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized,
    {
        if !self.bins.is_empty() && rng.gen_range(0..20) == 1 {
            self.inversion(rng)
        }
    }

    //卷的随机范围的反向顺序。
    fn inversion<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized,
    {
        let start = rng.gen_range(0..self.bins.len());
        let end = rng.gen_range(start..self.bins.len());
        self.bins[start..end].reverse();
    }
}

impl<'a, B> Unit for OptimizerUnit<'a, B>
where
    B: Bin + Send + Clone,
{
    fn fitness(&self) -> f64 {
        let fitness = if self.bins.is_empty() {
            0.0
        } else {
            self.bins.iter().fold(0.0, |acc, b| acc + b.fitness()) / self.bins.len() as f64
        };

        if self.unused_cut_pieces.is_empty() {
            fitness
        } else {
            //如果存在未使用的切割块，则适应度低于0，因为它不是有效的
            fitness - 1.0
        }
    }

    fn breed_with<R>(&self, other: &OptimizerUnit<'a, B>, rng: &mut R) -> OptimizerUnit<'a, B>
    where
        R: Rng + ?Sized,
    {
        let mut new_unit = self.crossover(other, rng);
        new_unit.mutate(rng);
        new_unit
    }
}

/// Err
#[derive(Debug)]
pub enum Error {
    ///没有有效的方案
    NoFitForCutPiece(CutPiece),
}
fn no_fit_for_cut_piece_error(cut_piece: &CutPieceWithId) -> Error {
    Error::NoFitForCutPiece(CutPiece {
        quantity: 1,
        external_id: cut_piece.external_id,
        length: cut_piece.length,
    })
}
type Result<T> = std::result::Result<T, Error>;

///一个有效的解决方案的优化。
#[cfg_attr(feature = "serialize", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct Solution {

///范围在0.0到1.0之间，其中1.0是没有浪费的完美解决方案。
    pub fitness: f64,

    ///用于此解决方案的库存件，每个包含需求件布局
    pub stock_pieces: Vec<ResultStockPiece>,
///重量
    #[cfg_attr(feature = "serialize", serde(skip))]
    weight: usize,
}

///从矩形中优化矩形切割块的优化器
pub struct Optimizer {
    stock_pieces: Vec<StockPiece>,
    cut_pieces: Vec<CutPieceWithId>,
    cut_width: usize,
    random_seed: u64,
    allow_mixed_stock_sizes: bool,
}

impl Default for Optimizer {
    fn default() -> Self {
        Self {
            stock_pieces: Default::default(),
            cut_pieces: Default::default(),
            cut_width: Default::default(),
            random_seed: Default::default(),
            allow_mixed_stock_sizes: true,
        }
    }
}

impl Optimizer {
    /// 创建一个默认的Optimizer
    pub fn new() -> Self {
        Default::default()
    }

   ///添加一个库存件，优化器可以使用它来优化切割件。
    ///如果同一库存件被添加多次，数量将会是

    pub fn add_stock_piece(&mut self, stock_piece: StockPiece) -> &mut Self {
        let mut existing_stock_piece = self
            .stock_pieces
            .iter_mut()
            .find(|sp| sp.length == stock_piece.length && sp.weight == stock_piece.weight);

        if let Some(ref mut existing_stock_piece) = existing_stock_piece {
            match (&mut existing_stock_piece.quantity, stock_piece.quantity) {
                (Some(ref mut existing_quantity), Some(quantity)) => {
                    //如果已存在除数量外相同的库存件，则添加
                    *existing_quantity += quantity;
                }
                _ => {

                    existing_stock_piece.quantity = None;
                }
            }
        } else {
            // 添加
            self.stock_pieces.push(stock_piece);
        }

        self
    }

///添加一个库存件，优化器可以使用它来优化切割件。
///如果同一库存件被添加多次，数量将会是
///总结。如果任何一个数量为“无”，则该数量在其他相等的数量上
    pub fn add_stock_pieces<I>(&mut self, stock_pieces: I) -> &mut Self
    where
        I: IntoIterator<Item = StockPiece>,
    {
        stock_pieces.into_iter().for_each(|sp| {
            self.add_stock_piece(sp);
        });
        self
    }

///添加一个你想要切割的
    pub fn add_cut_piece(&mut self, cut_piece: CutPiece) -> &mut Self {
        for _ in 0..cut_piece.quantity {
            let cut_piece = CutPieceWithId {
                id: self.cut_pieces.len(),
                external_id: cut_piece.external_id,
                length: cut_piece.length,
            };

            self.cut_pieces.push(cut_piece);
        }

        self
    }

///添加您需要从库存中切割的所需切割
    pub fn add_cut_pieces<I>(&mut self, cut_pieces: I) -> &mut Self
    where
        I: IntoIterator<Item = CutPiece>,
    {
        cut_pieces.into_iter().for_each(|dp| {
            self.add_cut_piece(dp);
        });
        self
    }

///设置在切割片之间使用的切割宽度
    pub fn set_cut_width(&mut self, cut_width: usize) -> &mut Self {
        self.cut_width = cut_width;
        self
    }

///在优化器中设置遗传算法使用的随机种子
    pub fn set_random_seed(&mut self, seed: u64) -> &mut Self {
        self.random_seed = seed;
        self
    }

///设置优化器是否允许混合大小的卷在结果中。
///如果设置为false，并且给定多个库存尺寸，则只使用一个库存尺寸
    pub fn allow_mixed_stock_sizes(&mut self, allow: bool) -> &mut Self {
        self.allow_mixed_stock_sizes = allow;
        self
    }

///执行优化
    pub fn optimize<F>(&self, progress_callback: F) -> Result<Solution>
    where
        F: Fn(f64),
    {
        // 没有没有cut piece 返回空
        if self.cut_pieces.is_empty() {
            return Ok(Solution {
                fitness: 1.0,
                stock_pieces: Vec::new(),
                weight: 0,
            });
        }

        let size_set: FnvHashSet<usize> = self.stock_pieces.iter().map(|sp| sp.length).collect();

        let num_runs = size_set.len() + if self.allow_mixed_stock_sizes { 1 } else { 0 };
        let callback = |progress| {
            progress_callback(progress / num_runs as f64);
        };

        let mut best_result = if self.allow_mixed_stock_sizes {
            //优化所有库存大小
            self.optimize_with_stock_pieces::<BasicBin, _>(&self.stock_pieces.clone(), &callback)
        } else {
            Err(no_fit_for_cut_piece_error(&self.cut_pieces[0]))
        };

        //分别优化每个卷大小，看看是否有比
        //当优化与所有卷大小。
        for (i, length) in size_set.iter().enumerate() {
            let stock_pieces: Vec<StockPiece> = self
                .stock_pieces
                .iter()
                .filter(|sp| sp.length == *length)
                .cloned()
                .collect();

            let completed_runs = i + 1;
            if let Ok(solution) =
                self.optimize_with_stock_pieces::<BasicBin, _>(&stock_pieces, &|progress| {
                    progress_callback((completed_runs as f64 + progress) / num_runs as f64);
                })
            {
                match best_result {
                    Ok(ref best_solution) => {
                        //使用较低权重的溶液，但如果权重相同，则使用
                        //解决方案具有较高的健身得分。
                        if solution.fitness < 0.0 || best_solution.fitness < 0.0 {
                            if solution.fitness > best_solution.fitness {
                                best_result = Ok(solution);
                            }
                        } else if solution.weight < best_solution.weight
                            || (solution.weight == best_solution.weight
                                && solution.fitness > best_solution.fitness)
                        {
                            best_result = Ok(solution);
                        }
                    }
                    Err(_) => best_result = Ok(solution),
                }
            }
        }

        if let Ok(ref mut solution) = &mut best_result {
            solution
                .stock_pieces
                .sort_by_key(|p| cmp::Reverse(p.length));
        };

        best_result
    }

    fn optimize_with_stock_pieces<B, F>(
        &self,
        stock_pieces: &[StockPiece],
        progress_callback: &F,
    ) -> Result<Solution>
    where
        B: Bin + Clone + Send + Into<ResultStockPiece>,
        F: Fn(f64),
    {
        let cut_pieces: Vec<&CutPieceWithId> = self.cut_pieces.iter().collect();

        let units: Vec<OptimizerUnit<B>> = OptimizerUnit::generate_initial_units(
            stock_pieces,
            cut_pieces,
            self.cut_width,
            self.random_seed,
        )?;

        let population_size = units.len();
        let mut result_units = Population::new(units)
            .set_size(population_size)
            .set_rand_seed(self.random_seed)
            .set_breed_factor(0.5)
            .set_survival_factor(0.6)
            .epochs(100, progress_callback)
            .finish();

        let best_unit = &mut result_units[0];
        if !best_unit.unused_cut_pieces.is_empty() {
            return Err(no_fit_for_cut_piece_error(
                best_unit.unused_cut_pieces.iter().next().unwrap(),
            ));
        }

        let fitness = best_unit.fitness();
        let weight = best_unit.bins.iter().map(|bin| bin.weight()).sum();

        let used_stock_pieces: Vec<ResultStockPiece> =
            best_unit.bins.drain(..).map(|piece|{
               let result :ResultStockPiece = piece.into();
               let len = result.length;
               let wei = result.weight;
               let result_pieces = result.cut_pieces.iter().map(|piece|{
                let weight = (((piece.end - piece.start) as f32 / len as f32) * wei as f32) as usize;

                ResultCutPiece{
                    weight:weight,
                    ..*piece
                }
               }).collect();

               ResultStockPiece{
                cut_pieces:result_pieces,
                ..result
               }
            }).collect();

        Ok(Solution {
            fitness,
            stock_pieces: used_stock_pieces,
            weight,
        })
    }
}

