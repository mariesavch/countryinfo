#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use dioxus_sdk::storage::*;
use num_format::{Locale, ToFormattedString};
use serde::Deserialize;
use sir::{css, global_css, AppStyle};

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

    global_css!(
        "
       :root {
        --rosewater: #ff8389;
        --flamingo: #ff8389;
        --red: #ff8389;
        --maroon: #ff8389;
        --pink: #ff7eb6;
        --mauve: #be95ff;
        --peach: #d44a1c;
        --yellow: #ab8600;
        --green: #08bdba;
        --teal: #33b1ff;
        --sky: #33b1ff;
        --sapphire: #33b1ff;
        --blue: #78a9ff;
        --lavender: #78a9ff;
        --text: #ffffff;
        --subtext1: #f4f4f4;
        --subtext0: #e0e0e0;
        --overlay2: #adadad;
        --overlay1: #949494;
        --overlay0: #7a7a7a;
        --surface2: #4f4f4f;
        --surface1: #383838;
        --surface0: #2e2e2e;
        --base: #161616;
        --mantle: #0d0d0d;
        --crust: #000000;
    } 

    @media (prefers-color-scheme: light) {
        :root {
            --rosewater: #da1e28;
            --flamingo: #da1e28;
            --red: #da1e28;
            --maroon: #da1e28;
            --pink: #d02670;
            --mauve: #8a3ffc;
            --peach: #d44a1c;
            --yellow: #ab8600;
            --green: #007d79;
            --teal: #1192e8;
            --sky: #1192e8;
            --sapphire: #1192e8;
            --blue: #0f62fe;
            --lavender: #0f62fe;
            --text: #000000;
            --subtext1: #404040;
            --subtext0: #474747;
            --overlay2: #575757;
            --overlay1: #595959;
            --overlay0: #737373;
            --surface2: #8c8c8c;
            --surface1: #d1d1d1;
            --surface0: #e6e6e6;
            --base: #ffffff;
            --mantle: #f2f2f2;
            --crust: #ebebeb;
        }
    }

    :root {
        background-color: var(--base);
        color: var(--text);
        line-height: 1.6;
    }

    "
    );

    let underlined =css!("
    color: unset;
    text-decoration-line: underline; 
    text-decoration-thickness: 2px; 
    text-underline-offset: 4px; 
    transition-property: color, background-color, border-color, text-decoration-color, fill, stroke; 
    transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); 
    transition-duration: 300ms;
    text-decoration-color: var(--surface2);

    &:hover {
        text-decoration-color: var(--overlay2);
    }

    &:active {
        text-decoration-color: var(--overlay1);
    }
    ");

    let animated_list = css!(
        "
    @media (hover: hover) and (pointer: fine) {
        li {
            transition-property: all;
            transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
            transition-duration: 300ms;
        }
        &:hover li {
            opacity: 0.5;
        }
        &:hover li:hover {
            opacity: 1;
        }
    }
    "
    );

    let item = css!(
        "
        display: flex;
        flex-direction: column;
        padding-top: 12px;
        padding-bottom: 12px;
        gap: 4px;"
    );

    let input = css!(
        "
        all: unset;
        padding-top: 0.5rem;
        padding-bottom: 0.5rem; 
        padding-left: 1rem;
        padding-right: 1rem;
        border-radius: 0.375rem; 
        border: 1px solid var(--surface0); 
        text-transform: capitalize; 
        transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        transition-duration: 300ms; 
        color: var(--text);

        &:hover {
            border-color: var(--surface1);
        }
        &:focus {
            border-color: var(--surface2);
        }
        &:placeholder {
            color: var(--overlay0);
        }
        "
    );

    let section = css!(
        "
    padding-top: 24px;
    @media(min-width: 950px) {
        padding-top: 64px
    }
    "
    );

    rsx! {
        AppStyle {}
        main {
            margin_left: "auto",
            margin_right: "auto",
            max_width: "768px",
            padding_left: "24px",
            padding_right: "24px",
            padding_bottom: "80px",
            div { class: section,
                input {
                    aria_label: "Enter country",
                    placeholder: "Enter country",
                    spellcheck: false,
                    value: country,
                    r#type: "text",
                    autofocus: true,
                    oninput: move |event| country.set(event.value()),
                    class: input,
                }
                div { margin_top: "12px",
                    if let Some(Ok(data)) = countryinfo.read().as_ref() {
                        ul {
                            class: animated_list,
                            all: "unset",
                            display: "grid",
                            list_style_type: "none",
                            grid_template_columns: "repeat(auto-fit, minmax(300px, 1fr))",
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Official name" }
                                    span { "{data[0].name.official} {data[0].flag}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Capital" }
                                    span { "{data[0].capital[0]}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Region" }
                                    span { "{data[0].region}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Subregion" }
                                    span { "{data[0].subregion}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "LatLng" }
                                    span { "{data[0].latlng[0]}/{data[0].latlng[1]}" }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Capital LatLng" }
                                    span {
                                        "{data[0].capital_info.latlng[0]}/{data[0].capital_info.latlng[0]}"
                                    }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Timezones" }
                                    span {
                                        {data[0].timezones.iter().map(|timezone| rsx! {
                                            span { "{timezone} " }
                                        })}
                                    }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "TLD" }
                                    span {
                                        {data[0].tld.iter().map(|tld| rsx! {
                                            span { "{tld} " }
                                        })}
                                    }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Population" }
                                    span { "{data[0].population.to_formatted_string(&Locale::en)}" }
                                }
                            }
                            li {
                                div { class: item,
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
                                div { class: item,
                                    span { color: "var(--overlay0)", "Languages" }
                                    span {
                                        {data[0].languages.values().map(|lang| rsx! {
                                            span { "{lang} " }
                                        })}
                                    }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Currencies" }
                                    span {
                                        {data[0].currencies.values().map(|currency| rsx! {
                                            span { "{currency.name} ({currency.symbol}) " }
                                        })}
                                    }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Landlocked" }
                                    if data[0].landlocked {
                                        span { "Yes" }
                                    } else {
                                        span { "No" }
                                    }
                                }
                            }
                            li {
                                div { class: item,
                                    span { color: "var(--overlay0)", "Start of week" }
                                    span { class: "capitalize", "{data[0].startOfWeek}" }
                                }
                            }
                            li {
                                div { class: item,
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
                                div { class: item,
                                    span { color: "var(--overlay0)", "Maps" }
                                    span {
                                        a {
                                            class: underlined,
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
