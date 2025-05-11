use std::env::current_dir;
use std::fs;
use std::fs::create_dir_all;
use std::io::{Read, Write};
use std::path::Path;
use ndarray::{Array3, s};
use subprocess::{Popen, PopenConfig, Redirection};
use crate::core::mutator::timestamp::TimeStamp;
use crate::core::entity::Entity;
pub use ndarray::Array2;
use video_rs::encode::Settings;
use video_rs::{Encoder, Frame, Options, Time};
use video_rs::ffmpeg::color::Space::RGB;
use video_rs::ffmpeg::format::Pixel::{RGB24, RGBA, YUV420P};

pub trait Canvas {
    fn construct(&self);
    fn get_width_and_height(&self) -> (u32, u32);
    fn get_fps(&self) -> u8;
    fn get_entities(&self) -> Vec<&impl Entity>;
    fn get_background(&self, current_frame: &TimeStamp) -> Array3<u8>;
    fn unmask(rgba: u32) -> [u8; 4] {
        [
            ((rgba & 0xFF000000) >> 24) as u8,
            ((rgba & 0x00FF0000) >> 24) as u8,
            ((rgba & 0x0000FF00) >> 24) as u8,
            ((rgba & 0x000000FF) >> 24) as u8,
        ]
    }

    fn save(&self, end_dir: &str, name: &str, end: TimeStamp) {
        println!("Starting write");

        let FPS: u8 = self.get_fps();
        let duration: Time = Time::from_nth_of_a_second(FPS as usize);
        let mut position = Time::zero();
        let (WIDTH, HEIGHT): (u32, u32) = self.get_width_and_height();

        let dir: &str = &format!("{}/{}", current_dir().expect("couldn't get current directory").display(), end_dir);
        create_dir_all(dir).expect("Couldn't make directory");
        let path: &str = &format!("{}/{}", dir, name);
        video_rs::init().unwrap();
        let settings = Settings::preset_h264_custom(WIDTH as usize, HEIGHT as usize, YUV420P, Options::default());
        let mut encoder = Encoder::new(Path::new(path), settings).expect("failed to create encoder");

        let mut current_frame = TimeStamp::new_with_defaults(None, None, None);

        while current_frame < end {
            let mut frame = self.get_background(&current_frame);
            current_frame.increment();
            println!("processing frame {}", current_frame);
            for mut entity in self.get_entities() {
                if !entity.is_active_at(&current_frame) {
                    continue;
                }


                entity.tick(&current_frame);
                // TODO this casting nonsense is kinda ridiculous, is there a better way?
                let (upper_left_x_u32, upper_left_y_u32) = entity.upper_left_coords();
                let (upper_left_x, upper_left_y) = (upper_left_x_u32 as i32, upper_left_y_u32 as i32);
                let (size_x_u32, size_y_u32) = entity.get_size();
                let (size_x, size_y) = (size_x_u32 as i32, size_y_u32 as i32);
                let entity_render = entity.render(&current_frame, FPS);
                let end_x: i32 = if { WIDTH as i32 } < upper_left_x + size_x { WIDTH as i32 } else { size_x as i32 };
                let end_y: i32 = if { HEIGHT as i32} < upper_left_y + size_y { HEIGHT as i32 } else { size_y as i32 };
                println!("upper left x {}, upper left y {}, end_x {}, end_y {}, width {}, height {}", upper_left_x, upper_left_y, end_x, end_y, WIDTH, HEIGHT);
                frame.slice_mut(s![upper_left_x..{end_x + upper_left_x}, upper_left_y..{upper_left_y+end_y}, ..]).assign(&entity_render.slice(s![..end_x, ..end_y, ..]));
            }

            let last: Frame = frame.clone() as Frame;
            encoder
                .encode(&last, position)
                .expect("failed to encode frame");

            // Update the current position and add the inter-frame duration to it.
            position = position.aligned_with(duration).add();

        }
        encoder.finish().expect("failed to finish encoder");
    }
}


