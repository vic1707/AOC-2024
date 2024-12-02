advent_of_code::solution!(1);

fn split_in_vecs(input: &str) -> [Vec<usize>; 2] {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(str::as_bytes)
                .map(|n| n.iter().fold(0, |acc, &b| acc * 10 + usize::from(b - b'0')))
        })
        .fold([vec![], vec![]], |mut acc, mut l| {
            acc[0].push(l.next().unwrap());
            acc[1].push(l.next().unwrap());
            acc
        })
}

pub fn part_one(input: &str) -> Option<usize> {
    let [mut v1, mut v2] = split_in_vecs(input);

    v1.sort_unstable();
    v2.sort_unstable();

    Some(v1.iter().zip(v2).fold(0, |acc, (x, y)| acc + x.abs_diff(y)))
}

pub fn part_two(input: &str) -> Option<usize> {
    let [v1, v2] = split_in_vecs(input);

    Some(
        v1.iter()
            .fold(0, |acc, n| acc + v2.iter().filter(|&c| n == c).count() * n),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2113135));
    }

    #[test]
    fn test_example_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(19097157));
    }
}
