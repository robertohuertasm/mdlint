use crate::ruleset::RuleResultDetails;

crate trait VecExt {
    fn to_option(self) -> Option<Vec<RuleResultDetails>>;
}

impl VecExt for Vec<RuleResultDetails> {
    fn to_option(self) -> Option<Vec<RuleResultDetails>> {
        if !self.is_empty() {
            Some(self)
        } else {
            None
        }
    }
}