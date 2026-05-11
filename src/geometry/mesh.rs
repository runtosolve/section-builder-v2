use super::Vec2;
use super::rounding::Vertex;

/// Walk the rounded polyline, subdividing every straight run into
/// `n_per_seg` equal pieces. Arc vertices are passed through.
pub(super) fn mesh(vertices: &[Vertex], n_per_seg: usize) -> Vec<Vec2> {
    let mut out: Vec<Vec2> = Vec::new();
    out.push(vertices[0].point());

    for i in 1..vertices.len() {
        let prev = vertices[i - 1];
        let here = vertices[i];

        if prev.is_straight() && here.is_straight() && n_per_seg > 1 {
            let a = prev.point();
            let b = here.point();
            for k in 1..n_per_seg {
                let t = (k as f64) / (n_per_seg as f64);
                out.push(Vec2::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t));
            }
        }
        out.push(here.point());
    }

    out
}
