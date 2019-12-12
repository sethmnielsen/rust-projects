extern crate ndarray;

use ndarray::Array;
use ndarray::{arr3, Axis};
use std::iter::FromIterator;

fn main() {
    let a = Array::from_iter(0..28).into_shape((2, 7, 2)).unwrap();
    let mut iter = a.axis_chunks_iter(Axis(1), 2);

    // println!("{:?}", iter);
}
