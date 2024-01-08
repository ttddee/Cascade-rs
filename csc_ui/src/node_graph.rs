use egui_node_graph::GraphEditorState;

use csc_core::{
    graph_model::MyGraphState,
    node_model::{AllNodeTemplates, ImageType, MyNodeData, MyValueType, NodeType},
};

pub fn build_node_graph(
    context: &egui::Context,
    graph_state: &mut GraphEditorState<MyNodeData, ImageType, MyValueType, NodeType, MyGraphState>,
    user_state: &mut MyGraphState,
) {
    let _graph_response = egui::TopBottomPanel::bottom("nodegraph_panel")
        .default_height(400.)
        .resizable(true)
        .show(context, |ui| {
            graph_state.draw_graph_editor(ui, AllNodeTemplates, user_state, Vec::default())
        })
        .inner;
    //for node_response in graph_response.node_responses {
    // Here, we ignore all other graph events. But you may find
    // some use for them. For example, by playing a sound when a new
    // connection is created
    // if let NodeResponse::User(user_event) = node_response {
    //     match user_event {
    //         MyResponse::SetActiveNode(node) => user_state.active_node = Some(node),
    //         MyResponse::ClearActiveNode => user_state.active_node = None,
    //     }
    // }
    //if let NodeResponse::User()
    //}

    // if let Some(node) = user_state.active_node {
    //     if graph_editor_state.graph.nodes.contains_key(node) {
    //         // let text = match evaluate_node(&graph_editor_state.graph, node, &mut HashMap::new()) {
    //         //     Ok(value) => format!("The result is: {:?}", value),
    //         //     Err(err) => format!("Execution error: {}", err),
    //         // };
    //         // ctx.debug_painter().text(
    //         //     egui::pos2(10.0, 35.0),
    //         //     egui::Align2::LEFT_TOP,
    //         //     text,
    //         //     TextStyle::Button.resolve(&ctx.style()),
    //         //     egui::Color32::WHITE,
    //         // );
    //     } else {
    //         user_state.active_node = None;
    //     }
    // }
}
