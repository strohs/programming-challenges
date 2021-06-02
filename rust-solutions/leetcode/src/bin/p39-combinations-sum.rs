/// # 39. Combination Sum
/// Given an array of distinct integers candidates and a target integer target, return a list of
/// all unique combinations of candidates where the chosen numbers sum to target. You may return
/// the combinations in any order.
///
/// The same number may be chosen from candidates an unlimited number of times. Two combinations
/// are unique if the frequency of at least one of the chosen numbers is different.
/// It is guaranteed that the number of unique combinations that sum up to target is less than
/// 150 combinations for the given input.
///
/// ## Example 1
/// `Input: candidates = [2,3,6,7], target = 7`
/// `Output: [[2,2,3],[7]]`
///
///
/// candidates will always have at least one element


pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    use std::collections::HashSet;

    // pop from front of vd, until the sum of the popped elements is >= target
    // fn pop_until(vd: &mut VecDeque<i32>, target: i32) -> i32 {
    //     let mut sum = 0;
    //     while let Some(n) = vd.pop_front() {
    //         sum += n;
    //         if sum >= target {
    //             break;
    //         }
    //     }
    //     sum
    // }

    // temp storage for combinations
    let mut combos: HashSet<Vec<i32>> = HashSet::new();

    // filter out all numbers > target and then sort the remaining candidates
    let mut candidates: Vec<i32> = candidates.into_iter().filter(|n| *n <= target).collect();
    candidates.sort();

    // i is the index of the current number being checked for sum combos
    let mut i = 0_usize;

    while i < candidates.len() {
        // current combinations
        let mut cc: Vec<i32> = vec![];
        let mut sum: i32 = 0;
        // current number from candidates being examined
        let mut k = i;

        while k < candidates.len() {

            if sum < target {
                cc.push(candidates[k]);
                sum += candidates[k];
            } else if sum > target {
                // pop front
                if let Some(popped) = cc.get(0) {
                    sum -= *popped;
                    cc.remove(0);
                }
            } else {
                // sum == target, clone cc into combos
                combos.insert(cc.to_vec());

                // pop front
                if let Some(popped) = cc.get(0) {
                    sum -= *popped;
                    cc.remove(0);
                }

                // push next element
                if let Some(n) = candidates.get(k + 1) {
                    k += 1;
                    cc.push(*n);
                    sum += *n;
                }
            }
            if cc.is_empty() || candidates[i] != cc[0] {
                break;
            }
        }
        i += 1;
    }

    combos.into_iter().collect()
}

fn main() {}

// // sum == target, clone cc into combos
// let temp: Vec<i32> = cc.iter().copied().collect();
// combos.insert(temp);
//
// if let Some(n) = candidates.get(k + 1) {
// k += 1;
// let popped_sum = pop_until(&mut cc, *n);
// sum -= popped_sum;
// println!("dbg {:?}  cur_sum={}", &cc, sum);
// } else {
// // no next digit to add
// break;
// }

#[cfg(test)]
mod tests {
    use crate::combination_sum;

    #[test]
    fn ex1() {
        let candidates = vec![2,3,4];
        let target = 9;
        let combos = combination_sum(candidates, target);
        dbg!(&combos);
    }

    #[test]
    fn ex2() {
        let candidates = vec![1,2,3,4];
        let target = 9;
        let combos = combination_sum(candidates, target);
        dbg!(&combos);
    }

    #[test]
    fn ex3() {
        let candidates = vec![2];
        let target = 1;
        let combos = combination_sum(candidates, target);
        assert!(combos.is_empty());
    }

    #[test]
    fn ex4() {
        let candidates = vec![2,3,6,7];
        let target = 7;
        let combos = combination_sum(candidates, target);
        assert!(combos.contains(&vec![2,2,3]));
        assert!(combos.contains(&vec![7]));
    }

    #[test]
    fn ex5() {
        let candidates = vec![1];
        let target = 1;
        let combos = combination_sum(candidates, target);
        assert!(combos.contains(&vec![1]));
    }

    #[test]
    fn ex6() {
        let candidates = vec![2,3,5];
        let target = 8;
        let combos = combination_sum(candidates, target);
        dbg!(&combos);
    }

}