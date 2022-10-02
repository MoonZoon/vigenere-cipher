use crate::{
    cipher::{self, Cipher},
    ui::color,
};
use std::rc::Rc;
use zoon::{format, *};

pub struct App {
    key: &'static str,
    dictionary: &'static str,
    phrase: Mutable<String>,
    cipher: Rc<Cipher>,
}

impl App {
    pub fn new() -> Self {
        let key = "°¡! RüST íS CóÓL ¡!°";
        #[allow(clippy::invisible_characters)]
        let dictionary = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ ¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ "##;
        Self {
            key,
            dictionary,
            phrase: Mutable::default(),
            cipher: Rc::new(Cipher::new(dictionary, key)),
        }
    }

    // ------ ------
    //    Signals
    // ------ ------

    fn encoding_result(&self) -> impl Signal<Item = cipher::Result> {
        let cipher = Rc::clone(&self.cipher);
        self.phrase.signal_ref(move |phrase| cipher.encode(phrase))
    }

    fn decoding_result(&self) -> impl Signal<Item = cipher::Result> {
        let cipher = Rc::clone(&self.cipher);
        self.encoding_result()
            .map(move |encoding_result| encoding_result.and_then(|message| cipher.decode(&message)))
    }

    // ------ ------
    //     View
    // ------ ------

    pub fn root(&self) -> impl Element {
        El::new()
            // @TODO improve `Scrollbars` and refactor once possible:
            // - https://caniuse.com/css-scrollbar
            // - https://css-tricks.com/the-current-state-of-styling-scrollbars-in-css/
            .update_raw_el(|raw_el| {
                raw_el
                    .style_group(
                        StyleGroup::new("::-webkit-scrollbar")
                            .style("height", "10px")
                            .style("width", "10px"),
                    )
                    .style_group(
                        StyleGroup::new("::-webkit-scrollbar-thumb")
                            .style("background", color::scrollbar_thumb().into_cow_str())
                            .style("border-radius", "6px"),
                    )
                    .style_group(
                        StyleGroup::new("::-webkit-scrollbar-thumb:hover")
                            .style("background", color::scrollbar_thumb_hover().into_cow_str()),
                    )
                    .style_group(
                        StyleGroup::new("::-webkit-scrollbar-track")
                            .style("background", color::background().into_cow_str())
                            .style("border-radius", "6px"),
                    )
            })
            .s(Scrollbars::both())
            .s(Padding::new().x(10).y(26))
            .s(Background::new().color(color::background_body()))
            .s(Height::fill())
            .s(Font::new()
                .size(16)
                .color(color::text_bright())
                .family([FontFamily::new("system-ui"), FontFamily::SansSerif]))
            .child(self.content())
    }

    fn content(&self) -> impl Element {
        Column::new()
            .s(Width::growable().max(800))
            .s(Align::new().center_x())
            .s(Gap::both(20))
            .item(self.title())
            .item(self.key())
            .item(self.phrase())
            .item(self.encoding_error())
            .item(self.encoded())
            .item(self.decoded())
            .item(self.dictionary())
            .item(self.footer())
    }

    fn title(&self) -> impl Element {
        El::with_tag(Tag::H1)
            .s(Font::new()
                .size(35)
                .weight(FontWeight::SemiBold)
                .wrap_anywhere())
            .child("Real-Time Vigenère Cipher")
    }

    fn key(&self) -> impl Element {
        self.key_value_field("Key", always(self.key), color::encoding())
    }

    fn phrase(&self) -> impl Element {
        let (focused, focused_signal) = Mutable::new_and_signal(false);
        let phrase = self.phrase.clone();
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
                Placeholder::new("Enter a phrase...")
                    .s(Font::new().color(color::form_placeholder())),
            )
            .s(Width::fill())
            .text_signal(phrase.signal_cloned())
            .label_hidden("phrase")
            .on_change(move |new_phrase| phrase.set(new_phrase))
            .on_focused_change(move |is_focused| focused.set_neq(is_focused))
    }

    fn encoding_error(&self) -> impl Element {
        El::new()
            .s(Font::new().color(color::encoding()))
            .child_signal(
                self.encoding_result()
                    .map(Result::err)
                    .map_some(|error| match error {
                        cipher::Error::InvalidMessageChar(character) => {
                            format!("Invalid character: {character}")
                        }
                        cipher::Error::InvalidKeyChar(character) => {
                            format!("Invalid key character: {character}")
                        }
                    }),
            )
    }

    fn encoded(&self) -> impl Element {
        self.key_value_field(
            "Encoded",
            self.encoding_result().map(Result::ok),
            color::encoding(),
        )
    }

    fn decoded(&self) -> impl Element {
        self.key_value_field(
            "Decoded",
            self.decoding_result().map(Result::ok),
            color::decoding(),
        )
    }

    fn dictionary(&self) -> impl Element {
        Paragraph::new()
            .s(Font::new().color(color::text_main()))
            .content("The encoding dictionary includes the following set of ")
            .content(self.dictionary.len())
            .content(" characters:\n")
            .content("[")
            .content(
                El::new()
                    .s(Font::new()
                        .color(color::dictionary())
                        .family([FontFamily::new("Courier")])
                        .wrap_anywhere())
                    .child(self.dictionary),
            )
            .content("]")
    }

    // ------ footer ------

    fn footer(&self) -> impl Element {
        Column::new()
            .s(Borders::new().top(Border::new().color(color::border())))
            .s(Padding::new().top(11))
            .s(Gap::both(16))
            .item(self.inspired_by())
            .item(self.repo())
    }

    fn inspired_by(&self) -> impl Element {
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

    fn repo(&self) -> impl Element {
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
        &self,
        key: &str,
        value: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
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
                    .child_signal(value.map(|value| value.into_option_cow_str())),
            )
            .content(El::new().child("]"))
    }
}
