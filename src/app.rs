use crate::model::SessionInitializerImpl;
use crate::pages::{AboutPage, ChallengePage, HomePage};
use crate::route::Route;
use konnektoren_core::controller::{ControllerPlugin, DebugPlugin};
use konnektoren_yew::i18n::{I18nConfig, I18nProvider};
use konnektoren_yew::prelude::repository_provider::create_repositories;
use konnektoren_yew::providers::{
    use_game_controller, DesignProvider, GameControllerProvider, RepositoryProvider, ThemeProvider,
};
use konnektoren_yew::repository::LocalStorage;
use std::sync::Arc;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

fn switch_main(route: Route) -> Html {
    match route {
        Route::About => html! {<AboutPage /> },
        Route::Home => html! {<HomePage />},
        Route::Challenge { id } => html! {
            <ChallengePage id={id}/>
        },
    }
}

#[function_component(InitApp)]
pub fn init_app() -> Html {
    let game_controller = use_game_controller();

    use_effect_with((), move |_| {
        let game_controller = game_controller.clone();
        let debug_plugin = Arc::new(DebugPlugin::new(Arc::new(log::logger())));
        debug_plugin
            .load(game_controller.controller)
            .unwrap_or_else(|e| {
                log::error!("Failed to load debug plugin: {:?}", e);
            });
    });

    html! {
        <div>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let i18n_config = I18nConfig::default();
    let storage = LocalStorage::new(None);
    let session_initializer = SessionInitializerImpl;
    let repository_config = create_repositories(storage, Arc::new(session_initializer));

    html! {
        <RepositoryProvider config={repository_config}>
        <I18nProvider config={i18n_config}>
        <DesignProvider>
        <ThemeProvider>
        <GameControllerProvider>
            <InitApp />
            <BrowserRouter>
                <Switch<Route> render={switch_main} />
            </BrowserRouter>
        </GameControllerProvider>
        </ThemeProvider>
        </DesignProvider>
        </I18nProvider>
        </RepositoryProvider>
    }
}
