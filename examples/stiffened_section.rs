use std::fs::File;
use std::io::Write;

use section_builder_v2::{Vec2, build_finite_strip_model};

fn main() {
    // Centerline polyline (mm) of a section with web stiffeners and notched
    // top/bottom flanges. Sharp-corner coordinates — corners are rounded by
    // `build_finite_strip_model`.
    let points = [
        Vec2::new(50.8, 76.2),
        Vec2::new(7.62, 76.2),
        Vec2::new(0.0, 68.58),
        Vec2::new(-10.16, 76.2),
        Vec2::new(-50.8, 76.2),
        Vec2::new(-50.8, 38.1),
        Vec2::new(-43.18, 27.94),
        Vec2::new(-50.8, 17.78),
        Vec2::new(-50.8, -20.32),
        Vec2::new(-43.18, -30.48),
        Vec2::new(-50.8, -40.64),
        Vec2::new(-50.8, -104.14),
        Vec2::new(-10.16, -104.14),
        Vec2::new(0.0, -96.52),
        Vec2::new(10.949055396, -104.731791547),
        Vec2::new(50.939311981, -105.605087280),
    ];

    let radius = 3.0;
    let thickness = 1.5;
    let elements_per_segment = 4;

    let model = build_finite_strip_model(&points, radius, thickness, elements_per_segment);

    println!(
        "nodes: {}, elements: {}",
        model.nodes.len(),
        model.elements.len()
    );
    for n in &model.nodes {
        println!("  node {:>3}  x={:10.4}  y={:10.4}", n.id, n.x, n.y);
    }

    write_svg("stiffened_section.svg", &points, &model).expect("write svg");
    println!("wrote stiffened_section.svg");
}

fn write_svg(
    path: &str,
    sharp: &[Vec2],
    model: &section_builder_v2::FiniteStripModel,
) -> std::io::Result<()> {
    let pad = 10.0;
    let (mut min_x, mut max_x) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut min_y, mut max_y) = (f64::INFINITY, f64::NEG_INFINITY);
    for n in &model.nodes {
        min_x = min_x.min(n.x);
        max_x = max_x.max(n.x);
        min_y = min_y.min(n.y);
        max_y = max_y.max(n.y);
    }
    let w = max_x - min_x + 2.0 * pad;
    let h = max_y - min_y + 2.0 * pad;

    // Flip y so structural +y points up in the SVG.
    let tx = |x: f64| x - min_x + pad;
    let ty = |y: f64| (max_y - y) + pad;

    let mut f = File::create(path)?;
    writeln!(
        f,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w:.3} {h:.3}" width="420">"#
    )?;
    writeln!(
        f,
        r#"<rect width="100%" height="100%" fill="white"/>"#
    )?;

    // Elements as a single polyline.
    let mut d = String::from("M ");
    for (i, n) in model.nodes.iter().enumerate() {
        if i > 0 {
            d.push_str(" L ");
        }
        d.push_str(&format!("{:.4} {:.4}", tx(n.x), ty(n.y)));
    }
    writeln!(
        f,
        r##"<path d="{d}" fill="none" stroke="#1f77b4" stroke-width="0.6"/>"##
    )?;

    // Mesh nodes.
    for n in &model.nodes {
        writeln!(
            f,
            r##"<circle cx="{:.4}" cy="{:.4}" r="0.6" fill="#1f77b4"/>"##,
            tx(n.x),
            ty(n.y)
        )?;
    }

    // Original sharp-corner points (input polyline).
    for p in sharp {
        writeln!(
            f,
            r##"<circle cx="{:.4}" cy="{:.4}" r="1.2" fill="none" stroke="#d62728" stroke-width="0.5"/>"##,
            tx(p.x),
            ty(p.y)
        )?;
    }

    writeln!(f, "</svg>")?;
    Ok(())
}
