use *;

mod checker_impl;
mod parser_impl;
mod visualizer_impl;
pub mod drawable_components;
pub mod sizes;

use self::checker_impl::*;
use self::parser_impl::*;
use self::visualizer_impl::*;

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