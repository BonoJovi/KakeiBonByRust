# Dependabot Alert #1: glib VariantStrIter Unsoundness

**Alert:** GHSA-wrw7-89jp-8q8g  
**Severity:** Medium  
**Status:** Open (Monitoring)  
**Date Investigated:** 2025-12-03 04:11 JST

## Summary

Unsoundness in `Iterator` and `DoubleEndedIterator` impls for `glib::VariantStrIter`

## Affected Dependency

- **Package:** `glib v0.18.5`
- **Dependency Type:** Indirect (via `tauri` → `gtk v0.18.2`)
- **Patched Versions:** Not available for 0.18.x series (likely fixed in v0.21.5)

## Investigation Result

### Findings

1. **Direct Usage Check:**
   - ✅ No usage of `VariantStrIter` in our codebase
   - ✅ No usage of `VariantStr` in our codebase
   - ✅ No direct import of `glib::` in our source code

2. **Dependency Chain:**
   ```
   app → tauri v2.9.3 → gtk v0.18.2 → glib v0.18.5
   ```

3. **Actual Risk:** **Minimal to none**
   - The vulnerability only affects code that directly uses `glib::VariantStrIter`
   - Our application does not use this type
   - Indirect dependency only (through Tauri's GTK bindings)

### Why Cannot Update

- `gtk v0.18.2` requires `glib v0.18.x`
- Cannot update `glib` independently without updating entire GTK stack
- Waiting for Tauri to update to newer GTK/glib versions

## Action Taken

- **Decision:** Keep alert open to track upstream updates
- **Rationale:** 
  - Will be automatically resolved when Tauri updates dependencies
  - Serves as a reminder to update Tauri regularly
  - No immediate security risk to our application

## References

- GitHub Advisory: https://github.com/advisories/GHSA-wrw7-89jp-8q8g
- Latest glib version: 0.21.5 (released 2025-12-02)

---

*Last Updated: 2025-12-03 04:11 JST*
