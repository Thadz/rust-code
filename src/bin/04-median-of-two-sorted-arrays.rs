#[test]
fn it_works() {
    let delta = 1e-10;
    let merge_and_sort = |l: &Vec<i64>, r: &Vec<i64>| {
        let mut merged: Vec<i64> = l.clone();
        merged.append(&mut r.clone());
        merged.sort();
        merged
    };

    let l: Vec<i64> = vec![1, 2, 3, 4, 5];
    let r: Vec<i64> = vec![6, 7, 8, 9, 10];

    let sorted: Vec<i64> = merge_and_sort(&l, &r);
    for (i, v) in sorted.into_iter().enumerate() {
        assert_eq!(find_kth_sorted_arrays(&l, &r, i), v);
    }
    assert!((find_median_sorted_arrays(&l, &r) - 5.5).abs() < delta);

    let l = vec![1, 3, 5, 7, 9];
    let r = vec![2, 4, 6, 8, 10];

    let sorted: Vec<i64> = merge_and_sort(&l, &r);
    for (i, v) in sorted.into_iter().enumerate() {
        assert_eq!(find_kth_sorted_arrays(&l, &r, i), v);
    }
    assert!((find_median_sorted_arrays(&l, &r) - 5.5).abs() < delta);

    let l = vec![1, 3, 5];
    let r = vec![2, 4, 6, 8, 10, 12];

    let sorted: Vec<i64> = merge_and_sort(&l, &r);
    for (i, v) in sorted.into_iter().enumerate() {
        assert_eq!(find_kth_sorted_arrays(&l, &r, i), v);
    }
    assert!((find_median_sorted_arrays(&l, &r) - 5.0).abs() < delta);
}

fn find_median_sorted_arrays(l: &[i64], r: &[i64]) -> f64 {
    let m1 = find_kth_sorted_arrays(l, r, l.len() / 2 + r.len() / 2);
    let m2 = find_kth_sorted_arrays(l, r, l.len() / 2 + r.len() / 2 + 1);
    if (l.len() + r.len()) % 2 == 0 {
        (m1 as f64 + m2 as f64) / 2.0
    } else {
        m1 as f64
    }
}

fn find_kth_sorted_arrays(l: &[i64], r: &[i64], k: usize) -> i64 {
    use std::cmp::min;

    if k == 0 && !l.is_empty() && !r.is_empty() {
        return min(l[0], r[0]);
    }
    if k == 1 && !l.is_empty() && !r.is_empty() {
        let mut first_at_most_four = Vec::with_capacity(4);
        for v in l.into_iter().take(2).chain(r.into_iter().take(2)) {
            first_at_most_four.push(v);
        }
        first_at_most_four.sort();
        return *first_at_most_four[1];
    }
    if l.is_empty() && !r.is_empty() && k < r.len() {
        return r[k];
    }
    if !l.is_empty() && r.is_empty() && k < l.len() {
        return l[k];
    }
    if k < l.len() + r.len() {
        let l_idx = min(l.len(), k >> 1);
        let (ll, lr) = l.split_at(l_idx);
        let r_idx = min(r.len(), k >> 1);
        let (rl, rr) = r.split_at(r_idx);
        if l[l_idx - 1] < r[r_idx - 1] {
            return find_kth_sorted_arrays(lr, r, k - ll.len());
        } else {
            return find_kth_sorted_arrays(rr, l, k - rl.len());
        }
    }
    panic!("invalid state, l: {:?}, r: {:?}, k: {}", l, r, k);
}
