pub mod api;
pub mod app;
pub mod challenge_effect;
pub mod claim;
mod components;
pub mod footer;
pub mod is_correct;
mod model;
mod pages;
pub mod points;
mod route;
pub mod telegram;
pub mod version;
pub mod wallet;

pub mod prelude {
    pub use crate::app::App;
    pub use crate::challenge_effect::ChallengeEffectComponent;
    pub use crate::footer::FooterComp;
    pub use crate::model::*;
    pub use crate::pages::*;
    pub use crate::points::PointsComp;
    pub use crate::version::VersionComp;
    pub use crate::wallet::{WalletComp, WalletCompProps};
}
