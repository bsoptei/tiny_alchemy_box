use crate::elements::*;

use crate::*;
#[allow(unused_imports)]
use pest::Parser;
use pest::error::InputLocation::{Pos, Span};
use pest::iterators::{Pair, Pairs};
use pest_derive::*;


#[derive(Parser)]
#[grammar = "tab_grammar.pest"]
pub struct TabParser;

impl StrToTabParser for TabParser {
    fn parse_tab(&self, input: &str) -> TabParsingResult {
        match TabParser::parse(Rule::tab, input) {
            Ok(tab_parsed) => {
                match tab_parsed.map(|tab| Self::extract_tab(tab.into_inner())).next() {
                    Some(res) => res,
                    _ => Err(String::from("Something terrible happened!!!!"))
                }
            }
            Err(e) => {
                Err(
                    match e.location {
                        Pos(pos) => format!("Syntax error at position {}", pos),
                        Span((start, end)) => format!("Syntax error at position {} to {}", start, end)
                    }
                )
            }
        }
    }
}

impl TabParser {
    fn extract_num<Num: FromStr>(rule: &Pair<Rule>) -> Result<Num, String> {
        match rule.as_rule() {
            Rule::num => str2num(rule.as_str()),
            _ => Err(format!("Invalid input {}", rule.as_str()))
        }
    }

    fn extract_notes(rules: Pairs<Rule>) -> Result<TabItem, String> {
        let mut len_temp: Option<Length> = None;
        let mut temp_notes: Vec<Note> = Vec::new();
        let mut is_dotted: bool = false;
        let mut tuplet: u8 = 2;
        let mut is_linked: bool = false;
        let mut modifier: Option<NotesModifier> = None;
        for rule in rules {
            match rule.as_rule() {
                Rule::note => {
                    let mut string_temp: u8 = 0;
                    let mut fret_temp: i8 = 0;
                    for (i, num) in rule.into_inner().enumerate() {
                        let num_str = num.as_str();
                        if i == 0 {
                            string_temp = str2num(num_str)
                                .and_then(
                                    |n| if n != 0 { Ok(n) } else { Err(String::from("String number cannot be 0")) }
                                )?;
                        } else if i == 1 {
                            if num_str == "X" {
                                fret_temp = -1;
                            } else {
                                fret_temp = str2num(num_str)?;
                            }
                        }
                    }
                    temp_notes.push(Note::new(string_temp, fret_temp));
                }
                Rule::length => {
                    len_temp = Length::from_token(rule.as_str());
                }
                Rule::dot => is_dotted = true,
                Rule::tuplet => {
                    for notes_elem_concrete in rule.into_inner() {
                        tuplet = Self::extract_num(&notes_elem_concrete).unwrap_or_else(|_| 2);
                    }
                }
                Rule::link => is_linked = true,
                Rule::notes_modifier => {
                    modifier = NotesModifier::from_token(rule.as_str());
                }
                _ => {}
            }
        }
        match len_temp {
            Some(length) => {
                Ok(
                    TabItem::new(
                        NotesOrRest::Notes { notes: temp_notes },
                        length,
                        is_dotted,
                        tuplet,
                        is_linked,
                        modifier,
                    )
                )
            }
            _ => Err(String::from("Could not read length of notes."))
        }
    }

    fn extract_rest(rules: Pairs<Rule>) -> Result<TabItem, String> {
        let mut len_temp: Option<Length> = None;
        let mut is_dotted: bool = false;
        let mut tuplet: u8 = 2;
        let mut is_linked: bool = false;

        for rest_elem in rules {
            match rest_elem.as_rule() {
                Rule::length => {
                    len_temp = Length::from_token(rest_elem.as_str());
                }
                Rule::dot => is_dotted = true,
                Rule::tuplet => {
                    for rest_elem_concrete in rest_elem.into_inner() {
                        tuplet = Self::extract_num(&rest_elem_concrete).unwrap_or_else(|_| 2);
                    }
                }
                Rule::link => is_linked = true,
                _ => {}
            }
        }
        match len_temp {
            Some(length) => {
                Ok(
                    TabItem::new(
                        NotesOrRest::Rest,
                        length,
                        is_dotted,
                        tuplet,
                        is_linked,
                        None,
                    )
                )
            }
            _ => Err(String::from("Could not read length of rest."))
        }
    }

