use std::collections::HashMap;
use unicase::UniCase;

#[derive(Debug)]
struct TextAnalysis <'a> {
    word_counts: HashMap<UniCase<&'a str>, usize>,
    longest_word: &'a str,
    word_count: usize
}

impl <'a, T> From<&'a T> for TextAnalysis <'a>
where T: AsRef<str>
{
    fn from(str_like: &'a T) -> TextAnalysis<'a> {
        let text = str_like.as_ref();
        let word_count = text.split_whitespace().count();
        let mut word_counts = HashMap::with_capacity(word_count);
        let mut longest_word: &str = "";
        for word in text.split_whitespace() {
            let insensitive = insensitive_word(word);
            if word.len() > longest_word.len() {
                longest_word = word;
            }
            let counter = word_counts.entry(insensitive).or_insert(0);
            *counter += 1;
        }

        Self {
            word_counts,
            longest_word,
            word_count
        }
    }
}

impl <'a> TextAnalysis <'a> {
    pub fn most_used_word(&self) -> Option<&UniCase<&str>> {
        let mut most_used = None;
        let mut count = 0;

        for (word, cnt) in self.word_counts.iter() {
            if *cnt > count {
                count = *cnt;
                most_used.replace(word);
            }
        }

        most_used
    }
}

fn insensitive_word(word: &str) -> UniCase<&str> {
    UniCase::new(word.trim_matches(|c|
            c == '.' ||
            c == '?' ||
            c == '!' ||
            c == ',' ||
            c == ';' ||
            c == ':' ||
            c == '\'' ||
            c == '"'
    ))
}


#[allow(unused_mut)]
fn main() {
    let mut text = std::fs::read_to_string("blog-post.txt").unwrap();

    {
        let analysis = TextAnalysis::from(&text);

        println!("{:#?}", analysis);

        println!("The most used word is [{:?}]", analysis.most_used_word());
    }
}
