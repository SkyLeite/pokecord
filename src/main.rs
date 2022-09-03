use jpeg_encoder::ColorType;
use rustboyadvance_core;
use rustboyadvance_core::prelude::GamepakBuilder;
use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufWriter};
use std::path::Path;
use std::rc::Rc;
use std::thread::sleep;

pub struct Interface {}

impl rustboyadvance_core::VideoInterface for Interface {
    fn render(&mut self, buffer: &[u32]) {
        println!("Rendered!");
        let path = Path::new("/mnt/hdd/projects/pokecord/frame.png");
        let file = File::create(path).unwrap();

        let ref mut w = BufWriter::new(file);
        let mut encoder = jpeg_encoder::Encoder::new_file(path.to_str().unwrap(), 100).unwrap();
        let mut frame: Vec<u8> = vec![0; 240 * 160 * 4];

        for i in 0..buffer.len() {
            let color = buffer[i];
            frame[4 * i + 0] = ((color >> 16) & 0xff) as u8;
            frame[4 * i + 1] = ((color >> 8) & 0xff) as u8;
            frame[4 * i + 2] = (color & 0xff) as u8;
            frame[4 * i + 3] = 255;
        }

        let mut encoder = png::Encoder::new(w, 240, 160); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&frame).unwrap(); // Save
    }
}

impl rustboyadvance_core::AudioInterface for Interface {
    fn get_sample_rate(&self) -> i32 {
        44100
    }

    fn push_sample(&mut self, _samples: &[i16]) {}
}

impl rustboyadvance_core::InputInterface for Interface {
    fn poll(&mut self) -> u16 {
        rustboyadvance_core::keypad::KEYINPUT_ALL_RELEASED
    }
}

fn main() {
    let bios_path = Path::new("/mnt/hdd/projects/pokecord/gba_bios.bin");
    let rom_path = Path::new("/mnt/hdd/projects/pokecord/pokemon-leaf-green.gba");
    let bios = std::fs::read(&bios_path).unwrap();
    let rom = std::fs::read(&rom_path).unwrap();
    let gamepak = GamepakBuilder::new()
        .take_buffer(rom.to_vec().into_boxed_slice())
        .without_backup_to_file()
        .build()
        .unwrap();
    let interface = Rc::new(RefCell::new(Interface {}));

    let mut gba = rustboyadvance_core::gba::GameBoyAdvance::new(
        bios.to_vec().into_boxed_slice(),
        gamepak,
        interface.clone(),
        interface.clone(),
        interface.clone(),
    );

    &gba.skip_bios();
    let _ = &gba.frame();
    println!("First frame rendered!");
    sleep(std::time::Duration::from_millis(16));
    let _ = &gba.frame();
}
