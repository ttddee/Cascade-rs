// //use crate::nodegraph::NodeGraphExample;
// use eframe::egui::Visuals;

// pub fn init_ui() -> Result<(), eframe::Error> {
//     let native_options = eframe::NativeOptions {
//         initial_window_size: Some(egui::vec2(1920.0, 1080.0)),
//         ..Default::default()
//     };

//     eframe::run_native("Cascade Image Editor", native_options, Box::new(|cc| Box::new(MainWindow::new(cc))))
// }

// #[derive(Default)]
// struct MainWindow {}

// impl MainWindow {
//     fn new(cc: &eframe::CreationContext<'_>) -> Self {
//         // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
//         // Restore app state using cc.storage (requires the "persistence" feature).
//         // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
//         // for e.g. egui::PaintCallback.

//         // Box::new(|cc: &eframe::CreationContext| {
//         //     cc.egui_ctx.set_visuals(Visuals::dark());
//         //     #[cfg(feature = "persistence")]
//         //     {
//         //         Box::new(NodeGraphExample::new(cc))
//         //     }
//         //     #[cfg(not(feature = "persistence"))]
//         //     Box::<NodeGraphExample>::default()
//         // });

//         Self::default()
//     }
// }

// impl eframe::App for MainWindow {
//     fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
//          egui::SidePanel::right("properties_panel").default_width(300.).show(ctx, |ui| {
//              ui.label("Hello World!");
//           });
//           egui::TopBottomPanel::bottom("nodegraph_panel").min_height(600.).resizable(true).show(ctx, |ui| {
     
            
//           });
//          egui::CentralPanel::default().show(ctx, |ui| {
//              ui.heading("My egui Application");
//          });
     
//     }
//  }