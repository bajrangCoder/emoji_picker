use std::time::Duration;

use dioxus::prelude::*;
use dioxus_clipboard::prelude::use_clipboard;
use dioxus_sdk::utils::timing::use_debounce;
use emojis::{Emoji, Group};

#[derive(PartialEq, Props, Clone)]
pub struct EmojiGridProps {
    pub selected_group: Group,
    pub search_results: Option<Vec<&'static Emoji>>,
}

#[component]
pub fn EmojiGrid(props: EmojiGridProps) -> Element {
    let mut show_toast = use_signal(|| false);
    let mut toast_message = use_signal(|| String::new());

    let mut hide_toast = use_debounce(Duration::from_secs(2), {
        let mut show_toast = show_toast.clone();
        move |_| {
            show_toast.set(false);
        }
    });

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
            if show_toast() {
                div { class: "toast-notification", "{toast_message}" }
            }
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
                                        let mut clipboard = use_clipboard();
                                        if let Ok(_) = clipboard.set(emoji_str.to_string()) {
                                            toast_message.set(format!("Copied {} to clipboard!", emoji_str));
                                            show_toast.set(true);
                                            hide_toast.action(());
                                        } else {
                                            toast_message.set("Failed to copy emoji".to_string());
                                            show_toast.set(true);
                                            hide_toast.action(());
                                        }
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
