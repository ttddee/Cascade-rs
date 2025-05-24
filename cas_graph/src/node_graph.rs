#![allow(clippy::use_self)]

use std::collections::HashMap;

use egui::{Color32, Ui};
use egui_snarl::{
    ui::{AnyPins, NodeLayout, PinInfo, PinPlacement, SnarlStyle, SnarlViewer, WireStyle},
    InPin, InPinId, NodeId, OutPin, OutPinId, Snarl,
};

use crate::node::{DemoNode, ExprNode};

const STRING_COLOR: Color32 = Color32::from_rgb(0x00, 0xb0, 0x00);
const NUMBER_COLOR: Color32 = Color32::from_rgb(0xb0, 0x00, 0x00);
const IMAGE_COLOR: Color32 = Color32::from_rgb(0xb0, 0x00, 0xb0);
const UNTYPED_COLOR: Color32 = Color32::from_rgb(0xb0, 0xb0, 0xb0);

pub struct DemoViewer;

impl SnarlViewer<DemoNode> for DemoViewer {
    #[inline]
    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<DemoNode>) {
        // Validate connection
        #[allow(clippy::match_same_arms)] // For match clarity
        match (&snarl[from.id.node], &snarl[to.id.node]) {
            (DemoNode::Sink, _) => {
                unreachable!("Sink node has no outputs")
            }
            (_, DemoNode::Sink) => {}
            (_, DemoNode::Number(_)) => {
                unreachable!("Number node has no inputs")
            }
            (_, DemoNode::String(_)) => {
                unreachable!("String node has no inputs")
            }
            (DemoNode::Number(_), DemoNode::ShowImage(_)) => {
                return;
            }
            (DemoNode::ShowImage(_), DemoNode::ShowImage(_)) => {
                return;
            }
            (DemoNode::String(_), DemoNode::ShowImage(_)) => {}
            (DemoNode::ExprNode(_), DemoNode::ExprNode(_)) if to.id.input == 0 => {
                return;
            }
            (DemoNode::ExprNode(_), DemoNode::ExprNode(_)) => {}
            (DemoNode::Number(_), DemoNode::ExprNode(_)) if to.id.input == 0 => {
                return;
            }
            (DemoNode::Number(_), DemoNode::ExprNode(_)) => {}
            (DemoNode::String(_), DemoNode::ExprNode(_)) if to.id.input == 0 => {}
            (DemoNode::String(_), DemoNode::ExprNode(_)) => {
                return;
            }
            (DemoNode::ShowImage(_), DemoNode::ExprNode(_)) => {
                return;
            }
            (DemoNode::ExprNode(_), DemoNode::ShowImage(_)) => {
                return;
            }
        }

        for &remote in &to.remotes {
            snarl.disconnect(remote, to.id);
        }

        snarl.connect(from.id, to.id);
    }

    fn title(&mut self, node: &DemoNode) -> String {
        match node {
            DemoNode::Sink => "Sink".to_owned(),
            DemoNode::Number(_) => "Number".to_owned(),
            DemoNode::String(_) => "String".to_owned(),
            DemoNode::ShowImage(_) => "Show image".to_owned(),
            DemoNode::ExprNode(_) => "Expr".to_owned(),
        }
    }

    fn inputs(&mut self, node: &DemoNode) -> usize {
        match node {
            DemoNode::Sink | DemoNode::ShowImage(_) => 1,
            DemoNode::Number(_) | DemoNode::String(_) => 0,
            DemoNode::ExprNode(expr_node) => 1 + expr_node.bindings.len(),
        }
    }

    fn outputs(&mut self, node: &DemoNode) -> usize {
        match node {
            DemoNode::Sink => 0,
            DemoNode::Number(_)
            | DemoNode::String(_)
            | DemoNode::ShowImage(_)
            | DemoNode::ExprNode(_) => 1,
        }
    }

    #[allow(clippy::too_many_lines)]
    #[allow(refining_impl_trait)]
    fn show_input(&mut self, pin: &InPin, ui: &mut Ui, snarl: &mut Snarl<DemoNode>) -> PinInfo {
        match snarl[pin.id.node] {
            DemoNode::Sink => {
                assert_eq!(pin.id.input, 0, "Sink node has only one input");

                match &*pin.remotes {
                    [] => {
                        ui.label("None");
                        PinInfo::circle().with_fill(UNTYPED_COLOR)
                    }
                    [remote] => match snarl[remote.node] {
                        DemoNode::Sink => unreachable!("Sink node has no outputs"),
                        DemoNode::Number(value) => {
                            assert_eq!(remote.output, 0, "Number node has only one output");
                            ui.label(format_float(value));
                            PinInfo::circle().with_fill(NUMBER_COLOR)
                        }
                        DemoNode::String(ref value) => {
                            assert_eq!(remote.output, 0, "String node has only one output");
                            ui.label(format!("{value:?}"));

                            PinInfo::circle().with_fill(STRING_COLOR).with_wire_style(
                                WireStyle::AxisAligned {
                                    corner_radius: 10.0,
                                },
                            )
                        }
                        DemoNode::ExprNode(ref expr) => {
                            assert_eq!(remote.output, 0, "Expr node has only one output");
                            ui.label(format_float(expr.eval()));
                            PinInfo::circle().with_fill(NUMBER_COLOR)
                        }
                        DemoNode::ShowImage(ref uri) => {
                            assert_eq!(remote.output, 0, "ShowImage node has only one output");

                            let image = egui::Image::new(uri).show_loading_spinner(true);
                            ui.add(image);

                            PinInfo::circle().with_fill(IMAGE_COLOR)
                        }
                    },
                    _ => unreachable!("Sink input has only one wire"),
                }
            }
            DemoNode::Number(_) => {
                unreachable!("Number node has no inputs")
            }
            DemoNode::String(_) => {
                unreachable!("String node has no inputs")
            }
            DemoNode::ShowImage(_) => match &*pin.remotes {
                [] => {
                    let input = snarl[pin.id.node].string_in();
                    egui::TextEdit::singleline(input)
                        .clip_text(false)
                        .desired_width(0.0)
                        .margin(ui.spacing().item_spacing)
                        .show(ui);
                    PinInfo::circle().with_fill(STRING_COLOR).with_wire_style(
                        WireStyle::AxisAligned {
                            corner_radius: 10.0,
                        },
                    )
                }
                [remote] => {
                    let new_value = snarl[remote.node].string_out().to_owned();

                    egui::TextEdit::singleline(&mut &*new_value)
                        .clip_text(false)
                        .desired_width(0.0)
                        .margin(ui.spacing().item_spacing)
                        .show(ui);

                    let input = snarl[pin.id.node].string_in();
                    *input = new_value;

                    PinInfo::circle().with_fill(STRING_COLOR).with_wire_style(
                        WireStyle::AxisAligned {
                            corner_radius: 10.0,
                        },
                    )
                }
                _ => unreachable!("Sink input has only one wire"),
            },
            DemoNode::ExprNode(_) if pin.id.input == 0 => {
                let changed = match &*pin.remotes {
                    [] => {
                        let input = snarl[pin.id.node].string_in();
                        let r = egui::TextEdit::singleline(input)
                            .clip_text(false)
                            .desired_width(0.0)
                            .margin(ui.spacing().item_spacing)
                            .show(ui)
                            .response;

                        r.changed()
                    }
                    [remote] => {
                        let new_string = snarl[remote.node].string_out().to_owned();

                        egui::TextEdit::singleline(&mut &*new_string)
                            .clip_text(false)
                            .desired_width(0.0)
                            .margin(ui.spacing().item_spacing)
                            .show(ui);

                        let input = snarl[pin.id.node].string_in();
                        if new_string == *input {
                            false
                        } else {
                            *input = new_string;
                            true
                        }
                    }
                    _ => unreachable!("Expr pins has only one wire"),
                };

                if changed {
                    let expr_node = snarl[pin.id.node].expr_node();

                    if let Ok(expr) = syn::parse_str(&expr_node.text) {
                        expr_node.expr = expr;

                        let values = Iterator::zip(
                            expr_node.bindings.iter().map(String::clone),
                            expr_node.values.iter().copied(),
                        )
                        .collect::<HashMap<String, f64>>();

                        let mut new_bindings = Vec::new();
                        expr_node.expr.extend_bindings(&mut new_bindings);

                        let old_bindings =
                            std::mem::replace(&mut expr_node.bindings, new_bindings.clone());

                        let new_values = new_bindings
                            .iter()
                            .map(|name| values.get(&**name).copied().unwrap_or(0.0))
                            .collect::<Vec<_>>();

                        expr_node.values = new_values;

                        let old_inputs = (0..old_bindings.len())
                            .map(|idx| {
                                snarl.in_pin(InPinId {
                                    node: pin.id.node,
                                    input: idx + 1,
                                })
                            })
                            .collect::<Vec<_>>();

                        for (idx, name) in old_bindings.iter().enumerate() {
                            let new_idx =
                                new_bindings.iter().position(|new_name| *new_name == *name);

                            match new_idx {
                                None => {
                                    snarl.drop_inputs(old_inputs[idx].id);
                                }
                                Some(new_idx) if new_idx != idx => {
                                    let new_in_pin = InPinId {
                                        node: pin.id.node,
                                        input: new_idx,
                                    };
                                    for &remote in &old_inputs[idx].remotes {
                                        snarl.disconnect(remote, old_inputs[idx].id);
                                        snarl.connect(remote, new_in_pin);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                PinInfo::circle()
                    .with_fill(STRING_COLOR)
                    .with_wire_style(WireStyle::AxisAligned {
                        corner_radius: 10.0,
                    })
            }
            DemoNode::ExprNode(ref expr_node) => {
                if pin.id.input <= expr_node.bindings.len() {
                    match &*pin.remotes {
                        [] => {
                            let node = &mut snarl[pin.id.node];
                            ui.label(node.label_in(pin.id.input));
                            ui.add(egui::DragValue::new(node.number_in(pin.id.input)));
                            PinInfo::circle().with_fill(NUMBER_COLOR)
                        }
                        [remote] => {
                            let new_value = snarl[remote.node].number_out();
                            let node = &mut snarl[pin.id.node];
                            ui.label(node.label_in(pin.id.input));
                            ui.label(format_float(new_value));
                            *node.number_in(pin.id.input) = new_value;
                            PinInfo::circle().with_fill(NUMBER_COLOR)
                        }
                        _ => unreachable!("Expr pins has only one wire"),
                    }
                } else {
                    ui.label("Removed");
                    PinInfo::circle().with_fill(Color32::BLACK)
                }
            }
        }
    }

    #[allow(refining_impl_trait)]
    fn show_output(&mut self, pin: &OutPin, ui: &mut Ui, snarl: &mut Snarl<DemoNode>) -> PinInfo {
        match snarl[pin.id.node] {
            DemoNode::Sink => {
                unreachable!("Sink node has no outputs")
            }
            DemoNode::Number(ref mut value) => {
                assert_eq!(pin.id.output, 0, "Number node has only one output");
                ui.add(egui::DragValue::new(value));
                PinInfo::circle().with_fill(NUMBER_COLOR)
            }
            DemoNode::String(ref mut value) => {
                assert_eq!(pin.id.output, 0, "String node has only one output");
                let edit = egui::TextEdit::singleline(value)
                    .clip_text(false)
                    .desired_width(0.0)
                    .margin(ui.spacing().item_spacing);
                ui.add(edit);
                PinInfo::circle()
                    .with_fill(STRING_COLOR)
                    .with_wire_style(WireStyle::AxisAligned {
                        corner_radius: 10.0,
                    })
            }
            DemoNode::ExprNode(ref expr_node) => {
                let value = expr_node.eval();
                assert_eq!(pin.id.output, 0, "Expr node has only one output");
                ui.label(format_float(value));
                PinInfo::circle().with_fill(NUMBER_COLOR)
            }
            DemoNode::ShowImage(_) => {
                ui.allocate_at_least(egui::Vec2::ZERO, egui::Sense::hover());
                PinInfo::circle().with_fill(IMAGE_COLOR)
            }
        }
    }

    fn has_graph_menu(&mut self, _pos: egui::Pos2, _snarl: &mut Snarl<DemoNode>) -> bool {
        true
    }

    fn show_graph_menu(&mut self, pos: egui::Pos2, ui: &mut Ui, snarl: &mut Snarl<DemoNode>) {
        ui.label("Add node");
        if ui.button("Number").clicked() {
            snarl.insert_node(pos, DemoNode::Number(0.0));
            ui.close_menu();
        }
        if ui.button("Expr").clicked() {
            snarl.insert_node(pos, DemoNode::ExprNode(ExprNode::new()));
            ui.close_menu();
        }
        if ui.button("String").clicked() {
            snarl.insert_node(pos, DemoNode::String(String::new()));
            ui.close_menu();
        }
        if ui.button("Show image").clicked() {
            snarl.insert_node(pos, DemoNode::ShowImage(String::new()));
            ui.close_menu();
        }
        if ui.button("Sink").clicked() {
            snarl.insert_node(pos, DemoNode::Sink);
            ui.close_menu();
        }
    }

    fn has_dropped_wire_menu(&mut self, _src_pins: AnyPins, _snarl: &mut Snarl<DemoNode>) -> bool {
        true
    }

    fn show_dropped_wire_menu(
        &mut self,
        pos: egui::Pos2,
        ui: &mut Ui,
        src_pins: AnyPins,
        snarl: &mut Snarl<DemoNode>,
    ) {
        // In this demo, we create a context-aware node graph menu, and connect a wire
        // dropped on the fly based on user input to a new node created.
        //
        // In your implementation, you may want to define specifications for each node's
        // pin inputs and outputs and compatibility to make this easier.

        type PinCompat = usize;
        const PIN_NUM: PinCompat = 1;
        const PIN_STR: PinCompat = 2;
        const PIN_IMG: PinCompat = 4;
        const PIN_SINK: PinCompat = PIN_NUM | PIN_STR | PIN_IMG;

        const fn pin_out_compat(node: &DemoNode) -> PinCompat {
            match node {
                DemoNode::Sink => 0,
                DemoNode::String(_) => PIN_STR,
                DemoNode::ShowImage(_) => PIN_IMG,
                DemoNode::Number(_) | DemoNode::ExprNode(_) => PIN_NUM,
            }
        }

        const fn pin_in_compat(node: &DemoNode, pin: usize) -> PinCompat {
            match node {
                DemoNode::Sink => PIN_SINK,
                DemoNode::Number(_) | DemoNode::String(_) => 0,
                DemoNode::ShowImage(_) => PIN_STR,
                DemoNode::ExprNode(_) => {
                    if pin == 0 {
                        PIN_STR
                    } else {
                        PIN_NUM
                    }
                }
            }
        }

        ui.label("Add node");

        match src_pins {
            AnyPins::Out(src_pins) => {
                assert!(
                    src_pins.len() == 1,
                    "There's no concept of multi-input nodes in this demo"
                );

                let src_pin = src_pins[0];
                let src_out_ty = pin_out_compat(snarl.get_node(src_pin.node).unwrap());
                let dst_in_candidates = [
                    ("Sink", (|| DemoNode::Sink) as fn() -> DemoNode, PIN_SINK),
                    ("Show Image", || DemoNode::ShowImage(String::new()), PIN_STR),
                    ("Expr", || DemoNode::ExprNode(ExprNode::new()), PIN_STR),
                ];

                for (name, ctor, in_ty) in dst_in_candidates {
                    if src_out_ty & in_ty != 0 && ui.button(name).clicked() {
                        // Create new node.
                        let new_node = snarl.insert_node(pos, ctor());
                        let dst_pin = InPinId {
                            node: new_node,
                            input: 0,
                        };

                        // Connect the wire.
                        snarl.connect(src_pin, dst_pin);
                        ui.close_menu();
                    }
                }
            }
            AnyPins::In(pins) => {
                let all_src_types = pins.iter().fold(0, |acc, pin| {
                    acc | pin_in_compat(snarl.get_node(pin.node).unwrap(), pin.input)
                });

                let dst_out_candidates = [
                    (
                        "Number",
                        (|| DemoNode::Number(0.)) as fn() -> DemoNode,
                        PIN_NUM,
                    ),
                    ("String", || DemoNode::String(String::new()), PIN_STR),
                    ("Expr", || DemoNode::ExprNode(ExprNode::new()), PIN_NUM),
                    ("Show Image", || DemoNode::ShowImage(String::new()), PIN_IMG),
                ];

                for (name, ctor, out_ty) in dst_out_candidates {
                    if all_src_types & out_ty != 0 && ui.button(name).clicked() {
                        // Create new node.
                        let new_node = ctor();
                        let dst_ty = pin_out_compat(&new_node);

                        let new_node = snarl.insert_node(pos, new_node);
                        let dst_pin = OutPinId {
                            node: new_node,
                            output: 0,
                        };

                        // Connect the wire.
                        for src_pin in pins {
                            let src_ty =
                                pin_in_compat(snarl.get_node(src_pin.node).unwrap(), src_pin.input);
                            if src_ty & dst_ty != 0 {
                                // In this demo, input pin MUST be unique ...
                                // Therefore here we drop inputs of source input pin.
                                snarl.drop_inputs(*src_pin);
                                snarl.connect(dst_pin, *src_pin);
                                ui.close_menu();
                            }
                        }
                    }
                }
            }
        };
    }

    fn has_node_menu(&mut self, _node: &DemoNode) -> bool {
        true
    }

    fn show_node_menu(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        snarl: &mut Snarl<DemoNode>,
    ) {
        ui.label("Node menu");
        if ui.button("Remove").clicked() {
            snarl.remove_node(node);
            ui.close_menu();
        }
    }

    fn has_on_hover_popup(&mut self, _: &DemoNode) -> bool {
        true
    }

    fn show_on_hover_popup(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        snarl: &mut Snarl<DemoNode>,
    ) {
        match snarl[node] {
            DemoNode::Sink => {
                ui.label("Displays anything connected to it");
            }
            DemoNode::Number(_) => {
                ui.label("Outputs integer value");
            }
            DemoNode::String(_) => {
                ui.label("Outputs string value");
            }
            DemoNode::ShowImage(_) => {
                ui.label("Displays image from URL in input");
            }
            DemoNode::ExprNode(_) => {
                ui.label("Evaluates algebraic expression with input for each unique variable name");
            }
        }
    }

    fn header_frame(
        &mut self,
        frame: egui::Frame,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        snarl: &Snarl<DemoNode>,
    ) -> egui::Frame {
        match snarl[node] {
            DemoNode::Sink => frame.fill(egui::Color32::from_rgb(70, 70, 80)),
            DemoNode::Number(_) => frame.fill(egui::Color32::from_rgb(70, 40, 40)),
            DemoNode::String(_) => frame.fill(egui::Color32::from_rgb(40, 70, 40)),
            DemoNode::ShowImage(_) => frame.fill(egui::Color32::from_rgb(40, 40, 70)),
            DemoNode::ExprNode(_) => frame.fill(egui::Color32::from_rgb(70, 66, 40)),
        }
    }
}

fn format_float(v: f64) -> String {
    let v = (v * 1000.0).round() / 1000.0;
    format!("{v}")
}
