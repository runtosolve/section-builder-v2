use section_builder_v2::{Vec2, build_finite_strip_model};

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}

#[test]
fn straight_line_no_corners() {
    let pts = [Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0)];
    let m = build_finite_strip_model(&pts, 0.0, 0.1, 5);
    assert_eq!(m.nodes.len(), 6);
    assert_eq!(m.elements.len(), 5);
    assert!(approx(m.nodes[3].x, 6.0, 1e-9));
    assert!(approx(m.nodes[3].y, 0.0, 1e-9));
}

#[test]
fn right_angle_rounds_to_arc() {
    // L shape with a 90° interior bend; r=1 means offset = tan(45°)=1.
    let pts = [
        Vec2::new(0.0, 0.0),
        Vec2::new(5.0, 0.0),
        Vec2::new(5.0, 5.0),
    ];
    let m = build_finite_strip_model(&pts, 1.0, 0.1, 1);

    // All arc points should be at distance r from the arc center (4, 1).
    let center = Vec2::new(4.0, 1.0);
    for n in &m.nodes {
        let d = ((n.x - center.x).powi(2) + (n.y - center.y).powi(2)).sqrt();
        if d < 1.5 {
            assert!(approx(d, 1.0, 1e-9), "node {} off arc: d={}", n.id, d);
        }
    }

    assert!(m.nodes.iter().any(|n| approx(n.x, 4.0, 1e-9) && approx(n.y, 0.0, 1e-9)));
    assert!(m.nodes.iter().any(|n| approx(n.x, 5.0, 1e-9) && approx(n.y, 1.0, 1e-9)));
}

#[test]
fn cee_like_section_topology() {
    // Lipped Cee skeleton (sharp corners): top lip, top flange, web,
    // bottom flange, bottom lip. Opening faces +x.
    let h = 8.0;
    let b = 2.0;
    let d = 0.5;
    let pts = [
        Vec2::new(b, h / 2.0),
        Vec2::new(b, h / 2.0 - d),
        Vec2::new(0.0, h / 2.0 - d),
        Vec2::new(0.0, -h / 2.0 + d),
        Vec2::new(b, -h / 2.0 + d),
        Vec2::new(b, -h / 2.0),
    ];
    let m = build_finite_strip_model(&pts, 0.25, 0.06, 4);

    assert_eq!(m.elements.len(), m.nodes.len() - 1);
    for w in m.nodes.windows(2) {
        assert_eq!(w[1].id, w[0].id + 1);
    }
}
