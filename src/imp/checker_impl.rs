use crate::{Checker, elements::*, TabParsingResult};
use photonix::*;

pub struct SimpleChecker;

impl Checker for SimpleChecker {
    fn check(&self, tab_parsing_result: TabParsingResult) -> TabParsingResult {
        match tab_parsing_result {
            Err(_) => tab_parsing_result,
            Ok(tab) => {
                let num_of_strings: u8 = *tab.get_ref_second();
                let bars: &Vec<Bar> = tab.get_ref();
                let error_msg: Option<String> =
                    bars.iter().enumerate().fold(None, |temp, (n, current_bar)| {
                        if temp.is_none() {
                            let mut error_str = String::default();

                            let sig: TimeSignature = *current_bar.get_ref();
                            let sig_time = sig.time();
                            if sig_time <= 0f32 {
                                error_str.push_str(
                                    &format!("Invalid time signature in bar {}. ", n + 1)
                                );
                                return Some(error_str);
                            }
                            let bar_time = current_bar.time();

                            let tolerance = 1e-5;
                            let times_match =  (bar_time - sig_time).abs() < tolerance;

                            if !times_match {
                                error_str.push_str(
                                    &format!("Total time of bar {} does not match signature. ", n + 1)
                                );
                                return Some(error_str);
                            }

                            let items: &Vec<TabItem> = current_bar.get_ref();
                            let string_numbers_ok = items.iter().all(|item| {
                                let content: &NotesOrRest = item.get_ref();
                                let all_is_ok = match content {
                                    NotesOrRest::Rest => true,
                                    NotesOrRest::Notes { notes } =>
                                        notes.iter().all(|note| {
                                            let string_num: u8 = *note.get_ref();
                                            string_num > 0 && string_num <= num_of_strings
                                        })
                                };
                                all_is_ok
                            });

                            if !string_numbers_ok {
                                error_str.push_str(
                                    &format!("One or more of the string numbers are not valid in bar {}. ", n + 1)
                                );
                                return Some(error_str);
                            }
                                None
                        } else { temp }
                    });
                match error_msg {
                    Some(msg) => Err(msg),
                    _ => Ok(tab)
                }
            }
        }
    }
}

trait Time<TimeUnit> {
    fn time(&self) -> TimeUnit;
}

impl Time<f32> for Bar {
    fn time(&self) -> f32 {
        let items: &Vec<TabItem> = self.get_ref();
        items.iter().map(|item| {
            let dotted: &Dotted = item.get_ref();
            let dotted_factor = if dotted.0 { 1.5 } else { 1.0 };

            let tuplet: u8 = *item.get_ref();
            let tuplet_factor = 2f32 / tuplet as f32;

            let len: Length = *item.get_ref();
            let len_u8: u8 = len.into();
            tuplet_factor * dotted_factor * 1f32 / len_u8 as f32
        }).sum()
    }
}

impl Time<f32> for TimeSignature {
    fn time(&self) -> f32 {
        let upper: u8 = *self.get_ref();
        let lower: Length = *self.get_ref();
        let lower_u8: u8 = lower.into();
        upper as f32 / lower_u8 as f32
    }
}