fn main() {
    let vs = &[0b1, 0b0, 0b1];
    println!("{:?}", vs);
}

fn gamma(vs: &[u16]) -> u16 {
    (0..16_u16)
        .map(|i| gamma_bit(vs, i) << (15 - i))
        .sum()
}

fn gamma_bit(vs: &[u16], idx: u16) -> u16 {
    let on_bits = vs
        .iter()
        .map(|v| (v >> (15 - idx)) & 1)
        .sum::<u16>() as usize;

    if (on_bits * 2) >= vs.len() { 1 } else { 0 }
}

fn epsilon(v: u16) -> u16 {
    !v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gamma_bit_as_one_for_idx_15() {
        let vs = &[0b1, 0b0, 0b1];

        assert_eq!(gamma_bit(vs, 15), 1);
        assert_eq!(gamma(vs), 1);
    }

    #[test]
    fn gamma_bit_as_one_for_idx_14() {
        let vs = &[0b10, 0b00, 0b10];

        assert_eq!(gamma_bit(vs, 14), 1);
        assert_eq!(gamma(vs), 2);
    }

    #[test]
    fn gamma_bit_as_two_for_idx_14() {
        let vs = &[0b100, 0b000, 0b010];

        assert_eq!(gamma_bit(vs, 14), 0);
    }

    #[test]
    fn gamma_using_challenge_1_inputs() {
        let vs = &[
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111,
            0b00111, 0b11100, 0b10000, 0b11001, 0b00010, 0b01010];

        assert_eq!(gamma(vs), 22);
    }

    #[test]
    fn epsilon_inverse_given_value() {
        assert_eq!(epsilon(1), 0b1111_1111_1111_1110);
        assert_eq!(epsilon(2), 0b1111_1111_1111_1101);
    }
}
