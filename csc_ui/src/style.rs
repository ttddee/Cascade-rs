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
// FGDark:         #9299a1
// FGMid:          #c5c5c5
// FGLight:        #b9b9b9

pub const COLOR_BG_DARK: Color32 = Color32::from_rgb(24, 27, 30);
pub const COLOR_BG_MID: Color32 = Color32::from_rgb(29, 32, 36);
pub const COLOR_BG_LIGHT: Color32 = Color32::from_rgb(40, 45, 49);

pub fn load_style(context: &mut egui::Context) {
    let visuals = Visuals {
        panel_fill: COLOR_BG_MID,
        faint_bg_color: COLOR_BG_LIGHT,
        extreme_bg_color: COLOR_BG_DARK,
        ..Default::default()
    };

    context.set_visuals(visuals);
}
