#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use dioxus_sdk::storage::*;
use num_format::{Locale, ToFormattedString};
use serde::Deserialize;

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
    #[serde(rename = "name")]
    name: Name,
    #[serde(rename = "tld")]
    tld: Vec<String>,
    capital: Vec<String>,
    #[serde(rename = "region")]
    region: String,
    #[serde(rename = "subregion")]
    subregion: String,
    #[serde(rename = "latlng")]
    latlng: Vec<f64>,
    #[serde(rename = "flag")]
    flag: String,
    #[serde(rename = "population")]
    population: u64,
    #[serde(rename = "timezones")]
    timezones: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct CapitalInfo {
    #[serde(rename = "latlng")]
    latlng: Vec<f64>,
}

#[derive(Deserialize, Debug)]
struct Name {
    #[serde(rename = "official")]
    official: String,
    // #[serde(rename = "nativeName")]
    // native_name: NativeName,
}

// #[derive(Deserialize, Debug)]
// struct NativeName {
//     #[serde(rename = "bar")]
//     bar: NativeNameBar,
// }
//
// #[derive(Deserialize, Debug)]
// struct NativeNameBar {
//     #[serde(rename = "official")]
//     official: String,
// }

async fn get_countryinfo(location: String) -> reqwest::Result<Vec<CountryData>> {
    reqwest::get(format!(
        "https://restcountries.com/v3.1/translation/{}?fields=name,capital,population,flag,region,subregion,timezones,latlng,capitalInfo,tld",
        location
    ))
    .await?
    .json::<Vec<CountryData>>()
    .await
}

#[component]
fn App() -> Element {
    let mut country =
        use_synced_storage::<LocalStorage, String>("country".to_string(), || "Austria".to_string());
    let countryinfo =
        use_resource(move || async move { get_countryinfo(country.to_string()).await });

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
                    class: "rounded-md border border-surface0 bg-base
                            py-2 px-4 capitalize shadow-sm
                            outline-none transition-colors duration-300
                            placeholder:text-overlay0 hover:border-surface1
                            focus:text-text focus:border-surface2"
                }
                div { class: "mt-6",
                    if let Some(Ok(data)) = countryinfo.read().as_ref() {
                        ul { class: "animated-list grid grid-cols-1 sm:grid-cols-2 mt-5",
                            li {
                                div { class: "flex py-3 flex-col gap-1",
                                    span { class: "text-overlay0", "Official name" }
                                    span { "{data[0].name.official} {data[0].flag}" }
                                }
                            }
                            li {
                                div { class: "flex py-3 flex-col gap-1",
                                    span { class: "text-overlay0", "Capital" }
                                    span { "{data[0].capital[0]}" }
                                }
                            }
                            li {
                                div { class: "flex py-3 flex-col gap-1",
                                    span { class: "text-overlay0", "Region" }
                                    span { "{data[0].region}" }
                                }
                            }
                            li {
                                div { class: "flex py-3 flex-col gap-1",
                                    span { class: "text-overlay0", "Subregion" }
                                    span { "{data[0].subregion}" }
                                }
                            }
                            li {
                                div { class: "flex py-3 flex-col gap-1",
                                    span { class: "text-overlay0", "LatLng" }
                                    span { "{data[0].latlng[0]}/{data[0].latlng[1]}" }
                                }
                            }
                            li {
                                div { class: "flex py-3 flex-col gap-1",
                                    span { class: "text-overlay0", "Timezones" }
                                    span { class: "w-[15rem] truncate text-ellipsis",
                                        {data[0].timezones.iter().map(|timezone|
                                        rsx!(
                                            span {"{timezone} "}
                                        ))}
                                    }
                                }
                            }
                            li {
                                div { class: "flex py-3 flex-col gap-1",
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
                                div { class: "flex py-3 flex-col gap-1",
                                    span { class: "text-overlay0", "Population" }
                                    span { "{data[0].population.to_formatted_string(&Locale::en)}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
