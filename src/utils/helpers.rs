pub fn parse_csv_to_vec(input: Option<Vec<u8>>) -> Vec<String> {
    input
        .map(|x| String::from_utf8(x.to_vec()).unwrap())
        .unwrap_or_default()
        .split(',')
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}
