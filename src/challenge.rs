use crate::claim::ClaimComp;
use konnektoren_core::{challenges::ChallengeResult, game::Game};
use konnektoren_yew::components::challenge::{ChallengeComponent, ResultSummaryComponent};
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

    let handle_finish = {
        let challenge_result = challenge_result.clone();
        Callback::from(move |result: ChallengeResult| {
            log::info!("Challenge Result: {:?}", result);
            challenge_result.set(Some(result.clone()));
        })
    };
    let challenge = game.create_challenge(&props.id);

    match challenge {
        Ok(challenge) => {
            let result_summary = match &*challenge_result {
                Some(result) => {
                    html! {
                        <ResultSummaryComponent challenge={challenge.clone()} challenge_result={result.clone()} />
                    }
                }
                None => html! {},
            };
            let claim = match &*challenge_result {
                Some(_result) => {
                    html! {
                        <ClaimComp address={props.address.clone()} amount={10} />
                    }
                }
                None => html! {},
            };
            html! {
                <div class="challenge-page">
                    {result_summary}
                    {claim}
                    <ChallengeComponent challenge={challenge.clone()} on_finish={handle_finish} />
                </div>
            }
        }
        Err(e) => {
            html! {
                <div class="challenge">
                    <h1>{ "Challenge" }</h1>
                    <p>{ format!("Error: {}", e) }</p>
                </div>
            }
        }
    }
}
