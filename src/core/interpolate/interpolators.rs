use std::cmp::PartialEq;
use std::marker::PhantomData;
use crate::core::mutator::timestamp::TimeStamp;

/// Trait for interpolation strategies
///
/// This trait defines the interface for all interpolation behavior in Ferrocious.
/// Users can implement this trait to create custom interpolation modes beyond the
/// built-in variants.
///
/// # Design Philosophy
///
/// - **Built-in enum** (`Interpolator`): Clone, Debug, eventually serializable - for common cases
/// - **Custom trait impls**: Full flexibility for advanced users who implement this trait
///
/// # Example: Custom Interpolator
///
/// ```
/// use ferrocious::core::interpolate::Interpolate;
/// use ferrocious::core::mutator::timestamp::TimeStamp;
///
/// struct PerlinNoise {
///     seed: u64,
///     scale: f32,
/// }
///
/// impl Interpolate<f32> for PerlinNoise {
///     fn at(&self, time: &TimeStamp, fps: u8) -> f32 {
///         let t = time.as_num_frames(fps) as f32;
///         // Your custom interpolation logic here
///         (t * self.scale).sin()
///     }
/// }
///
/// // Use in entity:
/// // custom_property: Box<dyn Interpolate<f32>>
/// ```
pub trait Interpolate<T: Interpolatable> {
    /// Evaluate the interpolation at a given time
    ///
    /// # Arguments
    /// * `time` - The timestamp to evaluate at
    /// * `fps` - Frames per second (for time-to-frame conversion)
    ///
    /// # Returns
    /// The interpolated value at the given time
    fn at(&self, time: &TimeStamp, fps: u32) -> T;
}

/// Trait for types that can be interpolated between values
///
/// This is the core requirement for animated properties in Ferrocious.
/// Any type that implements this trait can be used with the Interpolator system.
///
/// Currently provides linear interpolation via `lerp()`, but the trait is designed to be
/// extensible for other interpolation modes (cubic, bezier, step, etc.) in the future.
///
/// # Example: Implementing for Custom Types
///
/// ```
/// use ferrocious::core::interpolate::Interpolatable;
///
/// #[derive(Clone)]
/// struct Transform {
///     position: [f32; 2],
///     rotation: f32,
///     scale: f32,
/// }
///
/// impl Interpolatable for Transform {
///     fn lerp(&self, other: &Self, t: f32) -> Self {
///         Transform {
///             position: self.position.lerp(&other.position, t),
///             rotation: self.rotation.lerp(&other.rotation, t),
///             scale: self.scale.lerp(&other.scale, t),
///         }
///     }
/// }
/// ```
pub trait Interpolatable: Clone {
    /// Linear interpolation between self and other
    ///
    /// # Arguments
    /// * `other` - The target value to interpolate towards
    /// * `t` - Progress value, typically in range [0.0, 1.0]
    ///   - t=0.0 returns self
    ///   - t=1.0 returns other
    ///   - t=0.5 returns the midpoint
    fn lerp(&self, other: &Self, t: f32) -> Self;
}

// Implement Interpolatable for common types
impl Interpolatable for f32 {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Interpolatable for [f32; 2] {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        [
            self[0].lerp(&other[0], t),
            self[1].lerp(&other[1], t),
        ]
    }
}

impl Interpolatable for [f32; 4] {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        [
            self[0].lerp(&other[0], t),
            self[1].lerp(&other[1], t),
            self[2].lerp(&other[2], t),
            self[3].lerp(&other[3], t),
        ]
    }
}

/// Easing functions for smooth interpolation
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl EasingFunction {
    /// Apply easing to a linear progress value [0, 1]
    pub fn ease(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => t * (2.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
        }
    }
}

struct NeedsTo;
struct NeedsTime;

struct InterpolatorBuilder<T: Interpolatable, State> {
    from: T,
    to_: Option<T>,
    easing: Option<EasingFunction>,
    control_points: Vec<T>,
    _state: PhantomData<State>,
}

impl<T: Interpolatable> InterpolatorBuilder<T, NeedsTo> {
    pub fn to(self, to_: T) -> InterpolatorBuilder<T, NeedsTime> {
        InterpolatorBuilder {
            from: self.from,
            to_: Some(to_),
            easing: None,
            control_points: Vec::new(),
            _state: PhantomData,
        }
    }
}

