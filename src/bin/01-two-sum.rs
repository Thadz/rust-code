use std::collections::HashMap;

#[test]
fn it_works() {
    assert_eq!(two_sum(&[0, 1, 2], 3), (1, 2));
    assert_eq!(two_sum(&[2, 7, 11, 15], 9), (0, 1));
}

fn two_sum(nums: &[i32], target: i32) -> (usize, usize) {
    let mut id2value: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
        id2value.insert(nums[i], i);
    }
    let mut result: (usize, usize) = (0, 0);
    for num in nums {
        match (id2value.get(num), id2value.get(&(target - num))) {
            (Some(l), Some(r)) => result = if l < r {
                (*l, *r)
            } else {
                (*r, *l)
            },
            (None   , Some(_)) => (),
            (Some(_), None   ) => (),
            (None   , None   ) => (),
        }
    }
    return result;
}
