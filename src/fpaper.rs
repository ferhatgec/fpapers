// MIT License
//
// Copyright (c) 2023 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

#[derive(Clone)]
pub struct FPaper {
    pub raw_data: String,
}

impl Default for FPaper {
    fn default() -> Self {
        FPaper {
            raw_data: String::default()
        }
    }
}

impl FPaper {
    pub fn init(&mut self, filename: &String) {
        if let Ok(data) = std::fs::read_to_string(filename) {
            self.raw_data = data;
        }
    }

    pub fn feed(&mut self, data: &String) {
        self.raw_data = String::from(data);
    }
}