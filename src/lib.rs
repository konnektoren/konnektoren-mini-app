pub mod app;
pub mod challenge;
pub mod claim;
pub mod footer;
pub mod is_correct;
pub mod telegram;
pub mod wallet;

pub mod prelude {
    pub use crate::app::App;
    pub use crate::challenge::{ChallengeComp, ChallengeCompProps};
    pub use crate::footer::Footer;
    pub use crate::wallet::{WalletComp, WalletCompProps};
}
