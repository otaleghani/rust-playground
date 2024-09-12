use std::{cmp, ops::Range};
use unicode_segmentation::UnicodeSegmentation;

pub struct Line {
    // string: String,
    string: TextFragment,
}

enum GraphemeWidth {
    Half,
    Full
}

#[derive(Debug)]
struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        // Self {
        //     string: String::from(line_str),
        // }

        // TODO: check if grapheme could be rendered
        let replacement = Some('.');
        Self {
            string: {
                grapheme: String::from(line_str),
                rendered_width: 0, // Width with unicode-width crate
                replacement: Some('.'),
            }
        }
    }

    pub fn get(&self, range: Range<usize>) -> String {

        let start = range.start;
        let end = cmp::min(range.end, self.len());

        self.string
            .graphemes(true)
            .skip(start)
            .take(end.saturating_sub(start))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.string[..].graphemes(true).count()
    }
}
