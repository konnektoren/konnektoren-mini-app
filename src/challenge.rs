use konnektoren_core::game::Game;
use konnektoren_yew::components::challenge::ChallengeComponent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeCompProps {
    pub id: String,
}

#[function_component(ChallengeComp)]
pub fn challenge_comp(props: &ChallengeCompProps) -> Html {
    let game = Game::default();

    let challenge = game.create_challenge(&props.id);

    match challenge {
        Ok(challenge) => {
            html! {
                <div class="challenge">
                    <ChallengeComponent challenge={challenge} />
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
