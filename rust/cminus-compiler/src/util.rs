pub fn get_pos(program: &String, byte: usize) -> (u32, u32) {
    let mut line_count = 1;
    let mut col_count = 1;
    for (i, c) in program.chars().enumerate() {
        if c == '\n' {
            line_count += 1;
            col_count = 1
        }
        if i == byte {
            break;
        }
        col_count += 1
    }
    (line_count, col_count)
}
