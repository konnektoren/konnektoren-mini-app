use crate::pages::{AboutPage, HomePage};
use crate::route::Route;
use konnektoren_yew::i18n::{I18nConfig, I18nProvider};
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

fn switch_main(route: Route) -> Html {
    match route {
        Route::About => html! {<AboutPage /> },
        Route::Home => html! {<HomePage />},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let i18n_config = I18nConfig::default();

    html! {
        <div>
            <I18nProvider config={i18n_config}>
                <BrowserRouter>
                    <Switch<Route> render={switch_main} />
                </BrowserRouter>
            </I18nProvider>
        </div>
    }
}
