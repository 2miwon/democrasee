use crate::layouts::root_layout::*;
use crate::pages::*;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
    #[layout(RootLayout)]
    #[route("/")]
    HomePage { lang: Language },
    #[end_layout]
    #[end_nest]

    #[redirect("/", || Route::HomePage { lang: Language::default() })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Copy)]
pub enum Language {
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "en")]
    En,
}

impl Default for Language {
    fn default() -> Self {
        #[cfg(feature = "web")]
        {
            use std::str::FromStr;

            let w = web_sys::window().unwrap();
            let loc = w.location().pathname().unwrap_or_default().clone();
            let paths: Vec<_> = loc.split("/").collect();
            if paths.len() > 1 {
                return Language::from_str(paths[1]).unwrap();
            }
        }
        Language::Ko
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Ko => write!(f, "ko"),
            Language::En => write!(f, "en"),
        }
    }
}

impl std::str::FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ko" => Ok(Language::Ko),
            "en" => Ok(Language::En),
            _ => Ok(Language::Ko),
        }
    }
}

impl Language {
    pub fn to_string(&self) -> String {
        match self {
            Language::Ko => "ko".to_string(),
            Language::En => "en".to_string(),
        }
    }
}
