use crate::*;
use web_sys::*;
use std::f64::consts::PI;
use wasm_bindgen::JsCast;

pub fn get_canvas_element_by_id(id: &str) -> Option<HtmlCanvasElement> {
    let maybe_canvas = get_by_id(id);
    maybe_canvas.and_then(|canvas| canvas.dyn_into::<HtmlCanvasElement>().ok())
}

pub fn get_2d_context_from_canvas(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    canvas.get_context("2d")
        .ok()
        .and_then(
            |maybe_context| maybe_context.map(|context| context.dyn_into::<CanvasRenderingContext2d>())
        )
        .and_then(|res| res.ok())
}

pub fn draw_line(
    context: &CanvasRenderingContext2d,
    line_width: LineWidth<f64>,
    start: Coordinate2D<f64>,
    end: Coordinate2D<f64>,
) -> () {
    let previous_line_width = context.line_width();
    context.set_line_width(line_width.0);
    context.begin_path();
    context.move_to(start.x.0, start.y.0);
    context.line_to(end.x.0, end.y.0);
    context.stroke();
    context.set_line_width(previous_line_width);
}

pub fn fill_circle(context: &CanvasRenderingContext2d, position: Coordinate2D<f64>, radius: Radius<f64>) -> () {
    prepare_circle(context, position, radius, true);
}


pub fn stroke_circle(context: &CanvasRenderingContext2d, position: Coordinate2D<f64>, radius: Radius<f64>) -> () {
    prepare_circle(context, position, radius, false);
}

pub fn prepare_circle(context: &CanvasRenderingContext2d, position: Coordinate2D<f64>, radius: Radius<f64>, fill: bool) -> () {
    prepare_arc(context, position, radius, 0.0, PI * 2.0, LineWidth(1.0), fill);
}

#[allow(unused_must_use)]
pub fn prepare_arc(
    context: &CanvasRenderingContext2d,
    position: Coordinate2D<f64>,
    radius: Radius<f64>,
    start_angle: f64,
    end_angle: f64,
    line_width: LineWidth<f64>,
    fill: bool,
) -> () {
    let previous_line_width = context.line_width();
    context.set_line_width(line_width.0);
    context.begin_path();
    context.arc(position.x.0, position.y.0, radius.0, start_angle, end_angle);
    if fill {
        context.fill();
    } else {
        context.stroke();
    }
    context.set_line_width(previous_line_width);
}

pub fn clear_canvas(context: &CanvasRenderingContext2d, bottom_right: Coordinate2D<f64>) -> () {
    context.clear_rect(0.0, 0.0, bottom_right.x.0, bottom_right.y.0);
}

pub fn canvas_bottom_right(canvas: &HtmlCanvasElement) -> Coordinate2D<f64> {
    Coordinate2D::new(X(canvas.width().into()), Y(canvas.height().into()))
}

#[allow(unused_must_use)]
pub fn fill_text(
    context: &CanvasRenderingContext2d,
    text: &str,
    position: Coordinate2D<f64>,
    font_size: Option<u64>,
) -> () {
    let draw_text = || context.fill_text(text, position.x.0, position.y.0);
    match font_size {
        Some(font_size) => {
            let original_font = context.font();
            context.set_font(&format!("{}px sans-serif", font_size));
            draw_text();
            context.set_font(&original_font);
        }
        _ => {
            draw_text();
        }
    }
}

pub fn get_first_by_tag_name(tag_name: &str) -> Option<Element> {
    document().and_then(|doc| doc.get_elements_by_tag_name(tag_name).item(0))
}

pub fn get_by_id(id: &str) -> Option<Element> {
    document().and_then(|doc| doc.get_element_by_id(id))
}

fn document() -> Option<Document> {
    window().and_then(|window| window.document())
}
