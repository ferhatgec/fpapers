// MIT License
//
// Copyright (c) 2023 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use crate::{
    fpaper::FPaper,
    fpaper_marker_functions::*
};

pub struct FPaperExtract {
    pub clone: FPaper,
    pub extracted_text: String,

    is_start_marker: bool,
    is_start_marker_2: bool,
    is_start_marker_3: bool,
    is_start_marker_4: bool,
    is_start_marker_5: bool,
    is_start_marker_6: bool,

    is_start_of_text: bool,
    is_end_of_text: bool,

    is_style_marker: bool,

    is_left_align: bool,
    is_center_align: bool,
    is_right_align: bool,
    is_reset_align: bool
}

impl Default for FPaperExtract {
    fn default() -> Self {
        FPaperExtract {
            clone: FPaper::default(),

            extracted_text: String::new(),

            is_start_marker: false,
            is_start_marker_2: false,
            is_start_marker_3: false,
            is_start_marker_4: false,
            is_start_marker_5: false,
            is_start_marker_6: false,
            is_start_of_text: false,
            is_end_of_text: false,
            is_style_marker: false,
            is_left_align: false,
            is_center_align: false,
            is_right_align: false,
            is_reset_align: false,
        }
    }
}

impl FPaperExtract {
    pub fn init(&mut self, x: &FPaper) {
        self.clone = x.clone();
    }

    fn detect_style(&mut self, ch: char) {
        // note: not platform specific.
        if is_light_marker(&ch) {  self.extracted_text.push_str("\x1b[0m"); }
        else if is_bold_marker(&ch) { self.extracted_text.push_str("\x1b[1m"); }
        else if is_dim_marker(&ch) { self.extracted_text.push_str("\x1b[2m"); }
        else if is_italic_marker(&ch) { self.extracted_text.push_str("\x1b[3m"); }
        else if is_underlined_marker(&ch) { self.extracted_text.push_str("\x1b[4m"); }
        else if is_blink_marker(&ch) { self.extracted_text.push_str("\x1b[5m"); }
        else if is_rapid_blink_marker(&ch) {
            if cfg!(windows) {
                self.extracted_text.push_str("\x1b[6m");
            }}
        else if is_left_align(&ch) {
            self.is_right_align = false;
            self.is_center_align = false;
            self.is_reset_align = false;
            self.is_left_align = true; }
        else if is_center_align(&ch) {
            self.is_right_align = false;
            self.is_reset_align = false;
            self.is_left_align = false;
            self.is_center_align = true; }
        else if is_right_align(&ch) {
            self.is_reset_align = false;
            self.is_left_align = false;
            self.is_center_align = false;
            self.is_right_align = true; }
        else if is_reset_align(&ch) {
            self.is_reset_align = false;
            self.is_left_align = false;
            self.is_center_align = false;
            self.is_right_align = false; }
        else if is_color_reset(&ch) { self.extracted_text.push_str("\x1b[0m"); }
        else {
            let val: u32 = u32::from(ch);

            if (val >= 40 && val <= 49) || (val >= 100 && val <= 109) {
                self.extracted_text.push_str(format!("\x1b[{}m",
                                                            val - 10).as_str());
            }
        }
    }

    fn detect(&mut self, ch: char) {
        if self.is_style_marker {
            self.detect_style(ch);
            self.is_style_marker = false;
            return;
        }

        if !self.is_start_marker { self.is_start_marker = is_start_marker(&ch); }
        else if !self.is_start_marker_2 { self.is_start_marker_2 = is_start_marker_2(&ch); }
        else if !self.is_start_marker_3 { self.is_start_marker_3 = is_start_marker_3(&ch); }
        else if !self.is_start_marker_4 { self.is_start_marker_4 = is_start_marker_4(&ch); }
        else if !self.is_start_marker_5 { self.is_start_marker_5 = is_start_marker_5(&ch); }
        else if !self.is_start_marker_6 { self.is_start_marker_6 = is_start_marker_6(&ch); }
        else if !self.is_start_of_text { self.is_start_of_text = is_start_of_text(&ch); }
        else if self.is_start_of_text {
            if is_style_marker(&ch) {
                self.is_style_marker = true;
                return;
            }

            if is_end_of_text(&ch) {
                self.is_end_of_text = true;
                return;
            }

            self.extracted_text.push(ch);
        }
    }

    pub fn compile(&mut self) {
        for ch in self.clone.clone().raw_data.chars() {
            if self.is_end_of_text { break; }
            self.detect(ch);
        }
    }

    pub fn extract(&mut self) {
        self.compile();
        println!("{}", self.extracted_text);
    }
}