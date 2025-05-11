use ndarray::s;
use ndarray::{Array2, Array3, Shape};
use vulkano::device::QueueFlags;
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::VulkanLibrary;
use ferrocious::core::{Canvas, Entity, TimeStamp};
use ferrocious::core::vulkan;
use ferrocious::core::vulkan::initialize_vulkan;

fn main() {
    tracing_subscriber::fmt::init();
    let end = TimeStamp::new(0, 4, 0);
    let canvas: NewCanvas<BaseEntity> = NewCanvas(
    vec![BaseEntity::new(1024, 1024, TimeStamp::new(0, 0, 0), TimeStamp::new(0, 4, 0))], Array3::ones((1024, 1024, 3)));
    canvas.save("output", "thing.mp4", end)

}


#[derive(Clone)]
struct BaseEntity {
    width: u32,
    height: u32,
    entrance_frame: TimeStamp,
    exit_frame: TimeStamp,
    pixels: Array3<u8>
}

impl BaseEntity {
    fn new (width: u32, height: u32, entrance_frame: TimeStamp, exit_frame: TimeStamp) -> BaseEntity {
        BaseEntity {
            width,
            height,
            entrance_frame,
            exit_frame,
            pixels: Default::default(),
        }
    }
}

impl Entity for BaseEntity {
    fn render(&self, &active_frame: &TimeStamp, fps: u8) -> Array3<u8> {
        pixels
    }


    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn is_active_at(&self, &frame: &TimeStamp) -> bool {
        self.entrance_frame <= frame && frame <= self.exit_frame
    }

    fn upper_left_coords(&self) -> (u32, u32) {
        (0, 0)
    }

    fn tick(&mut self, frame: &TimeStamp) {

    }

}

struct NewCanvas<E: Entity>(Vec<E>, Array3<u8>);

impl<E> Canvas for NewCanvas<E> where E: Entity + Clone {
    fn construct(&self) {
        todo!()
    }

    fn get_width_and_height(&self) -> (u32, u32) {
        (1024, 1024)
    }

    fn get_fps(&self) -> u8 {
        24
    }

    fn get_entities(&self) -> Vec<&E> {
        self.0.clone()
    }

    fn get_background(&self, current_frame: &TimeStamp) -> Array3<u8> {
        let end_frame = (4u32 * fps as u32) as f64;
        self.pixels.slice_mut(s![.., .., 0]).fill((256f64 * (active_frame.as_num_frames(fps) as f64 / end_frame)) as u8);
        pixels.slice_mut(s![.., .., 1]).fill((256f64 * ((end_frame - active_frame.as_num_frames(fps) as f64) / end_frame)) as u8);
    }
}


