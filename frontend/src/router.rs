use yew::{Html, html};
use yew_router::prelude::*;

use crate::form::Form;
use crate::products::Products;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/addproduct")]
    AddProduct,

    #[at("/about")]
    About,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {  <Products />  },
        Route::AddProduct => html! { <Form /> },
        Route::About=> html! { <h1>{ "About" }</h1> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
