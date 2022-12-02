use std::fs;

pub fn read(filename: &str) -> String {
    let file_read_error = "Should have been able to read file";
    let input = fs::read_to_string(filename)
        .expect(file_read_error);
    input
}