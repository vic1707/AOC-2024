#![feature(iter_array_chunks)]

advent_of_code::solution!(1);

fn split_in_vecs(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .flat_map(str::split_ascii_whitespace)
        .map(str::as_bytes)
        .map(|n| n.iter().fold(0, |acc, &b| acc * 10 + usize::from(b - b'0')))
        .array_chunks::<2>()
        .map(|[x, y]| (x, y))
        .unzip()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut v1, mut v2) = split_in_vecs(input);

    v1.sort_unstable();
    v2.sort_unstable();

    Some(v1.iter().zip(v2).fold(0, |acc, (x, y)| acc + x.abs_diff(y)))
}

static mut COUNTS: [u8; 99999] = [0; 99999];
pub fn part_two(input: &str) -> Option<usize> {
    let (v1, v2) = split_in_vecs(input);

    for n in v2.into_iter() {
        unsafe {
            COUNTS[n] += 1;
        }
    }

    Some(
        v1.into_iter()
            .fold(0, |acc, n| acc + n * usize::from(unsafe { COUNTS[n] })),
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
