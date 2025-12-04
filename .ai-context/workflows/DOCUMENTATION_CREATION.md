# Documentation Creation Guidelines

**Last Updated**: 2025-12-05 01:37 JST

## Overview

This document defines the process and standards for creating and maintaining project documentation. Following these guidelines ensures consistency, reliability, and quality across all documentation.

---

## Core Principles

### 0. Process Over Speed

**⚠️ CRITICAL: Never skip steps to save time.**

**Time Management Policy:**
- **Default**: Follow ALL steps in order, regardless of time
- **Rush Mode**: Only when developer explicitly instructs "急ぎで作成してください" or similar
- **Priority**: Process completeness > Creation speed

**Why this matters:**
- Skipped steps create technical debt
- Process integrity ensures quality
- Learning is lost without evaluation step
- Future contributors depend on complete process

**When developer says "急ぎで":**
- Acknowledge the rush mode explicitly
- Ask which steps can be skipped
- Document what was skipped for later review

---

### 0.1. Continuous Improvement (Mandatory)

**⚠️ STOP: After every document creation, evaluate and improve these guidelines.**

**This is NOT optional. This is part of the creation process.**

**Evaluation Questions (Must answer before committing):**
1. ❓ Did I encounter any unexpected issues?
2. ❓ Should new commands/tools be added to guidelines?
3. ❓ Did I discover a better approach?
4. ❓ Should the template be updated?
5. ❓ Were there pain points in the process?

**If YES to ANY → Update guidelines NOW.**

**This applies to ALL documentation types:**
- API references
- User guides
- Developer guides
- Architecture documents
- README files

**Why this is important:**
- Guidelines stay current and relevant
- Knowledge is captured while fresh
- Future contributors benefit from improvements
- Documentation quality improves over time

---

### 1. Code-First Verification

**All documentation MUST be verified against actual implementation code.**

- ❌ Don't: Copy from old documentation without verification
- ❌ Don't: Assume APIs match documentation
- ✅ Do: Check `src/` for actual implementation
- ✅ Do: Verify parameters, return types, and behavior
- ✅ Do: Test API calls when possible

**Example Verification Process:**
```bash
# Find the actual implementation
grep -A 20 "fn api_function_name" src/lib.rs src/services/*.rs

# Check Tauri command registration
grep "api_function_name" src/lib.rs

# Verify parameter types
grep -B 5 -A 10 "#\[tauri::command\]" src/lib.rs | grep -A 10 "fn api_function_name"
```

### 2. Screen-Based Organization

APIs are documented based on the screens that use them:

- **API_COMMON.md**: Shared APIs (Session, I18n, Encryption, System, Validation)
- **API_AUTH.md**: Login/Setup screen APIs
- **API_USER.md**: User management screen APIs
- **API_CATEGORY.md**: Category management screen APIs
- **API_TRANSACTION.md**: Transaction management screen APIs
- **API_ACCOUNT.md**: Account management screen APIs
- **API_MASTER_DATA.md**: Shop/Manufacturer/Product APIs
- **API_AGGREGATION.md**: Aggregation screen APIs
- **API_SETTINGS.md**: Settings screen APIs

**Rationale**: Developers typically work on specific screens, so grouping APIs by screen improves usability.

### 3. Minimal Code Examples

**Keep code examples concise and focused.**

- Show only essential parameters
- Focus on structure and usage patterns
- Avoid lengthy implementation details
- Link to detailed examples when needed

**Good Example:**
```javascript
const user = await invoke('login_user', { username, password });
```

**Bad Example:**
```javascript
// Too much detail in API reference
async function loginWithFullValidation() {
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    
    // Client-side validation
    if (!username || username.length < 3) {
        alert('Username must be at least 3 characters');
        return;
    }
    
    // ... 50 more lines ...
}
```

### 4. Bilingual Support

- **Phase 1**: Create Japanese version first
- **Phase 2**: Translate to English
- **Reason**: Primary developers are Japanese; translate in batch for efficiency

