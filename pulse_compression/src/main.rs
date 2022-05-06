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
