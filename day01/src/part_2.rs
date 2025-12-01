use std::iter;

/// Brute force solution, same as part 1 just expanding out into individual inputs
pub fn solution(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (dir, num) = line.split_at(1);
            let num = num.parse::<usize>().unwrap();

            // This and the .flatten() are the only thing that changed
            iter::repeat((dir, 1i32)).take(num)
        })
        .flatten()
        .scan(50, |cur, (dir, num)| {
            *cur = match dir {
                "L" => (*cur - num).rem_euclid(100),
                "R" => (*cur + num).rem_euclid(100),
                _ => panic!("Unrecognized direction {}", dir),
            };
            Some(*cur)
        })
        .filter(|&e| e == 0)
        .count()
}

/// Smarter solution that doesn't involve brute force. This doesn't quite work,
/// there are some more edge cases.
pub fn solution_smart(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (dir, num) = line.split_at(1);
            let num = num.parse::<i32>().unwrap();

            (dir, num)
        })
        .scan(50, |cur, (dir, num)| {
            let next = match dir {
                "L" => *cur - num,
                "R" => *cur + num,
                _ => panic!("Unrecognized direction {}", dir),
            };

            // Quotient gives us roughly how many times we passed zero as a result of this operation
            let mut times_passed_zero = next.div_euclid(100).abs();
            // There are some edge cases. TODO: There are more edge cases to fix!
            if *cur == 0 && times_passed_zero > 0 {
                times_passed_zero -= 1;
            } else if (next == 0 && times_passed_zero == 0) {
                times_passed_zero += 1;
            }

            // Remainder - what number we ended up on
            let rem = next.rem_euclid(100);

            // println!(
            //     "{dir}{num}\tcur: {cur:4}\tnext: {next:4}\trem: {rem:02}\ttpz: {times_passed_zero}"
            // );

            *cur = rem;

            Some(times_passed_zero as usize)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 6);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 5937);
    }
}
