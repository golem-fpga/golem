use crate::macguiver::buffer::DrawBuffer;
use crate::macguiver::platform::sdl::SdlPlatform;
use crate::platform::GoLEmPlatform;
use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::BinaryColor;
use image::DynamicImage;
use mister_fpga::config_string::{ConfigMenu, LoadFileInfo};
use mister_fpga::types::StatusBitMap;
use sdl3::event::Event;
use sdl3::gamepad::{Axis, Button};
use sdl3::keyboard::Scancode;
use std::io::{Read, Write};
use std::path::Path;

pub struct NullSaveState;

impl super::SaveState for NullSaveState {
    fn is_dirty(&self) -> bool {
        unreachable!()
    }

    fn write_to(&mut self, _writer: impl Write) -> Result<(), String> {
        unreachable!()
    }

    fn read_from(&mut self, _reader: impl Read) -> Result<(), String> {
        unreachable!()
    }
}

pub struct NullCore;

impl super::Core for NullCore {
    type SaveState = NullSaveState;

    fn name(&self) -> &str {
        ""
    }

    fn load_file(&mut self, _path: &Path, _file_info: Option<LoadFileInfo>) -> Result<(), String> {
        unreachable!()
    }

    fn version(&self) -> Option<&str> {
        unreachable!()
    }

    fn mount_sav(&mut self, path: &Path) -> Result<(), String> {
        unreachable!()
    }

    fn check_sav(&mut self) -> Result<(), String> {
        unreachable!()
    }

    fn menu_options(&self) -> &[ConfigMenu] {
        unreachable!()
    }

    fn trigger_menu(&mut self, _menu: &ConfigMenu) -> Result<bool, String> {
        unreachable!()
    }

    fn status_mask(&self) -> StatusBitMap {
        unreachable!()
    }

    fn status_bits(&self) -> StatusBitMap {
        unreachable!()
    }

    fn set_status_bits(&mut self, _bits: StatusBitMap) {
        unreachable!()
    }

    fn take_screenshot(&mut self) -> Result<DynamicImage, String> {
        unreachable!()
    }

    fn key_down(&mut self, _key: Scancode) {
        unreachable!()
    }

    fn key_up(&mut self, _key: Scancode) {
        unreachable!()
    }

    fn sdl_button_down(&mut self, _joystick_idx: u8, _button: Button) {
        unreachable!()
    }

    fn sdl_button_up(&mut self, _joystick_idx: u8, _button: Button) {
        unreachable!()
    }

    fn sdl_axis_motion(&mut self, _controller: u8, _axis: Axis, _value: i16) {
        unreachable!()
    }

    fn save_states(&mut self) -> Option<&mut [Self::SaveState]> {
        unreachable!()
    }
}

pub struct NullCoreManager;

impl super::CoreManager for NullCoreManager {
    type Core = NullCore;

    fn load_core(&mut self, _path: impl AsRef<Path>) -> Result<Self::Core, String> {
        unreachable!("Platform should never run in NULL.")
    }

    fn get_current_core(&mut self) -> Result<Self::Core, String> {
        unreachable!("Platform should never run in NULL.")
    }

    fn load_menu(&mut self) -> Result<Self::Core, String> {
        unreachable!("Platform should never run in NULL.")
    }

    fn show_menu(&mut self) {
        unreachable!()
    }

    fn hide_menu(&mut self) {
        unreachable!()
    }
}

#[derive(Default)]
pub struct NullPlatform;

impl GoLEmPlatform for NullPlatform {
    type Color = BinaryColor;
    type CoreManager = NullCoreManager;

    fn init(&mut self, _flags: &crate::Flags) {
        unreachable!("Platform should never run in NULL.")
    }

    fn update_toolbar(&mut self, _buffer: &DrawBuffer<Self::Color>) {
        unreachable!("Platform should never run in NULL.")
    }
    fn update_main(&mut self, _buffer: &DrawBuffer<Self::Color>) {
        unreachable!("Platform should never run in NULL.")
    }

    fn toolbar_dimensions(&self) -> Size {
        unreachable!("Platform should never run in NULL.")
    }
    fn main_dimensions(&self) -> Size {
        unreachable!("Platform should never run in NULL.")
    }

    fn events(&mut self) -> Vec<Event> {
        unreachable!("Platform should never run in NULL.")
    }

    fn sdl(&mut self) -> &mut SdlPlatform<Self::Color> {
        unreachable!()
    }

    fn start_loop(&mut self) {}
    fn end_loop(&mut self) {}

    fn core_manager_mut(&mut self) -> &mut Self::CoreManager {
        unreachable!("Platform should never run in NULL.")
    }
}