---

## Document Structure Template

### Standard Section Order

```markdown
# [Feature] API Reference

**Last Updated**: YYYY-MM-DD HH:MM JST

## Overview
Brief description of the APIs covered in this document.

---

## Table of Contents
1. [API Group 1](#api-group-1)
2. [API Group 2](#api-group-2)
...

---

## API Group 1

### api_function_name

Description of what this API does.

**Parameters:**
- `param1` (Type): Description
- `param2` (Type): Description

**Returns:**
- `ReturnType`: Description

**Usage Example:**
```javascript
const result = await invoke('api_function_name', { param1, param2 });
```

**Notes:**
- Important behavior notes
- Edge cases
- Related APIs

---

## Data Structures

### StructName

```rust
pub struct StructName {
    pub field1: Type,
    pub field2: Type,
}
```

---

## Error Handling

### Common Error Patterns

| Error Message | Cause | Solution |
|--------------|-------|----------|
| "Error message" | Cause | How to fix |

---

## Security Considerations

Key security points related to these APIs.

---

## Test Coverage

- ✅ Test 1
- ✅ Test 2
- ⏳ Test 3 (pending)

---

## Related Documents

- [Other API Doc](./OTHER_API.md)
- Implementation: `src/path/to/file.rs`

---

**Change History:**
- YYYY-MM-DD: Initial version
```

---

## API Documentation Workflow

### Step 1: Identify APIs

```bash
# List all Tauri commands
grep -A 1 "#\[tauri::command\]" src/lib.rs | grep "^async fn\|^fn" | sed 's/async fn //' | sed 's/fn //' | sed 's/(.*$//' | sort

# Count APIs by category
# (Use Python script to categorize by domain)
```

### Step 2: Verify Implementation

For each API:

1. **Locate source code:**
   ```bash
   grep -n "fn api_name" src/lib.rs src/services/*.rs
   ```

2. **Check parameters:**
   - Read function signature
   - Note parameter names (camelCase in frontend)
   - Note parameter types

3. **Check return type:**
   - `Result<T, String>` → Success returns `T`, Error returns `String`
   - `Option<T>` → Can return `null`

4. **Check behavior:**
   - Read implementation
   - Note side effects (database updates, session changes)
   - Note validation rules

### Step 3: Document API

Use the template above, filling in:

- API name (exact command name)
- Description (what it does, when to use)
- Parameters (name, type, description)
- Return value (type, structure)
- **Minimal** usage example
- Important notes

### Step 4: Add to TODO.md

```markdown
#### Phase 1: Japanese Version
- [x] API_COMMON.md - Description
- [x] API_AUTH.md - Description
- [ ] API_USER.md - Description (next)
```

Update status after completion:

```markdown
- [x] API_USER.md - Description
```

### Step 5: Evaluate and Improve Guidelines (MANDATORY)

**⚠️ CHECKPOINT: DO NOT SKIP THIS STEP**

**This step is as important as writing the document itself.**

---

#### 5.1. Pause and Reflect (2 minutes minimum)

**Ask yourself these questions:**

| # | Question | Yes/No | Notes |
|---|----------|--------|-------|
| 1 | Were any steps unclear or confusing? | | |
| 2 | Did I discover better tools/commands? | | |
| 3 | Did implementation differ from docs? | | |
| 4 | Should template be improved? | | |
| 5 | Were there repetitive tasks to automate? | | |

---

#### 5.2. Update Guidelines

**If ANY answer was "Yes", update guidelines NOW:**

1. **For unclear steps:**
   ```markdown
   Add clarification to the relevant step section
   ```

2. **For new tools/commands:**
   ```markdown
   Add to "Tools and Scripts" section
   Example: "Use `rg -A 15 'pattern'` to get more context"
   ```

3. **For common patterns:**
   ```markdown
   Add to "Common Pitfalls" or "Best Practices" section
   ```