impl<T: Interpolatable> InterpolatorBuilder<T, NeedsTime> {
    pub fn ease(mut self, easing: EasingFunction) -> Self {
        self.easing = Some(easing);
        self
    }

    /// Add a control point to the Bezier curve
    /// Can be called multiple times to add more control points
    /// - 0 control points: Linear interpolation
    /// - 2 control points: Cubic Bezier (optimized)
    /// - n control points: General n+2 point Bezier
    pub fn through(mut self, control_point: T) -> Self {
        self.control_points.push(control_point);
        self
    }

    pub fn over(self, start: TimeStamp, end: TimeStamp) -> Interpolator<T> {
        let num_controls = self.control_points.len();

        if num_controls == 0 {
            // No control points: linear interpolation
            Interpolator::Linear {
                from: self.from,
                to: self.to_.unwrap(),
                start,
                end,
                easing: self.easing.unwrap_or(EasingFunction::Linear),
            }
        }
        else if num_controls == 2 {
            // Exactly 2 control points: use optimized Cubic variant
            let mut controls = self.control_points;
            let p2 = controls.pop().unwrap();
            let p1 = controls.pop().unwrap();
            Interpolator::Cubic {
                p0: self.from,
                p1,
                p2,
                p3: self.to_.unwrap(),
                start,
                end,
            }
        }
        else {
            // Any other number: use general Bezier
            let mut points = Vec::with_capacity(num_controls + 2);
            points.push(self.from);
            points.extend(self.control_points);
            points.push(self.to_.unwrap());
            Interpolator::Bezier { points, start, end }
        }
    }
}


/// Data-driven interpolator for any value over time
///
/// This enum represents animations as pure data (no function pointers or trait objects).
/// Works with any type that implements `Interpolatable`.
#[derive(Clone, Debug)]
pub enum Interpolator<T: Interpolatable> {
    /// Constant value (never changes)
    Constant(T),

    /// Linear interpolation between two values
    Linear {
        from: T,
        to: T,
        start: TimeStamp,
        end: TimeStamp,
        easing: EasingFunction,
    },

    /// Interpolation through multiple keyframes
    Keyframes {
        keyframes: Vec<(TimeStamp, T)>,
        easing: EasingFunction,
    },

    /// Cubic Bezier interpolation with two control points
    /// Provides smooth curves with more control than linear easing
    Cubic {
        p0: T,  // Start point
        p1: T,  // First control point
        p2: T,  // Second control point
        p3: T,  // End point
        start: TimeStamp,
        end: TimeStamp,
    },

    /// General n-point Bezier interpolation
    /// Supports any number of control points using De Casteljau's algorithm
    Bezier {
        points: Vec<T>,
        start: TimeStamp,
        end: TimeStamp,
    },
}

/// De Casteljau's algorithm for evaluating Bezier curves of any degree
/// Works with any Interpolatable type by using only lerp operations
fn de_casteljau<T: Interpolatable>(points: &[T], t: f32) -> T {
    debug_assert!(!points.is_empty(), "Bezier curve requires at least one point");

    if points.len() == 1 {
        return points[0].clone();
    }

    let mut working = points.to_vec();

    while working.len() > 1 {
        working = working.windows(2)
            .map(|w| w[0].lerp(&w[1], t))
            .collect();
    }

    working.remove(0)
}



