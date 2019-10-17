use std::any::Any;

pub mod drawable;

#[cfg(target_os = "macos")]
pub use crate::platform::metal::Renderer as Renderer;
#[cfg(target_os = "macos")]
pub use crate::platform::metal::Scene as Scene;
#[cfg(target_os = "macos")]
pub use crate::platform::metal::buffers as buffers;
#[cfg(target_os = "macos")]
pub use crate::platform::metal::shader as shader;
#[cfg(target_os = "macos")]
pub use crate::platform::metal::texture as texture;

#[cfg(target_os = "windows")]
pub use crate::platform::d3d::Renderer as Renderer;
#[cfg(target_os = "windows")]
pub use crate::platform::d3d::Scene as Scene;
#[cfg(target_os = "windows")]
pub use crate::platform::d3d::buffers as buffers;
#[cfg(target_os = "windows")]
pub use crate::platform::d3d::shader as shader;
#[cfg(target_os = "windows")]
pub use crate::platform::d3d::texture as texture;
