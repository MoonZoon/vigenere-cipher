use zoon::*;

pub mod cipher;
pub mod view;

// ------ ------
//    States
// ------ ------

static KEY: &str = "°¡! RüST íS CóÓL ¡!°";

#[allow(clippy::invisible_characters)]
static DICTIONARY: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ ¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ "##;

#[static_ref]
fn phrase() -> &'static Mutable<String> {
    Mutable::default()
}

// ------ ------
//    Signals
// ------ ------

fn encoding_result() -> impl Signal<Item = cipher::Result> {
    phrase().signal_ref(|phrase| cipher::encode(phrase))
}

fn decoding_result() -> impl Signal<Item = cipher::Result> {
    encoding_result()
        .map(|encoding_result| encoding_result.and_then(|message| cipher::decode(&message)))
}
