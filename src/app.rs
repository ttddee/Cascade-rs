use crate::egui_tools::EguiRenderer;
use cas_graph::graph_ui::GraphUI;
use cas_graph::snarl_graph::{DemoNode, DemoViewer};
use egui::Id;
use egui_snarl::ui::{SnarlStyle, SnarlWidget};
use egui_snarl::Snarl;
use egui_wgpu::wgpu::SurfaceError;
use egui_wgpu::{wgpu, ScreenDescriptor};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct AppState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'static>,
    pub scale_factor: f32,
    pub egui_renderer: EguiRenderer,
    pub scene_rect: egui::Rect,
}

impl AppState {
    async fn new(
        instance: &wgpu::Instance,
        surface: wgpu::Surface<'static>,
        window: &Window,
        width: u32,
        height: u32,
    ) -> Self {
        let power_pref = wgpu::PowerPreference::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: power_pref,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let features = wgpu::Features::empty();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: features,
                    required_limits: Default::default(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let selected_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let swapchain_format = swapchain_capabilities
            .formats
            .iter()
            .find(|d| **d == selected_format)
            .expect("failed to select proper surface texture format!");

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *swapchain_format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 0,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        let egui_renderer = EguiRenderer::new(&device, surface_config.format, None, 1, window);

        let scale_factor = 1.0;

        let scene_rect = egui::Rect::NOTHING;

        Self {
            device,
            queue,
            surface,
            surface_config,
            egui_renderer,
            scale_factor,
            scene_rect,
        }
    }

    fn resize_surface(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }
}

pub struct App {
    instance: wgpu::Instance,
    state: Option<AppState>,
    window: Option<Arc<Window>>,
    snarl: Snarl<DemoNode>,
}

impl App {
    pub fn new() -> Self {
        let instance = egui_wgpu::wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        Self {
            instance,
            state: None,
            window: None,
            snarl: Snarl::new(),
        }
    }

    async fn set_window(&mut self, window: Window) {
        let window = Arc::new(window);
        let initial_width = 1360;
        let initial_height = 768;

        let _ = window.request_inner_size(PhysicalSize::new(initial_width, initial_height));

        let surface = self
            .instance
            .create_surface(window.clone())
            .expect("Failed to create surface!");

        let state = AppState::new(
            &self.instance,
            surface,
            &window,
            initial_width,
            initial_width,
        )
        .await;

        self.window.get_or_insert(window);
        self.state.get_or_insert(state);
    }

    fn handle_resized(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.state.as_mut().unwrap().resize_surface(width, height);
        }
    }

    fn handle_redraw(&mut self) {
        // Attempt to handle minimizing window
        if let Some(window) = self.window.as_ref() {
            if let Some(min) = window.is_minimized() {
                if min {
                    println!("Window is minimized");
                    return;
                }
            }
        }

        let state = self.state.as_mut().unwrap();

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [state.surface_config.width, state.surface_config.height],
            pixels_per_point: self.window.as_ref().unwrap().scale_factor() as f32
                * state.scale_factor,
        };

        let surface_texture = state.surface.get_current_texture();

        match surface_texture {
            Err(SurfaceError::Outdated) => {
                // Ignoring outdated to allow resizing and minimization
                println!("wgpu surface outdated");
                return;
            }
            Err(_) => {
                surface_texture.expect("Failed to acquire next swap chain texture");
                return;
            }
            Ok(_) => {}
        };

        let surface_texture = surface_texture.unwrap();

        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let window = self.window.as_ref().unwrap();

        {
            state.egui_renderer.begin_frame(window);

            // ---------------------------------------------------------

            // let mut graph = NodeGraph::new();

            // graph.add_node(egui::pos2(100., 100.));
            // graph.add_node(egui::pos2(300., 120.));

            // egui::Area::new(egui::Id::new("graph_area"))
            //     .default_size(egui::vec2(1200., 1200.))
            //     .show(state.egui_renderer.context(), |ui| {
            //         egui::containers::Scene::new().show(ui, &mut state.scene_rect, |ui| {
            //             // Draw something inside the scene
            //             ui.label("Hello, egui Scene!");
            //             let graph_ui = GraphUI {};
            //             graph_ui.draw_graph(ui, &graph);
            //         });
            //     });

            // ---------------------------------------------------------

            egui_extras::install_image_loaders(&state.egui_renderer.context());

            // cx.style_mut(|style| style.animation_time *= 10.0);

            // let snarl = cx.storage.map_or_else(Snarl::new, |storage| {
            //     storage
            //         .get_string("snarl")
            //         .and_then(|snarl| serde_json::from_str(&snarl).ok())
            //         .unwrap_or_default()
            // });
            // // let snarl = Snarl::new();

            // let style = cx.storage.map_or_else(default_style, |storage| {
            //     storage
            //         .get_string("style")
            //         .and_then(|style| serde_json::from_str(&style).ok())
            //         .unwrap_or_else(default_style)
            // });

            //let mut snarl = Snarl::new();

            egui::TopBottomPanel::top("top_panel").show(state.egui_renderer.context(), |ui| {
                // The top panel is often a good place for a menu bar:

                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            state
                                .egui_renderer
                                .context()
                                .send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);

                    egui::widgets::global_theme_preference_switch(ui);

                    if ui.button("Clear All").clicked() {
                        self.snarl = Snarl::default();
                    }
                });
            });

            egui::SidePanel::left("style").show(state.egui_renderer.context(), |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    //egui_probe::Probe::new(&mut self.style).show(ui);
                });
            });

            // egui::SidePanel::right("selected-list").show(ctx, |ui| {
            //     egui::ScrollArea::vertical().show(ui, |ui| {
            //         ui.strong("Selected nodes");

            //         let selected = get_selected_nodes(Id::new("snarl-demo"), ui.ctx());

            //         let mut selected = selected
            //             .into_iter()
            //             .map(|id| (id, &self.snarl[id]))
            //             .collect::<Vec<_>>();

            //         selected.sort_by_key(|(id, _)| *id);

            //         let mut remove = None;

            //         for (id, node) in selected {
            //             ui.horizontal(|ui| {
            //                 ui.label(format!("{id:?}"));
            //                 ui.label(node.name());
            //                 ui.add_space(ui.spacing().item_spacing.x);
            //                 if ui.button("Remove").clicked() {
            //                     remove = Some(id);
            //                 }
            //             });
            //         }

            //         if let Some(id) = remove {
            //             self.snarl.remove_node(id);
            //         }
            //     });
            // });

            egui::CentralPanel::default().show(state.egui_renderer.context(), |ui| {
                SnarlWidget::new()
                    .id(Id::new("snarl-demo"))
                    //.style(self.style)
                    .show(&mut self.snarl, &mut DemoViewer, ui);
            });

            // ---------------------------------------------------------

            state.egui_renderer.end_frame_and_draw(
                &state.device,
                &state.queue,
                &mut encoder,
                window,
                &surface_view,
                screen_descriptor,
            );
        }

        state.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();
        pollster::block_on(self.set_window(window));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        // let egui render to process the event first
        self.state
            .as_mut()
            .unwrap()
            .egui_renderer
            .handle_input(self.window.as_ref().unwrap(), &event);

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.handle_redraw();

                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::Resized(new_size) => {
                self.handle_resized(new_size.width, new_size.height);
            }
            _ => (),
        }
    }
}
