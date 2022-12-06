use std::{fs, env};

fn find_marker(signal: &str, marker_len: usize) -> usize {
    signal
        .as_bytes()
        .windows(marker_len)
        .enumerate()
        .find_map(|(n, window)| {
            if window
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len()
                == marker_len
            {
                Some(n + marker_len)
            } else {
                None
            }
        })
        .unwrap_or(0)
}

fn find_sop_marker(signal: &str) -> usize {
    find_marker(signal, 4)
}

fn find_som_marker(signal: &str) -> usize {
    find_marker(signal, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_sop_marker_examples() {
        assert_eq!(find_sop_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_sop_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_sop_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_sop_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_sop_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn find_som_marker_examples() {
        assert_eq!(find_som_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_som_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_som_marker("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_som_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(find_som_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}

fn main() {
    let input_file_path = env::args()
        .nth(1)
        .unwrap_or("../../06/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Error reading input file {input_file_path}"));
    let sop_marker_chars = find_sop_marker(&input);
    println!("Characters read until start-of-packet detected: {sop_marker_chars}");
    let som_marker_chars = find_som_marker(&input);
    println!("Characters read until start-of-message detected: {som_marker_chars}");
}
