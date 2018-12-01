use std::f64::consts::PI;
use web_sys::CanvasRenderingContext2d;
use {Coordinate2D, Radius, X, Y};
use elements::*;
use helpers::*;
use imp::sizes;

pub trait Drawable<DrawingContext> {
    fn draw(&self, context: &DrawingContext, position: Coordinate2D<f64>) -> ();
}

pub trait AutoDrawable<DrawingContext> {
    fn draw(&self, context: &DrawingContext) -> ();
}

pub struct BarDots;

pub struct Crotchet;

pub struct Dot;

pub struct DrawableBarEnd<'a> { bar_end: &'a BarEnd, n_of_strings: u8 }

impl<'a> DrawableBarEnd<'a> {
    pub fn new(bar_end: &'a BarEnd, n_of_strings: u8) -> Self {
        Self { bar_end, n_of_strings }
    }
}

pub struct DrawableBarStart<'a> { bar_start: &'a BarStart, n_of_strings: u8 }

impl<'a> DrawableBarStart<'a> {
    pub fn new(bar_start: &'a BarStart, n_of_strings: u8) -> Self {
        Self { bar_start, n_of_strings }
    }
}

pub struct DrawableNotes<'a> { notes: &'a Vec<Note>, length: Length, line_n: f64, n_of_strings: u8 }

impl<'a> DrawableNotes<'a> {
    pub fn new(notes: &'a Vec<Note>, length: Length, line_n: f64, n_of_strings: u8) -> Self {
        Self { notes, length, line_n, n_of_strings }
    }

    fn draw_note(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        let note_line_height = sizes::y_space() * 1.5;
        let note_radius = sizes::r_default() * 0.225;

        let circle_fun = match self.length {
            Length::Whole | Length::Half => stroke_circle,
            _ => fill_circle
        };
        circle_fun(context, position.up(note_line_height), note_radius);

        let top = position.right(note_radius.into()).up(note_line_height * 2.0);

        if self.length != Length::Whole {
            draw_line(
                context,
                sizes::line_width_default(),
                position.right(note_radius.into()).up(note_line_height),
                top,
            );
        }
        self.draw_tails(context, top);
    }

    fn draw_tails(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        for tail in 0..self.length.num_of_tails() {
            let current_y = position.y + Y(tail as f64 * 5.0);
            let shifted_pos = position.at_y(current_y);
            draw_line(
                context,
                sizes::line_width_default(),
                shifted_pos,
                shifted_pos.right(sizes::item_width() * 0.67),
            );
        }
    }
}

pub struct DrawableRest(pub Length);

pub struct Link;

pub struct StringLines {
    n_of_strings: u8,
    line_n: f64,
    width: X<f64>,
}

pub struct Quaver(pub Length);

impl StringLines {
    pub fn new(n_of_strings: u8, line_n: f64, width: X<f64>) -> Self {
        Self { n_of_strings, line_n, width }
    }
}

pub struct Tuplet(pub u8);

type Context = CanvasRenderingContext2d;

impl Drawable<Context> for TabMetaData {
    fn draw(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        let meta_text =
            format!("Title: {} | Tuning: {} | Tempo: {}", &self.title, &self.tuning, &self.tempo);
        fill_text(context, &meta_text, position, Some(sizes::font_size()));
    }
}

impl AutoDrawable<Context> for StringLines {
    fn draw(&self, context: &Context) -> () {
        for string_num in 0..self.n_of_strings {
            let y = sizes::y_by_line_and_string(self.line_n, self.n_of_strings, string_num.into());
            draw_line(
                context,
                sizes::line_width_default() * 0.5,
                Coordinate2D::new(X(0.0), y),
                Coordinate2D::new(self.width, y),
            );
        }
    }
}

impl Drawable<Context> for TimeSignature {
    fn draw(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        let font_size = sizes::font_size();
        fill_text(context, &format!("{}", &self.get_upper()), position, Some(font_size));
        fill_text(context, &format!("{}", &self.get_lower()), position.down(sizes::y_space()), Some(font_size));
    }
}

impl Drawable<Context> for BarDots {
    fn draw(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        fill_circle(context, position, sizes::r_default() * 0.1);
        fill_circle(context, position.down(sizes::y_space()), sizes::r_default() * 0.1);
    }
}

impl Drawable<Context> for NotesModifier {
    fn draw(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        let mod_text = match &self {
            &NotesModifier::Vibrato => String::from("~~"),
            _ => format!("{:?}", &self)
        };
        fill_text(context, &mod_text, position, None);
    }
}

impl Drawable<Context> for Link {
    fn draw(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        prepare_arc(
            context,
            position,
            (sizes::item_width() * 0.4).into(),
            0.0,
            PI,
            sizes::line_width_default(),
            false,
        );
    }
}

