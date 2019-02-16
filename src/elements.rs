use crate::{
    elements::Length::*,
    FromToken,
};

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

impl FromToken for Length {
    fn from_token(token: &str) -> Option<Self> {
        match token {
            "1L" => Some(Whole),
            "2L" => Some(Half),
            "4L" => Some(Quarter),
            "8L" => Some(Eighth),
            "16L" => Some(Sixteenth),
            "32L" => Some(ThirtySecond),
            "64L" => Some(SixtyFourth),
            _ => None,
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

#[derive(PartialEq, Debug)]
pub struct Note {
    string_num: u8,
    fret_num: i8,
}

impl Note {
    pub fn new(string_num: u8, fret_num: i8) -> Self {
        Self { string_num, fret_num }
    }

    pub fn get_string_num(&self) -> u8 {
        self.string_num
    }

    pub fn get_fret_num(&self) -> i8 {
        self.fret_num
    }
}

#[derive(Debug)]
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

impl FromToken for NotesModifier {
    fn from_token(token: &str) -> Option<Self> {
        use crate::NotesModifier::*;
        match token {
            "SL" => Some(SL),
            "PM" => Some(PM),
            "HM" => Some(HM),
            "~~" => Some(Vibrato),
            "B1" => Some(B1),
            "B2" => Some(B2),
            "B3" => Some(B3),
            "B4" => Some(B4),
            "B5" => Some(B5),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct TabItem {
    content: NotesOrRest,
    length: Length,
    dotted: bool,
    tuplet: u8,
    linked: bool,
    modifier: Option<NotesModifier>,
}

#[allow(dead_code)]
impl TabItem {
    pub fn new(content: NotesOrRest,
               length: Length,
               dotted: bool,
               tuplet: u8,
               linked: bool,
               modifier: Option<NotesModifier>,
    ) -> Self {
        Self { content, length, dotted, tuplet, linked, modifier }
    }

    pub fn get_content(&self) -> &NotesOrRest {
        &self.content
    }

    pub fn get_length(&self) -> &Length {
        &self.length
    }

    pub fn get_modifier(&self) -> &Option<NotesModifier> {
        &self.modifier
    }

    pub fn get_tuplet(&self) -> u8 {
        self.tuplet
    }

    pub fn is_dotted(&self) -> bool {
        self.dotted
    }

    pub fn is_linked(&self) -> bool {
        self.linked
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TimeSignature {
    upper: u8,
    lower: Length,
}

impl TimeSignature {
    pub fn new(upper: u8, lower: u8) -> Self {
        Self { upper, lower: lower.into() }
    }

    pub fn new_lower_length(upper: u8, lower: Length) -> Self {
        Self { upper, lower }
    }

    pub fn common_time() -> Self {
        Self::new(4, 4)
    }

    pub fn get_upper(self) -> u8 {
        self.upper
    }

    pub fn get_lower(self) -> u8 {
        self.lower.into()
    }
}

#[derive(Debug)]
pub enum BarStart {
    Regular,
    Repeat,
}

impl FromToken for BarStart {
    fn from_token(token: &str) -> Option<Self> {
        match token {
            "|" => Some(BarStart::Regular),
            "|:" => Some(BarStart::Repeat),
            _ => None
        }
    }
}

#[derive(Debug)]
pub enum BarEnd {
    Regular,
    Repeat(u8),
}

#[derive(Debug)]
pub struct Bar {
    time_signature: TimeSignature,
    items: Vec<TabItem>,
    start: BarStart,
    end: BarEnd,
}

#[allow(dead_code)]
impl Bar {
    pub fn new(time_signature: TimeSignature, items: Vec<TabItem>, start: BarStart, end: BarEnd) -> Self {
        Self { time_signature, items, start, end }
    }

    pub fn length(&self) -> usize {
        self.items.len()
    }

    pub fn get_time_signature(&self) -> &TimeSignature {
        &self.time_signature
    }

    pub fn get_items(&self) -> &Vec<TabItem> {
        &self.items
    }

    pub fn get_start(&self) -> &BarStart {
        &self.start
    }

    pub fn get_end(&self) -> &BarEnd {
        &self.end
    }
}

#[derive(Clone, Debug)]
pub struct TabMetaData {
    pub title: String,
    pub number_of_strings: u8,
    pub tuning: String,
    pub tempo: u16,
}

impl TabMetaData {
    pub fn new(title: String, number_of_strings: u8, tuning: String, tempo: u16) -> Self {
        Self { title, number_of_strings, tuning, tempo }
    }
}

#[derive(Debug)]
pub struct Tab {
    metadata: TabMetaData,
    bars: Vec<Bar>,
}

impl Tab {
    pub fn new(metadata: TabMetaData, bars: Vec<Bar>) -> Self {
        Self { metadata, bars }
    }

    #[allow(dead_code)]
    pub fn get_bars(&self) -> &Vec<Bar> {
        &self.bars
    }

    pub fn get_metadata(&self) -> &TabMetaData {
        &self.metadata
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

#[derive(Debug)]
pub struct TabLine {
    bars: Vec<Bar>
}

impl TabLine {
    pub fn new(bars: Vec<Bar>) -> Self {
        Self { bars }
    }

    pub fn get_bars(&self) -> &Vec<Bar> {
        &self.bars
    }
}
