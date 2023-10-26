use std::{
    cmp::min,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut total_paper_length = 0;

    for line in buf_reader.lines() {
        let line = line.expect("read line");

        let (l, w, h) = parse_dimension(line);

        total_paper_length += get_paper_length(l, w, h);
    }

    println!("{total_paper_length}");
}

fn parse_dimension(line: String) -> (u32, u32, u32) {
    let dimensions: Vec<u32> = line
        .split("x")
        .into_iter()
        .map(|dimension| dimension.parse::<u32>().expect("an integer"))
        .collect();

    (dimensions[0], dimensions[1], dimensions[2])
}

fn get_paper_length(l: u32, w: u32, h: u32) -> u32 {
    let face_lw = l * w;
    let face_lh = l * h;
    let face_wh = w * h;

    let min_face = min(min(face_lw, face_lh), face_wh);

    2 * face_lw + 2 * face_lh + 2 * face_wh + min_face
}
