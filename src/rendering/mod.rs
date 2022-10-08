pub mod drawable;
pub mod sprites;
pub mod shapes;
pub mod buffers2;
pub mod model;

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
pub use crate::platform::d3d11::Renderer as Renderer;
#[cfg(target_os = "windows")]
pub use crate::platform::d3d11::Scene as Scene;
#[cfg(target_os = "windows")]
pub use crate::platform::d3d11::buffers as buffers;
#[cfg(target_os = "windows")]
pub use crate::platform::d3d11::shader as shader;
#[cfg(target_os = "windows")]
pub use crate::platform::d3d11::texture as texture;
