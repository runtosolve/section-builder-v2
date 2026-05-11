module SectionBuilder

export Vec2, NodeRow, ElementRow, FiniteStripModel, build_finite_strip_model

const LIB = let
    base = normpath(joinpath(@__DIR__, "..", "..", "..", "target", "release"))
    name = Sys.iswindows() ? "section_builder_v2.dll" :
           Sys.isapple()   ? "libsection_builder_v2.dylib" :
                             "libsection_builder_v2.so"
    joinpath(base, name)
end

const Vec2 = NTuple{2, Float64}

# Must match #[repr(C)] CNodeRow in src/ffi.rs.
struct NodeRow
    id::Csize_t
    x::Cdouble
    y::Cdouble
    xdof::UInt8
    ydof::UInt8
    zdof::UInt8
    qdof::UInt8
    stress::Cdouble
end

# Must match #[repr(C)] CElementRow in src/ffi.rs.
struct ElementRow
    id::Csize_t
    node_i::Csize_t
    node_j::Csize_t
    thickness::Cdouble
    material::UInt32
end

struct FiniteStripModel
    nodes::Vector{NodeRow}
    elements::Vector{ElementRow}
end

"""
    build_finite_strip_model(points, radius, thickness, target_element_size) -> FiniteStripModel

`points` is a `Vector{NTuple{2,Float64}}` (or `Vector{Tuple{Float64,Float64}}`)
of sharp-corner centerline points. Mirrors the Rust API.
"""
function build_finite_strip_model(
    points::AbstractVector{<:NTuple{2, Real}},
    radius::Real,
    thickness::Real,
    target_element_size::Real,
)
    length(points) >= 2 || throw(ArgumentError("need at least two points"))
    target_element_size > 0 || throw(ArgumentError("target_element_size must be > 0"))

    xy = Vector{Float64}(undef, 2 * length(points))
    @inbounds for (i, p) in pairs(points)
        xy[2i - 1] = p[1]
        xy[2i]     = p[2]
    end

    handle = ccall(
        (:sb_build_finite_strip_model, LIB),
        Ptr{Cvoid},
        (Ptr{Cdouble}, Csize_t, Cdouble, Cdouble, Cdouble),
        xy, length(points), radius, thickness, target_element_size,
    )
    handle == C_NULL && error("sb_build_finite_strip_model returned null")

    try
        n_nodes = ccall((:sb_fsm_num_nodes, LIB), Csize_t, (Ptr{Cvoid},), handle)
        n_elems = ccall((:sb_fsm_num_elements, LIB), Csize_t, (Ptr{Cvoid},), handle)

        nodes = Vector{NodeRow}(undef, n_nodes)
        elems = Vector{ElementRow}(undef, n_elems)

        ccall((:sb_fsm_copy_nodes, LIB), Cvoid,
              (Ptr{Cvoid}, Ptr{NodeRow}), handle, nodes)
        ccall((:sb_fsm_copy_elements, LIB), Cvoid,
              (Ptr{Cvoid}, Ptr{ElementRow}), handle, elems)

        return FiniteStripModel(nodes, elems)
    finally
        ccall((:sb_fsm_free, LIB), Cvoid, (Ptr{Cvoid},), handle)
    end
end

end # module
