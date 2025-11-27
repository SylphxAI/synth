//! Tree structure compatible with @sylphx/synth TypeScript package
//!
//! Uses arena-based storage for cache-friendly memory layout.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

use crate::position::Span;
use crate::error::{SynthError, SynthResult};

/// Get current timestamp (works in both WASM and native)
fn now() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now() as u64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }
}

/// Node ID type (index into the arena)
pub type NodeId = u32;

/// A node in the AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier (index in arena)
    pub id: NodeId,
    /// Node type (e.g., "heading", "paragraph", "text")
    #[serde(rename = "type")]
    pub node_type: String,
    /// Parent node ID (None for root)
    pub parent: Option<NodeId>,
    /// Child node IDs
    pub children: Vec<NodeId>,
    /// Source span
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
    /// Node-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
}

impl Node {
    /// Create a new node
    pub fn new(id: NodeId, node_type: impl Into<String>) -> Self {
        Self {
            id,
            node_type: node_type.into(),
            parent: None,
            children: Vec::new(),
            span: None,
            data: None,
        }
    }

    /// Set the parent
    pub fn with_parent(mut self, parent: NodeId) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Set the span
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Set data
    pub fn with_data(mut self, data: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(data);
        self
    }
}

/// Tree metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeMetadata {
    /// Language/format of the source
    pub language: String,
    /// Original source text
    pub source: String,
    /// Creation timestamp (ms since epoch)
    pub created: u64,
    /// Last modified timestamp (ms since epoch)
    pub modified: u64,
}

/// AST Tree structure
///
/// Uses arena-based storage for efficient memory layout:
/// - All nodes stored in a flat Vec
/// - Node IDs are array indices
/// - Cache-friendly iteration
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    /// Tree metadata
    pub(crate) meta: TreeMetadata,
    /// Root node ID (always 0)
    pub(crate) root: NodeId,
    /// Node storage (arena)
    pub(crate) nodes: Vec<Node>,
}

#[wasm_bindgen]
impl Tree {
    /// Create a new empty tree
    #[wasm_bindgen(constructor)]
    pub fn new(language: &str, source: &str) -> Self {
        let timestamp = now();

        let root = Node::new(0, "root");

        Self {
            meta: TreeMetadata {
                language: language.to_string(),
                source: source.to_string(),
                created: timestamp,
                modified: timestamp,
            },
            root: 0,
            nodes: vec![root],
        }
    }

    /// Get the root node ID
    #[wasm_bindgen(getter)]
    pub fn root_id(&self) -> NodeId {
        self.root
    }

    /// Get the number of nodes
    #[wasm_bindgen(getter)]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get the language
    #[wasm_bindgen(getter)]
    pub fn language(&self) -> String {
        self.meta.language.clone()
    }

    /// Get the source
    #[wasm_bindgen(getter)]
    pub fn source(&self) -> String {
        self.meta.source.clone()
    }

    /// Serialize tree to JSON
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(self)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Deserialize tree from JSON
    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: JsValue) -> Result<Tree, JsValue> {
        serde_wasm_bindgen::from_value(json)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

impl Tree {
    /// Add a node to the tree, returns the new node ID
    pub fn add_node(&mut self, mut node: Node) -> NodeId {
        let id = self.nodes.len() as NodeId;
        node.id = id;
        self.nodes.push(node);
        self.meta.modified = now();
        id
    }

    /// Get a node by ID
    pub fn get_node(&self, id: NodeId) -> SynthResult<&Node> {
        self.nodes.get(id as usize)
            .ok_or(SynthError::InvalidNodeId(id))
    }

    /// Get a mutable node by ID
    pub fn get_node_mut(&mut self, id: NodeId) -> SynthResult<&mut Node> {
        self.nodes.get_mut(id as usize)
            .ok_or(SynthError::InvalidNodeId(id))
    }

    /// Add a child to a parent node
    pub fn add_child(&mut self, parent_id: NodeId, child_id: NodeId) -> SynthResult<()> {
        // Set child's parent
        self.get_node_mut(child_id)?.parent = Some(parent_id);
        // Add to parent's children
        self.get_node_mut(parent_id)?.children.push(child_id);
        Ok(())
    }

    /// Get all nodes (for iteration)
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_creation() {
        let tree = Tree::new("markdown", "# Hello");
        assert_eq!(tree.node_count(), 1);
        assert_eq!(tree.language(), "markdown");
    }

    #[test]
    fn test_add_node() {
        let mut tree = Tree::new("markdown", "# Hello");

        let heading = Node::new(0, "heading");
        let id = tree.add_node(heading);

        assert_eq!(id, 1);
        assert_eq!(tree.node_count(), 2);
    }
}
