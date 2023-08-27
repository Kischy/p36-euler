pub fn get_decimal_reversed_number(mut num: u128) -> u128 {
    get_reversed_number(num, 10)
}

pub fn get_binary_reversed_number(mut num: u128) -> u128 {
    get_reversed_number(num, 2)
}

fn get_reversed_number(mut num: u128, base: u128) -> u128 {
    let mut reversed = 0;
    while num != 0 {
        reversed = reversed * base + num % base;
        num /= base;
    }
    reversed
}

#[cfg(test)]
mod tests {
    use super::get_binary_reversed_number;
    use super::get_decimal_reversed_number;
    #[test]
    fn get_reversed_number_tests() {
        assert_eq!(get_decimal_reversed_number(123), 321);
        assert_eq!(get_decimal_reversed_number(1), 1);
        assert_eq!(get_decimal_reversed_number(562), 265);
    }

    #[test]
    fn get_binary_reversed_number_tests() {
        assert_eq!(get_binary_reversed_number(123), 111);
        assert_eq!(get_binary_reversed_number(1), 1);
        assert_eq!(get_binary_reversed_number(562), 305);
    }
}
