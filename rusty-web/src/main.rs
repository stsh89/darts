#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    launch(App);
}

fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css" }

        section { class: "section",
            div { class: "container",
                div { class: "columns is-centered",
                    div { class: "column is-half",
                        div { class: "block",
                            form { action: "/games", method: "post",
                                div { class: "field",
                                    div { class: "control",
                                        button { class: "button", autofocus: "true", "New game" }
                                    }
                                }
                            }
                        }
                        div { class: "block",
                            h1 { class: "title", "Recent games" }
                        }
                        div { class: "block",
                            table { class: "table is-fullwidth is-striped",
                                thead { tr { th { "Create time" } th { "Update time" } th { "Actions" } } }
                                tbody {
                                    tr { td { "2022-01-01 00:00:00" } td { "2022-01-01 00:00:00" } td { a { href: "/games/1", "Show" } } }
                                }
                            }
                        }
                    }
                }
            }
        }

        // link { rel: "stylesheet", href: "main.css" }
        // img { src: "header.svg", id: "header" }
        // div { id: "links",
        //     a { href: "https://dioxuslabs.com/learn/0.5/", "📚 Learn Dioxus" }
        //     a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
        //     a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
        //     a { href: "https://github.com/DioxusLabs/dioxus-std", "⚙️ Dioxus Standard Library" }
        //     a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
        //         "💫 VSCode Extension"
        //     }
        //     a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        // }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server 2!".to_string())
}
