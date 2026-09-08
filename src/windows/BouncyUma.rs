use std::sync::Mutex;
use std::time::Instant;

use once_cell::sync::Lazy;

use crate::{
    core::Hachimi,
    il2cpp::{hook::umamusume::Director, types::*},
};

const DEFAULT_BPM: f32 = 120.0;
const DEFAULT_AMPLITUDE: f32 = 0.33;
const MAX_AMPLITUDE: f32 = 2.0;

#[derive(Debug)]
struct BouncyUmaState {
    elapsed: f32,
    last_update: Instant,
}

static STATE: Lazy<Mutex<BouncyUmaState>> = Lazy::new(|| Mutex::new(BouncyUmaState {
    elapsed: 0.0,
    last_update: Instant::now(),
}));

fn current_scale() -> Option<Vector3_t> {
    let config = Hachimi::instance().config.load();
    let config = &config.windows.bouncy_uma;
    if !config.enabled {
        return None;
    }

    let state = STATE.lock().unwrap();
    Some(next_scale(
        state.elapsed,
        config.bpm,
        config.amplitude.clamp(0.0, MAX_AMPLITUDE),
        config.easing_type,
    ))
}

fn ease(value: f32, easing_type: i32) -> f32 {
    match easing_type {
        0 => value,
        1 => -(std::f32::consts::PI * value).cos() * 0.5 + 0.5,
        2 => if value < 0.5 {
            2.0 * value * value
        } else {
            1.0 - (-2.0 * value + 2.0).powi(2) * 0.5
        },
        3 => if value < 0.5 {
            4.0 * value.powi(3)
        } else {
            1.0 - (-2.0 * value + 2.0).powi(3) * 0.5
        },
        4 => if value < 0.5 {
            8.0 * value.powi(4)
        } else {
            1.0 - (-2.0 * value + 2.0).powi(4) * 0.5
        },
        5 => if value < 0.5 {
            16.0 * value.powi(5)
        } else {
            1.0 - (-2.0 * value + 2.0).powi(5) * 0.5
        },
        6 => {
            if value == 0.0 || value == 1.0 {
                value
            } else if value < 0.5 {
                2.0_f32.powf(20.0 * value - 10.0) * 0.5
            } else {
                (2.0 - 2.0_f32.powf(-20.0 * value + 10.0)) * 0.5
            }
        },
        7 => if value < 0.5 {
            (1.0 - (1.0 - (2.0 * value).powi(2)).sqrt()) * 0.5
        } else {
            (((1.0 - (-2.0 * value + 2.0).powi(2)).sqrt()) + 1.0) * 0.5
        },
        8 => {
            let c1 = 1.70158;
            let c2 = c1 * 1.525;
            if value < 0.5 {
                ((2.0 * value).powi(2) * ((c2 + 1.0) * 2.0 * value - c2)) * 0.5
            } else {
                (((2.0 * value - 2.0).powi(2) * ((c2 + 1.0) * (2.0 * value - 2.0) + c2)) + 2.0) * 0.5
            }
        },
        9 => {
            let c5 = 2.0 * std::f32::consts::PI / 4.5;
            if value == 0.0 || value == 1.0 {
                value
            } else if value < 0.5 {
                -(2.0_f32.powf(20.0 * value - 10.0)
                    * ((20.0 * value - 11.125) * c5).sin()) * 0.5
            } else {
                (2.0_f32.powf(-20.0 * value + 10.0)
                    * ((20.0 * value - 11.125) * c5).sin()) * 0.5 + 1.0
            }
        },
        10 => {
            fn ease_out_bounce(mut value: f32) -> f32 {
                const N1: f32 = 7.5625;
                const D1: f32 = 2.75;
                if value < 1.0 / D1 {
                    N1 * value * value
                } else if value < 2.0 / D1 {
                    value -= 1.5 / D1;
                    N1 * value * value + 0.75
                } else if value < 2.5 / D1 {
                    value -= 2.25 / D1;
                    N1 * value * value + 0.9375
                } else {
                    value -= 2.625 / D1;
                    N1 * value * value + 0.984375
                }
            }

            if value < 0.5 {
                (1.0 - ease_out_bounce(1.0 - 2.0 * value)) * 0.5
            } else {
                (1.0 + ease_out_bounce(2.0 * value - 1.0)) * 0.5
            }
        },
        _ => value,
    }
}

fn next_scale(elapsed: f32, bpm: f32, amplitude: f32, easing_type: i32) -> Vector3_t {
    let beat_duration = 60.0 / bpm.max(1.0);
    let phase = (elapsed / beat_duration).rem_euclid(1.0);
    let ping_pong = if phase < 0.5 { phase * 2.0 } else { (1.0 - phase) * 2.0 };
    let centered = (ease(ping_pong, easing_type) - 0.5) * 2.0;

    Vector3_t {
        x: 1.0 + centered * amplitude,
        y: 1.0 - centered * amplitude,
        z: 1.0 + centered * amplitude,
    }
}

pub fn update(director: *mut Il2CppObject, delta_time: f32) {
    let config = Hachimi::instance().config.load();
    let config = &config.windows.bouncy_uma;
    let mut state = STATE.lock().unwrap();

    if !config.enabled {
        state.elapsed = 0.0;
        state.last_update = Instant::now();
        return;
    }

    state.elapsed += delta_time.max(0.0);
    state.last_update = Instant::now();
    let scale = next_scale(
        state.elapsed,
        config.bpm,
        config.amplitude.clamp(0.0, MAX_AMPLITUDE),
        config.easing_type,
    );

    for position in [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 20, 21,
    ] {
        let chara_object = Director::get_live_character_object(director, position);
        Director::apply_live_bone_scale_to_character(chara_object, 1, scale);
        Director::apply_live_bone_scale_to_character(chara_object, 2, scale);
    }
}

pub fn update_race(view: *mut Il2CppObject) {
    let now = Instant::now();
    let delta_time;
    {
        let mut state = STATE.lock().unwrap();
        delta_time = now.duration_since(state.last_update).as_secs_f32();
        state.elapsed += delta_time;
        state.last_update = now;
    }

    let Some(scale) = current_scale() else {
        return;
    };
    crate::il2cpp::hook::umamusume::RaceViewBase::apply_bouncy_scale(view, scale);
}

pub fn default_bpm() -> f32 { DEFAULT_BPM }
pub fn default_amplitude() -> f32 { DEFAULT_AMPLITUDE }
