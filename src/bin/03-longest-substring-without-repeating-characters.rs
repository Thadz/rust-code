 #[test]
 fn it_works() {
     assert_eq!(length_of_longest_substring("abcabcbb"), 3);
     assert_eq!(length_of_longest_substring("bbbbb"), 1);
     assert_eq!(length_of_longest_substring("pwwkew"), 3);
 }

 fn length_of_longest_substring(s: &str) -> usize {
     use std::str::Chars;
     use std::collections::HashSet;
     use std::cmp::max;

     let mut max_length = 0;
     let mut tracking_set: HashSet<char> = HashSet::new();
     let mut left_iter: Chars = s.chars();
     let right_iter: Chars = s.chars();

     for right_char in right_iter {
         if tracking_set.contains(&right_char) {
             max_length = max(max_length, tracking_set.len());

             for left_char in
                 left_iter
                     .by_ref() // so that we can still use left_iter later
                     .take_while(|x| *x != right_char) {

                 tracking_set.remove(&left_char);
             }
         }
         tracking_set.insert(right_char);
     }
     max_length
 }
 