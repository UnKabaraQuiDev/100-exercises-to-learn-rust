// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(vec: Vec<i32>) -> i32 {
    let v = vec.leak();

    let mut sum: i32 = 0;

    let part1 = v[0..v.len() / 2].to_vec();
    let handle1 = thread::spawn(move || part1.iter().sum::<i32>());

    let part2 = v[v.len() / 2..v.len()].to_vec();
    let handle2 = thread::spawn(move || part2.iter().sum::<i32>());

    sum += handle1.join().unwrap();
    sum += handle2.join().unwrap();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
