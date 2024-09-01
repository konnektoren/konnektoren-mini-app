use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

impl From<&str> for Route {
    fn from(query: &str) -> Self {
        if query.contains("page=about") {
            return Route::About;
        }
        Route::Home
    }
}
