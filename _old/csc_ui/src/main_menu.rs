use egui::menu;

pub fn build_main_menu(context: &egui::Context) {
    egui::TopBottomPanel::top("main_menu").show(context, |ui| {
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Exit").clicked() {
                    // …
                }
            });
        });
    });
}
