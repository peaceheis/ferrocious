use ferrocious::core::canvas::Canvas;
use ferrocious::core::entity::Entity;
use ferrocious::core::geometry::Transform;
use ferrocious::core::interpolate::{EasingFunction, Interpolatable, Interpolator};
use ferrocious::core::mutator::timestamp::TimeStamp;
#[cfg(feature = "stl")]
use ferrocious::stl::entities::{Polygon, TrigLine};
#[cfg(feature = "stl")]
use ferrocious::stl::shaders::colors::*;
use ferrocious::ts;
use std::f32::consts::PI;
use std::time::Instant;
use ferrocious::stl::entities::lines::LinearLine;

fn main() {
    let now = Instant::now();
    tracing_subscriber::fmt::init();

    #[cfg(feature = "stl")]
    {
        let end = TimeStamp::new(0, 5, 0);
        let canvas = TestCanvas::new();
        unsafe { canvas.save("output", "line.mp4", end) }
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
struct TestCanvas {
    entities: Vec<Box<dyn Entity>>,
}

#[cfg(feature = "stl")]
impl TestCanvas {
    fn new() -> Self {
        let mut entities = Vec::new();
        let line = LinearLine::new(
            Interpolator::linear([-1., -1.], [-1., 0.], ts!(0),ts!(5)),
            Interpolator::linear([0., 0.], [1., 1.], ts!(0),ts!(5)),
            Option::from(Interpolator::linear(5., 20., ts!(0, 0), ts!(5, 0))),
            Option::from(Interpolator::linear(PURPLE, WHITE, ts!(0, 0), ts!(5, 0))),);

        entities.push(Box::new(line) as Box<dyn Entity>);


        TestCanvas { entities }
    }
}

impl Canvas for TestCanvas {
    fn construct(&self) {

    }

    fn get_width_and_height(&self) -> (u32, u32) {
        (500u32, 700u32)
    }

    fn get_fps(&self) -> u32 {
        24
    }

    fn get_entities(&self) -> &Vec<Box<dyn Entity>> {
        &self.entities
    }

    fn get_background_color(&self, current_frame: &TimeStamp) -> [u8; 4] {
        [0, 0, 0, 255]
    }
}