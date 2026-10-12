# Repository refresh implementation plan

**Goal:** Update Rust dependencies and validate/refine PRs #11, #13, and #14 before merging them into main.

**Approach:** Preserve contributor commits on a maintenance branch, add focused regressions for the PR refinements, and validate the combined result locally and in CI. Preserve the existing SDK ABI; document bundled runtime limitations instead of silently replacing proprietary binaries.

**Constraints:** Windows, Linux, and macOS x86_64; current stable Rust and the dependency-driven minimum Rust version; no crate publication.

## Tasks

- [x] Inspect clean baseline, dependencies, PRs, and existing tests. Baseline send example does not compile; old bindgen layout tests abort on null dereferences.
- [x] Merge contributor branches locally for integration; request independent PR review.
- [ ] Refine audio construction: unsafe raw-pointer contract, null metadata, target-independent FourCC cast; test metadata, caller-owned storage, and unsafe-call requirement.
- [ ] Refine SDK linkage: actual bundled library filenames, build-output library search, macOS runtime loading, and explicit architecture support. Check all platforms in CI.
- [ ] Update image, bindgen, and xflags to current stable releases; refresh the workspace lockfile, editions, Cargo configuration, and binding layout assertions.
- [ ] Add CI, dependency update automation, and current setup/contribution instructions. Build and test all examples; run formatting, Clippy, documentation, and dependency security checks.
- [ ] Review final integration, fix important findings, push for CI, and merge once required checks pass. Verify original PR states and remote main SHA.

## Review focus

- Caller-owned audio storage must remain live and correctly aligned until the frame and sends finish.
- Empty metadata must be NULL, never an unterminated Rust string.
- C enum signedness differs across the platform bindings.
- Library search at link time and discovery at runtime are separate requirements.
- Bundled macOS binaries support Intel, not native Apple Silicon; generated layout assertions must not dereference null pointers.

## Decisions

- Work in a maintenance branch in the initially clean checkout; no unrelated changes were present.
- Keep the existing raw-pointer signature from #11 but make it unsafe and document the contract. This adds no allocation and makes the obligation explicit before the API is released.
- Preserve each PR's original commits so GitHub can recognize its merge when the integration reaches main.
