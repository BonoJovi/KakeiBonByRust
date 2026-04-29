# Release Workflow

Release checklist for KakeiBon. Run this before creating a release tag.

## Steps

### 1. Version Sync (3 files must match)

Update ALL three files to the same version:
- `Cargo.toml` → `version = "X.Y.Z"`
- `tauri.conf.json` → `"version": "X.Y.Z"`
- `package.json` → `"version": "X.Y.Z"`

Verify:
```bash
grep -h "version" Cargo.toml tauri.conf.json package.json | head -3
```

### 2. Update Documentation

- `CHANGELOG_ja.md` / `CHANGELOG_en.md` — Add version section
- `README.md` / `README_ja.md` / `README_en.md` — Version badge, release message
- `.ai-context/core/QUICK_REFERENCE.md` — Version number
- `zundou-website-vps/index.html` — `data-version` and `data-date` attributes

### 3. Pre-Release Check (MANDATORY)

**コミット前に必ず実行すること。スキップ禁止。**

```bash
./scripts/check-release.sh
```

This verifies version consistency, build, and tests. **All checks must pass before proceeding.**

### 4. Commit & Merge

```bash
git add -A && git commit -m "release: vX.Y.Z"
git checkout main && git merge dev
git push origin main
```

### 5. Tag & Push

```bash
git tag vX.Y.Z
git push origin vX.Y.Z
```

### 6. Return to dev

```bash
git checkout dev && git merge main
git push origin dev
```

## Common Mistakes

- Forgetting `package.json` → wrong release name on GitHub
- Forgetting `tauri.conf.json` → wrong asset filenames
- Pushing tag before main → tag points to wrong commit
- Forgetting `zundou-website-vps` version update
