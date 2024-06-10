use crate::claim::ClaimComp;
use gloo_timers::callback::Timeout;
use konnektoren_core::{
    challenges::{ChallengeResult, Performance},
    game::Game,
};
use konnektoren_yew::components::challenge::{
    ChallengeComponent, ChallengeEvent, ResultSummaryComponent,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeCompProps {
    pub id: String,
    pub address: String,
}

#[function_component(ChallengeComp)]
pub fn challenge_comp(props: &ChallengeCompProps) -> Html {
    let game = Game::default();
    let challenge_result = use_state(|| Option::<ChallengeResult>::None);
    let challenge = game.create_challenge(&props.id).unwrap_or_default();
    let feedback_class = use_state(|| "".to_string());

    let handle_event = {
        let challenge_result = challenge_result.clone();
        let feedback_class = feedback_class.clone();
        Callback::from(move |event: ChallengeEvent| match event {
            ChallengeEvent::Finish(result) => {
                challenge_result.set(Some(result.clone()));
            }
            ChallengeEvent::NextTask(_index) => {}
            ChallengeEvent::PreviousTask(_index) => {}
            ChallengeEvent::SolvedCorrect(_index) => {
                feedback_class.set("correct".to_string());
                let feedback_class = feedback_class.clone();
                Timeout::new(800, move || {
                    feedback_class.set("".to_string());
                })
                .forget();
            }
            ChallengeEvent::SolvedIncorrect(_index) => {
                feedback_class.set("incorrect".to_string());
                let feedback_class = feedback_class.clone();
                Timeout::new(800, move || {
                    feedback_class.set("".to_string());
                })
                .forget();
            }
        })
    };

    let result_summary = match &*challenge_result {
        Some(result) => {
            html! {
                <ResultSummaryComponent challenge={challenge.clone()} challenge_result={result.clone()} />
            }
        }
        None => html! {},
    };
    let claim = match &*challenge_result {
        Some(result) if challenge.performance(&result) >= 50 => {
            let amount: u32 = challenge.performance(&result).checked_div(10).unwrap_or(0) as u32;
            html! {
                <ClaimComp address={props.address.clone()} {amount} />
            }
        }
        _ => html! {},
    };
    html! {
        <div class={classes!("challenge-page", (*feedback_class).clone())}>
            {result_summary}
            {claim}
            <ChallengeComponent challenge={challenge.clone()} on_event={handle_event} />
        </div>
    }
}
