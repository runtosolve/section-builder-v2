//! Section geometry construction.
//!
//! Given an ordered polyline of *sharp-corner* centerline points and a single
//! corner radius, this module produces a CUFSM-compatible finite-strip model
//! (nodes + elements) by:
//!
//! 1. Computing the deflection angle at each interior vertex,
//! 2. Trimming each adjacent edge by `offset = r * tan(bend / 2)`,
//! 3. Inserting three intermediate points along the resulting circular arc
//!    (so each rounded corner contributes four sub-elements), and
//! 4. Subdividing each straight portion into `elements_per_segment` pieces.
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
/// * `elements_per_segment` – number of sub-elements per straight segment
///   (between rounded corners). Must be `>= 1`. Arc portions always contribute
///   four sub-elements (three interior points) when `radius > 0`.
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
    elements_per_segment: usize,
) -> FiniteStripModel {
    assert!(points.len() >= 2, "need at least two points");
    assert!(elements_per_segment >= 1, "elements_per_segment must be >= 1");

    let polyline = round_polyline(points, radius);
    let densified = mesh(&polyline, elements_per_segment);
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
