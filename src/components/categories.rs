use dioxus::prelude::*;
use emojis::Group;

#[derive(PartialEq, Props, Clone)]
pub struct CategoriesProps {
    pub selected_group: Group,
    pub on_select: EventHandler<Group>,
}
#[component]
pub fn Categories(props: CategoriesProps) -> Element {
    let categories = vec![
        (Group::SmileysAndEmotion, "😀"),
        (Group::PeopleAndBody, "👋"),
        (Group::AnimalsAndNature, "🐶"),
        (Group::FoodAndDrink, "🍕"),
        (Group::TravelAndPlaces, "✈️"),
        (Group::Activities, "⚽"),
        (Group::Objects, "💡"),
        (Group::Symbols, "❤️"),
        (Group::Flags, "🏁"),
    ];

    rsx! {
        div { class: "categories",
            {
                categories
                    .iter()
                    .map(|(group, icon)| {
                        let is_selected = *group == props.selected_group;
                        let class = if is_selected { "category active" } else { "category" };
                        let group = *group;
                        rsx! {
                            div {
                                key: "{group:?}",
                                class: "{class}",
                                onclick: move |_| props.on_select.call(group),
                                "{icon}"
                            }
                        }
                    })
            }
        }
    }
}
