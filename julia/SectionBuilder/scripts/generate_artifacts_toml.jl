#!/usr/bin/env julia
#
# Regenerate `julia/SectionBuilder/Artifacts.toml` from a GitHub release.
#
# Usage:
#   julia --project=julia/SectionBuilder julia/SectionBuilder/scripts/generate_artifacts_toml.jl \
#       --tag v0.1.0 [--repo owner/section-builder-v2]
#
# The script expects the release to contain one tarball per supported platform,
# named `section_builder_v2-<target-triple>.tar.gz`, where each tarball contains
# the library at `lib/<name>` (or `bin/<name>` on Windows).

using Pkg.Artifacts
using Pkg.BinaryPlatforms
using Pkg.GitTools
using Downloads
using SHA
using Tar
using CodecZlib
using TOML

const PKG_DIR        = normpath(joinpath(@__DIR__, ".."))
const ARTIFACTS_TOML = joinpath(PKG_DIR, "Artifacts.toml")
const ARTIFACT_NAME  = "section_builder_v2"

# Platform triple → Julia Platform object. Add rows here to support more targets.
const PLATFORMS = [
    ("x86_64-linux-gnu",   Platform("x86_64",  "linux";   libc="glibc")),
    ("aarch64-linux-gnu",  Platform("aarch64", "linux";   libc="glibc")),
    ("aarch64-macos",      Platform("aarch64", "macos")),
    ("x86_64-windows",     Platform("x86_64",  "windows")),
]

function parse_args(args)
    tag = ""
    repo = get(ENV, "GITHUB_REPOSITORY", "")
    i = 1
    while i <= length(args)
        a = args[i]
        if a == "--tag"
            tag = args[i+1]; i += 2
        elseif a == "--repo"
            repo = args[i+1]; i += 2
        else
            error("unknown argument: $a")
        end
    end
    isempty(tag)  && error("--tag is required (e.g. --tag v0.1.0)")
    isempty(repo) && error("--repo owner/name is required (or set GITHUB_REPOSITORY)")
    return (; tag, repo)
end

sha256_hex(path) = bytes2hex(open(SHA.sha256, path))

function download_and_hash(url::String)
    mktempdir() do tmp
        tarball = joinpath(tmp, basename(url))
        @info "Downloading" url
        Downloads.download(url, tarball)
        tarball_sha256 = sha256_hex(tarball)

        # Extract to compute the git-tree-sha1 (must match what Pkg computes on install).
        extract_dir = joinpath(tmp, "unpacked")
        mkpath(extract_dir)
        open(GzipDecompressorStream, tarball) do io
            Tar.extract(io, extract_dir)
        end
        tree_sha = bytes2hex(GitTools.tree_hash(extract_dir))
        return (; tarball_sha256, tree_sha)
    end
end

function main()
    opts = parse_args(ARGS)
    base = "https://github.com/$(opts.repo)/releases/download/$(opts.tag)"

    # Remove any stale binding for this artifact across all platforms.
    if isfile(ARTIFACTS_TOML)
        existing = TOML.parsefile(ARTIFACTS_TOML)
        delete!(existing, ARTIFACT_NAME)
        open(ARTIFACTS_TOML, "w") do io
            TOML.print(io, existing; sorted=true)
        end
    end

    for (triple, platform) in PLATFORMS
        url = "$base/section_builder_v2-$triple.tar.gz"
        info = download_and_hash(url)
        @info "Binding" triple url tree_sha=info.tree_sha
        bind_artifact!(
            ARTIFACTS_TOML,
            ARTIFACT_NAME,
            Base.SHA1(info.tree_sha);
            platform = platform,
            download_info = [(url, info.tarball_sha256)],
            lazy = true,
            force = true,
        )
    end

    @info "Updated $ARTIFACTS_TOML"
end

main()
