fn main() {
    let file = std::fs::read_to_string(std::env::args().skip(1).next().unwrap()).unwrap();
    let mut program = file.chars();

    while let Some(f) = program.next() {
        let argument = program.next().unwrap();
    }
}
