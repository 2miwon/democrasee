#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    pages::{
        finished_topics_component::FinishedTopics, highlighted_topic_component::HighlightedTopics,
        upcoming_topics_component::UpcomingTopics,
    },
    route::Language,
};

#[component]
pub fn HomePage(lang: Language) -> Element {
    let ctrl = super::controller::Controller::new()?;
    let _tr = super::i18n::translate_pages(&lang);

    rsx! {
        div { class: "flex flex-col gap-[100px] grid grid-cols-1 mb-[20px]",
            HighlightedTopics {
                class: "col-span-1",
                topics: ctrl.ongoing_topics(),
                onselect: |_| {},
            }
            div { class: "col-span-1 w-full flex flex-row items-start justify-center gap-x-[20px] gap-y-[60px] grid grid-cols-2 max-[1000px]:grid-cols-1",
                FinishedTopics { class: "col-span-1", topics: ctrl.finished_topics() }
                UpcomingTopics { class: "col-span-1", _topics: ctrl.upcoming_topics() }
            }
        }
    }
}
