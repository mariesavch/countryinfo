#![allow(non_snake_case)]

use css_in_rs::{make_styles, use_style_provider_quickstart, Classes, EmptyTheme};
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use dioxus_sdk::storage::*;
use num_format::{Locale, ToFormattedString};
use serde::Deserialize;
use std::collections::HashMap;

fn main() {
    dioxus_sdk::storage::set_dir!();
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    dioxus::launch(App);
}

#[derive(Deserialize, Debug)]
struct CountryData {
    #[serde(rename = "capitalInfo")]
    capital_info: CapitalInfo,
    name: Name,
    tld: Vec<String>,
    capital: Vec<String>,
    region: String,
    subregion: String,
    latlng: Vec<f64>,
    flag: String,
    population: u64,
    timezones: Vec<String>,
    languages: HashMap<String, String>,
    currencies: HashMap<String, Currency>,
    borders: Option<Vec<String>>,
    continents: Option<Vec<String>>,
    landlocked: bool,
    startOfWeek: String,
    maps: Maps,
}

#[derive(Deserialize, Debug)]
struct Currency {
    name: String,
    symbol: String,
}

#[derive(Deserialize, Debug)]
struct CapitalInfo {
    latlng: Vec<f64>,
}

#[derive(Deserialize, Debug)]
struct Name {
    official: String,
}

#[derive(Deserialize, Debug)]
struct Maps {
    openStreetMaps: String,
}

async fn get_countryinfo(location: String) -> reqwest::Result<Vec<CountryData>> {
    reqwest::get(format!(
        "https://restcountries.com/v3.1/translation/{}?fields=name,capital,population,flag,region,subregion,timezones,latlng,capitalInfo,tld,languages,currencies,borders,landlocked,startOfWeek,continents,maps",
        location
    ))
    .await?
    .json::<Vec<CountryData>>()
    .await
}

make_styles! {
    (_theme: EmptyTheme) -> MyClasses {
        ":root" {
            __rosewater: "#ff8389",
            __flamingo: "#ff8389",
            __red: "#ff8389",
            __maroon: "#ff8389",
            __pink: "#ff7eb6",
            __mauve: "#be95ff",
            __peach: "#d44a1c",
            __yellow: "#ab8600",
            __green: "#08bdba",
            __teal: "#33b1ff",
            __sky: "#33b1ff",
            __sapphire: "#33b1ff",
            __blue: "#78a9ff",
            __lavender: "#78a9ff",
            __text: "#ffffff",
            __subtext1: "#f4f4f4",
            __subtext0: "#e0e0e0",
            __overlay2: "#adadad",
            __overlay1: "#949494",
            __overlay0: "#7a7a7a",
            __surface2: "#4f4f4f",
            __surface1: "#383838",
            __surface0: "#2e2e2e",
            __base: "#161616",
            __mantle: "#0d0d0d",
            __crust: "#000000",
        },
    "@media (prefers-color-scheme: light)" {
        ":root" {
            __rosewater: "#da1e28",
            __flamingo: "#da1e28",
            __red: "#da1e28",
            __maroon: "#da1e28",
            __pink: "#d02670",
            __mauve: "#8a3ffc",
            __peach: "#d44a1c",
            __yellow: "#ab8600",
            __green: "#007d79",
            __teal: "#1192e8",
            __sky: "#1192e8",
            __sapphire: "#1192e8",
            __blue: "#0f62fe",
            __lavender: "#0f62fe",
            __text: "#000000",
            __subtext1: "#404040",
            __subtext0: "#474747",
            __overlay2: "#575757",
            __overlay1: "#595959",
            __overlay0: "#737373",
            __surface2: "#8c8c8c",
            __surface1: "#d1d1d1",
            __surface0: "#e6e6e6",
            __base: "#ffffff",
            __mantle: "#f2f2f2",
            __crust: "#ebebeb",
            }
        },
        ":root" {
            background_color: "var(--base)",
            color: "var(--text)",
            line_height: "1.6",
        },
        "@media (hover: hover) and (pointer: fine)" {
            ".animated_list li" {
                all: "unset",
                transition_property: "all",
                transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
                transition_duration: "300ms",
            },
            ".animated_list:hover li" {
                opacity: "0.5",
            },
            ".animated_list:hover li:hover" {
                opacity: "1",
            }
        },
        ".item" {
            display: "flex",
            flex_direction: "column",
            padding_top: "12px",
            padding_bottom: "12px",
            gap: "4px",
        },
        ".input" {
            all: "unset",
            padding_top: "0.5rem",
            padding_bottom: "0.5rem",
            padding_left: "1rem",
            padding_right: "1rem",
            border_radius: "0.375rem",
            border: "1px solid var(--surface0)",
            text_transform: "capitalize",
            transition_property: "color, background-color, border-color, text-decoration-color, fill, stroke",
            transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
            transition_duration: "300ms",
            color: "var(--text)",
        },
        ".input:hover" {
            border_color: "var(--surface1)",
        },
        ".input:focus" {
            border_color: "var(--surface2)",
        },
        ".input:placeholder" {
            color: "var(--overlay0)",
        },
        ".section" {
            padding_top: "24px",
        },
        "@media(min-width: 950px)" {
            ".section" {
                padding_top: "64px",
            }
        },
        ".underlined" {
            color: "unset",
            text_decoration_line: "underline",
            text_decoration_thickness: "2px",
            text_underline_offset: "4px",
            transition_property: "color, background-color, border-color, text-decoration-color, fill, stroke",
            transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
            transition_duration: "300ms",
            text_decoration_color: "var(--surface2)",
        },
        ".underlined:hover" {
            text_decoration_color: "var(--overlay2)",
        },
        ".underlined:active" {
            text_decoration_color: "var(--overlay1)",
        }
    }
}

