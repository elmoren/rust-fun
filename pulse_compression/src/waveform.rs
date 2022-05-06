use std::fmt;
use std::f64::consts::PI;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ComplexType {
    Real,
    Complex,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum WindowingType {
    None,
    Hanning,
    Hamming,
    Blackman,
}

#[derive(Clone)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

pub struct Waveform {
    sample_rate_hz: f64,
    start_freq_hz: f64,
    stop_freq_hz: f64,
    length_sec: f64,
    start_phase_deg: f32,
    ctype: ComplexType,
    windowing: WindowingType,
    pct_unwindowed: f32,
    samples: Vec<Float2>,
    num_samples: i32,
    num_windowed_samples: i32,
}

impl Waveform {
    pub fn new(
        sample_rate_hz: f64,
        start_freq_hz: f64,
        stop_freq_hz: f64,
        length_sec: f64,
        start_phase_deg: f32,
        ctype: ComplexType,
        windowing: WindowingType,
        pct_unwindowed: f32,
    ) -> Waveform {
        let mut wf = Waveform {
            sample_rate_hz: sample_rate_hz,
            start_freq_hz: start_freq_hz,
            stop_freq_hz: stop_freq_hz,
            length_sec: length_sec,
            start_phase_deg: start_phase_deg,
            ctype: ctype,
            windowing: windowing,
            pct_unwindowed: pct_unwindowed,
            samples: Vec::new(),
            num_samples: 0,
            num_windowed_samples: 0,
        };
        wf.update();
        wf
    }

    //
    // Updates the internal calculated fields when internal fields
    // are updated.
    //
    fn update(&mut self) {
        let num_samples = self.length_sec * self.sample_rate_hz;

        // Number of shaded samples per at start and  stop of wf.
        let num_windowed_samples = num_samples * (1.0 - self.pct_unwindowed as f64) / 2.0;

        self.num_samples = num_samples as i32;
        self.num_windowed_samples = num_windowed_samples as i32;
    }

    //
    // generate_waveform: Generates the waveform and stores in the 
    // samples vector.
    //
    pub fn generate_waveform(&mut self) {
        let phase_off_rad = self.start_phase_deg as f64 * PI / 180.0;

        for i in 0..self.num_samples {
            let time = (i as f64) / self.sample_rate_hz;
            let window_scale = self.get_window_scale(i);
            let bw = self.stop_freq_hz - self.start_freq_hz;
            let phase = phase_off_rad +
                time * self.start_freq_hz * 2.0 * PI +
                (time * time * bw * PI) / self.length_sec;

            let mut sample = Float2 { x: 0.0, y: 0.0 };
            sample.x = (f64::cos(phase) * window_scale as f64) as f32;

            match self.ctype {
                ComplexType::Real => sample.y = 0.0,
                ComplexType::Complex => sample.y =
                    (f64::sin(phase) * window_scale as f64) as f32
            };

            self.samples.push(sample);
        }
    }

    // Get window scale returns a value 0 to 1.0 to scale a sample
    // based on the selected windowing function and the giving
    // percent unwindowed
    fn get_window_scale(&self, sample: i32) -> f64 {
        if sample < self.num_windowed_samples ||
            sample > self.num_samples - self.num_windowed_samples {
            match self.windowing {
                WindowingType::None => 1.0,
                WindowingType::Hanning => self.calc_hanning(sample),
                WindowingType::Hamming => self.calc_hamming(sample),
                WindowingType::Blackman => self.calc_blackman(sample),
            }
        } else {
            1.0
        }
    }

    // Hanning Windowing
    fn calc_hanning(&self, sample: i32) -> f64 {
        let n = (sample + 1) as f64;
        let width = self.num_windowed_samples as f64 * 2.0;
        0.50 * (1.0 - f64::cos((n * 2.0 * PI) / (width - 1.0)))
    }

    // Hamming Windowing
    fn calc_hamming(&self, sample: i32) -> f64 {
        let n = (sample + 1) as f64;
        let width = self.num_windowed_samples as f64 * 2.0;
        0.54 - 0.46 * f64::cos((n * 2.0 * PI) / (width - 1.0))
    }

    // Blackman Windowing
    // approximating a0 = .42, a1 = 0.5 and a2 = 0.08
    fn calc_blackman(&self, sample: i32) -> f64 {
        let a0 = 0.42;
        let a1 = 0.5;
        let a2 = 0.08;
        let n = (sample + 1) as f64;
        let width = self.num_windowed_samples as f64 * 2.0;
        a0 - a1 * f64::cos((n * 2.0 * PI) / (width - 1.0)) +
            a2 * f64::cos((n * 4.0 * PI) / (width - 1.0))
    }

    pub fn samples(&self) -> Vec<Float2> {
        self.samples.clone()
    }
}

impl fmt::Display for Waveform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Waveform:\n")?;
        write!(f, "  Sample Rate Hz:  {}\n", self.sample_rate_hz)?;
        write!(f, "  Start Freq Hz:   {}\n", self.start_freq_hz)?;
        write!(f, "  Stop Freq Hz:    {}\n", self.stop_freq_hz)?;
        write!(f, "  Length Seconds:  {}\n", self.length_sec)?;
        write!(f, "  Total Samples:   {}\n", self.num_samples)?;
        write!(f, "  Start Phase Deg: {}\n", self.start_phase_deg)?;
        write!(f, "  Complex Type:    {:?}\n", self.ctype)?;
        write!(f, "  Windowing:       {:?}\n", self.windowing)?;
        write!(f, "  Windowed Samples:{}\n", self.num_windowed_samples)?;
        write!(f, "  Unwindowed Pct:  {}\n", self.pct_unwindowed)
    }
}
