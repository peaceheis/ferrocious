use std::fs;
use std::io::Write;
use std::path::Path;
use ndarray::s;
use subprocess::{Popen, PopenConfig, Redirection};
use crate::entity::Entity;
use crate::mutator::timestamp::TimeStamp;

pub trait Canvas {
    fn construct(&self);
    fn get_width_and_height(&self) -> (u32, u32);
    fn get_fps(&self) -> u32;
    fn get_entities(&self) -> Vec<impl Entity>;
    fn get_background(&self) -> ndarray::Array2<u32>;
    fn unmask(rgba: u32) -> [u8; 4] {
        [
            ((rgba & 0xFF000000) >> 24) as u8,
            ((rgba & 0x00FF0000) >> 24) as u8,
            ((rgba & 0x0000FF00) >> 24) as u8,
            ((rgba & 0x000000FF)>> 24) as u8,
        ]
    }


    fn launch_writing_subprocess(width: u32, height: u32, fps: u32, end_dir: &str, name: &str) -> Popen {

        let FFMPEG_BIN: &str =  if std::env::consts::OS == "windows" { "ffmpeg" } else { "ffmpeg.exe" };

        let command = [
            FFMPEG_BIN,
            "-y",  // overwrite output file if it exists
            "-f", "rawvideo",
            "-s", &(width.to_string() + "x" + height.to_string().as_str()),  // size of one frame
            "-pix_fmt", "rgba", //
            "-r", &fps.to_string(),  // frame rate
            "-i", "-",  // The input comes from a pipe
            "-an",  // Tells FFMPEG not to expect any audio
            "-loglevel", "error",
            "-vcodec", "libx264",
            &(end_dir.to_owned() + "/" + name)
        ];

        Popen::create(&command, PopenConfig {
            stdin: Redirection::Pipe,
            ..Default::default()
        }).expect("We should be able to make a pipe")
    }

    fn save(&self, end_dir: &str, name: &str, end: TimeStamp) {
        println!("Starting write");

        if !Path::new(end_dir).exists() {
            fs::create_dir_all(end_dir).expect("Should be able to make directory");
        }

        let FPS: u32 = self.get_fps();
        let (WIDTH, HEIGHT): (u32, u32) = self.get_width_and_height();

        let mut process = Self::launch_writing_subprocess(WIDTH, HEIGHT, self.get_fps(), end_dir, name);
        let mut current_frame = TimeStamp::new(None, None, None);

        while current_frame < end {
            let mut frame = self.get_background();
            println!("processing frame {}", current_frame);
            current_frame.increment();
            for entity in &mut self.get_entities() {
                if !entity.is_active_at(&current_frame) {
                    continue;
                }

                entity.tick(&current_frame);
                let (upper_left_x, upper_left_y)= entity.upper_left_coords();
                let (size_x, size_y) = entity.get_size();
                let entity_render = entity.render(&current_frame, FPS);
                let end_x: i32 = if WIDTH < upper_left_x + size_x {WIDTH as i32} else {size_x as i32};
                let end_y: i32 = if HEIGHT < upper_left_y + size_y {HEIGHT as i32} else {size_y as i32};
                frame.slice_mut(s![upper_left_x as i32..end_x, upper_left_y as i32..end_y]).assign(&entity_render.slice(s![..end_x, ..end_y]));

                let _ = &process.stdin.as_ref().expect("we should have stdin still").write(
                    &frame.iter().map(|&val| Self::unmask(val).into_iter()).flatten().collect::<Vec<u8>>()
                );

            }
        }

        let _ = process.stdin.as_ref().unwrap().sync_all();
        process.wait().unwrap();
        process.terminate().unwrap();
    }

}
