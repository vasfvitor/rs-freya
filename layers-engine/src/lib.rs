use dioxus_native_core::real_dom::Node;
use state::node::{NodeState, SizeMode};
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct NodeData {
    pub width: SizeMode,
    pub height: SizeMode,
    pub padding: (i32, i32, i32, i32),
    pub node: Option<Node<NodeState>>,
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Default, Clone, Debug)]
pub struct Layers {
    pub layers: HashMap<u16, Vec<RenderData>>,
}

#[derive(Default, Clone, Debug)]
pub struct RenderData {
    pub node: NodeData,
    pub viewport: Viewport,
    pub parent_viewport: Viewport,
}

impl Layers {
    pub fn add_element(
        &mut self,
        node: &NodeData,
        viewport: &Viewport,
        parent_viewport: &Viewport,
        layer_num: i16,
    ) -> i16 {
        let node_data = node.node.as_ref().unwrap();
        let n_layer = {
            if (node_data.height as i16) + node_data.state.style.z_index < 0 {
                0
            } else {
                (-node_data.state.style.z_index + (node_data.height as i16) - layer_num) as u16
            }
        };

        if !self.layers.contains_key(&n_layer) {
            self.layers.insert(n_layer, vec![]);
        }
        let layer = self.layers.get_mut(&n_layer).unwrap();
        layer.push(RenderData {
            node: node.clone(),
            viewport: viewport.clone(),
            parent_viewport: parent_viewport.clone(),
        });
        node_data.state.style.z_index + layer_num
    }
}