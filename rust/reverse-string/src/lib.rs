extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let gs = UnicodeSegmentation::graphemes(input, true);
    gs.rev().collect()
}
