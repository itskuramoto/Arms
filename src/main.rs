pub fn read_file_contents(filename: String) -> String {
    let contents = String::new();
    contents
}
fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_contents() {
        let contents = read_file_contents("text.txt".to_string());
        assert_eq!(contents, "from text.txt");
    }
}
