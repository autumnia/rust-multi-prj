#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use serde_json::json;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::router::Route;

#[function_component]
pub fn Form() -> Html {
    let navigator = use_navigator().unwrap();

    // input value
    let name_ref = NodeRef::default();
    let name_ref_outer = name_ref.clone();

    let price_ref = NodeRef::default();
    let price_ref_outer = price_ref.clone();

    let onclick = Callback::from( move |_| {
        // gloo_console::log!("Button clicked");
        let price = price_ref.cast::<HtmlInputElement>().unwrap();
        let name = name_ref.cast::<HtmlInputElement>().unwrap();
        gloo_console::log!(name.value() );
        gloo_console::log!(price.value() );

        wasm_bindgen_futures::spawn_local(async move {
            let product = json!({
                "name": name.value(),
                "price": price.value().parse::<i32>().unwrap()
            });

            let client = reqwest::Client::new();
            let res = client.post("http://localhost:3000/api/v1/products")
                .json(&product)
                .send()
                .await;
        });

        navigator.push(&Route::Home);
    });
    // submit form data

    html! {
        <div class="container">
            <h2> {"Add a Product"}</h2>
            <div>
                <label for="name" class="">
                    {"Name: "}
                    <input id="name" ref={name_ref_outer.clone()}
                        clase="formInput" type="text" />
                </label>

                <br />

                <label for="price" class="">
                    {"Price: "}
                    <input id="price" ref={price_ref_outer.clone()}
                        clase="formInput" type="number" />
                </label>

                <br />

                <button id="submit_button"
                    class="btn-primary"  {onclick} >
                    {"Add Product"}
                </button>
            </div>
        </div>
    }
}