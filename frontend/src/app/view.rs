use crate::ui::color;
use zoon::*;

pub fn root() -> impl Element {
    El::new()
        .s(Background::new().color(color::background_body()))
        .s(Height::fill())
        .s(Font::new()
            .size(16)
            .color(color::text_bright())
            .family([FontFamily::new("system-ui"), FontFamily::SansSerif]))
        .child(content())
}

fn content() -> impl Element {
    Column::new()
        .s(Width::growable().max(800))
        .s(Align::new().center_x())
        .s(Gap::both(20))
        .item(title())
        .item(key())
        .item(phrase())
        .item(encoded())
        .item(decoded())
        .item(dictionary())
        .item(footer())
}

fn title() -> impl Element {
    El::with_tag(Tag::H1)
        .s(Padding::new().top(26))
        .s(Font::new().size(35).weight(FontWeight::SemiBold))
        .child("Real-Time Vigenère Cipher")
}

fn key() -> impl Element {
    key_value_field("Key", always(super::KEY), color::encoding())
}

fn phrase() -> impl Element {
    let (focused, focused_signal) = Mutable::new_and_signal(false);
    TextArea::new()
        // @TODO refactor once TextArea is improved in MoonZoon
        // https://github.com/MoonZoon/MoonZoon/issues/44
        .update_raw_el(|raw_el| raw_el.style("resize", "vertical"))
        .s(Background::new().color(color::background()))
        .s(RoundedCorners::all(6))
        .s(Shadows::with_signal(focused_signal.map_true(|| {
            Shadow::new().spread(2).color(color::focus())
        })))
        .s(Height::exact(140).min(40))
        .s(Padding::all(10))
        .s(Font::new().color(color::form_text()))
        .placeholder(
            Placeholder::new("Enter a phrase...").s(Font::new().color(color::form_placeholder())),
        )
        .text_signal(super::phrase().signal_cloned())
        .label_hidden("phrase")
        .on_change(super::set_phrase)
        .on_focused_change(move |is_focused| focused.set_neq(is_focused))
}

fn encoded() -> impl Element {
    key_value_field(
        "Encoded",
        super::encoded().signal_cloned(),
        color::encoding(),
    )
}

fn decoded() -> impl Element {
    key_value_field(
        "Decoded",
        super::decoded().signal_cloned(),
        color::decoding(),
    )
}

fn dictionary() -> impl Element {
    Paragraph::new()
        .s(Font::new().color(color::text_main()))
        .content("The encoding dictionary includes the following set of 225 ASCII characters:\n")
        .content("[")
        .content(
            El::new()
                .s(Font::new()
                    .color(color::dictionary())
                    .family([FontFamily::new("Courier")])
                    .wrap_anywhere())
                .child([" ", super::DICTIONARY, " "].concat()),
        )
        .content("]")
}

// ------ footer ------

fn footer() -> impl Element {
    Column::new()
        .s(Borders::new().top(Border::new().color(color::border())))
        .s(Padding::new().top(10))
        .s(Gap::both(16))
        .item(inspired_by())
        .item(repo())
}

fn inspired_by() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Paragraph::new()
        .s(Font::new().size(14).color(color::text_muted()))
        .content("Inspired by ")
        .content(
            Link::new()
                .s(
                    Font::new()
                        .color(color::links())
                        .line(FontLine::new().underline_signal(hovered_signal))
                    )
                .label("Building a Real-Time Web Cipher with Rust, Sycamore and Trunk")
                .to("https://rsdlt.github.io/posts/rust-sycamore-trunk-wasm-iterators-vigenere-cipher/")
                .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        )
}

fn repo() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Link::new()
        .s(Align::new().left())
        .s(Font::new()
            .color(color::links())
            .line(FontLine::new().underline_signal(hovered_signal)))
        .label("GitHub Repo")
        .to("https://github.com/MoonZoon/vigenere-cipher")
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
}

// ------ key_value_field ------

fn key_value_field<'a>(
    key: &str,
    value: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
    value_color: HSLuv,
) -> impl Element {
    Paragraph::new()
        .content(
            Row::new()
                .s(Font::new().weight(FontWeight::SemiBold))
                .item(key)
                .item(": "),
        )
        .content(El::new().child("["))
        .content(
            El::new()
                .s(Font::new()
                    .color(value_color)
                    .family([FontFamily::new("Courier New")])
                    .wrap_anywhere())
                .child(Text::with_signal(value)),
        )
        .content(El::new().child("]"))
}
