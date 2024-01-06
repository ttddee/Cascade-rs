use egui::menu;

pub fn build_main_menu(ui: &mut egui::Ui) {
    menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Exit").clicked() {
                // …
            }
        });
    });
}
