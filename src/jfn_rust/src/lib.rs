pub mod app;
mod app_menu;
mod cli;
pub mod manager;
mod platform_install;
pub mod shell;
#[cfg(target_os = "linux")]
pub mod visibility;
mod window_geometry;

mod connection;
