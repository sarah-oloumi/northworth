# Dependency Automation

Northworth uses Renovate for dependency update pull requests.

Renovate is configured to update:

- Go modules from `go.mod` and `go.sum`
- GitHub Actions used in `.github/workflows/`
- git submodules, including `.impeccable`

The Impeccable submodule is pinned to a specific commit. Renovate should open pull requests when the tracked upstream branch has newer commits.

## Manual Impeccable Update

If Renovate is unavailable, update Impeccable manually:

```bash
git submodule update --remote .impeccable
```

Northworth does not require Node or npm. Impeccable is vendored as a pinned design reference only.

## Notes

Renovate's git submodule manager is opt-in and currently documented as beta by Renovate. Keep submodule update PRs small and review them before merging.
