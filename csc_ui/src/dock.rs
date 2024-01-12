use std::{collections::HashSet, sync::Arc};

use csc_core::{
    graph_model::NodeGraphState,
    node_model::{AllNodeTemplates, CsImageType, MyNodeData, MyValueType, NodeType},
};
use egui::{load::SizedTexture, CentralPanel, Frame, ImageSource, Ui, WidgetText};
use egui_dock::{AllowedSplits, DockArea, DockState, NodeIndex, Style, SurfaceIndex, TabViewer};
use egui_node_graph::GraphEditorState;
use egui_winit_vulkano::Gui;
use vulkano::image::view::ImageView;

use crate::properties_panel::PropertiesPanel;

pub struct MainDock {
    context: DockContext,
    tree: DockState<String>,
}

impl MainDock {
    pub fn show(&mut self, context: egui::Context) {
        CentralPanel::default()
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
    pub fn new(gui: &mut Gui, scene_image: Arc<ImageView>, scene_view_size: [u32; 2]) -> Self {
        let mut dock_state = DockState::new(vec!["Viewer".to_owned()]);

        let [a, _] = dock_state.main_surface_mut().split_below(
            NodeIndex::root(),
            0.65,
            vec!["Node Graph".to_owned()],
        );

        let [_, _] =
            dock_state
                .main_surface_mut()
                .split_right(a, 0.8, vec!["Properties".to_owned()]);

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
            scene_texture_id: gui.register_user_image_view(scene_image, Default::default()),
            scene_view_size,
            egui_context: gui.context(),
            graph_state: GraphEditorState::new(1.0),
            user_state: NodeGraphState::default(),
            properties_panel: PropertiesPanel::default(),
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
    scene_texture_id: egui::TextureId,
    scene_view_size: [u32; 2],
    egui_context: egui::Context,
    graph_state: GraphEditorState<MyNodeData, CsImageType, MyValueType, NodeType, NodeGraphState>,
    user_state: NodeGraphState,
    properties_panel: PropertiesPanel,
}

impl DockContext {
    fn viewer(&mut self, ui: &mut Ui) {
        egui::Frame::none()
            .inner_margin(0.)
            .fill(egui::Color32::BLACK)
            .show(ui, |ui| {
                ui.image(ImageSource::Texture(SizedTexture::new(
                    self.scene_texture_id,
                    [
                        self.scene_view_size[0] as f32,
                        self.scene_view_size[1] as f32,
                    ],
                )));
            });
    }

    fn node_graph(&mut self, ui: &mut Ui) {
        let _graph_response = self.graph_state.draw_graph_editor(
            ui,
            AllNodeTemplates,
            &mut self.user_state,
            Vec::default(),
        );
    }

    fn properties_panel(&mut self, ui: &mut Ui) {
        self.properties_panel
            .show(ui, &self.egui_context, &mut self.graph_state);
    }
}

impl TabViewer for DockContext {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.as_str().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "Viewer" => self.viewer(ui),
            "Node Graph" => self.node_graph(ui),
            "Properties" => self.properties_panel(ui),
            _ => {
                ui.label(tab.as_str());
            }
        }
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
        false
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
        self.open_tabs.remove(tab);
        true
    }
}
