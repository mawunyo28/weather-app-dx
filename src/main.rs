use dioxus::prelude::*;
pub mod weather;
use weather::Weather;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const _HEADER_SVG: Asset = asset!("/assets/header.svg");

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    FirstPage {},
    #[route("/:location")]
    SecondPage { location: String },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Title {"Weather Application"}
        document::Link {rel: "preconnect", href: "https://fonts.googleapis.com"}
        document::Link {rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "anonymous"}
        document::Link {href: "https://fonts.googleapis.com/css2?family=Doto:wght@100..900&display=swap", rel: "stylesheet"}
        document::Script {
            src: "https://kit.fontawesome.com/f747ddc7f0.js",
            crossorigin: "anonymous"
        }
        Router::<Route> {}
    }
}

#[component]
pub fn SecondPage(location: String) -> Element {
    let weather = use_server_future(move ||  fetch_weather(location.clone()))?;

   

    rsx! {
        div {
            class: "container",
            h2 {id: "title", "Weather Application" }

            img{
                src: "fake.com"
            }

            "Tamso",
            button {id:"button",   "Retry"}

            Link {
                to: Route::FirstPage {  },


            button {id:"back-button",  i {
                class: "fa-solid fa-arrow-left",
            }}

        }
        }

        Outlet::<Route> {}
    }
}

#[component]
pub fn FirstPage() -> Element {
    rsx! {


        div {
            class: "container",
            h2 {id: "title", "Weather Application" },
            span {
                input {type:"text", placeholder: "Location" , "What is your name"}

                Link {to: Route::SecondPage { location: String::from("Tamso").to_owned() },

                button { id: "next",  i {
                    class: "fa-solid fa-arrow-right"
                } }
            }


              },

        }

        Outlet::<Route> {}
    }
}

#[server]
async fn fetch_weather(location: String) -> Result<Weather, ServerFnError> {
    let request_url = format!(
        "https://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no
    ",
        "1dbfc060cacd42909e3180510251403", location
    );

    let weather = reqwest::Client::new()
        .get(request_url)
        .send()
        .await?
        .json::<Weather>()
        .await?;

    Ok(weather)
}
