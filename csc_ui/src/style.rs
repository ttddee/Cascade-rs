use ecolor::Color32;
use egui::Visuals;

// BGHover:        #626971
// Red:            #e5463d
// RedHover:       #fd4439
// Green:          #35e257
// GreenHover:     #31fb59
// BlueDark:       #254a73
// Blue:           #2372ef
// BlueHover:      #287bff

pub const CS_COLOR_BG_DARK: Color32 = Color32::from_rgb(24, 27, 30);
pub const CS_COLOR_BG_MID: Color32 = Color32::from_rgb(29, 32, 36);
pub const CS_COLOR_BG_LIGHT: Color32 = Color32::from_rgb(40, 45, 49);
pub const CS_COLOR_FG_DARK: Color32 = Color32::from_rgb(146, 153, 161);
pub const CS_COLOR_FG_MID: Color32 = Color32::from_rgb(185, 185, 185);
pub const CS_COLOR_FG_LIGHT: Color32 = Color32::from_rgb(197, 197, 197);
pub const CS_COLOR_RED: Color32 = Color32::from_rgb(229, 70, 61);
pub const CS_COLOR_GREEN: Color32 = Color32::from_rgb(53, 226, 87);
pub const CS_COLOR_BLUE: Color32 = Color32::from_rgb(35, 114, 239);

pub fn load_style(context: &mut egui::Context) {
    let visuals = Visuals {
        panel_fill: CS_COLOR_BG_MID,
        window_fill: CS_COLOR_BG_MID,
        faint_bg_color: CS_COLOR_BG_LIGHT,
        extreme_bg_color: CS_COLOR_BG_DARK,
        ..Default::default()
    };

    context.set_visuals(visuals);
}
