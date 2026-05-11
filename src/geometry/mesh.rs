use super::Vec2;
use super::rounding::Vertex;

/// Walk the rounded polyline, subdividing every straight run between two
/// `Straight` anchors into `max(2, ceil(length / target_element_size))` equal
/// pieces. Arc vertices are passed through.
pub(super) fn mesh(vertices: &[Vertex], target_element_size: f64) -> Vec<Vec2> {
    let mut out: Vec<Vec2> = Vec::new();
    out.push(vertices[0].point());

    for i in 1..vertices.len() {
        let prev = vertices[i - 1];
        let here = vertices[i];

        if prev.is_straight() && here.is_straight() {
            let a = prev.point();
            let b = here.point();
            let length = b.sub(a).length();
            let n = ((length / target_element_size).ceil() as usize).max(2);
            for k in 1..n {
                let t = (k as f64) / (n as f64);
                out.push(Vec2::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t));
            }
        }
        out.push(here.point());
    }

    out
}
