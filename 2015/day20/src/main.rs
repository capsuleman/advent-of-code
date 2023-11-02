fn main() {
    for house_number in 1_u128.. {
        let divisors = divisors::get_divisors(house_number);
        let score = divisors
            .into_iter()
            .filter(|divisor| house_number / divisor <= 50)
            .fold(11 * house_number, |sum, divisor| sum + 11 * divisor);

        if score >= 33100000 {
            println!("{house_number}");
            break;
        }
    }
}
