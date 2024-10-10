// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::Renderer::Wgpu;
use time::macros::format_description;
use tracing::level_filters::LevelFilter;
use tracing::subscriber;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use crate::core::statistics::gacha_statistics;
use crate::view::main_view::MainView;

mod core;
mod view;
mod widgets;

pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> eframe::Result {
    // 日志初始化
    let local_time = LocalTime::new(format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second]"
            ));
    let subscriber = tracing_subscriber::fmt::layer()
        .with_timer(local_time)
        .with_filter(LevelFilter::INFO);
    let subscriber = tracing_subscriber::registry().with(subscriber);
    subscriber::set_global_default(subscriber).unwrap();

    let icon = image::load_from_memory(include_bytes!("resource/icon/icon.ico")).unwrap();
    let icon = egui::IconData {
        width: icon.width(),
        height: icon.height(),
        rgba: icon.into_rgba8().into_raw(),
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_maximize_button(false)
            .with_inner_size([900.0, 520.0])
            .with_icon(icon),
        renderer: Wgpu,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        format!("鸣潮抽卡记录工具 v{}", VERSION).as_str(),
        options,
        Box::new(|cc| {
            Ok(Box::new(MainView::new(cc)))
        }),
    )
}