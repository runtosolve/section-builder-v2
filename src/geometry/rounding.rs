use super::Vec2;
use std::f64::consts::FRAC_PI_2;

/// A vertex emitted by [`round_polyline`]: either a corner anchor on a
/// straight portion or a point that lies on an arc.
#[derive(Debug, Clone, Copy)]
pub(super) enum Vertex {
    /// Endpoint of a straight segment. Two consecutive `Straight` vertices
    /// are connected by a single line that should be densified.
    Straight(Vec2),
    /// A point already lying on an arc — emit as-is (no further subdivision).
    Arc(Vec2),
}

impl Vertex {
    pub(super) fn point(self) -> Vec2 {
        match self {
            Vertex::Straight(p) | Vertex::Arc(p) => p,
        }
    }

    pub(super) fn is_straight(self) -> bool {
        matches!(self, Vertex::Straight(_))
    }
}

/// Replace each interior vertex of `points` with: (start-of-arc anchor on the
/// incoming edge, `n_arc - 1` interior arc points, end-of-arc anchor on the
/// outgoing edge). Returns a flat list of `Vertex`es ready for densification.
///
/// `n_arc` per corner is `max(ceil(4 * bend / 90°), ceil(arc_length / target_element_size))`,
/// guaranteeing at least 4 sub-elements per 90° of bend.
pub(super) fn round_polyline(
    points: &[Vec2],
    radius: f64,
    target_element_size: f64,
) -> Vec<Vertex> {
    let n = points.len();
    let mut out = Vec::with_capacity(n * 4);
    out.push(Vertex::Straight(points[0]));

    for i in 1..n - 1 {
        let prev = points[i - 1];
        let curr = points[i];
        let next = points[i + 1];

        let v_in = curr.sub(prev);
        let v_out = next.sub(curr);

        // Deflection angle in (-pi, pi]. cross > 0 means CCW (left turn),
        // cross < 0 means CW (right turn).
        let cross = v_in.x * v_out.y - v_in.y * v_out.x;
        let dot = v_in.x * v_out.x + v_in.y * v_out.y;
        let bend = cross.atan2(dot); // signed
        let bend_abs = bend.abs();

        // Collinear or zero radius → keep the sharp vertex.
        if radius <= 0.0 || bend_abs < 1e-12 {
            out.push(Vertex::Straight(curr));
            continue;
        }

        let offset = radius * (bend_abs / 2.0).tan();
        let u_in = v_in.normalized();
        let u_out = v_out.normalized();

        let arc_start = curr.sub(u_in.scale(offset));
        let arc_end = curr.add(u_out.scale(offset));

        // Arc center is perpendicular to the incoming edge at `arc_start`,
        // toward the inside of the bend.
        let perp = if cross >= 0.0 {
            Vec2::new(-u_in.y, u_in.x)
        } else {
            Vec2::new(u_in.y, -u_in.x)
        };
        let center = arc_start.add(perp.scale(radius));

        let radial = arc_start.sub(center);
        let sweep = bend; // signed

        // n_arc: at least 4 sub-elements per 90° of bend, plus respect target size.
        let n_by_angle = (4.0 * bend_abs / FRAC_PI_2).ceil() as usize;
        let arc_length = radius * bend_abs;
        let n_by_size = (arc_length / target_element_size).ceil() as usize;
        let n_arc = n_by_angle.max(n_by_size).max(1);

        out.push(Vertex::Straight(arc_start));
        for k in 1..n_arc {
            let theta = sweep * (k as f64) / (n_arc as f64);
            out.push(Vertex::Arc(center.add(radial.rotate(theta))));
        }
        out.push(Vertex::Straight(arc_end));
    }

    out.push(Vertex::Straight(points[n - 1]));
    out
}
