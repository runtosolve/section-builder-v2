using Test
using SectionBuilder

@testset "straight line, no corners" begin
    pts = [(0.0, 0.0), (10.0, 0.0)]
    m = build_finite_strip_model(pts, 0.0, 0.1, 2.0)
    @test length(m.nodes) == 6
    @test length(m.elements) == 5
    @test isapprox(m.nodes[4].x, 6.0; atol = 1e-9)
    @test isapprox(m.nodes[4].y, 0.0; atol = 1e-9)
end

@testset "L-shape rounds to arc" begin
    pts = [(0.0, 0.0), (5.0, 0.0), (5.0, 5.0)]
    m = build_finite_strip_model(pts, 1.0, 0.1, 10.0)
    cx, cy = 4.0, 1.0
    for n in m.nodes
        d = hypot(n.x - cx, n.y - cy)
        if d < 1.5
            @test isapprox(d, 1.0; atol = 1e-9)
        end
    end
    @test length(m.elements) == length(m.nodes) - 1
end
