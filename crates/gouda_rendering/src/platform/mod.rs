#[cfg(target_os = "macos")]
pub mod metal;

#[cfg(all(target_os = "windows", not(feature = "use_d3d12")))]
pub mod d3d11;

#[cfg(all(target_os = "windows", feature = "use_d3d12"))]
pub mod d3d12;
