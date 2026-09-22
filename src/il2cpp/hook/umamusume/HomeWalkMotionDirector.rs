use crate::{
    core::Hachimi,
    il2cpp::{symbols::get_method_addr, types::*},
};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

pub static WALK_PAUSED: AtomicBool = AtomicBool::new(false);
pub static WALK_REVERSE: AtomicBool = AtomicBool::new(false);
pub static WALK_SPEED_BITS: AtomicU32 = AtomicU32::new(1.0f32.to_bits());

pub fn is_walk_paused() -> bool {
    WALK_PAUSED.load(Ordering::Acquire)
}

pub fn set_walk_paused(paused: bool) {
    WALK_PAUSED.store(paused, Ordering::Release);
}

pub fn toggle_walk_pause() {
    WALK_PAUSED.fetch_xor(true, Ordering::AcqRel);
}

pub fn is_walk_reverse() -> bool {
    WALR_REVERSE_HELPER()
}

fn WALR_REVERSE_HELPER() -> bool {
    WALK_REVERSE.load(Ordering::Acquire)
}

pub fn set_walk_reverse(reverse: bool) {
    WALK_REVERSE.store(reverse, Ordering::Release);
}

pub fn toggle_walk_reverse() {
    WALK_REVERSE.fetch_xor(true, Ordering::AcqRel);
}

pub fn get_walk_speed() -> f32 {
    f32::from_bits(WALK_SPEED_BITS.load(Ordering::Acquire))
}

pub fn set_walk_speed(speed: f32) {
    WALK_SPEED_BITS.store(speed.to_bits(), Ordering::Release);
}

type AlterUpdateFn = extern "C" fn(this: *mut Il2CppObject);
extern "C" fn AlterUpdate(this: *mut Il2CppObject) {
    if this.is_null() {
        return;
    }

    if is_walk_paused() {
        let timer_ptr = unsafe { (this as *mut u8).add(0x48) as *mut f32 };
        let timer_before = unsafe { *timer_ptr };
        get_orig_fn!(AlterUpdate, AlterUpdateFn)(this);
        unsafe {
            *timer_ptr = timer_before;
        }
        return;
    }

    let multiplier = Hachimi::instance().config.load().home_walk_spawn_multiplier;
    let timer_ptr = unsafe { (this as *mut u8).add(0x48) as *mut f32 };
    let span_ptr = unsafe { (this as *mut u8).add(0x4c) as *mut f32 };
    let timer_before = unsafe { *timer_ptr };

    get_orig_fn!(AlterUpdate, AlterUpdateFn)(this);

    if (multiplier - 1.0).abs() > 0.001 {
        unsafe {
            let timer_after = *timer_ptr;
            // If the timer advanced during AlterUpdate, scale the elapsed time by multiplier
            if timer_after > timer_before {
                let dt = timer_after - timer_before;
                let span = *span_ptr;
                let new_timer = timer_before + dt * multiplier;
                *timer_ptr = if span > 0.0 {
                    new_timer.min(span)
                } else {
                    new_timer
                };
            }
        }
    }
}

type UpdateWalkerFn = extern "C" fn(this: *mut Il2CppObject, delta_time: f32);
extern "C" fn UpdateWalker(this: *mut Il2CppObject, delta_time: f32) {
    if this.is_null() {
        return;
    }

    let is_paused = is_walk_paused();
    let is_rev = is_walk_reverse();
    let speed = get_walk_speed();

    let dt = if is_paused {
        0.0
    } else {
        let dir = if is_rev { -1.0 } else { 1.0 };
        delta_time * speed * dir
    };

    get_orig_fn!(UpdateWalker, UpdateWalkerFn)(this, dt);

    // If reverse, clamp _walkLength >= 0.0 to prevent underflow crashes
    if is_rev && !is_paused {
        unsafe {
            // _walkMotionPlayerList at offset 0x38
            let player_list = *((this as *mut u8).add(0x38) as *mut *mut Il2CppObject);
            if !player_list.is_null() {
                let size = *((player_list as *mut u8).add(0x18) as *const i32);
                let items = *((player_list as *mut u8).add(0x10) as *mut *mut Il2CppObject);
                if !items.is_null() && size > 0 {
                    let items_ptr = (items as *mut u8).add(0x20) as *mut *mut Il2CppObject;
                    for i in 0..size {
                        let p = *items_ptr.add(i as usize);
                        if !p.is_null() {
                            // HomeWalkModelController._walkLength is at offset 0x9e8
                            let walk_len_ptr = (p as *mut u8).add(0x9e8) as *mut f32;
                            if *walk_len_ptr < 0.0 {
                                *walk_len_ptr = 0.0;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, HomeWalkMotionDirector);
    let AlterUpdate_addr = get_method_addr(HomeWalkMotionDirector, c"AlterUpdate", 0);
    new_hook!(AlterUpdate_addr, AlterUpdate);

    let UpdateWalker_addr = get_method_addr(HomeWalkMotionDirector, c"UpdateWalker", 1);
    new_hook!(UpdateWalker_addr, UpdateWalker);
}
