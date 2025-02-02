#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_popup::PopupService;

use crate::{components::icons::Logo, theme::Theme};

#[component]
pub fn CongraturationPopup(
    #[props(default ="congraturation_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
) -> Element {
    let theme: Theme = use_context();
    let theme = theme.get_data();
    let mut popup: PopupService = use_context();

    rsx! {
        div { id, class,
            div { class: "pt-[10px] flex flex-col items-center justify-start gap-[15px]",
                div { class: "flex flex-row items-center justify-center w-[88px] h-[88px] bg-[{theme.background}] rounded-[50%]",
                    Logo { width: 36, height: 43 }
                }

                div { class: "leading-[24px] text-[16px] font-regular text-white flex flex-row items-center justify-center text-center tracking-[0.005em]",
                    pre {
                        "‘서비스명’에 오신 것을 환영합니다!\n익명성과 신뢰를 바탕으로 안전한 투표 환경을 제공합니다."
                    }
                }

                button {
                    class: "w-[400px] h-[57px] rounded-[12px] mt-[20px] bg-[{theme.primary100}]  flex items-center justify-center text-white font-extrabold text-[18px] leading-[24px] tracking-[0.005em]",
                    onclick: move |_| {
                        popup.close();
                    },
                    "투표 시작하기"
                }
            }
        }
    }
}
