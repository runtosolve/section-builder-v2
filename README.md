# SectionBuilder

## Introduction
This project is a tool for simplifying createing finite strip models for analysis in CUFSM. 
It allows converting an ordered array of point coordinates into a CUFSM node and element input.

## Multi-Language Support
Written in Rust. This project has Julia bindings.

## Using from Julia

The Julia package lives in the [julia/SectionBuilder](julia/SectionBuilder) subdirectory of this repository. Add it to your Julia project using the `subdir` option:

```julia
using Pkg
Pkg.add(url="https://github.com/runtosolve/section-builder-v2", subdir="julia/SectionBuilder")
```

Or from the Pkg REPL (press `]` in the Julia REPL):

```
pkg> add https://github.com/runtosolve/section-builder-v2:julia/SectionBuilder
```

To pin a specific branch, tag, or commit, append `#<ref>`:

```
pkg> add https://github.com/runtosolve/section-builder-v2:julia/SectionBuilder
```

Then load it like any other package:

```julia
using SectionBuilder
```

Prebuilt native binaries are fetched automatically. On first `using SectionBuilder`, Julia downloads the platform-appropriate library (linux x86_64/aarch64, macOS x86_64/aarch64, Windows x86_64) from this repo's GitHub Release for the installed tag and caches it under `~/.julia/artifacts/`. No Rust toolchain is required on the consumer side.

To point at a locally built library instead (e.g. for development), set:

```bash
export SECTION_BUILDER_LIB=/abs/path/to/libsection_builder_v2.so
```

### Cutting a release (maintainers)

1. Bump `version` in [Cargo.toml](Cargo.toml) and [julia/SectionBuilder/Project.toml](julia/SectionBuilder/Project.toml).
2. Tag and push: `git tag v0.1.1 && git push --tags`.
3. The [release-julia-binaries](.github/workflows/release-julia-binaries.yml) workflow builds the cdylib for every supported platform, uploads tarballs to the GitHub Release, regenerates [julia/SectionBuilder/Artifacts.toml](julia/SectionBuilder/Artifacts.toml), and commits it back to `main`.