//! CUFSM finite-strip model row types.
//!
//! These mirror the `node` and `elem` matrices in the MATLAB CUFSM reference
//! (see `getCee.m`) and define the file-format contract independently of how
//! the geometry was generated.

/// One row of the CUFSM `node` matrix:
/// `[id, x, y, xdof, ydof, zdof, qdof, stress]`.
#[derive(Debug, Clone, Copy)]
pub struct NodeRow {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub xdof: u8,
    pub ydof: u8,
    pub zdof: u8,
    pub qdof: u8,
    pub stress: f64,
}

/// One row of the CUFSM `elem` matrix:
/// `[id, node_i, node_j, thickness, material]`.
#[derive(Debug, Clone, Copy)]
pub struct ElementRow {
    pub id: usize,
    pub node_i: usize,
    pub node_j: usize,
    pub thickness: f64,
    pub material: u32,
}

/// CUFSM finite-strip model: a node table and an element table.
#[derive(Debug, Clone)]
pub struct FiniteStripModel {
    pub nodes: Vec<NodeRow>,
    pub elements: Vec<ElementRow>,
}

/// Default material id used by the MATLAB reference (`100`).
pub(crate) const DEFAULT_MATERIAL: u32 = 100;
