use std::fs;

fn main() {
    let inputs = lines_to_num_vec(&load_input("inputs.txt"));
    let inc_counts = count_increases(&inputs, 2);

    println!("{} increases", inc_counts);
}

fn count_increases(inputs: &[u16], window_size: usize) -> usize {
    inputs.windows(window_size).filter(|ints| ints[1] > ints[0]).count()
}

fn load_input(fname: &str) -> String {
    fs::read_to_string(fname).unwrap()
}

fn lines_to_num_vec(s: &str) -> Vec<u16> {
    s.lines().map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_increases_with_only_one_element_in_list() {
        let count = count_increases(&[1], 2);

        assert_eq!(count, 0);
    }

    #[test]
    fn count_increases_with_no_elements_in_list() {
        let inputs: Vec<u16> = Vec::new();
        let count = count_increases(&inputs, 2);

        assert_eq!(count, 0);
    }

    #[test]
    fn count_increases_with_samples_given_in_challenge_1() {
        let inputs = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let count = count_increases(&inputs, 2);

        assert_eq!(count, 7);
    }

    #[test]
    fn lines_to_num_vec_contains_one_number() {
        let result = lines_to_num_vec("100");

        assert_eq!(result, vec![100]);
    }

    #[test]
    fn lines_to_num_vec_contains_two_numbers() {
        let result = lines_to_num_vec("200\n300");

        assert_eq!(result, vec![200, 300]);
    }

    #[test]
    fn lines_to_num_vec_has_empty_last_line() {
        let result = lines_to_num_vec("400\n500\n");

        assert_eq!(result, vec![400, 500]);
    }
}
