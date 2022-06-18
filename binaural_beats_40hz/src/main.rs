//! # Binaural beats 40 Hz generator
//!
//! `binaural_beats_40hz` is a console program to generate a WAV stereo file with 30 minutes,
//!  were the left channel is a sine wave at 420 Hz and the right channel is a sine wave at 460 Hz.
//! 
//! ## Info
//! Author:  Joao Nuno Carvalho <br>
//! Date:    2022.06.18         <br>
//! License: MIT Open Source    <br>
//! 
//!
//! ## To compile the final program inside cargo
//! cargo build --release
//! 
//! ## To execute the program directly do
//! Usage: "binaural_beats_40hz wav_target_filename.wav" <br>
//! <br>
//! ex: binaural_beats_40hz /dev/shm/binaural_beats_40hz.wav <br>
//! 
//! ## To execute the program inside cargo
//! cargo run --release /dev/shm/binaural_beats_40hz.wav
//! 
//! ## To generate the docs inside cargo
//! cargo doc <br>
//! cargo doc --open <br>

use std::env;
use std::process;
use hound;
use hound::Error;
use std::f64::consts::PI;

const SAMPLE_RATE: usize = 44100;
const SIGNAL_DURATION_SEC: usize = 60*30; // seconds * minutes
const LEFT_CHANNEL_FREQ: f64 = 420.0; // Hz
const RIGHT_CHANNEL_FREQ: f64 = 460.0; // Hz
const STARTING_PHASE: f64 = 0.0; // radians
const AMPLITUDE: f64 = 0.2; 

/// Size of the buffers that will be generated each time.
const _SINGLE_CHANNEL_BUF_SIZE: usize = SAMPLE_RATE * SIGNAL_DURATION_SEC;
const _INTERLEAVED_STEREO_CHANNEL_BUF_SIZE: usize = _SINGLE_CHANNEL_BUF_SIZE * 2; 

/// Usage: "binaural_beats_40hz wav_target_filename.wav"
static USAGE: &str = "   Usage: \"binaural_beats_40hz wav_target_filename.wav\" ";

fn main() {
    println!("Binaural beats 40 Hz generator...");
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(& args);
    write_wav_to_file(&cfg);
    println!("...ended generating WAV file.");
}

/// Configuration structure to parse the command line options.  
struct Config {
    filename: String,
}

impl Config {
    /// Constructor - Is were the parsing is made.
    /// It exists if an error occurs.
    fn new(args: &[String]) -> Config {
        if args.len() != 2 {
            println!(" Invalid number of parameters...");
            println!("{}", USAGE);
            process::exit(1)
        }
        if !args[1].ends_with(".wav") {
            println!(" Invalid filename extension...");
            println!("{}", USAGE);
            process::exit(1)
        }
        let filename = args[1].clone();
        Config { filename }
    }
}

/// Function that writes to file the WAV buffers from left and right channel.
fn write_wav_to_file(config: &Config) {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(&config.filename, spec).unwrap_or_else(|err| {
        println!(" Error opening file for writing...\n {}", err);
        println!("{}", USAGE);
        process::exit(1);
    });
    
    let left_buf = generate_sine(SAMPLE_RATE, SIGNAL_DURATION_SEC as f64, LEFT_CHANNEL_FREQ, STARTING_PHASE, AMPLITUDE);

    let right_buf = generate_sine(SAMPLE_RATE, SIGNAL_DURATION_SEC as f64, RIGHT_CHANNEL_FREQ, STARTING_PHASE, AMPLITUDE);
    
    assert_eq!(left_buf.len(), right_buf.len());

    let closure_on_error =  |err: Error| {
        println!(" Error writing to file ...\n {}", err);
        println!("{}", USAGE);
        process::exit(1);
    };

    let max_i16: f32 = f32::powf(2.0, 15.0);

    for i in 0..left_buf.len() {
        let left_sample = (left_buf[i] * max_i16) as i16;
        let right_sample = (right_buf[i] * max_i16) as i16;
        writer.write_sample(left_sample as i16).unwrap_or_else(closure_on_error);
        writer.write_sample(right_sample as i16).unwrap_or_else(closure_on_error);
    }
}

fn generate_sine(sample_rate: usize, signal_duration: f64, freq: f64, phase: f64, amplitude: f64) -> Vec<f32> {
    let _start = 0.0;
    let _stop = signal_duration;
    let step = 1.0 / sample_rate as f64;
    let num_steps = (signal_duration / step) as usize;
    let mut sample_buf: Vec<f32> = Vec::new();
    for i in 0..num_steps {
        let t: f64 = (i as f64)*step;
        let real_sample: f64 = amplitude*f64::sin(phase + 2.0_f64*PI*freq*t);
        sample_buf.push(real_sample as f32);
    }
    sample_buf
}
