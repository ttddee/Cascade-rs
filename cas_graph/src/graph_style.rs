use egui_snarl::ui::{NodeLayout, PinPlacement, SnarlStyle};

pub const fn style() -> SnarlStyle {
    SnarlStyle {
        node_layout: Some(NodeLayout::Basic),
        pin_placement: Some(PinPlacement::Edge),
        pin_size: Some(12.0),
        pin_stroke: Some(egui::Stroke::NONE),
        node_frame: Some(egui::Frame {
            inner_margin: egui::Margin::same(4),
            outer_margin: egui::Margin {
                left: 0,
                right: 0,
                top: 0,
                bottom: 4,
            },
            corner_radius: egui::CornerRadius::same(8),
            fill: egui::Color32::from_gray(30),
            stroke: egui::Stroke::NONE,
            shadow: egui::Shadow::NONE,
        }),
        bg_frame: Some(egui::Frame {
            inner_margin: egui::Margin::ZERO,
            outer_margin: egui::Margin::same(2),
            corner_radius: egui::CornerRadius::ZERO,
            fill: egui::Color32::from_gray(10),
            stroke: egui::Stroke::NONE,
            shadow: egui::Shadow::NONE,
        }),
        ..SnarlStyle::new()
    }
}