4. **For template improvements:**
   ```markdown
   Update the template in "API Document Template" section
   ```

---

#### 5.3. Document Improvements

```bash
# Save your changes
git add .ai-context/workflows/DOCUMENTATION_CREATION.md
git commit -m "docs(meta): improve guidelines based on [DOCUMENT_NAME] creation

- Added: [what you added]
- Updated: [what you updated]
- Fixed: [what you fixed]"
```

---

#### 5.4. Skip Prevention Mechanisms

**To avoid accidentally skipping this step:**

1. **Use checklist in Quality Checklist section**
2. **Set a mental reminder**: "Did I evaluate?" before committing
3. **Review TODO.md**: Add "Evaluate guidelines" as separate checkbox
4. **Verbalize**: Say out loud "Checkpoint - Evaluate guidelines"

**Example TODO.md format:**
```markdown
- [ ] Create API_XXX.md
- [ ] Verify against implementation
- [ ] ⚠️ EVALUATE GUIDELINES (MANDATORY)
- [ ] Update TODO.md
- [ ] Commit
```

---

**This is a standard task for ALL documentation creation, not just API docs.**

**Examples of improvements to document:**
- New validation commands discovered
- Better code examples found
- Common errors encountered
- Time-saving tips learned
- Structure improvements
- Parameter naming patterns
- Integration opportunities

---

### Step 6: Commit

```bash
git add docs/developer/ja/api/API_NAME.md
git add TODO.md
git commit -m "docs(api): add API_NAME reference (Japanese)"

# If guidelines were updated:
git add .ai-context/workflows/DOCUMENTATION_CREATION.md
git commit -m "docs(meta): improve documentation guidelines"
```

---

## Quality Checklist

**⚠️ Use this checklist BEFORE committing. Do not skip any item.**

### Documentation Quality

- [ ] All APIs verified against source code
- [ ] Parameter names match implementation (camelCase conversion noted)
- [ ] Return types are accurate
- [ ] Code examples are minimal and clear
- [ ] Error messages are from actual code
- [ ] Related documents are linked
- [ ] Timestamp is in JST format (user-facing docs)

### Process Quality (MANDATORY - Cannot Skip)

- [ ] ⚠️ **STOP POINT 1**: Evaluated guidelines (Step 5)
- [ ] ⚠️ **STOP POINT 2**: Answered all 5 evaluation questions
- [ ] ⚠️ **STOP POINT 3**: Updated guidelines if needed
- [ ] TODO.md is updated with completion status
- [ ] Committed with proper conventional commit message

### Final Verification

- [ ] Re-read this checklist from top to bottom
- [ ] Confirmed no steps were skipped
- [ ] Ready to commit

---

## Common Pitfalls

### ❌ Don't: Assume Parameter Names

```javascript
// Documentation says:
await invoke('add_user', { userName, userPassword });

// But actual implementation expects:
await invoke('add_user', { username, password });
```

**Solution**: Check actual Rust parameter names and note camelCase conversion.

### ❌ Don't: Copy SQL Without Verification

Old docs may contain outdated SQL queries.

**Solution**: Check `src/sql_queries.rs` or actual query strings in code.

### ❌ Don't: Include All Implementation Details

API reference ≠ Implementation guide

**Solution**: Focus on "what" and "how to use", not "how it works internally".

### ❌ Don't: Forget Error Messages

Generic error descriptions are not helpful.

**Solution**: Copy actual error strings from code:
```rust
Err("Password must be at least 16 characters long".to_string())
```

---

## Maintenance

### When to Update Documentation

- New API added → Add to relevant document
- API parameters changed → Update document
- API behavior changed → Update description and examples
- Bug fix changes behavior → Update notes
- Deprecation → Mark as deprecated, add alternative

### Versioning

- User-facing docs: Include "Last Updated" timestamp (JST)
- Add to "Change History" section at bottom
- Major changes: Increment section numbers if applicable

