use zoon::*;

pub mod view;

// ------ ------
//    States
// ------ ------

static KEY: &str = "°¡! RüST íS CóÓL ¡!°";

static DICTIONARY: &str = r##"!"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ ¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ"##;

#[static_ref]
fn phrase() -> &'static Mutable<String> {
    Mutable::default()
}

#[static_ref]
fn encoded() -> &'static Mutable<String> {
    Mutable::default()
}

#[static_ref]
fn decoded() -> &'static Mutable<String> {
    Mutable::default()
}

// ------ ------
//   Commands
// ------ ------

fn set_phrase(phrase: String) {
    self::phrase().set(phrase)
}
