//! Summarising a set of measurements without lying about them.
//!
//! Medians and interquartile ranges, not means. A mean over process startup times is
//! dominated by whichever run happened to collide with something else on the machine, and
//! reporting it hides exactly the variance a reader needs in order to judge the number.

use serde::{Deserialize, Serialize};

/// A summary of one set of samples.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Summary {
    /// How many samples went into it.
    pub runs: usize,
    /// The median.
    pub median: f64,
    /// The 25th percentile.
    pub p25: f64,
    /// The 75th percentile.
    pub p75: f64,
    /// The smallest sample.
    pub min: f64,
    /// The largest sample.
    pub max: f64,
}

impl Summary {
    /// Summarise a set of samples.
    ///
    /// # Panics
    ///
    /// Panics if `samples` is empty. A summary of nothing has no meaningful value and
    /// returning zero would be read as a measurement.
    pub fn of(samples: &[f64]) -> Summary {
        assert!(!samples.is_empty(), "cannot summarise an empty sample set");

        let mut sorted = samples.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        Summary {
            runs: sorted.len(),
            median: percentile(&sorted, 0.50),
            p25: percentile(&sorted, 0.25),
            p75: percentile(&sorted, 0.75),
            min: sorted[0],
            max: sorted[sorted.len() - 1],
        }
    }

    /// The interquartile range, which is the spread figure we publish.
    pub fn iqr(self) -> f64 {
        self.p75 - self.p25
    }

    /// Whether this summary is far enough from another to claim a difference.
    ///
    /// The test is deliberately crude and deliberately conservative: the two interquartile
    /// ranges must not overlap. A more sophisticated test on data this noisy would be
    /// false precision, and the rule that matters is in the README, which is that a
    /// difference we cannot separate is reported as no difference.
    pub fn separable_from(self, other: Summary) -> bool {
        self.p75 < other.p25 || other.p75 < self.p25
    }
}

/// Linear interpolated percentile over an already sorted slice.
fn percentile(sorted: &[f64], q: f64) -> f64 {
    if sorted.len() == 1 {
        return sorted[0];
    }
    #[allow(clippy::cast_precision_loss)]
    let position = q * (sorted.len() - 1) as f64;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let lower = position.floor() as usize;
    let upper = (lower + 1).min(sorted.len() - 1);
    #[allow(clippy::cast_precision_loss)]
    let fraction = position - lower as f64;
    sorted[lower] + (sorted[upper] - sorted[lower]) * fraction
}

#[cfg(test)]
mod tests {
    use super::Summary;

    #[test]
    fn the_median_of_an_odd_sample_set_is_the_middle_value() {
        let summary = Summary::of(&[3.0, 1.0, 2.0]);
        assert!((summary.median - 2.0).abs() < 1e-9);
        assert!((summary.min - 1.0).abs() < 1e-9);
        assert!((summary.max - 3.0).abs() < 1e-9);
        assert_eq!(summary.runs, 3);
    }

    #[test]
    fn the_median_of_an_even_sample_set_is_interpolated() {
        let summary = Summary::of(&[1.0, 2.0, 3.0, 4.0]);
        assert!((summary.median - 2.5).abs() < 1e-9);
    }

    #[test]
    fn one_outlier_moves_the_median_far_less_than_it_would_move_a_mean() {
        let clean = Summary::of(&[10.0, 10.0, 10.0, 10.0, 10.0]);
        let spiked = Summary::of(&[10.0, 10.0, 10.0, 10.0, 900.0]);
        assert!((clean.median - spiked.median).abs() < 1e-9);
        assert!(
            (spiked.max - 900.0).abs() < 1e-9,
            "the outlier is still reported"
        );
    }

    #[test]
    fn overlapping_distributions_are_not_claimed_as_a_difference() {
        let a = Summary::of(&[10.0, 11.0, 12.0, 13.0, 14.0]);
        let b = Summary::of(&[11.0, 12.0, 13.0, 14.0, 15.0]);
        assert!(!a.separable_from(b));
    }

    #[test]
    fn clearly_separated_distributions_are_claimed() {
        let fast = Summary::of(&[1.0, 1.1, 1.2, 1.3, 1.4]);
        let slow = Summary::of(&[40.0, 41.0, 42.0, 43.0, 44.0]);
        assert!(fast.separable_from(slow));
    }
}