    fn extract_time_signature(rules: Pairs<Rule>) -> Result<TimeSignature, String> {
        let mut upper_temp = 0;
        let mut lower_temp: Option<Length> = None;

        for (i, num_info) in rules.enumerate() {
            let num_info_str = num_info.as_str();
            if i == 0 {
                upper_temp = str2num(num_info_str)?;
            } else if i == 1 {
                lower_temp = Length::from_token(num_info_str);
            }
        }
        match lower_temp {
            Some(lower_temp) => {
                Ok(TimeSignature::new_lower_length(upper_temp, lower_temp))
            }
            _ => Err(String::from("Wrong time signature."))
        }
    }

    fn extract_bar(rules: Pairs<Rule>, time_signature: TimeSignature) -> Result<Bar, String> {
        let mut tab_items: Vec<TabItem> = Vec::new();
        let mut bar_start: BarStart = BarStart::Regular;
        let mut bar_end: BarEnd = BarEnd::Regular;

        for bar_elem_concrete in rules {
            match bar_elem_concrete.as_rule() {
                Rule::bar_start => {
                    if let Some(start) = BarStart::from_token(bar_elem_concrete.as_str()) {
                        bar_start = start
                    }
                }
                Rule::bar_end => {
                    let s = bar_elem_concrete.as_str();
                    if s.starts_with('|') {
                        bar_end = BarEnd::Regular;
                    } else if s.starts_with(":|") {
                        bar_end = BarEnd::Repeat(str2num(&s[2..])?);
                    }
                }
                Rule::notes => {
                    tab_items.push(Self::extract_notes(bar_elem_concrete.into_inner())?);
                }
                Rule::rest => {
                    tab_items.push(Self::extract_rest(bar_elem_concrete.into_inner())?);
                }
                _ => {}
            }
        }
        Ok(Bar::new(time_signature, tab_items, bar_start, bar_end))
    }

