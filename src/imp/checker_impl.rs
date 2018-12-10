use crate::{Checker, TabParsingResult};

pub struct BypassChecker;

impl Checker for BypassChecker {
    fn check(&self, tab_parsing_result: TabParsingResult) -> TabParsingResult {
        tab_parsing_result
    }
}
