use super::categories::Categories;
use super::emoji_grid::EmojiGrid;
use super::emoji_search::EmojiSearchIndex;
use dioxus::prelude::*;
use emojis::Group;
use std::rc::Rc;

#[component]
pub fn EmojiPicker() -> Element {
    let mut selected_group = use_signal(|| Group::SmileysAndEmotion);

    let mut search_query = use_signal(|| String::new());
    let mut search_results = use_signal(|| None);

    let search_index = use_memo(|| Rc::new(EmojiSearchIndex::new()));

    use_effect(move || {
        let results = if search_query().is_empty() {
            None
        } else {
            Some(search_index().search(&search_query()))
        };
        search_results.set(results);
    });

    rsx! {
        div { class: "emoji-picker",
            div { class: "emoji-picker-header",
                div { class: "search-container",
                    input {
                        class: "search-input",
                        r#type: "text",
                        placeholder: "Search emojis...",
                        value: "{search_query}",
                        oninput: move |event| search_query.set(event.value()),
                    }
                    div {
                        class: "clear-input",
                        role: "button",
                        "area-label": "Clear search",
                        onclick: move |_| search_query.set(String::new()),
                    }
                }
            }
            Categories {
                selected_group: selected_group(),
                on_select: move |group| selected_group.set(group),
            }
            EmojiGrid {
                selected_group: selected_group(),
                search_results: search_results(),
            }
        }
    }
}
