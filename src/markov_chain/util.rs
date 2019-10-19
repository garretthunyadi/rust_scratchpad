pub fn simplify_corpus(text: &str) -> String {
    text.split_ascii_whitespace()
        .map(|word| {
            word.chars()
                .filter(|&c| c.is_alphanumeric())
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}
#[test]
fn test_simplify_corpus() {
    assert_eq!(simplify_corpus("abc def ghi"), String::from("abc def ghi"));
    assert_eq!(
        simplify_corpus("ab,c def, ,ghi."),
        String::from("abc def ghi")
    );
}

pub fn simplify_word(word: &str) -> String {
    word.chars()
        .filter(|&c| c.is_alphanumeric())
        .collect::<String>()
}

#[test]
fn test_simplify_word() {
    assert_eq!(simplify_word("word"), String::from("word"));
    assert_eq!(simplify_word("word."), String::from("word"));
    assert_eq!(simplify_word("wor%d"), String::from("word"));
}

pub fn words(text: &str) -> Vec<&str> {
    text.split_ascii_whitespace().collect()
}
