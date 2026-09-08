use crate::{
    windows::free_camera::{self, CameraScene},
    il2cpp::{
        hook::UnityEngine_CoreModule::Transform,
        symbols::{get_method_addr, get_method_overload_addr, SingletonLike},
        types::*,
    },
};

static mut CLASS: *mut Il2CppClass = 0 as _;
static mut GET_MAIN_CAMERA_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_MainCamera, GET_MAIN_CAMERA_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

pub fn instance() -> *mut Il2CppObject {
    let Some(singleton) = SingletonLike::new(class()) else {
        return std::ptr::null_mut();
    };
    singleton.instance()
}

type NoArgsFn = extern "C" fn(this: *mut Il2CppObject);
extern "C" fn RaceCameraManager_AlterLateUpdate(this: *mut Il2CppObject) {
    free_camera::set_race_active();
    free_camera::tick();

    let active = free_camera::is_scene_enabled(CameraScene::Race);
    Transform::set_update_race_camera(active);
    get_orig_fn!(RaceCameraManager_AlterLateUpdate, NoArgsFn)(this);
    Transform::set_update_race_camera(false);
}

pub fn apply_paused_free_camera() {
    if !free_camera::is_scene_enabled(CameraScene::Race) {
        return;
    }

    let camera_manager = instance();
    if !camera_manager.is_null() {
        RaceCameraManager_AlterLateUpdate(camera_manager);

        let camera = get_MainCamera(camera_manager);
        if !camera.is_null() {
            let camera_transform = crate::il2cpp::hook::UnityEngine_CoreModule::Component::get_transform(camera);
            if !camera_transform.is_null() {
                Transform::set_update_race_camera(false);
                let mut position = free_camera::race_camera_pos(Vector3_t::default());
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
        }
    }
}

type RaceChangeCameraModeFn = extern "C" fn(this: *mut Il2CppObject, mode: i32, is_skip: bool);
extern "C" fn RaceCameraManager_ChangeCameraMode(this: *mut Il2CppObject, mode: i32, is_skip: bool) {
    if free_camera::is_scene_enabled(CameraScene::Race) {
        return;
    }
    get_orig_fn!(RaceCameraManager_ChangeCameraMode, RaceChangeCameraModeFn)(this, mode, is_skip);
}

// public bool PlayEventCamera(int targetHorseIndex, int[] rivalHorseIndexArray, int cameraId, bool isForceInPlaying = False, bool isForceUnPlayableArea = False) { }
type RacePlayEventCameraFn = extern "C" fn(
    this: *mut Il2CppObject,
    targetHorseIndex: i32,
    rivalHorseIndexArray: *mut Il2CppArray,
    cameraId: i32,
    isForceInPlaying: bool,
    isForceUnPlayableArea: bool,
) -> bool;
extern "C" fn RaceCameraManager_PlayEventCamera(
    this: *mut Il2CppObject,
    targetHorseIndex: i32,
    rivalHorseIndexArray: *mut Il2CppArray,
    cameraId: i32,
    isForceInPlaying: bool,
    isForceUnPlayableArea: bool,
) -> bool {
    if free_camera::is_scene_enabled(CameraScene::Race) {
        return false;
    }
    get_orig_fn!(RaceCameraManager_PlayEventCamera, RacePlayEventCameraFn)(this, targetHorseIndex, rivalHorseIndexArray, cameraId, isForceInPlaying, isForceUnPlayableArea)
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, "Gallop", RaceCameraManager);

    unsafe { CLASS = RaceCameraManager; }
    unsafe {
        GET_MAIN_CAMERA_ADDR = get_method_addr(RaceCameraManager, c"get_MainCamera", 0);
    }

    let RaceCameraManager_AlterLateUpdate_addr = get_method_addr(RaceCameraManager, c"AlterLateUpdate", 0);
    new_hook!(RaceCameraManager_AlterLateUpdate_addr, RaceCameraManager_AlterLateUpdate);

    let RaceCameraManager_ChangeCameraMode_addr = get_method_addr(RaceCameraManager, c"ChangeCameraMode", 2);
    new_hook!(RaceCameraManager_ChangeCameraMode_addr, RaceCameraManager_ChangeCameraMode);

    let RaceCameraManager_PlayEventCamera_addr = get_method_overload_addr(
        RaceCameraManager,
        "PlayEventCamera",
        &[
            Il2CppTypeEnum_IL2CPP_TYPE_I4, // int targetHorseIndex
            Il2CppTypeEnum_IL2CPP_TYPE_SZARRAY, // int[] rivalHorseIndexArray
            Il2CppTypeEnum_IL2CPP_TYPE_I4, // int cameraId
            Il2CppTypeEnum_IL2CPP_TYPE_BOOLEAN, // bool isForceInPlaying
            Il2CppTypeEnum_IL2CPP_TYPE_BOOLEAN // bool isForceUnPlayableArea
        ]
    );
    new_hook!(RaceCameraManager_PlayEventCamera_addr, RaceCameraManager_PlayEventCamera);
}
