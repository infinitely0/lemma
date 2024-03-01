#[derive(Debug)]
pub struct StringSplitter<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StringSplitter<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}
impl<'a> Iterator for StringSplitter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_pos) = self.remainder.find(self.delimiter) {
            let next_str = &self.remainder[0..next_pos];
            self.remainder = &self.remainder[(next_pos + next_str.len())..];
            Some(next_str)
        } else if self.remainder.is_empty() {
            None
        } else {
            let tail = self.remainder;
            self.remainder = "";
            Some(tail)
        }
    }
}

#[test]
fn it_works() {
    let str = "a b c d e";
    let ss: Vec<&str> = StringSplitter::new(str, " ").collect();

    let xs = vec!["a", "b", "c", "d", "e"];
    assert_eq!(ss, xs);
}
