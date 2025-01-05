use std::collections::HashMap;
use crate::math::{Vec3, Mat4};
use crate::geometry::Mesh;

type NodeId = usize;

#[derive(Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub local_matrix: Mat4,
    pub world_matrix: Mat4,  // Make this public
    dirty: bool,
}


impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            local_matrix: Mat4::identity(),
            world_matrix: Mat4::identity(),
            dirty: true,
        }
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.dirty = true;
    }

    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
        self.dirty = true;
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
        self.dirty = true;
    }

    fn update_local_matrix(&mut self) {
        if self.dirty {
            // Create transformation matrices
            let translation = Mat4::translation(self.position.x, self.position.y, self.position.z);
            let rotation_x = Mat4::rotation_x(self.rotation.x);
            let rotation_y = Mat4::rotation_y(self.rotation.y);
            let rotation_z = Mat4::rotation_z(self.rotation.z);
            let scale = Mat4::scaling(self.scale.x, self.scale.y, self.scale.z);

            // Combine matrices: T * Rz * Ry * Rx * S
            self.local_matrix = translation
                .multiply(&rotation_z)
                .multiply(&rotation_y)
                .multiply(&rotation_x)
                .multiply(&scale);

            self.dirty = false;
        }
    }
}

#[derive(Debug)]
pub struct SceneNode {
    pub id: NodeId,
    pub name: String,
    pub transform: Transform,
    pub mesh: Option<Mesh>,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub visible: bool,
}

impl SceneNode {
    pub fn new(id: NodeId, name: String) -> Self {
        Self {
            id,
            name,
            transform: Transform::new(),
            mesh: None,
            parent: None,
            children: Vec::new(),
            visible: true,
        }
    }
}

pub struct Scene {
    nodes: HashMap<NodeId, SceneNode>,
    root_nodes: Vec<NodeId>,
    next_id: NodeId,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root_nodes: Vec::new(),
            next_id: 0,
        }
    }

    pub fn create_node(&mut self, name: String) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;

        let node = SceneNode::new(id, name);
        self.nodes.insert(id, node);
        self.root_nodes.push(id);
        id
    }

    pub fn create_mesh_node(&mut self, name: String, mesh: Mesh) -> NodeId {
        let id = self.create_node(name);
        if let Some(node) = self.nodes.get_mut(&id) {
            node.mesh = Some(mesh);
        }
        id
    }

    pub fn set_parent(&mut self, child_id: NodeId, parent_id: NodeId) {
        // Remove from previous parent or root
        if let Some(node) = self.nodes.get(&child_id) {
            if let Some(old_parent) = node.parent {
                if let Some(parent_node) = self.nodes.get_mut(&old_parent) {
                    parent_node.children.retain(|&id| id != child_id);
                }
            } else {
                self.root_nodes.retain(|&id| id != child_id);
            }
        }

        // Update parent-child relationships
        if let Some(child_node) = self.nodes.get_mut(&child_id) {
            child_node.parent = Some(parent_id);
        }
        if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
            parent_node.children.push(child_id);
        }
    }

    pub fn update_transforms(&mut self) {
        // Start from root nodes and traverse the hierarchy
        for &root_id in &self.root_nodes.clone() {
            self.update_node_transform(root_id, Mat4::identity());
        }
    }

    fn update_node_transform(&mut self, node_id: NodeId, parent_world: Mat4) {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            // Update local matrix if necessary
            node.transform.update_local_matrix();

            // Calculate world matrix
            let world_matrix = parent_world.multiply(&node.transform.local_matrix);
            node.transform.world_matrix = world_matrix.clone();

            // Update children
            let children = node.children.clone();
            for child_id in children {
                self.update_node_transform(child_id, world_matrix.clone());
            }
        }
    }

    pub fn get_node(&self, id: NodeId) -> Option<&SceneNode> {
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut SceneNode> {
        self.nodes.get_mut(&id)
    }

    pub fn get_world_transform(&self, id: NodeId) -> Option<Mat4> {
        self.nodes.get(&id).map(|node| node.transform.world_matrix.clone())
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &SceneNode> {
        self.nodes.values()
    }

    pub fn traverse_visible<F>(&self, mut callback: F)
    where
        F: FnMut(&SceneNode),
    {
        for &root_id in &self.root_nodes {
            self.traverse_node(root_id, &mut callback);
        }
    }

    fn traverse_node<F>(&self, node_id: NodeId, callback: &mut F)
    where
        F: FnMut(&SceneNode),
    {
        if let Some(node) = self.nodes.get(&node_id) {
            if node.visible {
                callback(node);
                for &child_id in &node.children {
                    self.traverse_node(child_id, callback);
                }
            }
        }
    }

    // Scene management utilities
    pub fn remove_node(&mut self, id: NodeId) {
        if let Some(node) = self.nodes.remove(&id) {
            // Remove from parent's children
            if let Some(parent_id) = node.parent {
                if let Some(parent) = self.nodes.get_mut(&parent_id) {
                    parent.children.retain(|&child| child != id);
                }
            } else {
                self.root_nodes.retain(|&root| root != id);
            }

            // Recursively remove children
            for child_id in node.children {
                self.remove_node(child_id);
            }
        }
    }

    pub fn find_node_by_name(&self, name: &str) -> Option<NodeId> {
        self.nodes.iter()
            .find(|(_, node)| node.name == name)
            .map(|(&id, _)| id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_creation() {
        let mut scene = Scene::new();
        let node_id = scene.create_node("test".to_string());
        assert!(scene.get_node(node_id).is_some());
    }

    #[test]
    fn test_parent_child_relationship() {
        let mut scene = Scene::new();
        let parent_id = scene.create_node("parent".to_string());
        let child_id = scene.create_node("child".to_string());

        scene.set_parent(child_id, parent_id);

        let parent = scene.get_node(parent_id).unwrap();
        assert!(parent.children.contains(&child_id));

        let child = scene.get_node(child_id).unwrap();
        assert_eq!(child.parent, Some(parent_id));
    }

    #[test]
    fn test_transform_hierarchy() {
        let mut scene = Scene::new();
        let parent_id = scene.create_node("parent".to_string());
        let child_id = scene.create_node("child".to_string());

        if let Some(node) = scene.get_node_mut(parent_id) {
            node.transform.set_position(Vec3::new(1.0, 0.0, 0.0));
        }

        scene.set_parent(child_id, parent_id);
        if let Some(node) = scene.get_node_mut(child_id) {
            node.transform.set_position(Vec3::new(0.0, 1.0, 0.0));
        }

        scene.update_transforms();

        // Child's world position should be affected by parent's position
        if let Some(world_transform) = scene.get_world_transform(child_id) {
            let world_pos = world_transform.transform_vec3(&Vec3::zero());
            assert!((world_pos.x - 1.0).abs() < 1e-10);
            assert!((world_pos.y - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_node_removal() {
        let mut scene = Scene::new();
        let parent_id = scene.create_node("parent".to_string());
        let child_id = scene.create_node("child".to_string());

        scene.set_parent(child_id, parent_id);
        scene.remove_node(parent_id);

        assert!(scene.get_node(parent_id).is_none());
        assert!(scene.get_node(child_id).is_none());
    }
}
