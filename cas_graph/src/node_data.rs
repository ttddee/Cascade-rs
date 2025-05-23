use egui::Pos2;

pub struct NodeData {
    position: Pos2,
}

impl NodeData {
    pub fn new(position: Pos2) -> Self {
        Self { position: position }
    }

    pub fn position(&self) -> Pos2 {
        self.position
    }
}
