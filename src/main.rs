mod double_base_palindromes;
mod number_handeling;

use crate::double_base_palindromes::dbp::DBP;

fn main() {
    let problem_number = 36;
    let mut answer_p36 = 0;

    let mut dbp = DBP::new(1);

    for num in 1..1000000 {
        dbp.decimal = num;
        if dbp.is_double_base_palindromic() {
            answer_p36 += dbp.decimal;
        }
    }

    println!(
        "The answer to problem {} of project Euler is {}.",
        problem_number, answer_p36
    );
}
