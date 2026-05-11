//! Section geometry construction.
//!
//! Given an ordered polyline of *sharp-corner* centerline points and a single
//! corner radius, this module produces a CUFSM-compatible finite-strip model
//! (nodes + elements) by:
//!
//! 1. Computing the deflection angle at each interior vertex,
//! 2. Trimming each adjacent edge by `offset = r * tan(bend / 2)`,
//! 3. Inserting intermediate points along the resulting circular arc
//!    (at least 4 sub-elements per 90° of bend, and finer if needed to honor
//!    `target_element_size`), and
//! 4. Subdividing each straight portion into pieces no longer than
//!    `target_element_size`, with a minimum of 2 sub-elements per straight run.
//!
//! The MATLAB reference in `matlab/` uses an OO chain of `Line`/`Corner`
//! objects; here the same idea is expressed as plain vector math on `Vec2`,
//! which is more idiomatic and avoids the recursive class graph.

mod mesh;
mod rounding;
mod vec2;

pub use vec2::Vec2;

use crate::cufsm::{ElementRow, FiniteStripModel, NodeRow, DEFAULT_MATERIAL};
use mesh::mesh;
use rounding::round_polyline;

/// Build a CUFSM finite-strip model from a sharp-corner polyline by rounding
/// every interior vertex with the same radius `r`.
///
/// # Arguments
/// * `points` – ordered centerline points of the section, *with* sharp
///   corners (i.e. the imaginary intersection points if the section had
///   `r = 0`). The polyline is treated as open: endpoints stay sharp.
/// * `radius` – corner radius applied uniformly to every interior vertex.
///   Use `0.0` for a fully sharp section.
/// * `thickness` – plate thickness written into every element row.
/// * `target_element_size` – approximate target length of every sub-element.
///   Must be `> 0`. Straight runs get `max(2, ceil(length / target_element_size))`
///   pieces; arcs get `max(ceil(4 * bend / 90°), ceil(arc_length / target_element_size))`
///   pieces — so every 90° of bend always contributes at least 4 sub-elements.
///
/// # Preconditions
/// * At least two points.
/// * For every interior vertex the trim offset
///   `r * tan(bend / 2)` must be shorter than each adjacent segment;
///   otherwise corners would overlap. No check is performed — the caller
///   is responsible for supplying a feasible radius.
pub fn build_finite_strip_model(
    points: &[Vec2],
    radius: f64,
    thickness: f64,
    target_element_size: f64,
) -> FiniteStripModel {
    assert!(points.len() >= 2, "need at least two points");
    assert!(
        target_element_size > 0.0,
        "target_element_size must be > 0"
    );

    let polyline = round_polyline(points, radius, target_element_size);
    let densified = mesh(&polyline, target_element_size);
    to_cufsm_model(&densified, thickness)
}

/// Pack a list of polyline points into CUFSM `node` / `elem` rows.
fn to_cufsm_model(points: &[Vec2], thickness: f64) -> FiniteStripModel {
    let nodes: Vec<NodeRow> = points
        .iter()
        .enumerate()
        .map(|(i, p)| NodeRow {
            id: i + 1,
            x: p.x,
            y: p.y,
            xdof: 1,
            ydof: 1,
            zdof: 1,
            qdof: 1,
            stress: 0.0,
        })
        .collect();

    let elements: Vec<ElementRow> = (0..nodes.len().saturating_sub(1))
        .map(|i| ElementRow {
            id: i + 1,
            node_i: i + 1,
            node_j: i + 2,
            thickness,
            material: DEFAULT_MATERIAL,
        })
        .collect();

    FiniteStripModel { nodes, elements }
}
