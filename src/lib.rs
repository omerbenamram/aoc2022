use std::{ops::RangeInclusive, time::Duration};

use anyhow::Context;
use itertools::Itertools;

pub mod grid;

pub fn timed<R, F>(f: F) -> (R, Duration)
where
    F: Fn() -> R,
{
    let t0 = std::time::Instant::now();
    let result = f();

    (result, t0.elapsed())
}

/// Parses input of the form `\d-\d` to an inclusive range.
pub fn range_inclusive(i: &str) -> anyhow::Result<RangeInclusive<i32>> {
    let (s1, e1) = i
        .split('-')
        .map(|number| number.parse::<i32>())
        .collect_tuple()
        .context(r#"Expected input of the form `\d-\d`"#)?;

    Ok(s1?..=e1?)
}

#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}
