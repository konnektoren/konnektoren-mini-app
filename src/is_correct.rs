use konnektoren_core::challenges::{Challenge, ChallengeResult, ChallengeType};

pub trait IsCorrect {
    fn is_correct(&self, index: usize) -> bool;
}

impl IsCorrect for Challenge {
    fn is_correct(&self, index: usize) -> bool {
        log::info!("{:?}", self.challenge_result);
        let option = match self.challenge_type {
            ChallengeType::MultipleChoice(ref mc) => mc.options.get(index),
            ChallengeType::Informative(_) => unreachable!("Informative is not implemented"),
            ChallengeType::SortTable(_) => unreachable!("SortTable is not implemented"),
            ChallengeType::Custom(_) => unreachable!("Custom is not implemented"),
            ChallengeType::ContextualChoice(_) => {
                unreachable!("ContextualChoice is not implemented")
            }
            ChallengeType::GapFill(_) => unreachable!("GapFill is not implemented"),
            ChallengeType::Ordering(_) => unreachable!("Ordering is not implemented"),
            ChallengeType::Placeholder(_) => unreachable!("Placeholder is not implemented"),
        };
        let result = match self.challenge_result {
            ChallengeResult::MultipleChoice(ref mc) => mc.get(index),
            ChallengeResult::Informative => unreachable!("Informative is not implemented"),
            ChallengeResult::SortTable(_) => unreachable!("SortTable is not implemented"),
            ChallengeResult::Custom(_) => unreachable!("Custom is not implemented"),
            ChallengeResult::ContextualChoice(_)
            | ChallengeResult::GapFill(_)
            | ChallengeResult::Ordering(_) => todo!(),
        };
        log::info!("Option: {:?}, Result: {:?}", option, result);
        match (option, result) {
            (Some(option), Some(result)) => option == result,
            _ => false,
        }
    }
}
