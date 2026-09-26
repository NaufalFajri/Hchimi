use crate::il2cpp::{
    api::il2cpp_resolve_icall,
    symbols::get_method_addr,
    types::*,
};

#[cfg(target_os = "windows")]
use crate::windows::free_camera::{self, CameraScene};
#[cfg(target_os = "windows")]
type CameraGetFloatFn = extern "C" fn(this: *mut Il2CppObject) -> f32;
#[cfg(target_os = "windows")]
type CameraSetFloatFn = extern "C" fn(this: *mut Il2CppObject, value: f32);

static mut SET_FIELD_OF_VIEW_ADDR: usize = 0;
impl_addr_wrapper_fn!(set_fieldOfView, SET_FIELD_OF_VIEW_ADDR, (), this: *mut Il2CppObject, value: f32);

static mut GET_MAIN_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_main, GET_MAIN_ADDR, *mut Il2CppObject,);

static mut GET_ALL_CAMERAS_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_allCameras, GET_ALL_CAMERAS_ADDR, *mut Il2CppArray,);

#[cfg(target_os = "windows")]
fn should_override_near_clip() -> bool {
    free_camera::is_scene_enabled(CameraScene::Home)
        || free_camera::is_scene_enabled(CameraScene::Live)
        || free_camera::is_scene_enabled(CameraScene::Race)
}

#[cfg(target_os = "windows")]
extern "C" fn Camera_get_fieldOfView(this: *mut Il2CppObject) -> f32 {
    let scene = free_camera::scene();
    if scene == CameraScene::PhotoStudioCutPlay {
        // Let cut cameras return their real FOV so DoF and post-processing shaders are not distorted
        return get_orig_fn!(Camera_get_fieldOfView, CameraGetFloatFn)(this);
    }
    if let Some(fov) = free_camera::fov_for_scene(scene) {
        return fov;
    }
    get_orig_fn!(Camera_get_fieldOfView, CameraGetFloatFn)(this)
}

#[cfg(target_os = "windows")]
extern "C" fn Camera_set_nearClipPlane(this: *mut Il2CppObject, mut value: f32) {
    if should_override_near_clip() {
        value = 0.001;
    }
    get_orig_fn!(Camera_set_nearClipPlane, CameraSetFloatFn)(this, value);
}

#[cfg(target_os = "windows")]
extern "C" fn Camera_get_nearClipPlane(this: *mut Il2CppObject) -> f32 {
    if should_override_near_clip() {
        return 0.001;
    }
    get_orig_fn!(Camera_get_nearClipPlane, CameraGetFloatFn)(this)
}

#[cfg(target_os = "windows")]
extern "C" fn Camera_set_farClipPlane(this: *mut Il2CppObject, mut value: f32) {
    if free_camera::is_scene_enabled(CameraScene::Home)
        || free_camera::is_scene_enabled(CameraScene::Live)
        || free_camera::is_scene_enabled(CameraScene::Race)
    {
        value = 2500.0;
    }
    get_orig_fn!(Camera_set_farClipPlane, CameraSetFloatFn)(this, value);
}

#[cfg(target_os = "windows")]
extern "C" fn Camera_get_farClipPlane(this: *mut Il2CppObject) -> f32 {
    if free_camera::is_scene_enabled(CameraScene::Home)
        || free_camera::is_scene_enabled(CameraScene::Live)
        || free_camera::is_scene_enabled(CameraScene::Race)
    {
        return 2500.0;
    }
    get_orig_fn!(Camera_get_farClipPlane, CameraGetFloatFn)(this)
}

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, Camera);

    unsafe {
        SET_FIELD_OF_VIEW_ADDR =
            il2cpp_resolve_icall(c"UnityEngine.Camera::set_fieldOfView(System.Single)".as_ptr());
        GET_MAIN_ADDR = get_method_addr(Camera, c"get_main", 0);
        GET_ALL_CAMERAS_ADDR = get_method_addr(Camera, c"get_allCameras", 0);
    }

    #[cfg(target_os = "windows")]
    {
        let get_fieldOfView_addr =
            il2cpp_resolve_icall(c"UnityEngine.Camera::get_fieldOfView()".as_ptr());
        new_hook!(get_fieldOfView_addr, Camera_get_fieldOfView);

        let set_nearClipPlane_addr =
            il2cpp_resolve_icall(c"UnityEngine.Camera::set_nearClipPlane(System.Single)".as_ptr());
        new_hook!(set_nearClipPlane_addr, Camera_set_nearClipPlane);

        let get_nearClipPlane_addr =
            il2cpp_resolve_icall(c"UnityEngine.Camera::get_nearClipPlane()".as_ptr());
        new_hook!(get_nearClipPlane_addr, Camera_get_nearClipPlane);

        let set_farClipPlane_addr =
            il2cpp_resolve_icall(c"UnityEngine.Camera::set_farClipPlane(System.Single)".as_ptr());
        new_hook!(set_farClipPlane_addr, Camera_set_farClipPlane);

        let get_farClipPlane_addr =
            il2cpp_resolve_icall(c"UnityEngine.Camera::get_farClipPlane()".as_ptr());
        new_hook!(get_farClipPlane_addr, Camera_get_farClipPlane);
    }
}
