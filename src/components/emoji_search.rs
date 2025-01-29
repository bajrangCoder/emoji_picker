use emojis::Emoji;
use std::collections::HashMap;

#[derive(PartialEq)]
pub struct EmojiSearchIndex {
    emoji_map: HashMap<String, Vec<&'static Emoji>>,
}

impl EmojiSearchIndex {
    pub fn new() -> Self {
        let mut emoji_map = HashMap::new();

        // Index all emojis
        for emoji in emojis::iter() {
            // Index by shortcode
            if let Some(shortcode) = emoji.shortcode() {
                emoji_map
                    .entry(shortcode.to_lowercase())
                    .or_insert_with(Vec::new)
                    .push(emoji);
            }

            // Index by description
            let description = emoji.name();
            for word in description.split_whitespace() {
                emoji_map
                    .entry(word.to_lowercase())
                    .or_insert_with(Vec::new)
                    .push(emoji);
            }
        }

        Self { emoji_map }
    }

    pub fn search(&self, query: &str) -> Vec<&'static Emoji> {
        if query.is_empty() {
            return vec![];
        }

        let query = query.to_lowercase();
        let mut results: Vec<&'static Emoji> = self
            .emoji_map
            .iter()
            .filter(|(key, _)| key.contains(&query))
            .flat_map(|(_, emojis)| emojis.iter().copied())
            .collect();

        // Remove duplicates
        results.sort_by_key(|e| e.as_str());
        results.dedup();
        results
    }
}
