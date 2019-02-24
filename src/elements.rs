use crate::elements::Length::*;
use photonix::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Length {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
}

impl Length {
    pub fn num_of_tails(self) -> usize {
        match self {
            Whole | Half | Quarter => 0,
            Eighth => 1,
            Sixteenth => 2,
            ThirtySecond => 3,
            SixtyFourth => 4
        }
    }
}

impl From<&str> for Length {
    fn from(s: &str) -> Self {
        match s {
            "1L" => Whole,
            "2L" => Half,
            "4L" => Quarter,
            "8L" => Eighth,
            "16L" => Sixteenth,
            "32L" => ThirtySecond,
            "64L" => SixtyFourth,
            _ => panic!(format!("Could not create Length from {}", s))
        }
    }
}

impl From<u8> for Length {
    fn from(num: u8) -> Self {
        match num {
            1 => Whole,
            2 => Half,
            4 => Quarter,
            8 => Eighth,
            16 => Sixteenth,
            32 => ThirtySecond,
            64 => SixtyFourth,
            _ => panic!(format!("Cannot create length from {}", num))
        }
    }
}

impl Into<u8> for Length {
    fn into(self) -> u8 {
        use crate::Length::*;
        match self {
            Whole => 1,
            Half => 2,
            Quarter => 4,
            Eighth => 8,
            Sixteenth => 16,
            ThirtySecond => 32,
            SixtyFourth => 64
        }
    }
}

#[derive(PartialEq, Debug, GetRef, Set)]
pub struct Note {
    string_num: u8,
    fret_num: i8,
}

impl Note {
    pub fn new(string_num: u8, fret_num: i8) -> Self {
        Self { string_num, fret_num }
    }
}

#[derive(Debug, Set, Modify)]
pub enum NotesOrRest {
    Notes { notes: Vec<Note> },
    Rest,
}

#[derive(Debug)]
pub enum NotesModifier {
    SL,
    PM,
    HM,
    Vibrato,
    B1,
    B2,
    B3,
    B4,
    B5,
}

impl From<&str> for NotesModifier {
    fn from(s: &str) -> Self {
        use crate::NotesModifier::*;
        match s {
            "SL" => SL,
            "PM" => PM,
            "HM" => HM,
            "~~" => Vibrato,
            "B1" => B1,
            "B2" => B2,
            "B3" => B3,
            "B4" => B4,
            "B5" => B5,
            _ => panic!(format!("Could not create NotesModifier from {}", s))
        }
    }
}

#[derive(Debug, GetRef)]
pub struct Dotted(pub bool);

#[derive(Debug, GetRef)]
pub struct Linked(pub bool);

#[derive(Debug, GetRef, Set, Modify)]
pub struct TabItem {
    content: NotesOrRest,
    length: Length,
    dotted: Dotted,
    tuplet: u8,
    linked: Linked,
    modifier: Option<NotesModifier>,
}

zoom![TabItem => NotesOrRest => Vec<Note>];

impl TabItem {
    pub fn new(content: NotesOrRest,
               length: Length,
               dotted: bool,
               tuplet: u8,
               linked: bool,
               modifier: Option<NotesModifier>,
    ) -> Self {
        Self { content, length, dotted: Dotted(dotted), tuplet, linked: Linked(linked), modifier }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, GetRef, Set)]
pub struct TimeSignature {
    upper: u8,
    lower: Length,
}

impl TimeSignature {
    pub fn new(upper: u8, lower: u8) -> Self {
        Self { upper, lower: lower.into() }
    }
}

impl Default for TimeSignature {
    fn default() -> Self {
        Self::new(4, 4)
    }
}

#[derive(Debug)]
pub enum BarStart {
    Regular,
    Repeat,
}

impl From<&str> for BarStart {
    fn from(s: &str) -> Self {
        match s {
            "|" => BarStart::Regular,
            "|:" => BarStart::Repeat,
            _ => panic!(format!("Cannot create BarStart from {}", s))
        }
    }
}

#[derive(Debug)]
pub enum BarEnd {
    Regular,
    Repeat(u8),
}

#[derive(Debug, GetRef, Set, Modify)]
pub struct Bar {
    time_signature: TimeSignature,
    items: Vec<TabItem>,
    start: BarStart,
    end: BarEnd,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            time_signature: TimeSignature::default(),
            items: vec![],
            start: BarStart::Regular,
            end: BarEnd::Regular,
        }
    }
}

#[allow(dead_code)]
impl Bar {
    pub fn new(time_signature: TimeSignature, items: Vec<TabItem>, start: BarStart, end: BarEnd) -> Self {
        Self { time_signature, items, start, end }
    }

    pub fn length(&self) -> usize {
        self.items.len()
    }
}

#[derive(Clone, Debug, Get, GetRef, Set, Modify)]
pub struct TabMetaData {
    pub title: String,
    pub number_of_strings: u8,
    pub tuning: Tuning,
    pub tempo: u16,
}

#[derive(Clone, Debug, Set, Modify)]
pub struct Tuning(pub String);

impl TabMetaData {
    pub fn new(title: &str, number_of_strings: u8, tuning: &str, tempo: u16) -> Self {
        Self { title: title.to_string(), number_of_strings, tuning: Tuning(tuning.to_string()), tempo }
    }
}

#[derive(Debug, Get, GetRef, Set, Modify)]
pub struct Tab {
    metadata: TabMetaData,
    bars: Vec<Bar>,
}

zoom_all![Tab => TabMetaData => u8];
zoom![Tab => TabMetaData => u16];
zoom![Tab => TabMetaData => String];
zoom![Tab => TabMetaData => Tuning => String];

impl Tab {
    pub fn new(metadata: TabMetaData, bars: Vec<Bar>) -> Self {
        Self { metadata, bars }
    }

    pub fn into_lines(self, max_items_per_line: usize) -> Vec<TabLine> {
        let bars = self.bars;
        let lengths = bars.iter().map(|tab_bar| tab_bar.length()).collect::<Vec<usize>>();
        let bars_len = bars.len();
        let mut temp_bars: Vec<Vec<Bar>> = Vec::new();

        let mut temp: Vec<Bar> = Vec::new();
        let mut counter = 0;

        for (i, tab_bar) in bars.into_iter().enumerate() {
            let next_bar_len = *(lengths.get(i + 1).unwrap_or_else(|| &0usize));
            counter += tab_bar.length();
            temp.push(tab_bar);

            if counter >= max_items_per_line ||
                counter + next_bar_len > max_items_per_line ||
                i + 1 == bars_len {
                temp_bars.push(temp);
                temp = Vec::new();
                counter = 0;
            }
        }
        temp_bars.into_iter().map(TabLine::new).collect()
    }
}

#[derive(Debug, GetRef)]
pub struct TabLine {
    bars: Vec<Bar>
}

impl TabLine {
    pub fn new(bars: Vec<Bar>) -> Self {
        Self { bars }
    }
}
