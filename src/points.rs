use crate::telegram::{get_item, set_item};
use yew::prelude::*;

pub fn add_points(points: i32) {
    log::info!("Adding points: {}", points);
    wasm_bindgen_futures::spawn_local(async move {
        match get_item("points").await {
            Ok(result) => {
                let points_value = result.as_string().unwrap_or("0".to_string()).parse::<i32>().unwrap_or(0);
                let new_points = points_value + points;
                wasm_bindgen_futures::spawn_local(async move {
                    match set_item("points", &new_points.to_string()).await {
                        Ok(result) => {
                            match result.as_bool() {
                                Some(value) => {
                                    if value {
                                        log::info!("Points set to: {}", new_points);
                                    } else {
                                        log::error!("Error setting points: {:?}", result);
                                    }
                                }
                                None => {
                                    log::error!("Error setting points: {:?}", result);
                                }
                            }
                        }
                        Err(error) => {
                            log::error!("Error setting points: {:?}", error);
                        }
                    }
                });
            }
            Err(error) => {
                log::error!("Error getting points: {:?}", error);
            }
        }
    });
}

#[function_component(PointsComp)]
pub fn points_comp() -> Html {
    let points = use_state(|| 0);
    {
        let points = points.clone();
        use_effect_with(points.clone(), move |_| {
            let points = points.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match get_item("points").await {
                    Ok(result) => {
                        let points_value = result.as_string().unwrap_or("0".to_string()).parse::<u32>().unwrap_or(0);
                        points.set(points_value);
                    }
                    Err(error) => {
                        log::error!("Error getting points: {:?}", error);
                    }
                }
            });
            || ()
        });
    }

    html! {
        <div class="points">
            <span class="symbol">{"★"}</span>
            <span class="value">{*points}</span>
        </div>
    }
}
