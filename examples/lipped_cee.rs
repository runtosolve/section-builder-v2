use section_builder_v2::{Vec2, build_finite_strip_model};

fn main() {
    // Demo: a lipped Cee centerline sketched with sharp corners.
    // Opening faces +x, same orientation as the MATLAB `getCee`.
    let h = 8.0;
    let b = 2.0;
    let d = 0.5;
    let points = [
        Vec2::new(b, h / 2.0),         // top lip tip
        Vec2::new(b, h / 2.0 - d),     // top flange–lip corner (sharp)
        Vec2::new(0.0, h / 2.0 - d),   // top flange–web corner (sharp); web at x=0
        Vec2::new(0.0, -h / 2.0 + d),  // bottom flange–web corner (sharp)
        Vec2::new(b, -h / 2.0 + d),    // bottom flange–lip corner (sharp)
        Vec2::new(b, -h / 2.0),        // bottom lip tip
    ];

    let model = build_finite_strip_model(&points, 0.25, 0.06, 4);

    println!("nodes: {}, elements: {}", model.nodes.len(), model.elements.len());
    for n in model.nodes.iter().take(5) {
        println!("  node {:>3}  x={:8.4}  y={:8.4}", n.id, n.x, n.y);
    }
}
