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
    let mut wf = Waveform::new( 200e6,    // fs
                                10e6,     // start freq
                                20e6,     // stop freq
                                0.00001,  // duration
                                0.0,      // 0 start phase
                                ComplexType::Complex,
                                WindowingType::Hanning,
                                0.80);    // percent unwindowed

//    println!("{}", wf);
    wf.generate_waveform();
}
