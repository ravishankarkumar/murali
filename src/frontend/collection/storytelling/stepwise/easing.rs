/// Thin wrappers around `crate::frontend::animation::Ease` so that the
/// stepwise module has named free functions matching the spec.
/// All logic lives in `Ease::eval`; nothing is duplicated here.
use crate::frontend::animation::Ease;

/// Smoothstep: 3t² - 2t³  (delegates to `Ease::InOutSmooth`)
pub fn ease_in_out(t: f32) -> f32 {
    Ease::InOutSmooth.eval(t)
}

/// Quadratic ease-in: t²  (delegates to `Ease::InQuad`)
pub fn ease_in(t: f32) -> f32 {
    Ease::InQuad.eval(t)
}

/// Quadratic ease-out: 1-(1-t)²  (delegates to `Ease::OutQuad`)
pub fn ease_out(t: f32) -> f32 {
    Ease::OutQuad.eval(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ease_in_out_boundaries() {
        assert_eq!(ease_in_out(0.0), 0.0);
        assert_eq!(ease_in_out(1.0), 1.0);
    }

    #[test]
    fn ease_in_boundaries() {
        assert_eq!(ease_in(0.0), 0.0);
        assert_eq!(ease_in(1.0), 1.0);
    }

    #[test]
    fn ease_out_boundaries() {
        assert_eq!(ease_out(0.0), 0.0);
        assert_eq!(ease_out(1.0), 1.0);
    }

    // Feature: stepwise-component, Property 6: Easing functions are range-preserving
    // Validates: Requirements 6.2
    mod property_tests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn prop_easing_range_preserving(t in 0.0f32..=1.0f32) {
                // Feature: stepwise-component, Property 6: Easing functions are range-preserving
                prop_assert!(ease_in_out(t) >= 0.0 && ease_in_out(t) <= 1.0);
                prop_assert!(ease_in(t)     >= 0.0 && ease_in(t)     <= 1.0);
                prop_assert!(ease_out(t)    >= 0.0 && ease_out(t)    <= 1.0);
            }
        }
    }
}