impl<T: Interpolatable> Interpolator<T> {
    /// Evaluate the interpolator at a given time
    pub fn at(&self, time: &TimeStamp, fps: u32) -> T {
        match self {
            Interpolator::Constant(value) => value.clone(),

            Interpolator::Linear { from, to, start, end, easing } => {
                let progress = Self::compute_progress(time, start, end, fps);
                let eased = easing.ease(progress);
                from.lerp(to, eased)
            }

            Interpolator::Keyframes { keyframes, easing } => {
                if keyframes.is_empty() {
                    panic!("Keyframes interpolator must have at least one keyframe");
                }

                // Find the surrounding keyframes
                let mut prev_idx = 0;
                let mut next_idx = 0;

                for (i, (kf_time, _)) in keyframes.iter().enumerate() {
                    if kf_time <= time {
                        prev_idx = i;
                    }
                    if kf_time >= time && next_idx == 0 {
                        next_idx = i;
                        break;
                    }
                }

                // If we're at or past the last keyframe
                if next_idx == 0 || prev_idx == keyframes.len() - 1 {
                    return keyframes.last().unwrap().1.clone();
                }

                // If we're at or before the first keyframe
                if time <= &keyframes[0].0 {
                    return keyframes[0].1.clone();
                }

                // Interpolate between keyframes
                let (start_time, start_val) = &keyframes[prev_idx];
                let (end_time, end_val) = &keyframes[next_idx];

                let progress = Self::compute_progress(time, start_time, end_time, fps);
                let eased = easing.ease(progress);
                start_val.lerp(end_val, eased)
            }

            Interpolator::Cubic { p0, p1, p2, p3, start, end } => {
                let t = Self::compute_progress(time, start, end, fps);
                // De Casteljau's algorithm: recursive linear interpolation
                // This only uses lerp(), so it works for ANY Interpolatable type!

                // First level: lerp between adjacent control points
                let q0 = p0.lerp(p1, t);
                let q1 = p1.lerp(p2, t);
                let q2 = p2.lerp(p3, t);

                // Second level: lerp between the results
                let r0 = q0.lerp(&q1, t);
                let r1 = q1.lerp(&q2, t);

                // Final: lerp to get the point on the curve
                r0.lerp(&r1, t)
            }

            Interpolator::Bezier { points, start, end } => {
                let t = Self::compute_progress(time, start, end, fps);
                de_casteljau(points, t)
            }
        }
    }

    /// Compute linear progress [0, 1] between two timestamps
    fn compute_progress(current: &TimeStamp, start: &TimeStamp, end: &TimeStamp, fps: u32) -> f32 {
        if start == end {
            return 1.0;
        }

        let current_frames = current.as_num_frames(fps.into()) as f32;
        let start_frames = start.as_num_frames(fps.into()) as f32;
        let end_frames = end.as_num_frames(fps.into()) as f32;

        if current_frames <= start_frames {
            return 0.0;
        }
        if current_frames >= end_frames {
            return 1.0;
        }

        (current_frames - start_frames) / (end_frames - start_frames)
    }
}

// Helper constructors for common cases
impl<T: Interpolatable> Interpolator<T> {
    pub fn constant(value: T) -> Self {
        Interpolator::Constant(value)
    }

    pub fn linear(from: T, to: T, start: TimeStamp, end: TimeStamp) -> Self {
        Interpolator::Linear {
            from,
            to,
            start,
            end,
            easing: EasingFunction::Linear,
        }
    }

    pub fn ease_in_out(from: T, to: T, start: TimeStamp, end: TimeStamp) -> Self {
        Interpolator::Linear {
            from,
            to,
            start,
            end,
            easing: EasingFunction::EaseInOut,
        }
    }

    /// Create a cubic Bezier interpolation
    ///
    /// Provides smooth curves with explicit control points for fine-tuned animation.
    /// Uses De Casteljau's algorithm which only requires `lerp()`.
    ///
    /// # Arguments
    /// * `p0` - Start point
    /// * `p1` - First control point (influences curve near start)
    /// * `p2` - Second control point (influences curve near end)
    /// * `p3` - End point
    /// * `start` - Start time
    /// * `end` - End time
    ///
    /// # Example
    /// ```
    /// // Create an S-curve color transition
    /// use ferrocious::core::interpolate::Interpolator;
    /// use ferrocious::core::mutator::timestamp::TimeStamp;
    /// let color_anim = Interpolator::cubic(
    ///     [1.0, 0.0, 0.0, 1.0],  // Red
    ///     [1.0, 0.5, 0.0, 1.0],  // Control: Orange
    ///     [0.5, 0.0, 1.0, 1.0],  // Control: Purple
    ///     [0.0, 0.0, 1.0, 1.0],  // Blue
    ///     TimeStamp::new(0, 0, 0),
    ///     TimeStamp::new(0, 2, 0),
    /// );
    /// ```
    pub fn cubic(p0: T, p1: T, p2: T, p3: T, start: TimeStamp, end: TimeStamp) -> Self {
        Interpolator::Cubic {
            p0,
            p1,
            p2,
            p3,
            start,
            end,
        }
    }

