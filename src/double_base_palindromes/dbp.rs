use crate::number_handeling::number_handeling::get_binary_reversed_number;
use crate::number_handeling::number_handeling::get_decimal_reversed_number;

// DBP = Double-base Palindromes
pub struct DBP {
    pub decimal: u128,
}

impl DBP {
    pub fn new(decimal: u128) -> DBP {
        DBP { decimal }
    }

    pub fn is_double_base_palindromic(&self) -> bool {
        self.is_decimal_palindromic() && self.is_binary_palindromic()
    }

    fn is_decimal_palindromic(&self) -> bool {
        self.decimal == get_decimal_reversed_number(self.decimal)
    }

    fn is_binary_palindromic(&self) -> bool {
        self.decimal == get_binary_reversed_number(self.decimal)
    }
}

#[cfg(test)]
mod tests {
    use super::DBP;

    #[test]
    fn is_decimal_palindromic_tests() {
        let mut dbp = DBP::new(1);
        assert!(dbp.is_decimal_palindromic());
        dbp.decimal = 9;
        assert!(dbp.is_decimal_palindromic());
        dbp.decimal = 585;
        assert!(dbp.is_decimal_palindromic());
        dbp.decimal = 5885;
        assert!(dbp.is_decimal_palindromic());
        dbp.decimal = 91;
        assert!(dbp.is_decimal_palindromic() == false);
        dbp.decimal = 9111113;
        assert!(dbp.is_decimal_palindromic() == false);
    }

    #[test]
    fn is_binary_palindromic_tests() {
        let mut dbp = DBP::new(1);
        assert!(dbp.is_binary_palindromic());
        dbp.decimal = 9;
        assert!(dbp.is_binary_palindromic());
        dbp.decimal = 585;
        assert!(dbp.is_binary_palindromic());
        dbp.decimal = 5885;
        assert!(dbp.is_binary_palindromic() == false);
        dbp.decimal = 91;
        assert!(dbp.is_binary_palindromic() == false);
        dbp.decimal = 9111113;
        assert!(dbp.is_binary_palindromic() == false);
    }

    #[test]
    fn is_double_base_palindromic() {
        let mut dbp = DBP::new(1);
        assert!(dbp.is_double_base_palindromic());
        dbp.decimal = 9;
        assert!(dbp.is_double_base_palindromic());
        dbp.decimal = 585;
        assert!(dbp.is_double_base_palindromic());
        dbp.decimal = 5885;
        assert!(dbp.is_double_base_palindromic() == false);
        dbp.decimal = 91;
        assert!(dbp.is_double_base_palindromic() == false);
        dbp.decimal = 9111113;
        assert!(dbp.is_double_base_palindromic() == false);
    }
}
