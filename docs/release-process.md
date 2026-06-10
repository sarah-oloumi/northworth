# Release Process

Northworth uses semantic versioning for public releases and keeps unreleased development on `main`.

## Versioning

Use `MAJOR.MINOR.PATCH`:

- `MAJOR`: incompatible change to a documented user workflow, data format, CLI flag, HTTP route, or exported package API after `1.0.0`.
- `MINOR`: backward-compatible feature, new planner capability, new tax/reference dataset, or new supported workflow.
- `PATCH`: backward-compatible bug fix, documentation correction, dependency update, or tax/reference correction that does not change documented behavior incompatibly.

While Northworth is pre-`1.0.0`, use `0.MINOR.PATCH`:

- Start public preview releases at `v0.1.0`.
- Increment `MINOR` for meaningful feature milestones.
- Increment `PATCH` for fixes to the current preview line.
- Treat breaking changes as acceptable only when they are documented in the release notes.

## Tags

Create a tag when there is something useful for someone else to download, test, or compare.

Good reasons to tag:

- First runnable local app preview.
- A user-visible feature milestone.
- A tax/reference dataset snapshot.
- A security or correctness fix.
- A release candidate for broader testing.

Do not tag:

- Every merged PR.
- Formatting-only or documentation-only work unless it accompanies a release.
- Broken or unverified builds.
- Local/private experiments.

Tag format:

```text
v0.1.0
v0.2.0-rc.1
```

## Release Checklist

1. Confirm `main` is green in CI.
2. Confirm no private data is present in fixtures, docs, screenshots, or generated artifacts.
3. Update release notes with user-visible changes, data-format changes, and known limitations.
4. Create an annotated tag:

   ```bash
   git tag -a v0.1.0 -m "Northworth v0.1.0"
   git push origin v0.1.0
   ```

5. Create a GitHub release from the tag.

## References

- [Semantic Versioning 2.0.0](https://semver.org/)
- [Software Engineering at Google: Version Control and Branch Management](https://abseil.io/resources/swe-book/html/ch16.html)

