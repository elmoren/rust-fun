//
// Simple Pulse Compression Demo
//
// I wanted to do a rust project that would help me learn
// rust more and work with signals bit. So this is what I 
// came up with, maybe I'll add doppler at some point. 
//

mod waveform;

use waveform::*;

fn main() {
    let mut wf = Waveform::new(
        200e6,
        10e6,
        20e6,
        0.00001,
        0.0,
        ComplexType::Complex,
        WindowingType::Hanning,
        0.80,
    );

    wf.generate_waveform();

    println!("{}", wf);
    for sample in wf.samples() {
        println!("{} {}", sample.x, sample.y);
    }
}
