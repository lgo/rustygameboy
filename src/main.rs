extern crate sdl;
extern crate getopts;
extern crate time;

use getopts::Options;

use sdl::event::Event;

use std::env;
use std::time::Duration;
use std::thread;

use cpu::Cpu as RustyChip;

mod cpu;
mod opcode;
mod util;
mod display;
mod input;
mod loader;

static MS_PER_CYCLE: u64 = 1;
static MS_TO_NS: u64 = 1000000;

fn parse_arguments() -> String {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("d", "debug", "print debug information");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("d") {
        unsafe {
            util::DEBUG_MODE = true;
        }
    }
    if !matches.free.is_empty() {
        return matches.free[0].clone();
    }
    else {
        return String::from("pong");
    }
}

fn main() {

    let game_name: String = parse_arguments();

    let mut chip = RustyChip::new();

    loader::load_game(&mut chip, game_name);

    sdl::init(&[sdl::InitFlag::Video, sdl::InitFlag::Audio, sdl::InitFlag::Timer]);

    'main : loop {
        let start_time = time::precise_time_ns();

        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit                  => break 'main,
                Event::None                  => break 'event,
                Event::Key(key, state, _, _) => chip.input.press(key, state),
                _                            => {}
            }
        }

        let instr = chip.fetch_opcode();
        let cycles_executed = chip.execute_opcode(&instr);

        let execution_time = time::precise_time_ns() - start_time;
        let wait_time = MS_PER_CYCLE * cycles_executed * MS_TO_NS - execution_time;
        if wait_time > 0 {
            thread::sleep(Duration::from_millis(wait_time / MS_TO_NS))
        }
    }

    sdl::quit();
}
