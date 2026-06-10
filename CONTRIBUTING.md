# Contributing

Northworth uses small pull requests, clear change descriptions, and a short-lived branch workflow.

## Workflow

1. Branch from `main`.
2. Keep the change focused on one logical purpose.
3. Run the relevant tests before opening a pull request.
4. Open a pull request with context, verification, and any risk notes.
5. Merge after CI passes and the change has been reviewed.

`main` is the source of truth. Avoid long-lived development branches.

## Commit Messages

Use this format:

```text
type(scope): imperative summary
```

Examples:

```text
feat(tax): add Ontario surtax constants
fix(web): prevent settings form from losing local changes
docs(release): define tag policy
chore(deps): update Impeccable submodule
```

Allowed types:

- `feat`: user-visible capability or domain behavior
- `fix`: bug fix
- `docs`: documentation-only change
- `test`: test-only change
- `refactor`: structure change with no intended behavior change
- `perf`: performance improvement
- `build`: build, CI, release, or packaging change
- `chore`: maintenance that does not fit the above
- `revert`: revert a previous change

Rules:

- Keep the summary under 72 characters when practical.
- Use the imperative mood: `add`, not `added` or `adds`.
- Explain why in the body when the change is non-obvious.
- Reference issues or PRs in the body, not by cramming them into the summary.
- Do not include private financial details in commits, PRs, test data, or screenshots.

## Pull Requests

Pull requests should be small enough to review carefully. Prefer several focused PRs over one mixed PR.

Use squash merge for normal PRs. The PR title becomes the commit message on `main`, so the PR title must follow the same `type(scope): summary` format.

Every PR should include:

- Summary of behavior or documentation changed.
- Verification performed.
- Risk or privacy notes when relevant.
- Screenshots only when they contain fictional/demo data.

## References

This workflow is influenced by:

- [Software Engineering at Google: Code Review](https://abseil.io/resources/swe-book/html/ch09.html)
- [Software Engineering at Google: Version Control and Branch Management](https://abseil.io/resources/swe-book/html/ch16.html)
- [Software Engineering at Google: Critique](https://abseil.io/resources/swe-book/html/ch19.html)
