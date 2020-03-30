
fn vec_check(slices: &Vec<Vec<i32>>) -> usize {
    slices.iter()
        .flatten()
        .filter(|&n| *n < 0)
        .count()
}

fn main() {
    // let mut grid = [[0i32; 5]; 5];
    // grid = [
    //     [1,2,3,-4,5],
    //     [6,-7,8,9,1],
    //     [2,3,4,-5,6],
    //     [7,-8,9,0,1],
    //     [2,-3,4,5,6]
    // ];
    //
    // let mut g2 = [
    //     [1,-2,3],
    //     [4,-5,6],
    //     [7,-8,9]
    // ];
    //
    let v1 = vec![
        vec![-1, 2, 3],
        vec![4, -5, 6],
        vec![-7, 8, -9]
    ];
    //let v2 = vec![1,-2,3,-4,5,6,7,-8];
    // let ncount = v1.iter()
    //     .flatten()
    //     .filter(|&n| *n < 0)
    //     .count();


    println!("negative count is {}", vec_check(&v1));
}
