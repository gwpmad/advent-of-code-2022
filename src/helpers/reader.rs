use std::fs;

pub fn read_file(path: String) -> String {
    let file_string_result = fs::read_to_string(path);
    let file_string = match file_string_result {
        Ok(string) => string,
        Err(error) => panic!("Couldn't open the file - {error}"),
    };
    return file_string;
}
