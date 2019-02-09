mod checker_impl;
mod parser_impl;
mod visualizer_impl;
pub mod drawable_components;
pub mod sizes;

use crate::*;

use self::{
    checker_impl::*,
    parser_impl::*,
    visualizer_impl::*,
};

impl Dependencies<TabParser, BypassChecker, CanvasVisualizer> for Container {
    fn parser() -> TabParser {
        TabParser
    }

    fn checker() -> BypassChecker {
        BypassChecker
    }

    fn visualizer() -> CanvasVisualizer {
        CanvasVisualizer
    }
}

impl InputProcessor<TabParser, BypassChecker, CanvasVisualizer> for Container {}