use crate::{
    elements::*,
    helpers::*,
    TabParsingResult, Visualizer,
};
use photonix::*;

#[allow(dead_code)]
pub struct PrintVisualizer;

impl Visualizer<()> for PrintVisualizer {
    fn visualize(&self, tab_parsing_result: TabParsingResult) -> () {
        println!("{:?}", tab_parsing_result);
    }
}

pub struct CanvasVisualizer;

mod cvh {
    use crate::{
        Coordinate2D, X, Y,
        helpers::*,
        imp::sizes,
        imp::drawable_components::*,
    };
    use super::*;
    use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

    pub struct CanvasVisualizerHelper {
        canvas: HtmlCanvasElement,
        context: CanvasRenderingContext2d,
        tab_meta_data: TabMetaData,
        tab_lines: Vec<TabLine>,
    }

    #[allow(dead_code)]
    impl CanvasVisualizerHelper {
        pub fn new(
            canvas: HtmlCanvasElement,
            context: CanvasRenderingContext2d,
            tab_meta_data: TabMetaData,
            tab_lines: Vec<TabLine>,
        ) -> Self {
            Self { canvas, context, tab_meta_data, tab_lines }
        }

        pub fn draw_tab(self) -> () {
            self.init_drawing();
            self.tab_meta_data.draw(&self.context, Coordinate2D::new(X(10.0), Y(20.0)));

            let lines = &self.tab_lines;
            let mut current_signature: Option<TimeSignature> = None;

            for (line_n, line) in lines.iter().enumerate() {
                let line_n_f64 = line_n as f64;
                let top_y = sizes::y0_by_line(line_n_f64, self.n_of_strings());
                let x0 = sizes::item_width() / 2.0;
                let start_x = sizes::current_x(x0, 0);
                let start_pos = Coordinate2D::new(start_x, top_y);
                let mut items_in_line = 0;

                StringLines::new(self.n_of_strings(), line_n_f64, X(self.canvas.width().into()))
                    .draw(&self.context);

                for tab_bar in line.get_ref().iter() {
                    DrawableBarStart::new(tab_bar.get_ref(), self.n_of_strings())
                        .draw(&self.context, start_pos.set(sizes::current_x(x0, items_in_line)));

                    let bar_sig = Some(*tab_bar.get_ref());

                    if bar_sig != current_signature {
                        current_signature = bar_sig;
                        if let Some(signature) = current_signature {
                            signature.draw(
                                &self.context,
                                start_pos
                                    .set(sizes::current_x(x0, items_in_line))
                                    .modify(|x: X<f64>| x + sizes::item_width())
                                    .modify(|y: Y<f64>| y + sizes::line_height(self.n_of_strings()) * 0.45),
                            );
                        }
                        items_in_line += 1;
                    }
                    self.draw_tab_items(tab_bar.get_ref(), line_n_f64, items_in_line as f64);
                    items_in_line += tab_bar.length() + 2;

                    DrawableBarEnd::new(tab_bar.get_ref(), self.n_of_strings())
                        .draw(&self.context, start_pos.set(sizes::current_x(x0, items_in_line)));
                }
            }
        }

        fn n_of_strings(&self) -> u8 {
            self.tab_meta_data.number_of_strings
        }

        fn calculate_height(&self) -> u32 {
            ((self.tab_lines.len()) as f64
                * (sizes::line_height(self.n_of_strings()) + sizes::space_between_lines()).0
                + sizes::top_padding().0) as u32
        }

        fn init_drawing(&self) -> () {
            let canvas = &self.canvas;
            canvas.set_width(width_of_main());
            canvas.set_height(self.calculate_height());
            canvas.set_hidden(false);
            clear_canvas(&self.context, canvas_bottom_right(canvas));
        }

        fn draw_tab_items(&self, items: &Vec<TabItem>, line_n: f64, items_in_line: f64) -> () {
            for (item_n, item) in items.iter().enumerate() {
                let position = Coordinate2D::new(
                    sizes::x_by_item_n(item_n as f64 + items_in_line),
                    sizes::y0_by_line(line_n, self.n_of_strings()),
                );

                let length: Length = *item.get_ref();
                let content: &NotesOrRest = item.get_ref();
                match content {
                    NotesOrRest::Notes { notes } => {
                        DrawableNotes::new(notes, length, line_n, self.n_of_strings())
                            .draw(&self.context, position);
                    }
                    NotesOrRest::Rest => {
                        DrawableRest(length).draw(&self.context, position);
                    }
                }
                self.draw_extras(item, position);
            }
        }

        fn draw_extras(&self, item: &TabItem, position: Coordinate2D<f64>) -> () {
            let dotted: &Dotted = item.get_ref();
            if dotted.0 {
                Dot.draw(
                    &self.context,
                    position
                        .modify(|x: X<f64>| x + sizes::item_width() * 0.5)
                        .modify(|y: Y<f64>| y - sizes::y_space() * 1.45),
                );
            }
            let linked: &Linked = item.get_ref();
            if linked.0 {
                Link.draw(
                    &self.context,
                    position
                        .modify(|x: X<f64>| x + sizes::item_width() / 2.0)
                        .modify(|y: Y<f64>| y - sizes::y_space()),
                );
            }
            if let Some(modifier) = item.get_ref() {
                modifier.draw(
                    &self.context,
                    position.modify(|y: Y<f64>| y +
                        sizes::line_height(self.n_of_strings()) + sizes::space_between_strings()
                    ),
                );
            }
            let tuplet: u8 = *item.get_ref();
            if tuplet > 2 {
                Tuplet(tuplet).draw(&self.context, position);
            }
        }
    }
}

const MAX_ITEMS_PER_LINE: usize = 50;

impl Visualizer<()> for CanvasVisualizer {
    fn visualize(&self, tab_parsing_result: TabParsingResult) -> () {
        let maybe_canvas = get_canvas_element_by_id("display");
        let maybe_context =
            maybe_canvas.iter().flat_map(|canvas| get_2d_context_from_canvas(&canvas)).next();
        let maybe_error_msg_holder = get_by_id("error_msg_holder");
        match tab_parsing_result {
            Ok(tab) => {
                let metadata: &TabMetaData = tab.get_ref();
                if let (Some(canvas), Some(context), Some(error_msg_holder)) =
                (maybe_canvas, maybe_context, maybe_error_msg_holder) {
                    error_msg_holder.set_inner_html("");
                    cvh::CanvasVisualizerHelper::new(
                        canvas,
                        context,
                        metadata.clone(),
                        tab.into_lines(MAX_ITEMS_PER_LINE),
                    ).draw_tab();
                }
            }
            Err(msg) => {
                if let Some(error_msg_holder) = maybe_error_msg_holder {
                    error_msg_holder.set_inner_html(&msg);
                }
            }
        }
    }
}

fn width_of_main() -> u32 {
    get_first_by_tag_name("main")
        .map(|main| main.client_width())
        .unwrap_or_else(|| 0) as u32
}