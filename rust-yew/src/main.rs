use yew::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;
use web_sys::console;
use urlencoding::encode;


#[function_component(App)]
fn app() -> Html {
    let questions = use_state(Vec::new);
    let start = use_state(|| 0);
    let end = use_state(|| 1);
    let delete_id = use_state(String::new); // directly initializing with String::new
    let question_id = use_state(String::new); 
    let title = use_state(String::new); 
    let content = use_state(String::new); 
    let tags = use_state(Vec::new);

    let update_question_id = use_state(String::new);
    let update_title = use_state(String::new);
    let update_content = use_state(String::new);
    let update_tags = use_state(Vec::new);
    
    let on_id_add = {
        let question_id = question_id.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            question_id.set(input.value());
        })
    };
    
    let on_title_add = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };
    
    let on_content_add = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            content.set(input.value());
        })
    };
    
    let on_tags_add = {
        let tags = tags.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let tags_list = input.value().split(',').map(String::from).collect();
            tags.set(tags_list);
        })
    };

    let on_add_submit = {
        let question_id = question_id.clone();
        let title = title.clone();
        let content = content.clone();
        let tags = tags.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let data = serde_json::json!({
                "id": (*question_id).clone(),
                "title": (*title).clone(),
                "content": (*content).clone(),
                "tags": (*tags).clone()
            });
            wasm_bindgen_futures::spawn_local(async move {
                let url = "http://127.0.0.1:3030/add_question";
                match Request::post(url)
                    .header("Content-Type", "application/json")
                    .body(data.to_string())
                    .send()
                    .await {
                    Ok(response) => {
                        if response.ok() {
                            console::log_1(&"Question added successfully".into());
                        } else {
                            let status = response.status();
                            let status_text = response.status_text();
                            console::error_1(&format!("Failed to add question - Server responded with {}: {}", status, status_text).into());                        }
                    }
                    Err(err) => {
                        console::error_1(&format!("Error sending request: {:?}", err).into());
                    }
                }
            });
        })
    };
    

    // Callback to handle delete ID input
    let on_delete_id_input = {
        let delete_id = delete_id.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            delete_id.set(input.value());
        })
    };

    // Callback for form submission to delete a question
    let on_delete_submit = {
        let delete_id = delete_id.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default(); // Prevent the form from actually submitting
            let id = delete_id.to_string();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("http://127.0.0.1:3030/delete_questions/{}", encode(&id));
                match Request::delete(&url).send().await {
                    Ok(response) => {
                        if response.ok() {
                            console::log_1(&"Question deleted successfully".into());
                        } else {
                            console::error_1(&"Failed to delete the question".into());
                        }
                    }
                    Err(err) => {
                        console::error_1(&"Error sending delete request".into());
                        console::error_1(&format!("{:?}", err).into());
                    }
                }
            });
        })
    };

    let on_id_update = {
        let update_question_id = update_question_id.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            update_question_id.set(input.value());
        })
    };

    let on_title_update = {
        let update_title = update_title.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            update_title.set(input.value());
        })
    };

    let on_content_update = {
        let update_content = update_content.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            update_content.set(input.value());
        })
    };

    let on_tags_update = {
        let update_tags = update_tags.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let tags_list = input.value().split(',').map(String::from).collect();
            update_tags.set(tags_list);
        })
    };

    let on_update_submit = {
        let update_question_id = update_question_id.clone();
        let update_title = update_title.clone();
        let update_content = update_content.clone();
        let update_tags = update_tags.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let data = serde_json::json!({
                "id": (*update_question_id).clone(),
                "title": (*update_title).clone(),
                "content": (*update_content).clone(),
                "tags": (*update_tags).clone()
            });
            wasm_bindgen_futures::spawn_local({
                let update_question_id = update_question_id.clone();
                async move {
                    let url = format!("http://127.0.0.1:3030/update_question/{}", (*update_question_id).clone());
                    match Request::put(&url)
                        .header("Content-Type", "application/json")
                        .body(data.to_string())
                        .send()
                        .await {
                        Ok(response) => {
                            if response.ok() {
                                console::log_1(&"Question updated successfully".into());
                            } else {
                                let status = response.status();
                                let status_text = response.status_text();
                                console::error_1(&format!("Failed to update question - Server responded with {}: {}", status, status_text).into());
                            }
                        }
                        Err(err) => {
                            console::error_1(&format!("Error sending request: {:?}", err).into());
                        }
                    }
                }
            });
        })
    };
    
    
    
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
            
            <div style="display: flex; justify-content: center; margin-bottom: 20px;">
                <div style="margin: 0 20px; padding: 10px;">
                    <h3 style="text-align: center;">{ "ADD QUESTION" }</h3>
                    <form onsubmit={on_add_submit}>
                        <input type="text" placeholder="ID" oninput={on_id_add} />
                        <input type="text" placeholder="Title" oninput={on_title_add} />
                        <input type="text" placeholder="Content" oninput={on_content_add} />
                        <input type="text" placeholder="Tags (comma-separated)" oninput={on_tags_add} />
                        <button type="submit">{ "Add Question" }</button>
                    </form>
                </div>
                <div style="margin: 0 20px; padding: 10px;">
                    <h3 style="text-align: center;">{ "UPDATE QUESTION" }</h3>
                    <form onsubmit={on_update_submit}>
                        <input type="text" placeholder="Update ID" oninput={on_id_update} />
                        <input type="text" placeholder="Update Title" oninput={on_title_update} />
                        <input type="text" placeholder="Update Content" oninput={on_content_update} />
                        <input type="text" placeholder="Update Tags (comma-separated)" oninput={on_tags_update} />
                        <button type="submit">{ "Update Question" }</button>
                    </form>
                </div>
            </div>
    
            <div style="border: 2px solid red; padding: 10px; margin-bottom: 20px;">
                <form onsubmit={on_delete_submit}>
                    <input type="text" placeholder="Enter ID to delete" oninput={on_delete_id_input} />
                    <button type="submit">{ "Delete Question" }</button>
                </form>
            </div>
    
            <div>
                <form onsubmit={on_click_paginate}>
                    <input type="number" placeholder="Start" oninput={on_start_input} />
                    <input type="number" placeholder="End" oninput={on_end_input} />
                    <button type="submit">{ "Paginate" }</button>
                </form>
            </div>
    
            <div style="text-align: center; margin-bottom: 20px;">
                <button onclick={on_click_show_all}>{ "Show All Questions" }</button>
            </div>
    
            <div style="text-align: center;">
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