---

## Tools and Scripts

### API Analysis Script

```python
# scripts/analyze_apis.py
# Categorizes Tauri commands by domain
# Generates API distribution report
```

### Documentation Validation

```bash
# Check for broken links
grep -r "\[.*\](\./" docs/ | while read line; do
    # Validate file exists
done

# Check for outdated timestamps
# (Manual review recommended)
```

---

## Examples

### Good Documentation Example

See: `docs/developer/ja/api/API_COMMON.md`

**Why it's good:**
- Verified against `src/services/session.rs`
- Minimal, focused code examples
- Clear parameter descriptions
- Actual error messages from code
- Links to implementation files

### Bad Documentation Example

```markdown
### some_api

This API does something with data.

**Parameters:** Various

**Returns:** Results

**Example:** See implementation
```

**Why it's bad:**
- Vague description
- No parameter details
- No return type
- No actual example
- Not helpful for developers

---

## Future Improvements

- [ ] Automated API extraction from source
- [ ] Type definition generation (TypeScript .d.ts)
- [ ] Interactive API playground
- [ ] Auto-validation of documentation against code
- [ ] Version tracking per API

---

## Questions?

See:
- [Documentation Policy](../../docs/developer/en/guides/DOCUMENTATION_POLICY.md)
- [Project README](./.ai-context/README.md)
- Ask in project discussions

---

## Skip Prevention Strategies

**Why steps get skipped:**
1. Time pressure (real or perceived)
2. Momentum to complete tasks quickly
3. Forgetting in multi-document workflows
4. Evaluation feels less tangible than creation

**Strategies to prevent skipping:**

### Strategy 1: Explicit Checkpoints

Add visual/verbal markers at critical points:
```markdown
⚠️ STOP POINT: [Description]
```

Use these in:
- Step transitions
- Quality checklist
- TODO.md task lists

### Strategy 2: Verbalize Progress

Before each major step, say out loud (or think clearly):
```
"Now starting: [Step Name]"
"Checkpoint: [Step Name] complete"
"⚠️ MANDATORY: Evaluating guidelines"
```

### Strategy 3: TODO.md Sub-tasks

Break documentation creation into explicit sub-tasks:
```markdown
#### Create API_XXX.md
- [ ] Step 1: Plan document (5 min)
- [ ] Step 2: Verify APIs (10 min)
- [ ] Step 3: Write content (30 min)
- [ ] Step 4: Add examples (10 min)
- [ ] Step 5: ⚠️ EVALUATE GUIDELINES (5 min - MANDATORY)
- [ ] Step 6: Commit (2 min)
```

### Strategy 4: Developer Prompts

At completion, explicitly ask developer:
```
"Documentation complete. Before committing, should I evaluate the guidelines? (This is Step 5 - Mandatory)"
```

Wait for confirmation before proceeding.

### Strategy 5: Time Allocation

Allocate specific time to each phase:
- Creation: 70% of time
- Evaluation: 10% of time (NOT optional)
- Commit: 5% of time
- Review: 15% of time

### Strategy 6: Batch Evaluation

If creating multiple documents:
- Still evaluate after EACH document
- Keep notes of patterns across documents
- Do final comprehensive evaluation at the end

**Never say**: "I'll evaluate all at once later"
**Always do**: Evaluate immediately after each document

---

**Change History:**
- 2025-12-05 01:37 JST: Initial version (guidelines established)
- 2025-12-05 01:45 JST: Added continuous improvement process (Principle 0, Step 5)
- 2025-12-05 02:09 JST: Added "Process Over Speed" principle and skip prevention strategies
  - Added Principle 0: Process Over Speed
  - Renamed original Principle 0 to 0.1: Continuous Improvement
  - Enhanced Step 5 with mandatory checkpoints
  - Added comprehensive skip prevention strategies
  - Updated quality checklist with mandatory stop points
