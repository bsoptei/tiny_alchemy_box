use super::*;

pub fn current_x(x0: X<f64>, items: usize) -> X<f64> {
    x0 + (item_width() * (items as f64 + 1.0))
}

pub fn font_size() -> u64 {
    item_width().0 as u64
}

pub fn item_width() -> X<f64> {
    X(20.0)
}

pub fn line_height(n_of_strings: u8) -> Y<f64> {
    space_between_strings() * (n_of_strings - 1) as f64
}

pub fn line_width_default() -> LineWidth<f64> {
    LineWidth(1.0)
}

pub fn r_default() -> Radius<f64> {
    Radius(20.0)
}

pub fn space_between_lines() -> Y<f64> {
    y_space() * 5.0
}

pub fn space_between_strings() -> Y<f64> {
    y_space() * 0.75
}

pub fn top_padding() -> Y<f64> {
    y_space() * 7.5
}

pub fn x_by_item_n(n_item_in_line: f64) -> X<f64> {
    item_width() * 2.5 + item_width() * n_item_in_line
}

pub fn y0_by_line(line_num: f64, n_of_strings: u8) -> Y<f64> {
    (line_height(n_of_strings) + space_between_lines()) * line_num + top_padding()
}

pub fn y_by_line_and_string(line_num: f64, n_of_strings: u8, string_num: f64) -> Y<f64> {
    y0_by_line(line_num, n_of_strings) + space_between_strings() * string_num
}

pub fn y_space() -> Y<f64> {
    Y(20.0)
}
