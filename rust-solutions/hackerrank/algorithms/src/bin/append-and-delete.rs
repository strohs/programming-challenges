/// You have a string of lowercase English alphabetic letters. You can perform two types of operations on the string:
///
/// 1. Append a lowercase English alphabetic letter to the end of the string.
/// 2, Delete the last character in the string. Performing this operation on an empty string results in an empty string.
///
/// Given an integer, `k`, and two strings, `s` and `t`, determine whether or not you can
/// convert `s` to `t` by performing exactly `k` of the above operations on `s`. If it's possible,
/// print Yes. Otherwise, print No.


fn append_and_delete(s: String, t: String, k: usize) {
    // determine how many characters match from the beginning of s and t
    let match_count = s.chars().zip(t.chars())
        .take_while(|(sc, tc)| *sc == *tc)
        .count();

    let sb = s.len() - match_count;
    let tb = t.len() - match_count;

    dbg!(match_count, sb, tb);

    if (sb + tb) == k {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn main() {
    let s = String::from("hackerhappy");
    let t = String::from("h");
    let k: usize = 10;

    append_and_delete(s, t, k);
}