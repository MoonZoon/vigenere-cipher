use zoon::*;

macro_rules! color {
    ($color_name:ident => $color:expr) => {
        pub fn $color_name() -> HSLuv {
            $color
        }
    };
}

// https://www.hsluv.org/
// https://github.com/MoonZoon/MoonZoon/issues/98

color!(transparent => hsluv!(0, 0, 0, 0));

color!(background_body => hsluv!(244.3, 43.6, 17.1)); // #202b38
color!(background => hsluv!(238.4, 42.8, 11.3)); // #161f27
color!(background_alt => hsluv!(243, 44.3, 13.7)); // #1a242f

color!(selection => hsluv!(249.6, 94.9, 48.6)); // #1c76c5

color!(text_main => hsluv!(0, 0, 87.4)); // #dbdbdb
color!(text_bright => hsluv!(0, 0, 100)); // #ffffff
color!(text_muted => hsluv!(239.1, 11.7, 71.8)); // #a9b1ba

color!(links => hsluv!(244.4, 100, 68.2)); // #41adff
color!(focus => hsluv!(228.1, 100, 57.6, 67)); // #0096bfab
color!(border => hsluv!(240.8, 42.4, 43.4)); // #526980
color!(code => hsluv!(43.8, 100, 81.7)); // #ffbe85

color!(button_base => hsluv!(236.9, 52, 6.3)); // #0c151c
color!(button_hover => hsluv!(234.5, 60.3, 2.5)); // #040a0f

color!(scrollbar_thumb => hsluv!(234.5, 60.3, 2.5)); // #040a0f
color!(scrollbar_thumb_hover => hsluv!(0, 0, 0)); // #000000

color!(form_placeholder => hsluv!(0, 0, 69.2)); // #a9a9a9
color!(form_text => hsluv!(0, 0, 100)); // #ffffff

color!(variable => hsluv!(304.2, 83.8, 56.1)); // #d941e2
color!(highlight => hsluv!(43.8, 100, 81.7)); // #ffbe85

color!(encoding => hsluv!(17.3, 100, 62.2)); // #ff6347
color!(decoding => hsluv!(140.9, 85.5, 65.3)); // #3CB371
color!(dictionary => hsluv!(309.7, 63.5, 62.8)); // #DA70D6
