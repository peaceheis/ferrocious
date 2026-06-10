use ferrocious::core::canvas::Canvas;
use ferrocious::core::entity::Entity;
use ferrocious::core::geometry::Transform;
use ferrocious::core::interpolate::Interpolator;
use ferrocious::core::mutator::timestamp::TimeStamp;
#[cfg(feature = "stl")]
use ferrocious::stl::entities::{Polygon, TrigLine};
#[cfg(feature = "stl")]
use ferrocious::stl::shaders::colors::*;
use ferrocious::ts;
use std::f32::consts::PI;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    tracing_subscriber::fmt::init();

    #[cfg(feature = "stl")]
    {
        let end = TimeStamp::new(0, 5, 0);
        let canvas = SineWaveCanvas::new();
        unsafe { canvas.save("output", "sine_wave.mp4", end) }
    }

    let elapsed = now.elapsed();
    println!("Generated : {:.2?}", elapsed);

    #[cfg(not(feature = "stl"))]
    {
        println!("This example requires the 'stl' feature.");
        println!("Run with: cargo run --features stl");
    }
}

#[cfg(feature = "stl")]
struct DemoCanvas {
    entities: Vec<Box<dyn Entity>>,
}

#[cfg(feature = "stl")]
impl DemoCanvas {
    fn new() -> Self {
        let mut entities = Vec::new();

        // STRESS TEST: Create a grid of animated polygons
        let grid_size = 20; // 20x20 = 400 polygons
        let polygon_size = 0.02;

        for row in 0..grid_size {
            for col in 0..grid_size {
                // Position in normalized coordinates (-1 to 1)
                let x = -0.9 + (col as f32 / grid_size as f32) * 1.8;
                let y = -0.9 + (row as f32 / grid_size as f32) * 1.8;

                // Stagger animation start times based on position
                let delay_frames = ((row + col) % 24) as u8;
                let start = TimeStamp::new(0, 0, delay_frames.into());
                let end = TimeStamp::new(0, 10, delay_frames.into());

                // Alternate between different shapes and color schemes
                let shape_type = (row + col) % 4;

                // Rotation direction based on position (creates nice wave pattern)
                let rotation_dir = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };
                let num_rotations = 2.0; // Full rotations during animation

                let polygon = match shape_type {
                    0 => {
                        // Triangles: red -> cyan, spinning
                        Polygon::from_corners_animated(
                            vec![
                                [x, y + polygon_size],
                                [x - polygon_size * 0.866, y - polygon_size * 0.5],
                                [x + polygon_size * 0.866, y - polygon_size * 0.5],
                            ],
                            Interpolator::ease_in_out(
                                [1.0, 0.2, 0.2, 0.75],
                                [0.2, 1.0, 1.0, 0.75],
                                start,
                                end,
                            ),
                        )
                        .with_transform(Interpolator::linear(
                            Transform::new().with_center([x, y]),
                            Transform::new()
                                .with_center([x, y])
                                .with_rotation(rotation_dir * PI * 2.0 * num_rotations),
                            start,
                            end,
                        ))
                    }
                    1 => {
                        // Squares: blue -> yellow, spinning + scaling
                        Polygon::from_corners_animated(
                            vec![
                                [x - polygon_size, y - polygon_size],
                                [x + polygon_size, y - polygon_size],
                                [x + polygon_size, y + polygon_size],
                                [x - polygon_size, y + polygon_size],
                            ],
                            Interpolator::ease_in_out(
                                [0.2, 0.2, 1.0, 0.75],
                                [1.0, 1.0, 0.2, 0.75],
                                start,
                                end,
                            ),
                        )
                        .with_transform(Interpolator::ease_in_out(
                            Transform::new().with_center([x, y]).with_uniform_scale(0.5),
                            Transform::new()
                                .with_center([x, y])
                                .with_rotation(rotation_dir * PI * 2.0)
                                .with_uniform_scale(1.2),
                            start,
                            end,
                        ))
                    }
                    2 => {
                        // Pentagons: green -> magenta, spinning
                        Polygon::regular_ngon_animated(
                            [x, y],
                            polygon_size,
                            5,
                            0.0,
                            Interpolator::ease_in_out(
                                [0.2, 1.0, 0.2, 0.75],
                                [1.0, 0.2, 1.0, 0.75],
                                start,
                                end,
                            ),
                        )
                        .with_transform(Interpolator::linear(
                            Transform::new().with_center([x, y]),
                            Transform::new()
                                .with_center([x, y])
                                .with_rotation(rotation_dir * PI * 2.0 * num_rotations),
                            start,
                            end,
                        ))
                    }
                    _ => {
                        // Hexagons: orange -> purple, spinning opposite + pulsing
                        Polygon::regular_ngon_animated(
                            [x, y],
                            polygon_size,
                            6,
                            0.0,
                            Interpolator::ease_in_out(
                                [1.0, 0.5, 0.0, 0.75],
                                [0.5, 0.0, 1.0, 0.75],
                                start,
                                end,
                            ),
                        )
                        .with_transform(Interpolator::ease_in_out(
                            Transform::new().with_center([x, y]).with_uniform_scale(1.0),
                            Transform::new()
                                .with_center([x, y])
                                .with_rotation(-rotation_dir * PI * 2.0 * num_rotations)
                                .with_uniform_scale(0.7),
                            start,
                            end,
                        ))
                    }
                };

                entities.push(Box::new(polygon) as Box<dyn Entity>);
            }
        }