impl Drawable<Context> for Dot {
    fn draw(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        fill_circle(context, position, sizes::r_default() * 0.075);
    }
}

impl Drawable<Context> for Tuplet {
    fn draw(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        fill_text(context, &format!("{}", self.0), position.up(sizes::y_space() * 3.5), None);
    }
}

impl Drawable<Context> for Crotchet {
    fn draw(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        let (x, y) = (position.x, position.y);
        let x_step: X<f64> = sizes::item_width() * 0.25;
        let y_step: Y<f64> = Y(x_step.0);
        let (x1, y1, y2, y3) = (x + x_step, y + y_step, y + y_step * 2.0, y + y_step * 3.0);
        let y4 = y3 + y_step * 0.5;
        draw_line(context, sizes::line_width_default() * 1.5, position, Coordinate2D::new(x1, y1));
        draw_line(context, sizes::line_width_default() * 3.0, Coordinate2D::new(x1, y1), Coordinate2D::new(x, y2));
        draw_line(context, sizes::line_width_default() * 1.5, Coordinate2D::new(x, y2), Coordinate2D::new(x1, y3));
        prepare_arc(context, Coordinate2D::new(x1, y4), (x_step * 0.8).into(), PI * 0.6, PI * 1.4, sizes::line_width_default() * 2.0, false);
    }
}

impl Drawable<Context> for Note {
    fn draw(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        let fret_num = self.get_fret_num();
        let text =
            if fret_num >= 0 { format!("{}", fret_num) } else { String::from("X") };
        fill_text(context, &text, position, None);
    }
}

impl<'a> Drawable<Context> for DrawableBarStart<'a> {
    fn draw(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        let height = sizes::line_height(self.n_of_strings);

        draw_line(
            context,
            sizes::line_width_default() * 2.0,
            position,
            position.down(height),
        );
        let x_space = sizes::item_width() * 0.25;
        let p_left = position.left(x_space);

        if let BarStart::Repeat = self.bar_start {
            draw_line(
                context,
                sizes::line_width_default() * 4.0,
                p_left,
                p_left.down(height),
            );
            BarDots.draw(
                context,
                position.right(x_space * 2.0).down(height * 0.45),
            );
        }
    }
}

impl<'a> Drawable<Context> for DrawableBarEnd<'a> {
    fn draw(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        let height = sizes::line_height(self.n_of_strings);

        draw_line(
            context,
            sizes::line_width_default() * 2.0,
            position,
            position.down(height),
        );
        let x_space = sizes::item_width() * 0.25;

        if let BarEnd::Repeat(repeats) = self.bar_end {
            if *repeats > 2 {
                fill_text(context, &format!("{}x", repeats), position.up(sizes::y_space() / 2.0), None);
            }
            draw_line(
                context,
                sizes::line_width_default() * 4.0,
                position.right(x_space),
                position.right(x_space).down(height),
            );
            BarDots.draw(
                context,
                position.left(x_space * 2.0).down(height * 0.45),
            );
        }
    }
}

impl Drawable<Context> for DrawableRest {
    fn draw(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        let small_gap = sizes::y_space() * 1.5;
        let larger_gap = sizes::y_space() * 2.5;
        match self.0 {
            Length::Whole => {
                draw_line(
                    context,
                    sizes::line_width_default() * 5.0,
                    position.up(small_gap),
                    position.right(sizes::item_width() * 0.4).up(small_gap),
                );
            }
            Length::Half => {
                draw_line(
                    context,
                    sizes::line_width_default() * 2.5,
                    position.up(small_gap),
                    position.right(sizes::item_width() * 0.4).up(small_gap),
                );
            }
            Length::Quarter => { Crotchet.draw(context, position.up(larger_gap)); }
            _ => { Quaver(self.0).draw(context, position.up(larger_gap)); }
        }
    }
}

impl Drawable<Context> for Quaver {
    fn draw(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        draw_line(
            context,
            sizes::line_width_default() * 1.5,
            position,
            position.left(sizes::item_width() * 0.25).down(sizes::y_space()),
        );
        for tail in 0..self.0.num_of_tails() {
            let shift = tail as f64 * 2.0;
            let radius: Radius<f64> = (sizes::item_width() * 0.2).into();

            prepare_arc(
                context,
                position.left(radius.into()).left(shift.into()).down(shift.into()),
                radius, 0.0, PI, sizes::line_width_default() * 1.5,
                false,
            );
        }
    }
}

impl<'a> Drawable<Context> for DrawableNotes<'a> {
    fn draw(&self, context: &Context, position: Coordinate2D<f64>) -> () {
        self.draw_note(context, position);

        for note in self.notes {
            note.draw(
                context,
                position.at_y(
                    sizes::y_by_line_and_string(
                        self.line_n,
                        self.n_of_strings,
                        note.get_string_num().into()) - sizes::y_space() / 2.0
                ),
            );
        }
    }
}