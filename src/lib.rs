pub mod api;
pub mod app;
pub mod challenge;
pub mod claim;
pub mod footer;
pub mod is_correct;
pub mod points;
pub mod telegram;
pub mod version;
pub mod wallet;

pub mod prelude {
    pub use crate::app::App;
    pub use crate::challenge::{ChallengeComp, ChallengeCompProps};
    pub use crate::footer::FooterComp;
    pub use crate::points::PointsComp;
    pub use crate::version::VersionComp;
    pub use crate::wallet::{WalletComp, WalletCompProps};
}
