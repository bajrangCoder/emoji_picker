use dioxus::prelude::*;
use emojis::{Emoji, Group};

#[derive(PartialEq, Props, Clone)]
pub struct EmojiGridProps {
    pub selected_group: Group,
    pub search_results: Option<Vec<&'static Emoji>>,
}

#[component]
pub fn EmojiGrid(props: EmojiGridProps) -> Element {
    let emojis = if let Some(results) = &props.search_results {
        results.iter().map(|&e| e).collect::<Vec<_>>()
    } else {
        props.clone().selected_group.emojis().collect::<Vec<_>>()
    };

    let group_name = if props.search_results.is_some() {
        "Search Results".to_string()
    } else {
        format!("{:?}", props.selected_group)
    };
    rsx! {
        div { class: "emoji-grid-container",
            div { class: "category-title", "{group_name}" }
            div { class: "emoji-grid",
                {
                    emojis
                        .iter()
                        .map(|emoji| {
                            let emoji_str = emoji.as_str();
                            rsx! {
                                div {
                                    key: "{emoji_str}",
                                    class: "emoji",
                                    title: "{emoji_str}",
                                    onclick: move |_| {
                                        println!("Selected emoji: {}", emoji_str);
                                    },
                                    "{emoji_str}"
                                }
                            }
                        })
                }
            }
        }
    }
}
