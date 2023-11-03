const ROW: u64 = 2947;
const COLUMN: u64 = 3029;

fn main() {
    let mut code = 20151125;

    for _ in 1..translate_coords(ROW, COLUMN) {
        code = get_next_code(code);
    }
    println!("{}", code);
}

fn get_next_code(code: u64) -> u64 {
    (code * 252533) % 33554393
}

fn translate_coords(row: u64, column: u64) -> u64 {
    let diagonal = row + column - 1;
    let diagonal_fist_value = 1 + (diagonal * (diagonal - 1)) / 2;

    diagonal_fist_value + column - 1
}
