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

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, HomeWalkMotionDirector);
    let AlterUpdate_addr = get_method_addr(HomeWalkMotionDirector, c"AlterUpdate", 0);
    new_hook!(AlterUpdate_addr, AlterUpdate);
}
