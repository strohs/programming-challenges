use std::collections::HashMap;

/// # Letter Combinations of a phone number
/// Given a string containing digits from `2-9` inclusive, return all possible letter combinations
/// that the number could represent.
/// A mapping of digit to letters (just like on the telephone buttons) is given below. Note that
/// 1 does not map to any letters.
/// ```
/// 2 -> a,b
/// 3 -> d,e,f
/// 4 -> g,h,i
/// 5 -> j,k,l
/// 6 -> m,n,o
/// 7 -> p,q,r,s
/// 8 -> t,u,v
/// 9 -> w,x,y,z
/// ```
/// ## Example
/// input: "23"
/// output: `["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]`
///
/// FYI permutation - you care about the order of the elements
///     n! / (n - k)!
/// combinations - you don't care about the order of the elements
///    combination formula, you have n objects and want to choose k    = n! / k!(n - k!)

fn build_digit_map() -> HashMap<char, Vec<&'static str>> {
    let mut m = HashMap::with_capacity(8);
    m.insert('2', vec!["a","b","c"]);
    m.insert('3', vec!["d","e","f"]);
    m.insert('4', vec!["g","h","i"]);
    m.insert('5', vec!["j","k","l"]);
    m.insert('6', vec!["m","n","o"]);
    m.insert('7', vec!["p","q","r","s"]);
    m.insert('8', vec!["t","u","v"]);
    m.insert('9', vec!["w","x","y","z"]);
    m
}

/// combine each string in s1 with each string in s2
fn combine_sets(s1: &Vec<String>, s2: &Vec<String>) -> Vec<String> {
    let mut combis = vec![];
    for s1 in s1.iter() {
        for s2 in s2.iter() {
            let mut s = String::new();
            s.push_str(&s1.clone());
            s.push_str(&s2.clone());
            combis.push(s);
        }
    }
    combis
}

fn main() {
    let digit_map = build_digit_map();
    let input1 = "8888";
    let digits = input1.chars().collect::<Vec<char>>();

    let mut curr = digit_map.get(&digits[0]).unwrap().iter().map(|&s| String::from(s)).collect::<Vec<String>>();
    for curr_digit in digits.iter().skip(1) {
        let next = digit_map.get(curr_digit).unwrap().iter().map(|&s| String::from(s)).collect::<Vec<String>>();
        curr = combine_sets(&curr, &next);
    }
    println!("{:#?}",&curr);
}