// MIT License
//
// Copyright (c) 2023 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod fpaper;
pub mod extract;

pub enum FPaperMarkers {
    StartMarker = 0x02,
    StartMarker2 = 0x46,
    StartMarker3 = 0x50,
    StartMarker4 = 0x61,
    StartMarker5 = 0x67,
    StartMarker6 = 0x65,

    StartOfText = 0x26,
    EndOfText = 0x15,

    StyleMarker = 0x1A,
    LightSet = 0x30,
    BoldSet = 0x31,
    DimSet = 0x32,
    ItalicSet = 0x33,
    UnderlinedSet = 0x34,
    BlinkSet = 0x35,
    RapidBlinkSet = 0x36,

    ColorReset = 0x72,

    // These styles must be rendered by renderer implementation
    AlignLeftSet = 0x7B,
    AlignCenterSet = 0x7C,
    AlignRightSet = 0x7D,
    AlignReset = 0x7E
}

mod fpaper_marker_functions {
    use crate::FPaperMarkers::*;

    pub fn is_start_marker(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(StartMarker as u32).unwrap(); }
    pub fn is_start_marker_2(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(StartMarker2 as u32).unwrap(); }
    pub fn is_start_marker_3(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(StartMarker3 as u32).unwrap(); }
    pub fn is_start_marker_4(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(StartMarker4 as u32).unwrap(); }
    pub fn is_start_marker_5(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(StartMarker5 as u32).unwrap(); }
    pub fn is_start_marker_6(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(StartMarker6 as u32).unwrap(); }


    pub fn is_start_of_text(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(StartOfText as u32).unwrap(); }
    pub fn is_end_of_text(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(EndOfText as u32).unwrap(); }

    pub fn is_style_marker(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(StyleMarker as u32).unwrap(); }
    pub fn is_light_marker(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(LightSet as u32).unwrap(); }
    pub fn is_bold_marker(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(BoldSet as u32).unwrap(); }
    pub fn is_dim_marker(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(DimSet as u32).unwrap(); }
    pub fn is_italic_marker(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(ItalicSet as u32).unwrap(); }
    pub fn is_underlined_marker(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(UnderlinedSet as u32).unwrap(); }
    pub fn is_blink_marker(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(BlinkSet as u32).unwrap(); }
    pub fn is_rapid_blink_marker(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(RapidBlinkSet as u32).unwrap(); }

    pub fn is_color_reset(ch: &char) -> bool { return *ch ==
                                                std::char::from_u32(ColorReset as u32).unwrap(); }

    pub fn is_left_align(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(AlignLeftSet as u32).unwrap(); }
    pub fn is_center_align(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(AlignCenterSet as u32).unwrap(); }
    pub fn is_right_align(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(AlignRightSet as u32).unwrap(); }
    pub fn is_reset_align(ch: &char) -> bool { return *ch == 
                                                std::char::from_u32(AlignReset as u32).unwrap(); }
}

#[cfg(test)]
mod tests {
    use crate::extract::FPaperExtract;

    #[test]
    fn reading_from_file() {
        use crate::fpaper::FPaper;

        let mut value = FPaperExtract::default();
        let mut main = FPaper::default();
        let val = std::fs::read_to_string("example.fpaper").unwrap();

        main.feed(&val);
        value.clone = main;
        value.extract();
    }
}
