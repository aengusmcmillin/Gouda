use crate::window::{GameWindow, WindowProps};
use crate::rendering::Renderer;
use std::rc::Rc;

#[cfg(target_os = "macos")]
pub mod osx;

#[cfg(target_os = "macos")]
pub use osx::OSXPlatformLayer as PlatformLayer;

#[cfg(target_os = "macos")]
pub mod metal;

#[cfg(target_os = "windows")]
pub mod win32;

#[cfg(target_os = "windows")]
pub mod d3d;

#[cfg(target_os = "windows")]
pub use win32::Win32PlatformLayer as PlatformLayer;
