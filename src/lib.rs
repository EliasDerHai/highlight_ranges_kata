#![allow(unused, dead_code)]

use std::{fmt::format, usize};

#[derive(Debug, PartialEq, Eq)]
enum HighlightingBoundry {
    Start, // <em>
    End,   // </em>
}

#[derive(Debug, PartialEq, Eq)]
enum HighlightingError {
    OverlappingRanges,
    RangesOutOfBounds,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HighlightRange {
    /// inclusive
    lower: u32,
    /// exclusive
    upper: u32,
}

impl HighlightRange {
    /// lower = inclusive, upper = exclusive, swaps upper and lower if necessary
    fn new(lower: u32, upper: u32) -> Self {
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

#[derive(Debug, Default)]
struct FoldState<'a> {
    out: String,       // mutable output
    offset: usize,     // mutable offset
    original: &'a str, // original input text
}

impl<'a> From<&'a str> for FoldState<'a> {
    fn from(value: &'a str) -> Self {
        FoldState {
            out: String::new(),
            offset: 0,
            original: value,
        }
    }
}

fn highlight_text(
    input: &str,
    highlights: Vec<HighlightRange>,
) -> Result<String, HighlightingError> {
    validate_ranges(input.len(), &highlights)?;

    let mut highlights = highlights
        .iter()
        .flat_map(|hr| {
            [
                (hr.lower as usize, HighlightingBoundry::Start),
                (hr.upper as usize, HighlightingBoundry::End),
            ]
        })
        .collect::<Vec<(usize, HighlightingBoundry)>>();

    highlights.sort_by_key(|(idx, _)| *idx);

    let out = highlights.iter().fold(
        FoldState::from(input),
        |mut state, (next_high_pos, next_high_kind)| {
            let next: &str = &state.original[state.offset..*next_high_pos];
            state.out += next;
            state.out += match next_high_kind {
                HighlightingBoundry::Start => "<em>",
                HighlightingBoundry::End => "</em>",
            };
            state.offset = *next_high_pos;
            state
        },
    );

    Ok(out.out + &input[out.offset..])
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

        assert_eq!("<em>Hello</em><em> world</em>", actual);
    }

    #[test]
    fn should_return_err_invalid_ranges() {
        let actual = highlight_text(
            "Hello world",
            vec![HighlightRange::new(0, 5), HighlightRange::new(4, 11)],
        );

        assert_eq!(Err(HighlightingError::OverlappingRanges), actual);
    }

    #[test]
    fn should_return_err_out_of_bounds() {
        let actual = highlight_text("Hello world", vec![HighlightRange::new(0, 50)]);

        assert_eq!(Err(HighlightingError::RangesOutOfBounds), actual);
    }
}