    /// Create a general n-point Bezier interpolation
    ///
    /// For curves with more than 4 points (more than 2 control points).
    /// Uses De Casteljau's algorithm.
    pub fn bezier(points: Vec<T>, start: TimeStamp, end: TimeStamp) -> Self {
        Interpolator::Bezier { points, start, end }
    }

    pub fn from(value: T) -> InterpolatorBuilder<T, NeedsTo> {
        InterpolatorBuilder {
            from: value,
            to_: None,
            easing: None,
            control_points: Vec::new(),
            _state: PhantomData,
        }
    }
}

// Implement Interpolate trait for the built-in Interpolator enum
impl<T: Interpolatable> Interpolate<T> for Interpolator<T> {
    fn at(&self, time: &TimeStamp, fps: u32) -> T {
        // Delegate to the existing evaluate method
        Interpolator::at(self, time, fps)
    }
}

#[cfg(test)]
mod tests {
    use crate::ts;
    use super::*;

    #[test]
    fn test_lerp_f32() {
        assert_eq!(0.0_f32.lerp(&10.0, 0.0), 0.0);
        assert_eq!(0.0_f32.lerp(&10.0, 0.5), 5.0);
        assert_eq!(0.0_f32.lerp(&10.0, 1.0), 10.0);
    }

    #[test]
    fn test_lerp_array() {
        let a = [0.0, 0.0, 0.0, 1.0];
        let b = [1.0, 1.0, 1.0, 1.0];
        let mid = a.lerp(&b, 0.5);
        assert_eq!(mid, [0.5, 0.5, 0.5, 1.0]);
    }

