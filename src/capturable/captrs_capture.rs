use crate::capturable::{Capturable, Recorder};
use captrs::Capturer;
use std::boxed::Box;
use std::error::Error;

#[derive(Clone)]
pub struct CaptrsCapturable {
    id: u8,
    width: u32,
    height: u32,
    offset_x: i32,
    offset_y: i32,
}

impl CaptrsCapturable {
    pub fn new(id: u8, width: u32, height: u32, offset_x: i32, offset_y: i32) -> CaptrsCapturable {
        CaptrsCapturable {
            id,
            width,
            height,
            offset_x,
            offset_y,
        }
    }
}

impl Capturable for CaptrsCapturable {
    fn name(&self) -> String {
        format!("Desktop {} (captrs)", self.id).into()
    }
    fn geometry_relative(&self) -> Result<(f64, f64, f64, f64), Box<dyn Error>> {
        Ok((0.0, 0.0, 1.0, 1.0))
    }
    fn before_input(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn recorder(&self, _capture_cursor: bool) -> Result<Box<dyn Recorder>, Box<dyn Error>> {
        Ok(Box::new(CaptrsRecorder::new(self.id)))
    }
    fn geometry(&self) -> Result<(u32, u32), Box<dyn Error>> {
        Ok((self.width, self.height))
    }
    fn geometry_offset(&self) -> Result<(i32, i32), Box<dyn Error>> {
        Ok((self.offset_x, self.offset_y))
    }
}

pub struct CaptrsRecorder {
    capturer: Capturer,
}

impl CaptrsRecorder {
    pub fn new(id: u8) -> CaptrsRecorder {
        CaptrsRecorder {
            capturer: Capturer::new(id.into()).unwrap(),
        }
    }
}

impl Recorder for CaptrsRecorder {
    fn capture(&mut self) -> Result<crate::video::PixelProvider, Box<dyn Error>> {
        let _ = self.capturer.capture_store_frame();
        let (w, h) = self.capturer.geometry();
        Ok(crate::video::PixelProvider::BGR0(
            w as usize,
            h as usize,
            unsafe { std::mem::transmute(self.capturer.get_stored_frame().unwrap()) },
        ))
    }
}