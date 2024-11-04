#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use dioxus_sdk::storage::*;
use num_format::{Locale, ToFormattedString};
use serde::Deserialize;
use tailwind_fuse::tw_merge;

const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

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

#[component]
fn App() -> Element {
    let mut country =
        use_synced_storage::<LocalStorage, String>("country".to_string(), || "".to_string());
    let countryinfo =
        use_resource(move || async move { get_countryinfo(country.to_string()).await });
    let item = "flex py-3 flex-col gap-1";

    rsx! {
        main { class: "mx-auto max-w-[850px] px-6 pb-20",
            div { class: "pt-6 min-[950px]:pt-16",
                input {
                    aria_label: "Enter country",
                    placeholder: "Enter country",
                    spellcheck: false,
                    value: country,
                    r#type: "text",
                    autofocus: true,
                    oninput: move |event| country.set(event.value()),
                    class: tw_merge!(
                        "rounded-md border border-surface0 bg-base py-2 px-4 capitalize",
                        "outline-none transition-colors duration-300",
                        "placeholder:text-overlay0 hover:border-surface1",
                        "focus:text-text focus:border-surface2"
                    )
                }
                div { class: "mt-6",
                    if let Some(Ok(data)) = countryinfo.read().as_ref() {
                        ul { class: "animated-list grid grid-cols-1 sm:grid-cols-2 mt-5",
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Official name" }
                                    span { "{data[0].name.official} {data[0].flag}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Capital" }
                                    span { "{data[0].capital[0]}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Region" }
                                    span { "{data[0].region}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Subregion" }
                                    span { "{data[0].subregion}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "LatLng" }
                                    span { "{data[0].latlng[0]}/{data[0].latlng[1]}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Capital LatLng" }
                                    span {
                                        "{data[0].capital_info.latlng[0]}/{data[0].capital_info.latlng[0]}"
                                    }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Timezones" }
                                    span {
                                        {data[0].timezones.iter().map(|timezone|
                                        rsx!(
                                            span {"{timezone} "}
                                        ))}
                                    }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "TLD" }
                                    span {
                                        {data[0].tld.iter().map(|tld|
                                        rsx!(
                                            span {"{tld} "}
                                        ))}
                                    }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Population" }
                                    span { "{data[0].population.to_formatted_string(&Locale::en)}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Borders" }
                                    span {
                                        if let Some(borders) = &data[0].borders {
                                            { borders.iter().map(|border| rsx!(span {"{border} "})) }
                                        } else {
                                            span { "None" }
                                        }
                                    }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Languages" }
                                    span {
                                        {data[0].languages.values().map(|lang|
                                        rsx!(
                                            span {"{lang} "}
                                        ))}
                                    }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Currencies" }
                                    span {
                                        {data[0].currencies.values().map(|currency|
                                        rsx!(
                                            span {"{currency.name} ({currency.symbol}) "}
                                        ))}
                                    }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Landlocked" }
                                    if data[0].landlocked {
                                        span { "Yes" }
                                    } else {
                                        span { "No" }
                                    }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Start of week" }
                                    span { class: "capitalize", "{data[0].startOfWeek}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Continents" }
                                    span {
                                        if let Some(continents) = &data[0].continents {
                                            { continents.iter().map(|continent| rsx!(span {"{continent} "})) }
                                        }
                                    }
                                }
                            }
                            li {
                                div { class: item,
                                    span { class: "text-overlay0", "Maps" }
                                    span {
                                        a {
                                            class: "underlined",
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
