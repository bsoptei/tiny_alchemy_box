pub(crate) mod coordinates;
pub(crate) mod elements;
pub(crate) mod helpers;

mod imp;
mod tests;

pub(crate) use crate::{
    coordinates::*,
    elements::*,
};
pub(crate) use std::str::FromStr;
pub(crate) use wasm_bindgen::prelude::*;

pub(crate) fn str2num<Num: FromStr>(s: &str) -> Result<Num, String> {
    s.parse::<Num>().or_else(|_| Err(format!("Could not parse {}", s)))
}

type TabParsingResult = Result<Tab, String>;

pub(crate) trait StrToTabParser {
    fn parse_tab(&self, input: &str) -> TabParsingResult;
}

pub(crate) trait Checker {
    fn check(&self, tab_parsing_result: TabParsingResult) -> TabParsingResult;
}

pub(crate) trait Visualizer<Output> {
    fn visualize(&self, tab_parsing_result: TabParsingResult) -> Output;
}

pub(crate) struct Container;

pub(crate) trait Dependencies<P: StrToTabParser, C: Checker, V: Visualizer<()>> {
    fn parser() -> P;
    fn checker() -> C;
    fn visualizer() -> V;
}

pub(crate) trait InputProcessor<P: StrToTabParser, C: Checker, V: Visualizer<()>>: Dependencies<P, C, V> {
    fn process_input(input: &str) -> () {
        Self::visualizer().visualize(
            Self::checker().check(
                Self::parser().parse_tab(input)
            )
        );
    }
}

#[wasm_bindgen]
pub fn process(input: &str) -> () {
    Container::process_input(input);
}

pub trait Update<T> {
    fn update(self, elem: T) -> Self;
}

impl<T> Update<T> for Vec<T> {
    fn update(self, elem: T) -> Self {
        let mut temp_self = self;
        temp_self.push(elem);
        temp_self
    }
}