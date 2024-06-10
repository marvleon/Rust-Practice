use yew::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;
use web_sys::console;



#[function_component(App)]
fn app() -> Html {
    let questions = use_state(|| vec![]);
    let start = use_state(|| 0);
    let end = use_state(|| 1);

    {
        let questions = questions.clone();
        let start = *start;
        let end = *end;
        use_effect_with_deps(move |_| {
            let questions = questions.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("http://127.0.0.1:3030/questions?start={}&end={}", start, end);
                match Request::get(&url).send().await {
                    Ok(response) => {
                        if response.ok() {
                            match response.json::<Vec<Question>>().await {
                                Ok(fetched_questions) => {
                                    console::log_1(&"Questions fetched successfully".into());
                                    console::log_1(&format!("{:?}", fetched_questions).into());
                                    questions.set(fetched_questions);
                                }
                                Err(err) => {
                                    console::error_1(&"Failed to parse JSON".into());
                                    console::error_1(&format!("{:?}", err).into());
                                }
                            }
                        } else {
                            console::error_1(&"Request failed".into());
                        }
                    }
                    Err(err) => {
                        console::error_1(&"Failed to fetch questions".into());
                        console::error_1(&format!("{:?}", err).into());
                    }
                }
            });
            || ()
        }, (start, end));
    }


    let on_click_show_all = {
        let questions = questions.clone();
        Callback::from(move |_| {
            let questions = questions.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("http://127.0.0.1:3030/questions").send().await {
                    Ok(response) => {
                        if response.ok() {
                            match response.json::<Vec<Question>>().await {
                                Ok(fetched_questions) => {
                                    console::log_1(&"All questions fetched successfully".into());
                                    console::log_1(&format!("{:?}", fetched_questions).into());
                                    questions.set(fetched_questions);
                                }
                                Err(err) => {
                                    console::error_1(&"Failed to parse JSON".into());
                                    console::error_1(&format!("{:?}", err).into());
                                }
                            }
                        } else {
                            console::error_1(&"Request failed".into());
                        }
                    }
                    Err(err) => {
                        console::error_1(&"Failed to fetch questions".into());
                        console::error_1(&format!("{:?}", err).into());
                    }
                }
            });
        })
    };

let on_click_paginate = {
        let questions = questions.clone();
        let start = start.clone();
        let end = end.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let questions = questions.clone();
            let start_value = *start;
            let end_value = *end;
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("http://127.0.0.1:3030/question?start={}&end={}", start_value, end_value);
                match Request::get(&url).send().await {
                    Ok(response) => {
                        if response.ok() {
                            match response.json::<Vec<Question>>().await {
                                Ok(fetched_questions) => {
                                    console::log_1(&"Paginated questions fetched successfully".into());
                                    console::log_1(&format!("{:?}", fetched_questions).into());
                                    questions.set(fetched_questions);
                                }
                                Err(err) => {
                                    console::error_1(&"Failed to parse JSON".into());
                                    console::error_1(&format!("{:?}", err).into());
                                }
                            }
                        } else {
                            console::error_1(&"Request failed".into());
                        }
                    }
                    Err(err) => {
                        console::error_1(&"Failed to fetch questions".into());
                        console::error_1(&format!("{:?}", err).into());
                    }
                }
            });
        })
    };

    let on_start_input = {
        let start = start.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<u32>() {
                    start.set(value);
                }
            }
        })
    };

    let on_end_input = {
        let end = end.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<u32>() {
                    end.set(value);
                }
            }
        })
    };

    
    html! {
        <div>
            <marquee>{ "WORK IN PROGRESS" }</marquee>
            <h1 style="text-align:center;">{ "Marvin's Rust Web App" }</h1>
            <div>{ "Empty container for future forms" }</div>
            <div>{ "Empty container for future forms" }</div>
            <div>
                <button onclick={on_click_show_all}>{ "Show All Questions" }</button>
                <form onsubmit={on_click_paginate}>
                    <input type="number" placeholder="Start" oninput={on_start_input} />
                    <input type="number" placeholder="End" oninput={on_end_input} />
                    <button type="submit">{ "Paginate" }</button>
                </form>
                <h2>{ "Questions" }</h2>
                <ul>
                    { for questions.iter().map(|q| html! { <li>{ format!("{} - {}", q.title, q.content) }</li> }) }
                </ul>
            </div>
            <marquee>{ "WORK IN PROGRESS" }</marquee>
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
