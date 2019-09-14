//! Records a WAV file (roughly 3 seconds long) using the default input device and format.
//!
//! The input data is recorded to "$CARGO_MANIFEST_DIR/recorded.wav".

extern crate cpal;
extern crate failure;

use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

fn main() -> Result<(), failure::Error> {
    // Use the default host for working with audio devices.
    let host = cpal::default_host();

    // Setup the default input device and stream with the default input format.
    let device = host.default_input_device().expect("Failed to get default input device");
    println!("Default input device: {}", device.name()?);
    let format = device.default_input_format().expect("Failed to get default input format");
    println!("Default input format: {:?}", format);
    let event_loop = host.event_loop();
    let stream_id = event_loop.build_input_stream(&device, &format)?;
    event_loop.play_stream(stream_id)?;

    event_loop.run(move |id, event| {
        let data = match event {
            Ok(data) => data,
            Err(err) => {
                eprintln!("an error occurred on stream {:?}: {}", id, err);
                return;
            }
        };
        match data {
            cpal::StreamData::Input { buffer: cpal::UnknownTypeInputBuffer::U16(buffer) } => {
                println!("buffer u16 len={}", buffer.len());
                for sample in buffer.iter() {
                    if *sample != 0 {
                        println!("non-zero sample {}", *sample);
                    }
                }
            },
            cpal::StreamData::Input { buffer: cpal::UnknownTypeInputBuffer::I16(buffer) } => {
                println!("buffer i16 len={}", buffer.len());
                for sample in buffer.iter() {
                    if *sample != 0 {
                        println!("non-zero sample {}", *sample);
                    }
                }
            },
            cpal::StreamData::Input { buffer: cpal::UnknownTypeInputBuffer::F32(buffer) } => {
                println!("buffer f32 len={}", buffer.len());
                for sample in buffer.iter() {
                    if *sample != 0.0 {
                        println!("non-zero sample {}", *sample);
                    }
                }
            },
            _ => panic!("unrecognized stream data"),
        }
    });
}
