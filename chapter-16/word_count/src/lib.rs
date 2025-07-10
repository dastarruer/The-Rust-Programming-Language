pub struct WordCounter<'a> {
    text: &'a str,
}

impl<'a> WordCounter<'a> {
    fn count_words(&self) -> i32 {
        15
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        let wc = WordCounter { text: "hello world" };
        assert_eq!(wc.count_words(), 2);
    }
}
