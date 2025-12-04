use itertools::*;
use sscanf::sscanf;
advent_of_code::solution!(2);

struct InvalidId1;
struct InvalidId2;

pub trait InvalidID {
    fn invalid_id(id: &str) -> bool;
}

impl InvalidID for InvalidId1 {
    fn invalid_id(id: &str) -> bool {
        if id.len() % 2 == 1 {
            return false;
        }
        let half = id.len() / 2;
        if id[..half] == id[half..] {
            return true;
        }
        false
    }
}

impl InvalidID for InvalidId2 {
    fn invalid_id(id: &str) -> bool {
        let n = id.len();
        if n < 2 {
            return false; // cannot be at least two repeats
        }

        // Try all possible pattern lengths
        for pat_len in 1..=n / 2 {
            if !n.is_multiple_of(pat_len) {
                continue; // must divide the total length
            }

            let repeats = n / pat_len;
            if repeats < 2 {
                continue; // must repeat at least twice
            }

            let pat = &id[0..pat_len];
            // Build a string by repeating `pat` and compare
            if pat.repeat(repeats) == id {
                return true;
            }
        }

        false
    }
}

pub fn solve<Checker: InvalidID>(input: &str) -> Option<u64> {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split(',')
        .map(|range_str| sscanf!(range_str, "{u64}-{u64}").expect("unable to parse"))
        .flat_map(|(beg, end)| beg..=end)
        .filter(|num| Checker::invalid_id(&num.to_string()))
        .sum::<u64>()
        .into()
}

pub fn part_one(input: &str) -> Option<u64> {
    solve::<InvalidId1>(input)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve::<InvalidId2>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }
    #[test]
    fn test_invalid_id() {
        assert!(InvalidId1::invalid_id("123123"));
        assert!(InvalidId1::invalid_id("11"));
        assert!(InvalidId1::invalid_id("99989998"));
        assert!(!InvalidId1::invalid_id("123321"));
        assert!(!InvalidId1::invalid_id("88188"));
    }

    #[test]
    fn test_invalid_id2() {
        assert!(InvalidId2::invalid_id("123123"));
        assert!(InvalidId2::invalid_id("113113113"));
        assert!(InvalidId2::invalid_id("99989998"));
        assert!(!InvalidId2::invalid_id("123321"));
        assert!(!InvalidId2::invalid_id("88188"));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
