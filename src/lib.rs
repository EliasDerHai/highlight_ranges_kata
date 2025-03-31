#![allow(unused, dead_code)]

use std::fmt::format;

#[derive(Debug, Clone, PartialEq, Eq)]
struct HighlightRange {
    /// inclusive
    lower: u32,
    /// exclusive
    upper: u32,
}

impl HighlightRange {
    /// lower = exclusive, upper = inclusive, swaps upper and lower if necessary
    fn new(lower: u32, upper: u32) -> HighlightRange {
        if lower < upper {
            HighlightRange { lower, upper }
        } else {
            HighlightRange {
                lower: upper,
                upper: lower,
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RangeMatch {
    Open,
    Close,
    Both,
    None,
}

fn check_match(idx: u32, highlights: &Vec<HighlightRange>) -> RangeMatch {
    let open = highlights.iter().any(|h| h.lower == (idx as u32));
    let close = highlights.iter().any(|h| h.upper == (idx as u32));

    match (open, close) {
        (true, true) => RangeMatch::Both,
        (true, false) => RangeMatch::Open,
        (false, true) => RangeMatch::Close,
        (false, false) => RangeMatch::None,
    }
}

#[derive(Debug, PartialEq, Eq)]
enum HighlightingError {
    OverlappingRanges,
    RangesOutOfBounds,
}

fn validate_ranges(
    input_len: usize,
    highlights: &Vec<HighlightRange>,
) -> Result<(), HighlightingError> {
    for h in highlights {
        if h.upper as usize > input_len || h.lower as usize >= input_len {
            return Err(HighlightingError::RangesOutOfBounds);
        }
    }

    let mut sorted = highlights.clone();
    sorted.sort_by_key(|r| r.lower);

    for i in 1..sorted.len() {
        if sorted[i].lower < sorted[i - 1].upper {
            return Err(HighlightingError::OverlappingRanges);
        }
    }

    Ok(())
}

fn highlight_text(
    input: &str,
    highlights: Vec<HighlightRange>,
) -> Result<String, HighlightingError> {
    validate_ranges(input.len(), &highlights)?;

    let input = [input, " "].concat(); // mitigate last character issue by adding an empty
    // space which is trimmed out in the end
    let out = input
        .chars()
        .enumerate()
        .map(|(idx, c)| {
            let range_match = check_match(idx as u32, &highlights);

            if range_match != RangeMatch::None {
                println!("{idx}: '{c}'");
            }

            match range_match {
                RangeMatch::Open => format!("<em>{c}"),
                RangeMatch::Close => format!("</em>{c}"),
                RangeMatch::Both | RangeMatch::None => c.to_string(),
            }
        })
        .collect::<Vec<String>>()
        .join("")
        .trim()
        .to_string();

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_swap_false_upper_and_lower() {
        assert_eq!(HighlightRange::new(0, 5), HighlightRange::new(5, 0));
    }

    #[test]
    fn should_hightlight() {
        let actual = highlight_text("Hello world", vec![HighlightRange::new(0, 5)]).unwrap();

        assert_eq!("<em>Hello</em> world", actual);
    }

    #[test]
    fn should_hightlight_multiple_ranges() {
        let actual = highlight_text(
            "Hello world",
            vec![HighlightRange::new(0, 5), HighlightRange::new(6, 11)],
        )
        .unwrap();

        assert_eq!("<em>Hello</em> <em>world</em>", actual);
    }

    #[test]
    fn should_hightlight_multiple_ranges_when_ranges_touch() {
        let actual = highlight_text(
            "Hello world",
            vec![HighlightRange::new(0, 5), HighlightRange::new(5, 11)],
        )
        .unwrap();

        assert_eq!("<em>Hello world</em>", actual);
    }

    #[test]
    fn should_return_err_invalid_ranges() {
        let actual = highlight_text(
            "Hello world",
            vec![HighlightRange::new(0, 5), HighlightRange::new(3, 11)],
        );

        assert_eq!(Err(HighlightingError::OverlappingRanges), actual);
    }

    #[test]
    fn should_return_err_out_of_bounds() {
        let actual = highlight_text("Hello world", vec![HighlightRange::new(0, 50)]);

        assert_eq!(Err(HighlightingError::RangesOutOfBounds), actual);
    }
}
