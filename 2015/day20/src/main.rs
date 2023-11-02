fn main() {
    for house_number in 1_u128.. {
        let divisors = divisors::get_divisors(house_number);
        let score = divisors
            .into_iter()
            .fold(10 * (1 + house_number), |sum, divisor| sum + 10 * divisor);

        if score >= 33100000 {
            println!("{house_number}");
            break;
        }
    }
}
