use emojis::Emoji;
use std::collections::HashMap;

#[derive(PartialEq)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    emojis: Vec<&'static Emoji>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            emojis: Vec::new(),
        }
    }
}

#[derive(PartialEq)]
pub struct EmojiSearchIndex {
    trie: TrieNode,
}

impl EmojiSearchIndex {
    pub fn new() -> Self {
        let mut root = TrieNode::new();

        for emoji in emojis::iter() {
            // Index shortcode
            if let Some(shortcode) = emoji.shortcode() {
                Self::insert_word(&mut root, &shortcode.to_lowercase(), emoji);
            }

            // Index description words
            for word in emoji.name().split_whitespace() {
                Self::insert_word(&mut root, &word.to_lowercase(), emoji);
            }
        }

        Self { trie: root }
    }

    fn insert_word(node: &mut TrieNode, word: &str, emoji: &'static Emoji) {
        let mut current = node;
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert_with(TrieNode::new);
            current.emojis.push(emoji);
        }
    }

    pub fn search(&self, query: &str) -> Vec<&'static Emoji> {
        if query.is_empty() {
            return vec![];
        }

        let query = query.to_lowercase();
        let mut current = &self.trie;

        for ch in query.chars() {
            if let Some(node) = current.children.get(&ch) {
                current = node;
            } else {
                return vec![];
            }
        }

        // Return all emojis at this node
        let mut results = current.emojis.clone();
        results.sort_by_key(|e| e.as_str());
        results.dedup();
        results
    }
}
