use super::goal::TimeFilter;

/// Enum representing Timing Scenario for the provided
/// timing range (after_time and before_time)
#[derive(PartialEq, Debug)]
pub enum TimingScenario {
    /// Unbounded timing scenario where neither `after_time` nor `before_time` is defined
    Unbounded,
    /// Bounded timing scenario where both `after_time` and `before_time` are defined,
    /// and `after_time` is less than or equal to `before_time`
    Bounded,
    /// Timing scenario where only `after_time` is defined and `before_time` is `None`
    AfterOnly,
    /// Timing scenario where only `before_time` is defined and `after_time` is `None`
    BeforeOnly,
    /// Timing scenario where `after_time` is greater than `before_time`, indicating a time range that wraps around midnight
    Overflow,
}

impl TimeFilter {
    /// Determines the timing scenario based on the `TimeFilter.after_time` and `TimeFilter.before_time` inputs.
    /// - Returns a `TimingScenario` variant that represents the corresponding timing scenario.
    pub fn determine_timing_scenario(&self) -> TimingScenario {
        match (self.before_time, self.after_time) {
            (None, None) => TimingScenario::Unbounded,
            (None, Some(_)) => TimingScenario::AfterOnly,
            (Some(_), None) => TimingScenario::BeforeOnly,
            (Some(before), Some(after)) if before >= after => TimingScenario::Bounded,
            (Some(before), Some(after)) if before <  after => TimingScenario::Overflow,
            _ => TimingScenario::Unbounded,
        }
    }
}

#[cfg(test)]
mod tests {
    mod timing_scenario {
        use crate::models::{goal::TimeFilter, utils::TimingScenario};

        /// Test the scenario where both `after_time` and `before_time` are `None`,
        /// which should result in the `Unbounded` variant
        #[test]
        pub(crate) fn test_unbounded() {
            let filter = TimeFilter {
                after_time: None,
                before_time: None,
                on_days: None,
                not_on: None,
            };
            let scenario = filter.determine_timing_scenario();
            assert_eq!(scenario, TimingScenario::Unbounded);
        }

        /// Test the scenario where only `after_time` is defined,
        /// which should result in the `AfterOnly` variant
        #[test]
        pub(crate) fn test_after_only() {
            let filter = TimeFilter {
                after_time: Some(10),
                before_time: None,
                on_days: None,
                not_on: None,
            };
            let scenario = filter.determine_timing_scenario();
            assert_eq!(scenario, TimingScenario::AfterOnly);
        }

        /// Test the scenario where only `before_time` is defined,
        /// which should result in the `BeforeOnly` variant
        #[test]
        pub(crate) fn test_before_only() {
            let filter = TimeFilter {
                after_time: None,
                before_time: Some(20),
                on_days: None,
                not_on: None,
            };
            let scenario = filter.determine_timing_scenario();
            assert_eq!(scenario, TimingScenario::BeforeOnly);
        }

        /// Test the scenario where both `after_time` and `before_time` are defined and
        /// `after_time` is less than `before_time`, which should result in
        /// the `Bounded` variant
        #[test]
        pub(crate) fn test_bounded() {
            let filter = TimeFilter {
                after_time: Some(10),
                before_time: Some(20),
                on_days: None,
                not_on: None,
            };
            let scenario = filter.determine_timing_scenario();
            assert_eq!(scenario, TimingScenario::Bounded);
        }

        /// Test the scenario where both `after_time` and `before_time` are defined and
        /// `after_time` is equal to `before_time`, which should result in
        /// the `Bounded` variant
        #[test]
        pub(crate) fn test_bounded_and_both_are_equal() {
            let filter = TimeFilter {
                after_time: Some(10),
                before_time: Some(10),
                on_days: None,
                not_on: None,
            };
            let scenario = filter.determine_timing_scenario();
            assert_eq!(scenario, TimingScenario::Bounded);
        }

        /// Test the scenario where both `after_time` and `before_time` are defined and
        /// `after_time` is greater than `before_time`, which should result in the
        /// `Overflow` variant
        #[test]
        pub(crate) fn test_overflow() {
            let filter = TimeFilter {
                after_time: Some(20),
                before_time: Some(10),
                on_days: None,
                not_on: None,
            };
            let scenario = filter.determine_timing_scenario();
            assert_eq!(scenario, TimingScenario::Overflow);
        }
    }
}
