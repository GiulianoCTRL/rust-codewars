#![allow(dead_code)]
/// You probably know the "like" system from Facebook and other pages.
/// People can "like" blog posts, pictures or other items. We want to
/// create the text that should be displayed next to such an item.
///
/// Implement the function which takes an array containing the names of people
/// that like an item.
fn likes(names: &[&str]) -> String {
    match names {
        [] => "no one likes this".to_string(),
        [a] => format!("{} likes this", a),
        [a, b] => format!("{} and {} like this", a, b),
        [a, b, c] => format!("{}, {} and {} like this", a, b, c),
        [a, b, rest @ ..] => format!("{}, {} and {} others like this", a, b, rest.len()),
    }
}

/// You might know some pretty large perfect squares. But what about the NEXT one?
///
/// Complete the findNextSquare method that finds the next integral perfect
/// square after the one passed as a parameter. Recall that an integral
/// perfect square is an integer n such that sqrt(n) is also an integer.
///
/// If the parameter is itself not a perfect square then -1 should be returned.
/// You may assume the parameter is non-negative.
fn find_next_square(sq: u64) -> Option<u64> {
    match (sq as f64).sqrt() {
        x if x == x.floor() => Some(((x + 1.0) as u64).pow(2)),
        _ => None,
    }
}

/// You are given an array(list) strarr of strings and an integer k. Your task
/// is to return the first longest string consisting of k consecutive strings
/// taken in the array.
/// n being the length of the string array, if n = 0 or k > n or k == 0 return "".
fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let n = strarr.len();
    if k > n || k == 0 || n == 0 {
        "".to_string()
    } else {
        strarr
            .windows(k)
            .map(|s| s.join(""))
            .rev()
            .max_by_key(|s| s.chars().count())
            .unwrap()
    }
}

/// Finish the solution so that it sorts the passed in array of numbers. If the
/// function passes in an empty array or null/nil value then it should return
/// an empty array.
fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
    let mut arr = arr.to_owned();
    arr.sort_unstable();
    arr
}

fn count_bits(n: i64) -> u32 {
    // format!("{:b}", n).matches('1').count() as u32
    n.count_ones()
}

/// Middle Earth is about to go to war. The forces of good will have many
/// battles with the forces of evil. Different races will certainly be
/// involved. Each race has a certain worth when battling against others.
/// On the side of good we have the following races, with their associated
/// worth:
fn good_vs_evil(good: &str, evil: &str) -> String {
    use std::cmp::Ordering;

    let good_worth: [usize; 6] = [1, 2, 3, 3, 4, 10];
    let evil_worth: [usize; 7] = [1, 2, 2, 2, 3, 5, 10];

    let get_strength = |s: &str, good: bool| {
        s.split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .enumerate()
            .map(|(i, n)| match good {
                true => n * good_worth[i],
                false => n * evil_worth[i],
            })
            .sum::<usize>()
    };

    match get_strength(good, true).cmp(&get_strength(evil, false)) {
        Ordering::Greater => "Battle Result: Good triumphs over Evil",
        Ordering::Less => "Battle Result: Evil eradicates all trace of Good",
        Ordering::Equal => "Battle Result: No victor on this battle field",
    }
    .to_string()
}

/// Given two speeds v1 (A's speed, integer > 0) and v2 (B's speed, integer > 0)
/// and a lead g (integer > 0) how long will it take B to catch A?
/// The result will be an array [hour, min, sec] which is the time needed in
/// hours, minutes and seconds (round down to the nearest second) or a string
/// in some languages. f v1 >= v2 then return None .
fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        None
    } else {
        let time = (g * 3600) / (v2 - v1);
        Some(vec![time / 3600, time / 60 % 60, time % 60])
    }
}

/// Given an array of ones and zeroes, convert the equivalent binary value to
/// an integer.
/// Eg: [0, 0, 0, 1] is treated as 0001 which is the binary representation of 1.
fn binary_slice_to_number(slice: &[u32]) -> u32 {
    slice.iter().fold(0, |acc, i| (acc << 1) + i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_likes() {
        assert_eq!(likes(&[]), "no one likes this");
        assert_eq!(likes(&["Peter"]), "Peter likes this");
        assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
        assert_eq!(
            likes(&["Max", "John", "Mark"]),
            "Max, John and Mark like this"
        );
        assert_eq!(
            likes(&["Alex", "Jacob", "Mark", "Max"]),
            "Alex, Jacob and 2 others like this"
        );
    }

    #[test]
    fn test_find_next_square() {
        assert_eq!(find_next_square(121), Some(144));
        assert_eq!(find_next_square(625), Some(676));
        assert_eq!(find_next_square(319_225), Some(320_356));
        assert_eq!(find_next_square(15_241_383_936), Some(15_241_630_849));
        assert_eq!(find_next_square(155), None);
        assert_eq!(find_next_square(342_786_627), None);
    }

    #[test]
    fn test_basics_longest_consec() {
        let test = |strarr, k, exp| assert_eq!(&longest_consec(strarr, k), exp);
        test(
            vec!["zone", "abigail", "theta", "form", "libe", "zas"],
            2,
            "abigailtheta",
        );
        test(
            vec![
                "ejjjjmmtthh",
                "zxxuueeg",
                "aanlljrrrxx",
                "dqqqaaabbb",
                "oocccffuucccjjjkkkjyyyeehh",
            ],
            1,
            "oocccffuucccjjjkkkjyyyeehh",
        );
        test(vec![], 3, "");
        test(
            vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"],
            3,
            "ixoyx3452zzzzzzzzzzzz",
        );
        test(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
        test(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
    }

    #[test]
    fn test_sort_numbers() {
        assert_eq!(sort_numbers(&vec![1, 2, 3, 10, 5]), vec![1, 2, 3, 5, 10]);
        assert_eq!(sort_numbers(&vec![]), vec![]);
        assert_eq!(sort_numbers(&vec![20, 2, 10]), vec![2, 10, 20]);
        assert_eq!(sort_numbers(&vec![2, 20, 10]), vec![2, 10, 20]);
        assert_eq!(sort_numbers(&vec![2, 10, 20]), vec![2, 10, 20]);
    }

    #[test]
    fn test_count_bits() {
        assert_eq!(count_bits(0), 0);
        assert_eq!(count_bits(4), 1);
        assert_eq!(count_bits(7), 3);
        assert_eq!(count_bits(9), 2);
        assert_eq!(count_bits(10), 2);
    }

    #[test]
    fn test_good_vs_evil() {
        assert_eq!(
            good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"),
            "Battle Result: Good triumphs over Evil"
        );
        assert_eq!(
            good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"),
            "Battle Result: Evil eradicates all trace of Good"
        );
        assert_eq!(
            good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"),
            "Battle Result: No victor on this battle field"
        );
    }

    #[test]
    fn test_race() {
        assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
        assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
        assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
        assert_eq!(race(820, 81, 550), None);
    }

    #[test]
    fn test_binary_slice_to_number() {
        assert_eq!(binary_slice_to_number(&vec![0, 0, 0, 1]), 1);
        assert_eq!(binary_slice_to_number(&vec![0, 0, 1, 0]), 2);
        assert_eq!(binary_slice_to_number(&vec![1, 1, 1, 1]), 15);
        assert_eq!(binary_slice_to_number(&vec![0, 1, 1, 0]), 6);
    }
}
