#[cfg(test)]
mod tests {
    use crate::elements::*;
    use crate::Length::*;

    #[test]
    fn tab_into_lines() {
        let mut bars: Vec<Bar> = Vec::new();
        for _ in 1..=16 {
            let mut items: Vec<TabItem> = Vec::new();
            for _ in 1..=16 {
                items.push(TabItem::new(NotesOrRest::Rest, Sixteenth, false, 2, false, None))
            }
            bars.push(
                Bar::new(TimeSignature::default(), items, BarStart::Regular, BarEnd::Regular)
            );
        }
        let my_tab = Tab::new(
            TabMetaData::new(
                "foo",
                4,
                "EADG",
                120
            ),
            bars
        );
        let my_lines = my_tab.into_lines(64);

        assert_eq!(
            4,
            my_lines.len()
        )

    }
}