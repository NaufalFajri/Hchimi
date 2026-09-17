use crate::{
    il2cpp::{
        hook::UnityEngine_CoreModule::{Transform, Component, Behaviour},
        symbols::get_method_addr, 
        types::*
    },
    windows::free_camera::{self, CameraScene},
};

type NoArgsFn = extern "C" fn(this: *mut Il2CppObject);
type GetCameraFn = extern "C" fn(this: *mut Il2CppObject) -> *mut Il2CppObject;

static mut GET_CAMERA_ADDR: usize = 0;
static mut GET_CINEMA_BRAIN_ADDR: usize = 0;

fn get_Camera(this: *mut Il2CppObject) -> *mut Il2CppObject {
    let func: GetCameraFn = unsafe { std::mem::transmute(GET_CAMERA_ADDR) };
    func(this)
}

fn get_CinemaBrain(this: *mut Il2CppObject) -> *mut Il2CppObject {
    let func: GetCameraFn = unsafe { std::mem::transmute(GET_CINEMA_BRAIN_ADDR) };
    func(this)
}

extern "C" fn HomeCameraSwitcher_AlterUpdate(this: *mut Il2CppObject) {
    free_camera::set_home_active();
    free_camera::tick();
    get_orig_fn!(HomeCameraSwitcher_AlterUpdate, NoArgsFn)(this);

    let cinemachine_brain = get_CinemaBrain(this);

    if free_camera::is_scene_enabled(CameraScene::Home) {
        if !cinemachine_brain.is_null() && Behaviour::get_enabled(cinemachine_brain) {
            Behaviour::set_enabled(cinemachine_brain, false);
        }

        let camera = get_Camera(this);
        if !camera.is_null() {
            let camera_transform = Component::get_transform(camera);
            if !camera_transform.is_null() {
                let mut position = free_camera::camera_pos();
                Transform::set_position_Injected(camera_transform, &mut position);
                if let Some(mut rotation) = free_camera::camera_rotation() {
                    Transform::set_rotation_Injected(camera_transform, &mut rotation);
                } else {
                    let mut look_at = free_camera::camera_look_at();
                    let mut world_up = Vector3_t {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    };
                    Transform::Internal_LookAt_Injected(camera_transform, &mut look_at, &mut world_up);
                }
            }
        }
    } else {
        if !cinemachine_brain.is_null() && !Behaviour::get_enabled(cinemachine_brain) {
            Behaviour::set_enabled(cinemachine_brain, true);
        }
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, HomeCameraSwitcher);

    unsafe {
        GET_CAMERA_ADDR = get_method_addr(HomeCameraSwitcher, c"get_Camera", 0);
        GET_CINEMA_BRAIN_ADDR = get_method_addr(HomeCameraSwitcher, c"get_CinemaBrain", 0);
    }

    let alter_update_addr = get_method_addr(HomeCameraSwitcher, c"AlterUpdate", 0);
    new_hook!(alter_update_addr, HomeCameraSwitcher_AlterUpdate);
}
