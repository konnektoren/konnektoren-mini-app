use serde::{Deserialize, Serialize};
use yew_router::prelude::*;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct AddressParam {
    pub(crate) address: String,
}

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/challenge/:id")]
    Challenge { id: String },
}

impl From<&str> for Route {
    fn from(query: &str) -> Self {
        if query.contains("page=about") {
            return Route::About;
        }
        if query.contains("page=challenge") {
            let id = query
                .split('&')
                .find(|part| part.starts_with("id="))
                .and_then(|id_part| id_part.split('=').nth(1))
                .unwrap_or("")
                .to_string();
            return Route::Challenge { id };
        }
        Route::Home
    }
}