        println!("Created {} polygons for stress test", entities.len());

        Self { entities }
    }
}

#[cfg(feature = "stl")]
struct PrideCanvas {
    entities: Vec<Box<dyn Entity>>,
}

#[cfg(feature = "stl")]
impl PrideCanvas {
    fn new() -> Self {
        let colors = vec![RED, ORANGE, YELLOW];
        let mut entities = colors
            .iter()
            .enumerate()
            .map(|(i, iter)| {
                Box::new(Polygon::rectangle(
                    [-1.0, (i as f32 + 1.0) / 3f32 - (1.0 / 6.0) - 1.0],
                    2.0,
                    1.0 / 3.0,
                    iter.clone(),
                )
                .with_transform(Interpolator::linear(
                    Transform::new()
                        .with_position([-1.0, (i as f32 + 1.0) / 3f32 - (1.0 / 6.0) - 1.0]),
                    Transform::new()
                        .with_position([1.0, (i as f32 + 1.0) / 3f32 - (1.0 / 6.0) - 1.0]),
                    TimeStamp::new(0, ((i / 6) * 5) as u32, 0),
                    TimeStamp::new(0, 10, 0),
                ))) as Box<dyn Entity>
            })
            .collect::<Vec<_>>();
        Self { entities }
    }
}

#[cfg(feature = "stl")]
impl Canvas for PrideCanvas {
    fn construct(&self) {
        // No longer needed - entities are defined in new()
    }

    fn get_width_and_height(&self) -> (u32, u32) {
        (640, 640)
    }

    fn get_fps(&self) -> u32 {
        60
    }

    fn get_entities(&self) -> &Vec<Box<dyn Entity>> {
        &self.entities
    }

    fn get_background_color(&self, _current_frame: &TimeStamp) -> [u8; 4] {
        [1u8, 1u8, 1u8, 1u8]
    }
}

#[cfg(feature = "stl")]
impl Canvas for DemoCanvas {
    fn construct(&self) {
        // No longer needed - entities are defined in new()
    }

    fn get_width_and_height(&self) -> (u32, u32) {
        (1080, 1380)
    }

    fn get_fps(&self) -> u32 {
        48
    }

    fn get_entities(&self) -> &Vec<Box<dyn Entity>> {
        &self.entities
    }

    fn get_background_color(&self, _current_frame: &TimeStamp) -> [u8; 4] {
        [0u8, 0u8, 0u8, 0u8] // Dark blue-gray background
    }
}

#[cfg(feature = "stl")]
struct SineWaveCanvas {
    entities: Vec<Box<dyn Entity>>,
}

#[cfg(feature = "stl")]
impl SineWaveCanvas {
    fn new() -> Self {
        // Create a simple sine wave
        let sine_wave = TrigLine::sin()
            .start((-2.0 * PI).into())
            .end((16.0 * PI).into())
            .amplitude(0.5.into())
            .start_point(Interpolator::Cubic {
                p0: [0., 0.],
                p1: [-0.1, 0.],
                p2: [-0.3, 0.],
                p3: [0., 0.],
                start: ts!(0),
                end: ts!(5),
            })
            .frequency(Interpolator::Cubic {
                p0: 5.,
                p1: 8.,
                p2: 10.,
                p3: 5.,
                start: ts!(0),
                end: ts!(5),
            })
            .phase(Interpolator::Cubic {
                p0: 0.,
                p1: 5. * PI,
                p2: 7. * PI,
                p3: 8. * PI,
                start: ts!(0),
                end: ts!(5),
            })
            .thickness(0.01.into())
            .build();
        
        let entities = vec![Box::new(sine_wave) as Box<dyn Entity>];

        Self { entities }
    }
}

#[cfg(feature = "stl")]
impl Canvas for SineWaveCanvas {
    fn construct(&self) {}

    fn get_width_and_height(&self) -> (u32, u32) {
        (1280, 720)
    }

    fn get_fps(&self) -> u32 {
        24
    }

    fn get_entities(&self) -> &Vec<Box<dyn Entity>> {
        &self.entities
    }

    fn get_background_color(&self, _current_frame: &TimeStamp) -> [u8; 4] {
        [20, 20, 30, 255] // Dark blue background
    }
}
