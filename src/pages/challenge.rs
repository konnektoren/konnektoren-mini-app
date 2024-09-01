use crate::model::{GameLoader, WebSession};
use crate::prelude::ChallengeEffectComponent;
use crate::route::{AddressParam, Route};
use crate::{claim::ClaimComp, points::add_points};
use konnektoren_core::{
    challenges::{ChallengeResult, Performance},
    game::Game,
};
use konnektoren_yew::components::challenge::ResultSummaryComponent;
use std::ops::Div;
use yew::prelude::*;
use yew_router::components::Link;
use yew_router::hooks::use_location;

#[derive(Properties, PartialEq)]
pub struct ChallengePageProps {
    pub id: String,
}

#[function_component(ChallengePage)]
pub fn challenge_comp(props: &ChallengePageProps) -> Html {
    let location = use_location().unwrap();
    let address = location.query::<AddressParam>().unwrap().address;

    let game = Game::load_game();
    let challenge_result = use_state(|| Option::<ChallengeResult>::None);
    let challenge = game.create_challenge(&props.id).unwrap_or_default();
    let feedback_class = use_state(|| "".to_string());
    let web_session = WebSession::default();
    let current_level = web_session.session.game_state.current_game_path;

    let challenge_config = game.game_paths[current_level]
        .get_challenge_config(&props.id)
        .unwrap();

    let handle_finish = {
        let challenge_result = challenge_result.clone();
        let challenge = challenge.clone();
        Callback::from(move |result: ChallengeResult| {
            let points = challenge.performance(&result).div(10);
            add_points(points);
            challenge_result.set(Some(result.clone()));
        })
    };

    let result_summary = match &*challenge_result {
        Some(result) => {
            html! {
                <>
                <ResultSummaryComponent challenge={challenge.clone()} challenge_result={result.clone()} />
                <Link<Route> to={Route::Home}>
                    <span class="link-text">{ "Back to Home" }</span>
                </Link<Route>>
                </>
            }
        }
        None => html! {},
    };
    let claim = match &*challenge_result {
        Some(result) if challenge.performance(&result) >= 50 => {
            let amount: u32 = challenge.performance(&result).checked_div(10).unwrap_or(0) as u32;
            html! {
                <ClaimComp address={address.clone()} {amount} />
            }
        }
        _ => html! {},
    };
    html! {
        <div class={classes!("challenge-page", (*feedback_class).clone())}>
            {result_summary}
            {claim}
            <ChallengeEffectComponent challenge={challenge.clone()} variant={challenge_config.variant.clone()} on_finish={handle_finish} />
        </div>
    }
}
