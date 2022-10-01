use zoon::*;

pub mod cipher;
pub mod view;

// ------ ------
//    States
// ------ ------

static KEY: &str = "°¡! RüST íS CóÓL ¡!°";

static DICTIONARY: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ ¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ "##;

#[static_ref]
pub(self) fn phrase() -> &'static Mutable<String> {
    Mutable::default()
}

// ------ ------
//    Signals
// ------ ------

fn encoded() -> impl Signal<Item = String> {
    phrase().signal_ref(|phrase| cipher::encode(phrase).unwrap_throw())
}

fn decoded() -> impl Signal<Item = String> {
    encoded().map(|encoded_message| cipher::decode(&encoded_message).unwrap_throw())
}