    #[test]
    fn test_constant_interpolator() {
        let interp = Interpolator::constant([1.0, 0.0, 0.0, 1.0]);
        let result = interp.at(&ts!(5), 24);
        assert_eq!(result, [1.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_linear_interpolator() {
        let interp = Interpolator::linear(
            0.0_f32,
            10.0,
            ts!(0),
            ts!(1),
        );

        // At start
        assert_eq!(interp.at(&ts!(0), 24), 0.0);
        // At end
        assert_eq!(interp.at(&ts!(1), 24), 10.0);
        // At midpoint (0.5 seconds = 12 frames at 24fps)
        let mid = interp.at(&ts!(0, 12), 24);
        assert!((mid - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_cubic_interpolator() {
        // Create a cubic interpolation from 0 to 10 with control points
        let interp = Interpolator::cubic(
            0.0_f32,   // p0: start at 0
            3.0,       // p1: pull upward early
            7.0,       // p2: pull upward late
            10.0,      // p3: end at 10
            ts!(0),
            ts!(1),
        );

        // At start
        let start = interp.at(&ts!(0), 24);
        assert!((start - 0.0).abs() < 0.01);

        // At end
        let end = interp.at(&ts!(1), 24);
        assert!((end - 10.0).abs() < 0.01);

        // At midpoint - should follow the bezier curve, not be exactly 5.0
        let mid = interp.at(&ts!(0, 12), 24);
        // With control points at 3 and 7, midpoint should be around 5
        // but will be influenced by the curve
        assert!(mid > 4.0 && mid < 6.0, "Mid value was {}", mid);
    }

    #[test]
    fn test_cubic_works_with_arrays() {
        // Demonstrate that cubic works with ANY Interpolatable type (like colors!)
        let color_curve = Interpolator::cubic(
            [1.0, 0.0, 0.0, 1.0],  // Red
            [1.0, 0.5, 0.0, 1.0],  // Orange (control)
            [0.5, 0.0, 1.0, 1.0],  // Purple (control)
            [0.0, 0.0, 1.0, 1.0],  // Blue
            ts!(0),
            ts!(1),
        );

        let start_color = color_curve.at(&ts!(0), 24);
        assert_eq!(start_color, [1.0, 0.0, 0.0, 1.0]);

        let end_color = color_curve.at(&ts!(1), 24);  // at end, not start
        assert_eq!(end_color, [0.0, 0.0, 1.0, 1.0]);

        // Middle should be some blend influenced by the orange/purple controls
        let mid_color = color_curve.at(&ts!(0, 12), 24);
        // Should have some of all colors mixed
        assert!(mid_color[0] > 0.0); // Some red
        assert!(mid_color[2] > 0.0); // Some blue
    }

    #[test]
    fn test_custom_interpolator_via_trait() {
        // Demonstrate the trait-based approach for custom interpolators
        struct SineWave {
            amplitude: f32,
            frequency: f32,
        }

        impl Interpolate<f32> for SineWave {
            fn at(&self, time: &TimeStamp, fps: u32) -> f32 {
                let t = time.as_num_frames(fps) as f32 / fps as f32;
                self.amplitude * (t * self.frequency * 2.0 * std::f32::consts::PI).sin()
            }
        }

        let wave = SineWave {
            amplitude: 1.0,
            frequency: 1.0, // 1 Hz
        };

        // At t=0, sin(0) = 0
        assert!((wave.at(&ts!(0), 24) - 0.0).abs() < 0.01);

        // At t=0.25s, sin(π/2) ≈ 1
        let quarter = wave.at(&ts!(0, 6), 24); // 6 frames at 24fps = 0.25s
        assert!(quarter > 0.9 && quarter < 1.1);
    }

    #[test]
    fn test_both_approaches_are_compatible() {
        // Built-in enum approach (cloneable)
        let builtin = Interpolator::linear(
            0.0_f32,
            10.0,
            ts!(0),
            ts!(1),
        );

        // Custom trait approach (flexible)
        struct DoubleValue;
        impl Interpolate<f32> for DoubleValue {
            fn at(&self, _time: &TimeStamp, _fps: u32) -> f32 {
                42.0
            }
        }

        let custom = DoubleValue;

        // Both implement Interpolate trait
        let builtin_result = builtin.at(&ts!(0, 12), 24);  // midpoint
        let custom_result = custom.at(&ts!(0, 12), 24);

        assert!((builtin_result - 5.0).abs() < 0.01);
        assert_eq!(custom_result, 42.0);

        // Builtin can be cloned
        let _cloned = builtin.clone();
        // Custom doesn't need to be Clone - user's choice!
    }

    // Builder API tests
    #[test]
    fn test_builder_linear() {
        let interp = Interpolator::from(0.0_f32)
            .to(10.0)
            .over(ts!(0), TimeStamp::new(0, 1, 0));

        assert_eq!(interp.at(&ts!(0), 24), 0.0);
        assert_eq!(interp.at(&ts!(1), 24), 10.0);

        let mid = interp.at(&ts!(0, 12), 24);
        assert!((mid - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_builder_with_easing() {
        let interp = Interpolator::from(0.0_f32)
            .to(10.0)
            .ease(EasingFunction::EaseInOut)
            .over(ts!(0), ts!(1));

        // At midpoint with ease-in-out, value should still be around 5
        // but with different acceleration curve
        let mid = interp.at(&ts!(0, 12), 24);
        assert!(mid > 4.0 && mid < 6.0);

        // Endpoints should be exact
        assert_eq!(interp.at(&ts!(0), 24), 0.0);
        assert_eq!(interp.at(&ts!(1), 24), 10.0);
    }

    #[test]
    fn test_builder_cubic_through() {
        // Chained .through() calls for cubic (2 control points)
        let interp = Interpolator::from([1.0, 0.0, 0.0, 1.0])  // Red
            .to([0.0, 0.0, 1.0, 1.0])                          // Blue
            .through([1.0, 0.5, 0.0, 1.0])                     // Orange (control 1)
            .through([0.5, 0.0, 1.0, 1.0])                     // Purple (control 2)
            .over(ts!(0), ts!(1));

        // Endpoints should match
        let start = interp.at(&ts!(0), 24);
        assert_eq!(start, [1.0, 0.0, 0.0, 1.0]);

        let end = interp.at(&ts!(1), 24);
        assert_eq!(end, [0.0, 0.0, 1.0, 1.0]);

        // Middle should be influenced by control points
        let mid = interp.at(&ts!(0, 12), 24);
        assert!(mid[0] > 0.0); // Some red
        assert!(mid[2] > 0.0); // Some blue
    }

    #[test]
    fn test_builder_with_arrays() {
        // Verify builder works with different Interpolatable types
        let color_interp = Interpolator::from([0.0, 0.0, 0.0, 1.0])
            .to([1.0, 1.0, 1.0, 1.0])
            .over(ts!(0), ts!(2));

        let mid = color_interp.at(&TimeStamp::new(0, 1, 0), 24);
        assert!((mid[0] - 0.5).abs() < 0.01);
        assert!((mid[1] - 0.5).abs() < 0.01);
        assert!((mid[2] - 0.5).abs() < 0.01);
    }

    // Generalized Bezier tests
    #[test]
    fn test_bezier_quadratic_one_control_point() {
        // Quadratic Bezier (3 points total: start + 1 control + end)
        let interp = Interpolator::from(0.0_f32)
            .to(10.0)
            .through(5.0)  // single control point
            .over(ts!(0), ts!(1));

        // Should be a Bezier variant (not Cubic, since we have 1 control point)
        assert!(matches!(interp, Interpolator::Bezier { .. }));

        // Endpoints
        assert!((interp.at(&ts!(0), 24) - 0.0).abs() < 0.01);
        assert!((interp.at(&ts!(1), 24) - 10.0).abs() < 0.01);

        // Midpoint should be influenced by control
        let mid = interp.at(&ts!(0, 12), 24);
        assert!(mid > 4.0 && mid < 6.0);
    }

    #[test]
    fn test_bezier_quartic_three_control_points() {
        // Quartic Bezier (5 points total: start + 3 controls + end)
        let interp = Interpolator::from(0.0_f32)
            .to(10.0)
            .through(2.0)
            .through(5.0)
            .through(8.0)
            .over(ts!(0), ts!(1));

        // Should be a Bezier variant
        assert!(matches!(interp, Interpolator::Bezier { .. }));

        // Endpoints
        assert!((interp.at(&ts!(0), 24) - 0.0).abs() < 0.01);
        assert!((interp.at(&ts!(1), 24) - 10.0).abs() < 0.01);

        // Midpoint should be somewhere reasonable
        let mid = interp.at(&ts!(0, 12), 24);
        assert!(mid > 3.0 && mid < 7.0);
    }

    #[test]
    fn test_bezier_constructor() {
        // Direct constructor for n-point Bezier
        let points = vec![0.0_f32, 2.0, 5.0, 8.0, 10.0];  // 5 points
        let interp = Interpolator::bezier(points, ts!(0), ts!(1));

        assert!((interp.at(&ts!(0), 24) - 0.0).abs() < 0.01);
        assert!((interp.at(&ts!(1), 24) - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_de_casteljau_matches_cubic() {
        // Verify that Bezier with 4 points gives same results as optimized Cubic
        let p0 = [1.0, 0.0, 0.0, 1.0];
        let p1 = [1.0, 0.5, 0.0, 1.0];
        let p2 = [0.5, 0.0, 1.0, 1.0];
        let p3 = [0.0, 0.0, 1.0, 1.0];

        let cubic = Interpolator::cubic(p0, p1, p2, p3, ts!(0), ts!(1));
        let bezier = Interpolator::bezier(vec![p0, p1, p2, p3], ts!(0), ts!(1));

        // Should produce identical results at various times
        for frame in [0, 6, 12, 18, 24] {
            let t = ts!(0, frame);
            let cubic_val = cubic.at(&t, 24);
            let bezier_val = bezier.at(&t, 24);

            for i in 0..4 {
                assert!(
                    (cubic_val[i] - bezier_val[i]).abs() < 0.0001,
                    "Mismatch at frame {}, component {}: cubic={} bezier={}",
                    frame, i, cubic_val[i], bezier_val[i]
                );
            }
        }
    }

    #[test]
    fn test_builder_two_controls_uses_cubic() {
        // Verify that exactly 2 control points produces optimized Cubic variant
        let interp = Interpolator::from(0.0_f32)
            .to(10.0)
            .through(3.0)
            .through(7.0)
            .over(ts!(0), ts!(1));

        // Should be Cubic, not Bezier
        assert!(matches!(interp, Interpolator::Cubic { .. }));
    }
}
