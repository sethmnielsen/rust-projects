// fn main() {
//     let n_main: usize = 100;
//     let n_main_ref: &usize = &n_main;
//     println!("{}", inc_ref(n_main_ref));
//   }
  
//   fn inc_ref(n_inc_ref: &usize) -> usize {
//     n_inc_ref + 1
//   }

fn main() {
    let array_main: [Vec<u8>; 3] = [vec![1], vec![2, 4], vec![]];
    let array_main_ref: &[Vec<u8>] = &array_main;
    print(array_main_ref);
}
  
fn print(array_ref: &[Vec<u8>]) {
    for e in array_ref.iter() {
        println!("{:?}", e)
    }
}


//extern crate ndarray;

//use ndarray::Array;
//use ndarray::{arr3, Axis};
//use std::iter::FromIterator;

//fn main() {
    //let a = Array::from_iter(0..28).into_shape((2, 7, 2)).unwrap();
    //let mut iter = a.axis_chunks_iter(Axis(1), 2);

     //println!("{:?}", iter);
//}