    fn extract_tab(rules: Pairs<Rule>) -> Result<Tab, String> {
        let mut time_signature = TimeSignature::common_time();
        let mut bars_temp: Vec<Bar> = Vec::new();
        let mut song_title = String::from("");
        let mut num_of_strings = 0;
        let mut song_tuning = String::from("");
        let mut tempo = 0;

        for tab_elem in rules {
            match tab_elem.as_rule() {
                Rule::title_declaration => {
                    for title in tab_elem.into_inner() {
                        song_title.push_str(title.as_str());
                    }
                }
                Rule::number_of_strings_declaration => {
                    for num in tab_elem.into_inner() {
                        num_of_strings = str2num(num.as_str()).and_then(
                            |n| if n > 0 { Ok(n) } else { Err(format!("Invalid number of strings {}", n)) }
                        )?;
                    }
                }
                Rule::tuning_declaration => {
                    for tuning in tab_elem.into_inner() {
                        song_tuning.push_str(tuning.as_str());
                    }
                }
                Rule::tempo_declaration => {
                    for num in tab_elem.into_inner() {
                        tempo = str2num(num.as_str())?;
                    }
                }
                Rule::time_signature => {
                    time_signature = Self::extract_time_signature(tab_elem.into_inner())?
                }
                Rule::bar => {
                    bars_temp.push(Self::extract_bar(tab_elem.into_inner(), time_signature)?);
                }
                _ => {}
            }
        }
        Ok(
            Tab::new(
                TabMetaData::new(song_title, num_of_strings, song_tuning, tempo),
                bars_temp)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_length() {
        for len in vec!["1L", "2L", "4L", "8L", "16L", "32L", "64L"].iter() {
            assert!(TabParser::parse(Rule::length, len).is_ok());
        }

        assert!(TabParser::parse(Rule::length, "3").is_err());
        assert!(TabParser::parse(Rule::length, "1").is_err());
    }

    #[test]
    fn parser_rest() {
        assert!(TabParser::parse(Rule::rest, "R(4L)").is_ok());
        assert!(TabParser::parse(Rule::rest, "rest(4L)").is_ok());
        assert!(TabParser::parse(Rule::rest, "R(4L)->").is_ok());
        assert!(TabParser::parse(Rule::rest, "R(8L.)").is_ok());
        assert!(TabParser::parse(Rule::rest, "R(8L3let)").is_ok());
        assert!(TabParser::parse(Rule::rest, "R(8L.3let)").is_ok());
        assert!(TabParser::parse(Rule::rest, "R(32L.)->").is_ok());
        assert!(TabParser::parse(Rule::rest, "R(16L)->").is_ok());

        assert!(TabParser::parse(Rule::rest, "R()").is_err());
    }

    #[test]
    fn parser_note() {
        assert!(TabParser::parse(Rule::note, "S1F0").is_ok());
        assert!(TabParser::parse(Rule::note, "string1F0").is_ok());
        assert!(TabParser::parse(Rule::note, "S1fret0").is_ok());
        assert!(TabParser::parse(Rule::note, "string1fret0").is_ok());
        assert!(TabParser::parse(Rule::note, "S1F24").is_ok());
        assert!(TabParser::parse(Rule::note, "S1X").is_ok());

        assert!(TabParser::parse(Rule::note, "S1FX").is_err());
    }

    #[test]
    fn parser_notes() {
        assert!(TabParser::parse(Rule::notes, "N(S4F2S5F2S6F0,1L)").is_ok());
        assert!(TabParser::parse(Rule::notes, "N(S4F2,1L)").is_ok());
        assert!(TabParser::parse(Rule::notes, "notes(S4F2,1L)").is_ok());
        assert!(TabParser::parse(Rule::notes, "N(S4F2S5F2S6F0,1L)->").is_ok());
        assert!(TabParser::parse(Rule::notes, "N(S4F2S5F2S6F0,1L.)").is_ok());
        assert!(TabParser::parse(Rule::notes, "N(S4F2S5F2S6F0,1L.)->").is_ok());
        assert!(TabParser::parse(Rule::notes, "N(S4F2S5F2S6F0,1L,PM)").is_ok());
        assert!(TabParser::parse(Rule::notes, "N(S4F2S5F2S6F0,1L,PM)->").is_ok());
        assert!(TabParser::parse(Rule::notes, "N(S4F2S5F2S6F0,1L.,PM)").is_ok());
        assert!(TabParser::parse(Rule::notes, "N(S4F2S5F2S6F0,1L.,PM)->").is_ok());
    }

    #[test]
    fn parser_bar_end() {
        assert!(TabParser::parse(Rule::bar_end, "|").is_ok());
        assert!(TabParser::parse(Rule::bar_end, ":|").is_ok());
        assert!(TabParser::parse(Rule::bar_end, ":|3").is_ok());
        assert!(TabParser::parse(Rule::bar_end, ":|12").is_ok());
    }

    #[test]
    fn parser_bar_start() {
        assert!(TabParser::parse(Rule::bar_start, "B|").is_ok());
        assert!(TabParser::parse(Rule::bar_start, "bar|").is_ok());
        assert!(TabParser::parse(Rule::bar_start, "B|:").is_ok());

        assert!(TabParser::parse(Rule::bar_start, "|").is_err());
    }

    #[test]
    fn parser_time_signature() {
        assert!(TabParser::parse(Rule::time_signature, "T(4/4L)").is_ok());
        assert!(TabParser::parse(Rule::time_signature, "time(4/4L)").is_ok());

        assert!(TabParser::parse(Rule::time_signature, "T(3/4L)").is_ok());
        assert!(TabParser::parse(Rule::time_signature, "T(7/8L)").is_ok());
        assert!(TabParser::parse(Rule::time_signature, "T(11/8L)").is_ok());
        assert!(TabParser::parse(Rule::time_signature, "T(9/16L)").is_ok());

        assert!(TabParser::parse(Rule::time_signature, "T(4/3L)").is_err());
        assert!(TabParser::parse(Rule::time_signature, "T(/4L)").is_err());
        assert!(TabParser::parse(Rule::time_signature, "(3/4L)").is_err());
        assert!(TabParser::parse(Rule::time_signature, "T()").is_err());
        assert!(TabParser::parse(Rule::time_signature, "T(2)").is_err());
    }

    #[test]
    fn parser_bar() {
        assert!(TabParser::parse(Rule::bar, "B|N(S4F2,1L.,PM)|").is_ok());
        assert!(TabParser::parse(Rule::bar, "B|N(S4F2,1L.,PM)->|").is_ok());
        assert!(TabParser::parse(Rule::bar, "B|R(16L)->|").is_ok());
        assert!(TabParser::parse(Rule::bar, "B|R(16L)|").is_ok());
        assert!(TabParser::parse(Rule::bar, "B|R(2L)R(2L)|").is_ok());
        assert!(TabParser::parse(Rule::bar, "B|N(S4F2,2L,PM)R(16L)|").is_ok());
        assert!(TabParser::parse(Rule::bar, "B|N(S4F2,16L,PM)->N(S4F2,16L,PM)|").is_ok());
        assert!(TabParser::parse(Rule::bar, "B|N(S4F2,16L,PM)N(S4F2,16L,PM)|").is_ok());

        assert!(TabParser::parse(Rule::bar, "B|:N(S4F2,1L.,PM)|").is_ok());
        assert!(TabParser::parse(Rule::bar, "B|:N(S4F2,1L.,PM)->|").is_ok());
        assert!(TabParser::parse(Rule::bar, "B|:R(16L)->|").is_ok());
        assert!(TabParser::parse(Rule::bar, "B|:R(16L)|").is_ok());
        assert!(TabParser::parse(Rule::bar, "B|:R(2L)R(2L)|").is_ok());
        assert!(TabParser::parse(Rule::bar, "B|:N(S4F2,2L,PM)R(16L)|").is_ok());
        assert!(TabParser::parse(Rule::bar, "B|:N(S4F2,16L,PM)->N(S4F2,16L,PM)|").is_ok());
        assert!(TabParser::parse(Rule::bar, "B|:N(S4F2,16L,PM)N(S4F2,16L,PM)|").is_ok());
    }

    #[test]
    fn parser_title_declaration() {
        assert!(TabParser::parse(Rule::title_declaration, "title foo bar;").is_ok());
        assert!(TabParser::parse(Rule::title_declaration, "title The Thing That Should Not Be;").is_ok());
        assert!(TabParser::parse(Rule::title_declaration, "title I'm the Ocean;").is_ok());
    }

    #[test]
    fn parser_tuning_declaration() {
        assert!(TabParser::parse(Rule::tuning_declaration, "tuning EADG;").is_ok());
        assert!(TabParser::parse(Rule::tuning_declaration, "tuning CGCFAd;").is_ok());
    }

    #[test]
    fn parser_n_of_strings_declaration() {
        assert!(TabParser::parse(Rule::number_of_strings_declaration, "number of strings 8;").is_ok());
    }

    #[test]
    fn parser_tab() {
        assert!(
            TabParser::parse(
                Rule::tab,
                "title I'm the Ocean;\nnumber of strings 8;\ntuning DGDGCFAd;\ntempo 120;\nT(7/8L)B|N(S4F2,16L)N(S4F2,16L)|B|N(S4F1,16L)|",
            ).is_ok()
        );
    }

    #[test]
    fn parser_num() {
        assert!(TabParser::parse(Rule::num, "1").is_ok());
        assert!(TabParser::parse(Rule::num, "21").is_ok());

        assert!(TabParser::parse(Rule::num, "-21").is_err());
        assert!(TabParser::parse(Rule::num, "-2a1").is_err());
    }
}