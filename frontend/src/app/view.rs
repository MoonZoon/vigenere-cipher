use zoon::{named_color::*, *};

pub fn root() -> impl Element {
    El::new()
        .child(content())
}

fn content() -> impl Element {
    Column::new()
        .s(Align::new().center_x())
        .s(Gap::both(20))
        .s(Font::new().color(GRAY_0).size(30))
        .item(title())
        .item(key())
        .item(phrase())
        .item(encoded())
        .item(decoded())
        .item(dictionary())
        .item(footer())
}

fn title() -> impl Element {
    El::new()
        .child("Element")
}

fn key() -> impl Element {
    El::new()
        .child("Element")
}

fn phrase() -> impl Element {
    El::new()
        .child("Element")
}

fn encoded() -> impl Element {
    El::new()
        .child("Element")
}

fn decoded() -> impl Element {
    El::new()
        .child("Element")
}

fn dictionary() -> impl Element {
    El::new()
        .child("Element")
}

fn footer() -> impl Element {
    El::new()
        .child("Element")
}
