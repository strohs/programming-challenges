/// You have a string of lowercase English alphabetic letters. You can perform two types of operations on the string:
///
/// 1. Append a lowercase English alphabetic letter to the end of the string.
/// 2, Delete the last character in the string. Performing this operation on an empty string results in an empty string.
///
/// Given an integer, `k`, and two strings, `s` and `t`, determine whether or not you can
/// convert `s` to `t` by performing exactly `k` of the above operations on `s`. If it's possible,
/// print Yes. Otherwise, print No.

package algorithms.easy;

import java.util.stream.IntStream;

public class AppendAndDelete {


    // returns number of chars that matched in each string, returns the count once first non-matching char is found
    static long matchingCharCount(String s, String t) {
        int minLength = Math.min(s.length(), t.length());
        for (int i = 0; i < minLength; i++) {
            if (s.charAt(i) != t.charAt(i)) {
                return i;
            }
        }
        return minLength;
    }

    static String appendAndDelete(String s, String t, int k) {
        long count = matchingCharCount(s, t);
        long sb = s.length() - count;
        long tb = t.length() - count;

        if (s.length() + t.length() <= k) {
            // in this case, k is large enough so that we could delete all of s and then add all of t's chars to it
            return "Yes";
        } else {
            //k < s + t, need to see if there are chars in common

            if (count == 0) {
                // no chars in common and k is too small to delete all of s and then add all of t
                return "No";
            } else {
                // there are some chars in common

                // if s.len < t.len then tb must == k to be successful
                if (s.length() < t.length()) {
                    if (tb == k) return "Yes"; else return "No";
                }

                // if s.len > t.len
                if (s.length() > t.length()) {

                }

                if (sb + tb == k) {
                    return "Yes";
                } else {
                    return "No";
                }
            }
        }
    }

    public static void main(String[] args) {
        String s = "hackerhappy";
        String t = "hackerrank";
        int k = 9;
        System.out.println(appendAndDelete(s, t, k));

        System.out.println(appendAndDelete("aba", "aba", 6)); // yes

        System.out.println(appendAndDelete("ashley", "ash", 3)); // yes

        System.out.println(appendAndDelete("ash", "ashley", 3)); // yes

//        System.out.println(appendAndDelete("tzhbhgd", "aba", 14)); // yes
//
//        System.out.println(appendAndDelete("xyz", "abcdefg", 7)); //mo
//
        System.out.println(appendAndDelete("aaaaaaaaaa", "aaaaa", 7)); // yes

        System.out.println(appendAndDelete("aaaaabaaaaa", "aaaaa", 6)); // yes
//
//        System.out.println(appendAndDelete("aaa", "aaaaa", 2)); // yes

    }
}
