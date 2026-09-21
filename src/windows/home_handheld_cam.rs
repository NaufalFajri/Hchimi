use std::f32::consts::PI;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default)]
pub struct HandheldCamConfig {
    pub intensity_x: f32,
    pub intensity_y: f32,
    pub intensity_z: f32,
    pub speed: f32,
    pub enabled: bool,
}

impl Default for HandheldCamConfig {
    fn default() -> Self {
        Self {
            intensity_x: 0.04,
            intensity_y: 0.02,
            intensity_z: 0.04,
            speed: 1.0,
            enabled: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OctaveSet {
    pub freq: [f32; 4],
    pub phase: [f32; 4],
    pub amp: [f32; 4],
}

#[derive(Debug)]
pub struct HomeHandheldCam {
    pub config: HandheldCamConfig,
    pub time: f32,
    pub octaves: [OctaveSet; 3],
}

impl Default for HomeHandheldCam {
    fn default() -> Self {
        Self::new()
    }
}

impl HomeHandheldCam {
    pub fn new() -> Self {
        let base_freq = [0.317, 0.271, 0.389];
        let phase_off = [0.0, 2.0 * PI / 3.0, 4.0 * PI / 3.0];
        
        let mut octaves = [
            OctaveSet { freq: [0.0; 4], phase: [0.0; 4], amp: [0.0; 4] },
            OctaveSet { freq: [0.0; 4], phase: [0.0; 4], amp: [0.0; 4] },
            OctaveSet { freq: [0.0; 4], phase: [0.0; 4], amp: [0.0; 4] },
        ];

        for axis in 0..3 {
            let bf = base_freq[axis];
            let po = phase_off[axis];

            octaves[axis].freq[0] = bf * 1.000;
            octaves[axis].freq[1] = bf * 1.923;
            octaves[axis].freq[2] = bf * 3.731;
            octaves[axis].freq[3] = bf * 7.417;

            octaves[axis].phase[0] = po + 0.000;
            octaves[axis].phase[1] = po + 1.234;
            octaves[axis].phase[2] = po + 2.718;
            octaves[axis].phase[3] = po + 0.577;

            octaves[axis].amp[0] = 1.000;
            octaves[axis].amp[1] = 0.500;
            octaves[axis].amp[2] = 0.250;
            octaves[axis].amp[3] = 0.125;
        }

        Self {
            config: HandheldCamConfig::default(),
            time: 0.0,
            octaves,
        }
    }

    pub fn sample_noise(&self, axis: usize) -> f32 {
        const TOTAL_AMP: f32 = 1.875; // 1 + 0.5 + 0.25 + 0.125
        let mut val = 0.0;

        for o in 0..4 {
            let angle = self.octaves[axis].freq[o] * self.time + self.octaves[axis].phase[o];
            val += angle.sin() * self.octaves[axis].amp[o];
        }

        val / TOTAL_AMP
    }

    pub fn tick(&mut self, dt: f32) -> (f32, f32, f32) {
        if !self.config.enabled {
            return (0.0, 0.0, 0.0);
        }

        self.time += dt * self.config.speed;

        let off_x = self.sample_noise(0) * self.config.intensity_x; // Carbon X
        let off_y = self.sample_noise(1) * self.config.intensity_y; // Carbon Y
        let off_z = self.sample_noise(2) * self.config.intensity_z; // Carbon Z

        (off_x, off_y, off_z)
    }
}
