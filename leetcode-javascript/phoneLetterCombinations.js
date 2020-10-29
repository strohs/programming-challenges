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

// build the map from digits to their corresponding letters
const digitMap = new Map();
digitMap.set("2",["a","b","c"]);
digitMap.set("3",["d","e","f"]);
digitMap.set("4",["g","h","i"]);
digitMap.set("5",["j","k","l"]);
digitMap.set("6",["m","n","o"]);
digitMap.set("7",["p","q","r","s"]);
digitMap.set("8",["t","u","v"]);
digitMap.set("9",["w","x","y","z"]);

// return a new array that is the cartesian product of the elements in collections s1 and s2
// s1 and s2 should be Iterables containing strings
function cartesianProduct(s1, s2) {
  let product = [];
  if (s1.length && !s2.length) {
    product = product.concat(s1);
  } else if (s2.length && !s1.length) {
    product = product.concat(s2);
  } else {
    for (let str1 of s1) {
      for (let str2 of s2) {
        product.push(str1.concat(str2));
      }
    }
  }
  return product;
}

// split the input numbers into an array of individual digit strings
const input1 = "2347768".split("");

// iterate through each digit's and combine the currDigit's letters with the next digit's letters
const combinations = input1.reduce((acc,currDigit) => {
  const letters = digitMap.get(currDigit);
  return cartesianProduct(acc, letters);
}, []);
console.log(combinations);