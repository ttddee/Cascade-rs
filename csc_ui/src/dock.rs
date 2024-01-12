use std::collections::HashSet;

use egui::{CentralPanel, Frame, Ui, WidgetText};
use egui_dock::{AllowedSplits, DockArea, DockState, NodeIndex, Style, SurfaceIndex, TabViewer};

pub struct MainDock {
    context: DockContext,
    tree: DockState<String>,
}

impl MainDock {
    pub fn show(&mut self, context: egui::Context) {
        CentralPanel::default()
            // When displaying a DockArea in another UI, it looks better
            // to set inner margins to 0.
            .frame(Frame::central_panel(&context.style()).inner_margin(0.))
            .show(&context, |ui| {
                let style = self
                    .context
                    .style
                    .get_or_insert(Style::from_egui(ui.style()))
                    .clone();

                DockArea::new(&mut self.tree)
                    .style(style)
                    .show_close_buttons(self.context.show_close_buttons)
                    .show_add_buttons(self.context.show_add_buttons)
                    .draggable_tabs(self.context.draggable_tabs)
                    .show_tab_name_on_hover(self.context.show_tab_name_on_hover)
                    .allowed_splits(self.context.allowed_splits)
                    .show_window_close_buttons(self.context.show_window_close)
                    .show_window_collapse_buttons(self.context.show_window_collapse)
                    .show_inside(ui, &mut self.context);
            });
    }
}

impl Default for MainDock {
    fn default() -> Self {
        let mut dock_state = DockState::new(vec!["Viewer".to_owned()]);

        let [_, _] = dock_state.main_surface_mut().split_right(
            NodeIndex::root(),
            0.8,
            vec!["Properties".to_owned()],
        );

        let [_, _] = dock_state.main_surface_mut().split_below(
            NodeIndex::root(),
            0.7,
            vec!["Node Graph".to_owned()],
        );

        let mut open_tabs = HashSet::new();

        for node in dock_state[SurfaceIndex::main()].iter() {
            if let Some(tabs) = node.tabs() {
                for tab in tabs {
                    open_tabs.insert(tab.clone());
                }
            }
        }
        let context = DockContext {
            style: None,
            open_tabs,

            show_window_close: true,
            show_window_collapse: true,
            show_close_buttons: true,
            show_add_buttons: false,
            draggable_tabs: true,
            show_tab_name_on_hover: false,
            allowed_splits: AllowedSplits::default(),
        };

        Self {
            context,
            tree: dock_state,
        }
    }
}
struct DockContext {
    pub style: Option<Style>,
    open_tabs: HashSet<String>,

    show_close_buttons: bool,
    show_add_buttons: bool,
    draggable_tabs: bool,
    show_tab_name_on_hover: bool,
    allowed_splits: AllowedSplits,
    show_window_close: bool,
    show_window_collapse: bool,
}

impl DockContext {}

impl TabViewer for DockContext {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.as_str().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        // match tab.as_str() {
        //     "Simple Demo" => self.simple_demo(ui),
        //     "Style Editor" => self.style_editor(ui),
        //     _ => {
        //         ui.label(tab.as_str());
        //     }
        // }
    }

    fn context_menu(
        &mut self,
        ui: &mut Ui,
        tab: &mut Self::Tab,
        _surface: SurfaceIndex,
        _node: NodeIndex,
    ) {
        // match tab.as_str() {
        //     "Simple Demo" => self.simple_demo_menu(ui),
        //     _ => {
        //         ui.label(tab.to_string());
        //         ui.label("This is a context menu");
        //     }
        // }
    }

    fn closeable(&mut self, tab: &mut Self::Tab) -> bool {
        //["Inspector", "Style Editor"].contains(&tab.as_str())
        false
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
        self.open_tabs.remove(tab);
        true
    }
}
