pub mod app;
mod cli;
pub mod manager;
mod platform_install;
#[cfg(target_os = "linux")]
pub mod visibility;
mod window_geometry;
