use std::{cmp, ops::Range};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub struct Line {
    // string: String,
    string: Vec<TextFragment>,
}

#[derive(Debug)]
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
        
        let mut result: Vec<TextFragment> = Vec::new();
        for grapheme in line_str.graphemes(true) {
            let width = grapheme.width();
            let mut replacement: Option<char> = Some('.');
            let rendered_width: GraphemeWidth;
            match width {
                0 => {
                    replacement = None;
                    rendered_width = GraphemeWidth::Half;
                }
                1 => rendered_width = GraphemeWidth::Half,
                2 => rendered_width = GraphemeWidth::Full,
                _ => rendered_width = GraphemeWidth::Full,
            };
            result.push(TextFragment {
                grapheme: grapheme.to_string(),
                replacement,
                rendered_width,
            });
        }

        Self {
            string: result,
        }
    }

    pub fn get(&self, range: Range<usize>) -> String {

        let start = range.start;
        let end = cmp::min(range.end, self.len());

        // self.string
        //     .graphemes(true)
        //     .skip(start)
        //     .take(end.saturating_sub(start))
        //     .collect()
        self.string[start..end]
    }

    pub fn len(&self) -> usize {
        self.string[..].graphemes(true).count()
    }
}