#[component]
fn App() -> Element {
    use_style_provider_quickstart(|| EmptyTheme);
    let cls: &MyClasses = &MyClasses::use_style();

    let mut country =
        use_synced_storage::<LocalStorage, String>("country".to_string(), || "".to_string());
    let countryinfo =
        use_resource(move || async move { get_countryinfo(country.to_string()).await });

    rsx! {
        main {
            margin_left: "auto",
            margin_right: "auto",
            max_width: "768px",
            padding_left: "24px",
            padding_right: "24px",
            padding_bottom: "80px",
            div { class: &cls.section as &str,
                input {
                    aria_label: "Enter country",
                    placeholder: "Enter country",
                    spellcheck: false,
                    value: country,
                    r#type: "text",
                    autofocus: true,
                    oninput: move |event| country.set(event.value()),
                    class: &cls.input as &str,
                }
                div { margin_top: "12px",
                    if let Some(Ok(data)) = countryinfo.read().as_ref() {
                        ul {
                            class: &cls.animated_list as &str,
                            all: "unset",
                            display: "grid",
                            list_style_type: "none",
                            grid_template_columns: "repeat(auto-fit, minmax(300px, 1fr))",
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Official name" }
                                    span { "{data[0].name.official} {data[0].flag}" }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Capital" }
                                    span { "{data[0].capital[0]}" }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Region" }
                                    span { "{data[0].region}" }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Subregion" }
                                    span { "{data[0].subregion}" }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "LatLng" }
                                    span { "{data[0].latlng[0]}/{data[0].latlng[1]}" }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Capital LatLng" }
                                    span {
                                        "{data[0].capital_info.latlng[0]}/{data[0].capital_info.latlng[0]}"
                                    }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Timezones" }
                                    span {
                                        {data[0].timezones.iter().map(|timezone| rsx! {
                                            span { "{timezone} " }
                                        })}
                                    }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "TLD" }
                                    span {
                                        {data[0].tld.iter().map(|tld| rsx! {
                                            span { "{tld} " }
                                        })}
                                    }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Population" }
                                    span { "{data[0].population.to_formatted_string(&Locale::en)}" }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Borders" }
                                    span {
                                        if let Some(borders) = &data[0].borders {
                                            {borders.iter().map(|border| rsx! {
                                                span { "{border} " }
                                            })}
                                        } else {
                                            span { "None" }
                                        }
                                    }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Languages" }
                                    span {
                                        {data[0].languages.values().map(|lang| rsx! {
                                            span { "{lang} " }
                                        })}
                                    }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Currencies" }
                                    span {
                                        {data[0].currencies.values().map(|currency| rsx! {
                                            span { "{currency.name} ({currency.symbol}) " }
                                        })}
                                    }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Landlocked" }
                                    if data[0].landlocked {
                                        span { "Yes" }
                                    } else {
                                        span { "No" }
                                    }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Start of week" }
                                    span { class: "capitalize", "{data[0].startOfWeek}" }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Continents" }
                                    span {
                                        if let Some(continents) = &data[0].continents {
                                            {continents.iter().map(|continent| rsx! {
                                                span { "{continent} " }
                                            })}
                                        }
                                    }
                                }
                            }
                            li {
                                div { class: &cls.item as &str,
                                    span { color: "var(--overlay0)", "Maps" }
                                    span {
                                        a {
                                            class: &cls.underlined as &str,
                                            target: "_blank",
                                            href: data[0].maps.openStreetMaps.to_string(),
                                            "OpenStreetMaps"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
