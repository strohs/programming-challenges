use std::thread;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

fn merge_sort<T>(mut v: Vec<T>, n: usize) where T: Ord {

    fn split<T>(slice: &mut[T], n: usize) -> (&mut[T], &mut[T]) {
        if slice.len() < n {
            (slice, &mut [])
        } else {
            slice.split_at_mut(n)
        }
    }

    // split `arr` into mutable slices of length n
    let mut splits: Arc<Mutex<&mut[T]>>;
    let mut s = split(&mut v, n);

    splits.push(s.0);
    while !s.1.is_empty() {
        s = split(s.1, n);
        splits.push(s.0);
    }


    // sort each slice
    let mut handles = vec![];
    for slice in splits {
        let handle = thread::spawn(move || {
            slice.sort();
        });
        handles.push(handle);
    }
    // wait for each thread to finish
    for handle in handles {
        handle.join().unwrap();
    }

    //dbg!(&arr);

}


fn main() {
    let mut v = vec![9,7,5,2,0,8,6,4,3,1,99,88];

    merge_sort(&mut arr, 4);

    // res.iter_mut()
    //     .for_each(|slice| slice.sort());
    //
    // dbg!(&res);
}