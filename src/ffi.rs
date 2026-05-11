//! C ABI exposing `build_finite_strip_model` for Julia (and other) consumers.
//!
//! Lifetime model: the builder returns an opaque `*mut FiniteStripModel`
//! handle owned by Rust. Callers query sizes, copy node/element tables into
//! caller-allocated buffers of `CNodeRow` / `CElementRow`, then release with
//! [`sb_fsm_free`].
//!
//! All raw-pointer dereferencing is confined to the three helpers
//! [`as_model`], [`as_slice`], and [`as_out_slice`]; the public `extern "C"`
//! functions themselves contain no `unsafe` blocks.

use crate::cufsm::{ElementRow, FiniteStripModel, NodeRow};
use crate::geometry::{Vec2, build_finite_strip_model};

#[repr(C)]
pub struct CNodeRow {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub xdof: u8,
    pub ydof: u8,
    pub zdof: u8,
    pub qdof: u8,
    pub stress: f64,
}

#[repr(C)]
pub struct CElementRow {
    pub id: usize,
    pub node_i: usize,
    pub node_j: usize,
    pub thickness: f64,
    pub material: u32,
}

impl From<&NodeRow> for CNodeRow {
    fn from(n: &NodeRow) -> Self {
        Self {
            id: n.id,
            x: n.x,
            y: n.y,
            xdof: n.xdof,
            ydof: n.ydof,
            zdof: n.zdof,
            qdof: n.qdof,
            stress: n.stress,
        }
    }
}

impl From<&ElementRow> for CElementRow {
    fn from(e: &ElementRow) -> Self {
        Self {
            id: e.id,
            node_i: e.node_i,
            node_j: e.node_j,
            thickness: e.thickness,
            material: e.material,
        }
    }
}

// --- Trust boundary -------------------------------------------------------
//
// Every raw-pointer dereference in this module lives in one of the three
// helpers below. Audit these and the rest of the file is safe Rust.

/// Borrow a model handle, or `None` if null.
///
/// # Safety
/// `p` must be null or a valid pointer to a `FiniteStripModel` that outlives
/// the returned borrow (i.e. obtained from `sb_build_finite_strip_model` and
/// not yet freed).
unsafe fn as_model<'a>(p: *const FiniteStripModel) -> Option<&'a FiniteStripModel> {
    if p.is_null() {
        None
    } else {
        Some(unsafe { &*p })
    }
}

/// View a caller-supplied input array as a slice, or `None` if null/empty.
///
/// # Safety
/// `ptr` must be null or point to at least `len` valid `T`s for reads.
unsafe fn as_slice<'a, T>(ptr: *const T, len: usize) -> Option<&'a [T]> {
    if ptr.is_null() || len == 0 {
        None
    } else {
        Some(unsafe { std::slice::from_raw_parts(ptr, len) })
    }
}

/// View a caller-supplied output buffer as a mutable slice, or `None` if null/empty.
///
/// # Safety
/// `ptr` must be null or point to at least `len` writable, non-aliased `T`s.
unsafe fn as_out_slice<'a, T>(ptr: *mut T, len: usize) -> Option<&'a mut [T]> {
    if ptr.is_null() || len == 0 {
        None
    } else {
        Some(unsafe { std::slice::from_raw_parts_mut(ptr, len) })
    }
}

// --- Public C ABI ---------------------------------------------------------

/// Build a finite-strip model from an interleaved `[x0, y0, x1, y1, ...]`
/// array of `n_points` points. Returns an owning handle, or null on bad
/// input. Free with [`sb_fsm_free`].
///
/// # Safety
/// `points_xy` must point to at least `2 * n_points` valid `f64`s.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sb_build_finite_strip_model(
    points_xy: *const f64,
    n_points: usize,
    radius: f64,
    thickness: f64,
    target_element_size: f64,
) -> *mut FiniteStripModel {
    if n_points < 2 || !(target_element_size > 0.0) {
        return std::ptr::null_mut();
    }
    let Some(raw) = (unsafe { as_slice(points_xy, n_points * 2) }) else {
        return std::ptr::null_mut();
    };
    let pts: Vec<Vec2> = raw
        .chunks_exact(2)
        .map(|c| Vec2::new(c[0], c[1]))
        .collect();
    let model = build_finite_strip_model(&pts, radius, thickness, target_element_size);
    Box::into_raw(Box::new(model))
}

/// Number of node rows. Returns 0 if `model` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sb_fsm_num_nodes(model: *const FiniteStripModel) -> usize {
    unsafe { as_model(model) }.map_or(0, |m| m.nodes.len())
}

/// Number of element rows. Returns 0 if `model` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sb_fsm_num_elements(model: *const FiniteStripModel) -> usize {
    unsafe { as_model(model) }.map_or(0, |m| m.elements.len())
}

/// Copy all node rows into the caller-owned `out` buffer, which must hold
/// at least `sb_fsm_num_nodes(model)` `CNodeRow`s.
///
/// # Safety
/// `out` must be valid for `sb_fsm_num_nodes(model)` writes of `CNodeRow`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sb_fsm_copy_nodes(model: *const FiniteStripModel, out: *mut CNodeRow) {
    let Some(m) = (unsafe { as_model(model) }) else { return };
    let Some(dst) = (unsafe { as_out_slice(out, m.nodes.len()) }) else { return };
    for (slot, n) in dst.iter_mut().zip(m.nodes.iter()) {
        *slot = CNodeRow::from(n);
    }
}

/// Copy all element rows into the caller-owned `out` buffer, which must hold
/// at least `sb_fsm_num_elements(model)` `CElementRow`s.
///
/// # Safety
/// `out` must be valid for `sb_fsm_num_elements(model)` writes of `CElementRow`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sb_fsm_copy_elements(
    model: *const FiniteStripModel,
    out: *mut CElementRow,
) {
    let Some(m) = (unsafe { as_model(model) }) else { return };
    let Some(dst) = (unsafe { as_out_slice(out, m.elements.len()) }) else { return };
    for (slot, e) in dst.iter_mut().zip(m.elements.iter()) {
        *slot = CElementRow::from(e);
    }
}

/// Release a model handle returned by [`sb_build_finite_strip_model`].
///
/// # Safety
/// `model` must be a handle returned by `sb_build_finite_strip_model` and
/// not already freed. Passing null is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sb_fsm_free(model: *mut FiniteStripModel) {
    if !model.is_null() {
        drop(unsafe { Box::from_raw(model) });
    }
}
