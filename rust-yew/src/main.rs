use yew::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;

#[function_component(App)]
fn app() -> Html {
    let questions = use_state(Vec::new);

    {
        let questions = questions.clone();
        use_effect_with_deps(move |_| {
            let questions = questions.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("http://127.0.0.1:3030/questions").send().await {
                    Ok(response) => {
                        if response.ok() {
                            match response.json::<Vec<Question>>().await {
                                Ok(fetched_questions) => {
                                    web_sys::console::log_1(&"Questions fetched successfully".into());
                                    web_sys::console::log_1(&format!("{:?}", fetched_questions).into());
                                    questions.set(fetched_questions);
                                }
                                Err(err) => {
                                    web_sys::console::log_1(&"Failed to parse JSON".into());
                                    web_sys::console::log_1(&format!("{:?}", err).into());
                                }
                            }
                        } else {
                            web_sys::console::log_1(&"Request failed".into());
                        }
                    }
                    Err(err) => {
                        web_sys::console::log_1(&"Failed to fetch questions".into());
                        web_sys::console::log_1(&format!("{:?}", err).into());
                    }
                }
            });
            || ()
        }, ());
    }

    html! {
        <div>
            <h1>{ "Questions" }</h1>
            <ul>
                { for questions.iter().map(|q| html! { <li>{ format!("{} - {}", q.title, q.content) }</li> }) }
            </ul>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties, Deserialize)]
struct Question {
    id: String,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

fn main() {
    yew::start_app::<App>();
}
