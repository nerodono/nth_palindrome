const NTH: u64 = 20;

fn extend_reversed(mut n: u64) -> u64 {
    let mut original = n;
    let digits = ((original as f64).log10().floor() as u32) + 1;
    let mut extended = 0u32;

    while extended != digits {
        n *= 10u64;
        n += original % 10;
        original /= 10;
        extended += 1;
    }

    n
}

fn calc_nth_relative_palindrome(digits: u64, nth: u64) -> u64 {
    if digits <= 1 {
        return nth;
    }

    if digits % 2 == 0 {
        let half = (digits / 2) - 1;
        let left_part = 10u64.pow(half as u32) + (nth - 1);
        extend_reversed(left_part)
    } else {
        let affecting_others = ((digits - 1) / 2) as u32;
        let aff_pow = 10u64.pow(affecting_others);

        let mid = (nth - 1) % 10;

        let add_pow = aff_pow;
        let add = (nth - 1) / 10;

        let dep_part = aff_pow + add * add_pow + mid;
        let ext = extend_reversed(dep_part);

        (ext / (aff_pow * 10)) * aff_pow + ext % aff_pow
    }
}

fn calc_group(nth: u64) -> u64 {
    ((((nth as f64) + 1.0) / 2.0).log10() + 1.0).floor() as u64
}

fn calc_single_row_in_group_elems(group: u64) -> u64 {
    9u64 * 10u64.pow((group - 1).try_into().unwrap())
}

fn calc_row(nth: u64, group: u64) -> (u64, u64) {
    let n_of_elms = 2u64 * (10u64.pow(group.try_into().unwrap()) - 1);
    let cur_group_single_row = calc_single_row_in_group_elems(group);
    let w_last_group = n_of_elms - cur_group_single_row;

    let rel_idx;
    let mod_value = if nth <= w_last_group {
        let prev_elems = w_last_group - cur_group_single_row;
        rel_idx = nth - prev_elems;
        1
    } else if nth > w_last_group && nth <= n_of_elms {
        rel_idx = nth - w_last_group;
        0
    } else {
        unreachable!("{w_last_group} {n_of_elms}");
    };

    (2 * group - mod_value, rel_idx)
}

fn main() {
    let group = calc_group(NTH);
    let (row, rel_idx) = calc_row(NTH, group);
    println!("nth = {NTH}");
    println!("GROUP = {group}");
    println!("ROW = {row}");
    println!("RELATIVE ID IN ROW = {rel_idx}");

    let palindrome = calc_nth_relative_palindrome(row, rel_idx);
    println!("PALINDROME = {palindrome}");
}
