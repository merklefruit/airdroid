/// Example: From "1,2,3" to ["1", "2", "3"]
pub fn parse_csv_to_vec(input: Option<Vec<u8>>) -> Vec<String> {
    let input = input
        .map(|x| String::from_utf8(x.to_vec()).unwrap())
        .unwrap_or_else(String::new);

    if input.is_empty() {
        return Vec::new();
    }

    input.split(',').map(|x| x.to_string()).collect()
}
