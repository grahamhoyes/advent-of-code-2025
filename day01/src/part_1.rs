pub fn solution(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (dir, num) = line.split_at(1);
            let num = num.parse::<i32>().unwrap();

            (dir, num)
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 3);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 1011);
    }
}
