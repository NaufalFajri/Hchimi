use crate::{
    il2cpp::{
        hook::UnityEngine_CoreModule::Transform,
        symbols::get_method_addr,
        types::*,
    },
    windows::free_camera::{self, CameraScene},
};

type NoArgsFn = extern "C" fn(this: *mut Il2CppObject);
type GetCameraTransFn = extern "C" fn(this: *mut Il2CppObject, camera_pos: i32) -> *mut Il2CppObject;
type GetCurrentCameraPosFn = extern "C" fn(this: *mut Il2CppObject) -> i32;

static mut GET_CAMERA_TRANS_ADDR: usize = 0;
static mut GET_CURRENT_CAMERA_POS_ADDR: usize = 0;

fn get_camera_trans(this: *mut Il2CppObject, camera_pos: i32) -> *mut Il2CppObject {
    let func: GetCameraTransFn = unsafe { std::mem::transmute(GET_CAMERA_TRANS_ADDR) };
    func(this, camera_pos)
}

fn get_current_camera_pos(this: *mut Il2CppObject) -> i32 {
    let func: GetCurrentCameraPosFn = unsafe { std::mem::transmute(GET_CURRENT_CAMERA_POS_ADDR) };
    func(this)
}

extern "C" fn HomeCameraSwitcher_AlterUpdate(this: *mut Il2CppObject) {
    free_camera::set_home_active();
    free_camera::tick();
    get_orig_fn!(HomeCameraSwitcher_AlterUpdate, NoArgsFn)(this);

    if !free_camera::is_scene_enabled(CameraScene::Home) {
        return;
    }

    let camera_transform = get_camera_trans(this, get_current_camera_pos(this));
    if camera_transform.is_null() {
        return;
    }

    let mut position = free_camera::camera_pos();
    Transform::set_position_Injected(camera_transform, &mut position);
    if let Some(mut rotation) = free_camera::camera_rotation() {
        Transform::set_rotation_Injected(camera_transform, &mut rotation);
    }
    else {
        let mut look_at = free_camera::camera_look_at();
        let mut world_up = Vector3_t { x: 0.0, y: 1.0, z: 0.0 };
        Transform::Internal_LookAt_Injected(camera_transform, &mut look_at, &mut world_up);
    }

}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, HomeCameraSwitcher);

    unsafe {
        GET_CAMERA_TRANS_ADDR = get_method_addr(HomeCameraSwitcher, c"GetCameraTrans", 1);
        GET_CURRENT_CAMERA_POS_ADDR = get_method_addr(HomeCameraSwitcher, c"get_CurrentCameraPos", 0);
    }

    let alter_update_addr = get_method_addr(HomeCameraSwitcher, c"AlterUpdate", 0);
    new_hook!(alter_update_addr, HomeCameraSwitcher_AlterUpdate);
}
