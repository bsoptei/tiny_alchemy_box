use crate::*;
use crate::elements::*;
use pest::{
    Parser,
    error::InputLocation::{Pos, Span},
    iterators::{Pair, Pairs},
};
use pest_derive::*;
use photonix::*;

#[derive(Parser)]
#[grammar = "tab_grammar.pest"]
pub struct TabParser;

impl StrToTabParser for TabParser {
    fn parse_tab(&self, input: &str) -> TabParsingResult {
        match Self::parse(Rule::tab, input) {
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
        let default =
            TabItem::new(NotesOrRest::Notes { notes: vec![] }, Length::Quarter, false, 0, false, None);

        Ok(
            rules.fold(default, |temp, current| {
                match current.as_rule() {
                    Rule::note => {
                        let default = Note::new(0, 0);
                        let new_note =
                            current.into_inner().enumerate().fold(default, |temp, (i, num)| {
                                let num_str = num.as_str();
                                if i == 0 {
                                    temp.set(str2num::<u8>(num_str).unwrap_or(0))
                                } else if i == 1 {
                                    if num_str == "X" { temp.set(-1i8) } else { temp.set(str2num::<i8>(num_str).unwrap_or(0)) }
                                } else { temp }
                            });
                        temp.modify_second(|notes: Vec<Note>| notes.update(new_note))
                    }
                    Rule::length => {
                        temp.set(Length::from(current.as_str()))
                    }
                    Rule::dot => temp.set(Dotted(true)),
                    Rule::tuplet => {
                        temp.set(
                            current
                                .into_inner()
                                .next()
                                .and_then(|rule| Self::extract_num::<u8>(&rule).ok()).unwrap_or(2)
                        )
                    }
                    Rule::link => temp.set(Linked(true)),
                    Rule::notes_modifier => temp.set(Some(NotesModifier::from(current.as_str()))),
                    _ => temp
                }
            })
        )
    }

    fn extract_rest(rules: Pairs<Rule>) -> Result<TabItem, String> {
        let default = TabItem::new(NotesOrRest::Rest, Length::Quarter, false, 2, false, None);
        Ok(
            rules.fold(default, |temp, rest_elem| {
                match rest_elem.as_rule() {
                    Rule::length => temp.set(Length::from(rest_elem.as_str())),
                    Rule::dot => temp.set(Dotted(true)),
                    Rule::tuplet => {
                        let q: u8 = rest_elem
                            .into_inner().next()
                            .map(|x: Pair<Rule>| Self::extract_num(&x).unwrap_or_else(|_| 2))
                            .unwrap_or_else(|| 2);
                        temp.set(q)
                    }
                    Rule::link => temp.set(Linked(true)),
                    _ => temp
                }
            })
        )
    }

    #[allow(dead_code)]
    fn extract_time_signature(rules: Pairs<Rule>) -> Result<TimeSignature, String> {
        Ok(
            rules.enumerate().fold(TimeSignature::default(), |temp, (i, num_info)| {
                let num_info_str = num_info.as_str();
                if i == 0 {
                    let new_upper: u8 = str2num(num_info_str).unwrap_or(*temp.get_ref());
                    temp.set(new_upper)
                } else if i == 1 { (temp.set(Length::from(num_info_str))) } else { temp }
            })
        )
    }

    fn extract_bar(rules: Pairs<Rule>, time_signature: TimeSignature) -> Result<Bar, String> {
        Ok(
            rules.fold(Bar::default().set(time_signature), |temp, current| {
                match current.as_rule() {
                    Rule::bar_start => temp.set(BarStart::from(current.as_str())),
                    Rule::bar_end => {
                        let s = current.as_str();
                        if s.starts_with('|') {
                            temp.set(BarEnd::Regular)
                        } else if s.starts_with(":|") {
                            temp.set(BarEnd::Repeat(str2num(&s[2..]).unwrap_or(2)))
                        } else { temp }
                    }
                    Rule::notes => {
                        temp.modify(
                            |items: Vec<TabItem>|
                                if let Some(new_item) =
                                Self::extract_notes(current.into_inner()).ok() {
                                    items.update(new_item)
                                } else { items }
                        )
                    }
                    Rule::rest => {
                        temp.modify(
                            |items: Vec<TabItem>|
                                if let Some(new_item) =
                                Self::extract_rest(current.into_inner()).ok() {
                                    items.update(new_item)
                                } else { items }
                        )
                    }
                    _ => temp
                }
            })
        )
    }

    fn extract_tab(rules: Pairs<Rule>) -> Result<Tab, String> {
        let default = (Tab::new(TabMetaData::new("", 0, "", 0), vec![]), TimeSignature::default());
        Ok(
            rules.fold(default, |(temp_tab, temp_sig), current| {
                match current.as_rule() {
                    Rule::title_declaration => {
                        (temp_tab.set_second(
                            current.into_inner().next().map(|rule| rule.as_str()).unwrap_or("").to_owned()
                        ), temp_sig)
                    }
                    Rule::number_of_strings_declaration => {
                        (temp_tab.set_second(
                            current.into_inner().next().and_then(|n| str2num::<u8>(n.as_str()).ok()).unwrap_or(0)
                        ), temp_sig)
                    }
                    Rule::tuning_declaration => {
                        (temp_tab.set_third(
                            current.into_inner().next().map(|rule| rule.as_str()).unwrap_or("").to_owned()
                        ), temp_sig)
                    }
                    Rule::tempo_declaration => {
                        (temp_tab.set_second(
                            current.into_inner().next().and_then(|n| str2num::<u16>(n.as_str()).ok()).unwrap_or(0)
                        ), temp_sig)
                    }
                    Rule::time_signature => {
                        (
                            temp_tab,
                            Self::extract_time_signature(current.into_inner())
                                .unwrap_or(TimeSignature::default())
                        )
                    }
                    Rule::bar => {
                        (temp_tab.modify(
                            |bars: Vec<Bar>|
                                if let Some(new_bar) =
                                Self::extract_bar(current.into_inner(), temp_sig).ok() {
                                    bars.update(new_bar)
                                } else { bars }
                        ), temp_sig)
                    }
                    _ => (temp_tab, temp_sig)
                }
            })
        ).map(|(tab, _)| tab)
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
        assert!(TabParser::parse(Rule::rest, "R4L").is_ok());
        assert!(TabParser::parse(Rule::rest, "rest4L").is_ok());
        assert!(TabParser::parse(Rule::rest, "R4L->").is_ok());
        assert!(TabParser::parse(Rule::rest, "R8L.").is_ok());
        assert!(TabParser::parse(Rule::rest, "R8L3let").is_ok());
        assert!(TabParser::parse(Rule::rest, "R8L.3let").is_ok());
        assert!(TabParser::parse(Rule::rest, "R32L.->").is_ok());
        assert!(TabParser::parse(Rule::rest, "R16L->").is_ok());

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
        assert!(TabParser::parse(Rule::notes, "N S4F2S5F2S6F0,1L").is_ok());
        assert!(TabParser::parse(Rule::notes, "N S4F2,1L").is_ok());
        assert!(TabParser::parse(Rule::notes, "notes S4F2,1L").is_ok());
        assert!(TabParser::parse(Rule::notes, "N S4F2 S5F2 S6F0, 1L->").is_ok());
        assert!(TabParser::parse(Rule::notes, "N S4F2 S5F2 S6F0, 1L.").is_ok());
        assert!(TabParser::parse(Rule::notes, "N S4F2 S5F2 S6F0, 1L.->").is_ok());
        assert!(TabParser::parse(Rule::notes, "N S4F2 S5F2 S6F0, 1L, PM").is_ok());
        assert!(TabParser::parse(Rule::notes, "N S4F2 S5F2 S6F0, 1L, PM->").is_ok());
        assert!(TabParser::parse(Rule::notes, "N S4F2 S5F2 S6F0, 1L., PM").is_ok());
        assert!(TabParser::parse(Rule::notes, "N S4F2 S5F2 S6F0, 1L., PM->").is_ok());
        assert!(TabParser::parse(Rule::notes, "N S4F2, 1L, B2").is_ok());
        assert!(TabParser::parse(Rule::notes, "N S4F2, 1L, ~~").is_ok());
        assert!(TabParser::parse(Rule::notes, "N S4F2, 1L, SL").is_ok());
        assert!(TabParser::parse(Rule::notes, "N S4F5, 1L, HM").is_ok());
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
        assert!(TabParser::parse(Rule::bar_start, "|").is_ok());
        assert!(TabParser::parse(Rule::bar_start, "|:").is_ok());
    }

    #[test]
    fn parser_time_signature() {
        assert!(TabParser::parse(Rule::time_signature, "T4/4L").is_ok());
        assert!(TabParser::parse(Rule::time_signature, "time4/4L").is_ok());

        assert!(TabParser::parse(Rule::time_signature, "T3/4L").is_ok());
        assert!(TabParser::parse(Rule::time_signature, "T7/8L").is_ok());
        assert!(TabParser::parse(Rule::time_signature, "T11/8L").is_ok());
        assert!(TabParser::parse(Rule::time_signature, "T9/16L").is_ok());

        assert!(TabParser::parse(Rule::time_signature, "T4/3L").is_err());
        assert!(TabParser::parse(Rule::time_signature, "T/4L").is_err());
        assert!(TabParser::parse(Rule::time_signature, "3/4L").is_err());
        assert!(TabParser::parse(Rule::time_signature, "T()").is_err());
        assert!(TabParser::parse(Rule::time_signature, "T2").is_err());
    }

    #[test]
    fn parser_bar() {
        assert!(TabParser::parse(Rule::bar, "|N S4F2,1L.,PM|").is_ok());
        assert!(TabParser::parse(Rule::bar, "|N S4F2,1L.,PM->|").is_ok());
        assert!(TabParser::parse(Rule::bar, "|R16L->|").is_ok());
        assert!(TabParser::parse(Rule::bar, "|R16L|").is_ok());
        assert!(TabParser::parse(Rule::bar, "|R2L R2L|").is_ok());
        assert!(TabParser::parse(Rule::bar, "|NS4F2,2L,PM R16L|").is_ok());
        assert!(TabParser::parse(Rule::bar, "|NS4F2,16L,PM-> NS4F2,16L,PM|").is_ok());
        assert!(TabParser::parse(Rule::bar, "|NS4F2,16L,PM NS4F2,16L,PM|").is_ok());

        assert!(TabParser::parse(Rule::bar, "|:N S4F2,1L.,PM|").is_ok());
        assert!(TabParser::parse(Rule::bar, "|:N S4F2,1L.,PM->|").is_ok());
        assert!(TabParser::parse(Rule::bar, "|:R16L->|").is_ok());
        assert!(TabParser::parse(Rule::bar, "|:R16L|").is_ok());
        assert!(TabParser::parse(Rule::bar, "|:R2LR2L|").is_ok());
        assert!(TabParser::parse(Rule::bar, "|:N S4F2,2L,PM R16L|").is_ok());
        assert!(TabParser::parse(Rule::bar, "|:N S4F2,16L,PM-> N S4F2,16L,PM|").is_ok());
        assert!(TabParser::parse(Rule::bar, "|:N S4F2,16L,PM N S4F2,16L,PM|").is_ok());
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
                "title I'm the Ocean;\nnumber of strings 8;\ntuning DGDGCFAd;\ntempo 120 bpm;\nT7/8L|N S4F2,16L N S4F2,16L||N S4F1,16L|",
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