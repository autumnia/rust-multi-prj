#![warn(unused_imports)]

mod products;
// use products::Products;

mod form;
// use form::Form;
mod router;
use router::{Route, switch};

use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};


#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <div class="container">
                <h1 class="title"> {"Autumnia App"}</h1>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </div>
        </BrowserRouter>

        // <div class="container">
        //     <h1 class="title"> {"Autumnia App"}</h1>
        //     <hr color="green" width="100%"/>
        //     <Form />
        //     <hr color="green" width="100%"/>
        //     <Products />
        //     <hr color="green" width="100%"/>
        //     <p></p>
        // </div>
    }


}

fn main() {
    yew::Renderer::<App>::new().render();
}
