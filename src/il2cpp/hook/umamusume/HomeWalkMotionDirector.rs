use crate::{
    core::Hachimi,
    il2cpp::{symbols::get_method_addr, types::*},
};

type AlterUpdateFn = extern "C" fn(this: *mut Il2CppObject);
extern "C" fn AlterUpdate(this: *mut Il2CppObject) {
    if this.is_null() {
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

type TryDecideNextWalkerFn = extern "C" fn(this: *mut Il2CppObject) -> bool;
extern "C" fn TryDecideNextWalker(this: *mut Il2CppObject) -> bool {
    if this.is_null() {
        return false;
    }

    let max_walkers = Hachimi::instance().config.load().home_walk_max_walkers;
    if max_walkers > 1 {
            let (can_spawn, forced_path, real_size, size_ptr) = unsafe {
                // Check if director is currently loading a model (_loadLock at offset 0x50)
                let load_lock = *((this as *mut u8).add(0x50) as *const bool);
                if load_lock {
                    (false, None, 0, std::ptr::null_mut())
                } else {
                    let player_list = *((this as *mut u8).add(0x38) as *mut *mut Il2CppObject);
                    if !player_list.is_null() {
                        let size_ptr = (player_list as *mut u8).add(0x18) as *mut i32;
                        let real_size = *size_ptr;
                        if real_size < max_walkers {
                            *size_ptr = 0;

                            let path_list = *((this as *mut u8).add(0x28) as *mut *mut Il2CppObject);
                            let mut forced_path: Option<*mut bool> = None;
                            if !path_list.is_null() {
                                let path_size = *((path_list as *mut u8).add(0x18) as *const i32);
                                let path_items = *((path_list as *mut u8).add(0x10) as *mut *mut Il2CppObject);
                                if !path_items.is_null() && path_size > 0 {
                                    let items_ptr = (path_items as *mut u8).add(0x20) as *mut *mut Il2CppObject;
                                    let mut any_free = false;
                                    for i in 0..path_size {
                                        let p = *items_ptr.add(i as usize);
                                        if !p.is_null() {
                                            let is_used_ptr = (p as *mut u8).add(0x10) as *mut bool;
                                            if !*is_used_ptr {
                                                any_free = true;
                                                break;
                                            }
                                        }
                                    }
                                    if !any_free {
                                        for i in 0..path_size {
                                            let p = *items_ptr.add(i as usize);
                                            if !p.is_null() {
                                                let is_used_ptr = (p as *mut u8).add(0x10) as *mut bool;
                                                *is_used_ptr = false;
                                                forced_path = Some(is_used_ptr);
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            (true, forced_path, real_size, size_ptr)
                        } else {
                            (false, None, 0, std::ptr::null_mut())
                        }
                    } else {
                        (false, None, 0, std::ptr::null_mut())
                    }
                }
            };

            if can_spawn {
                let res = get_orig_fn!(TryDecideNextWalker, TryDecideNextWalkerFn)(this);
                unsafe {
                    if !size_ptr.is_null() {
                        *size_ptr = real_size;
                    }
                    if !res {
                        if let Some(p) = forced_path {
                            *p = true;
                        }
                    }
                }
                return res;
            }
    }

    get_orig_fn!(TryDecideNextWalker, TryDecideNextWalkerFn)(this)
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, HomeWalkMotionDirector);
    let AlterUpdate_addr = get_method_addr(HomeWalkMotionDirector, c"AlterUpdate", 0);
    new_hook!(AlterUpdate_addr, AlterUpdate);

    let TryDecideNextWalker_addr = get_method_addr(HomeWalkMotionDirector, c"TryDecideNextWalker", 0);
    new_hook!(TryDecideNextWalker_addr, TryDecideNextWalker);
}
