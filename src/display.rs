#![allow(dead_code,unused_imports)]

use sdl::video;
use sdl::Rect;

pub struct Display {
    pub gfx: [[u8; 64]; 32],
    pub draw_flag: bool,
    screen: video::Surface
}

static SCALE: isize = 20;
static WIDTH: isize = 64;
static HEIGHT: isize = 32;

impl Display {
    pub fn new() -> Display {
        Display {
            gfx: [[0; 64]; 32],
            draw_flag: true,
            screen: video::set_video_mode(WIDTH*SCALE, HEIGHT*SCALE, 8,
                                          &[video::SurfaceFlag::HWSurface],
                                          &[video::VideoFlag::DoubleBuf]).unwrap()
        }
    }

}
