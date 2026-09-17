pub mod AxisControl;
pub mod ButtonControl;
pub mod DpadControl;
pub mod Gamepad;
pub mod InputControl;
pub mod Vector2Control;

pub fn init() {
    get_assembly_image_or_return!(image, "Unity.InputSystem.dll");

    Gamepad::init(image);
    AxisControl::init(image);
    InputControl::init(image);
    Vector2Control::init(image);
    DpadControl::init(image);
    ButtonControl::init(image);
}
