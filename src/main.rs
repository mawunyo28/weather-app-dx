use dioxus::{html::input, prelude::*};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const _HEADER_SVG: Asset = asset!("/assets/header.svg");

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
        
        SecondPage { location: "This" }


    }
}

#[component]
pub fn SecondPage(location: String) -> Element {
    let location = "Accra";
    let status = "Raining";
    rsx! {
        div {
            class: "container",
            h2 {id: "title", "Weather Application" }

            img{
                src: "https://cdn.weatherapi.com/weather/64x64/day/116.png"
            }

            "{location}, {status}",
            button {id:"button",  "Retry"}
        }
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
                button { id: "next",  "Next" }
              },
            
        }
    }
}




