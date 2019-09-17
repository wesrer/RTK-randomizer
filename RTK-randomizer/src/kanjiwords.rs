
use std::collections::HashMap;

pub struct KanjiWords{
    words: HashMap<u32, KanjiWord>,
}

pub struct KanjiWord {
    word: String,
    mnemonic: Option<String>
}

impl KanjiWords {
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
        }
    }

    pub fn add(&mut self, index: u32, kanji_word: KanjiWord){
        self.words.insert(index, kanji_word);
    }

    pub fn get_mnemonic(&self, index: u32) -> Option<String> {
        self.words.get(index).mnemonic
    }

    pub fn get(&self, index: u32) -> String {
        self.words.get(index).word
    }
}
