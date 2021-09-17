extern crate ndarray;

use ndarray::prelude::*;
use ndarray::{stack,Zip};
use ndarray_stats::QuantileExt;

use itertools::Itertools;
use std::iter::FromIterator;

use rand::prelude::*;

// Params
const T: i32 = 20; // time horizon
const GAMMA: f64 = 1.0;
const PRUNE_RES: f64 = 0.0001;

// pub fn play() {
//     let r = random::<f64>();
//     if r <=
// }

pub fn create_value_map() {
    let pz: Array2<f64> = array![[0.7, 0.3], [0.3, 0.7]]; // measurement probabilities
    let pt: Array2<f64> = array![[0.2, 0.8], [0.8, 0.2]]; // transition probabilities
    let y0: Array2<f64> = array![[-100., 100.], [100., -50.]];

    let prune_rng = Array::range(0., PRUNE_RES+1., PRUNE_RES).insert_axis(Axis(0));
    let probs = stack![ Axis(0), prune_rng, prune_rng.slice(s![.., ..;-1]) ];
    let mut y: Array2<f64> = Array2::<f64>::zeros((1, 2));

    for _ in 0..T {
        y = sense(&y, &pz);
        y = prune(&y, &probs);
        y = predict(&y, &pt, &y0);
        y = prune(&y, &probs);
    }

    println!("{:?}", y);
}

fn sense(y: &Array2<f64>, pz: &Array2<f64>) -> Array2<f64> {
    let y1 = y * &pz.column(0);
    let y2 = y * &pz.column(1);

    let rows = y.nrows();
    let mut y_new: Array2<f64> = Array2::<f64>::zeros((rows.pow(2), 2));
    for i in 0..rows {
        let start: usize = &i*rows;
        let end: usize = &(i+1)*rows;
        let res = &y1 + &y2.row(i);
        y_new.slice_mut(s![start..end, ..]).assign(&res);
    }
    y_new
}

fn predict(y: &Array2<f64>, pt: &Array2<f64>, y0: &Array2<f64>) -> Array2<f64> {
    let rows = y.nrows();
    let mut y_new: Array2<f64> = Array2::<f64>::zeros((rows+2, 2));

    let v1 = (y.dot(pt) - 1.)*GAMMA;
    y_new.slice_mut(s![..2, ..]).assign(&y0);
    y_new.slice_mut(s![2.., ..]).assign(&v1);

    y_new
}

fn prune(y: &Array2<f64>, probs: &Array2<f64>) -> Array2<f64> {
    let lines = y.dot(probs);
    let mut args: Array1<i32> = Array1::zeros(lines.ncols());
    Zip::from(&mut args)
        .and(lines.gencolumns())
        .apply( |arg, col| *arg = col.argmax().unwrap() as i32 );

    let indexes = Array::from_iter(args.into_iter().unique());
    let mut y_new: Array2<f64> = Array2::<f64>::zeros((indexes.len(), 2));

    Zip::from(y_new.genrows_mut())
         .and(&indexes)
         .apply( |mut y_new_row, index| y_new_row.assign(&y.slice(s![**index, ..])) );

    y_new
}