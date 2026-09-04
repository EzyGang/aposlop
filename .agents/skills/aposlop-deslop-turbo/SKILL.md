---
name: aposlop-deslop-turbo
description: Manually complete aposlop-deslop-tests, then perform architecture-wide and local refactors under aposlop-code-changes. Maximize net LOC deletion without changing observable behavior. Invoke only through the aposlop-deslop-turbo skill command.
compatibility: Requires the aposlop-deslop-tests and aposlop-code-changes skills.
user-invocable: true
disable-model-invocation: true
---

# Deslop turbo

Run two phases serially with one scope.
Default to the complete repository.
Include reachable shared owners.
Exclude generated, vendored, dependency, and build output.

## Phase 1: Tests

1. Invoke `aposlop-deslop-tests`.
2. Wait for all edits and checks.
3. Stop on failure, blocking, or active work.
4. Start Phase 2 only after test deslop completes.

No deletion is valid.
If nested invocation is unavailable, load and follow the installed skill unchanged.

## Phase 2: Production

1. Load `aposlop-code-changes`.
2. Map ownership, responsibilities, data flow, state lifetimes, boundaries, and calls.
3. Scan every production module for architectural reductions before local cleanup.
4. Trace contracts through tests, public documentation, configuration, interfaces, entry points, and callers.
5. Run baseline scenarios where those sources do not establish behavior.

Optimize the complete design, not each local diff.
Permit larger, cross-file refactors when they reduce net LOC and simplify ownership.
Tests are evidence, not the complete specification.

Preserve:

- Public APIs and accepted inputs.
- Values, formats, ordering, errors, diagnostics, and exits.
- Visible filesystem, network, process, and other effects.
- Configuration defaults and precedence.
- Security, accessibility, required concurrency, and required performance.

Keep behavior with an unclear contract.

### Architecture sweep

Actively find:

- Concepts represented, stored, validated, or transformed more than once.
- Responsibilities split across pass-through layers or mirrored modules.
- Abstractions with one implementation, one caller, or hypothetical variants.
- State synchronized across owners instead of derived or owned once.
- Wrappers, adapters, factories, or traits without a real boundary.
- Repeated conversion, validation, configuration, or error translation.
- Custom frameworks covered by the language, platform, or dependencies.
- Unrequired concurrency, async work, caching, genericity, extensibility, or configurability.

Move ownership, consolidate data models, and remove layers when behavior stays fixed.
Trace every affected caller.
Architectural defects are in scope because architecture is not behavior.
Observable defect fixes change behavior and must be reported separately.

Then remove dead code, duplicate logic, redundant control flow, temporaries, allocations, copies, clones, collections, visibility, options, and adapters.
Delete code instead of hiding it behind abstractions.
Do not share distinct behavior when sharing adds conditions or coupling.

### Broken tests

Apply `aposlop-deslop-tests` criteria to each test broken by a refactor.

1. Verify the observable path independently.
2. Restore production behavior when its contract changed.
3. Remove the test when behavior is unchanged and it protects no distinct contract.
4. Rewrite one observable test when a distinct contract still needs protection.
5. Stop when the contract remains ambiguous.

Never change an expected result to hide changed behavior.

## Verify

LOC reduction never permits code golf, unrelated merging, cosmetic renames, weaker safety, or unapproved API changes.
Preserve trust-boundary validation, data-loss protection, security, and accessibility.
Prefer boring, edge-case-correct code.
Review every module and caller until no safe reduction remains.
Measure production LOC before and after with one method when reliable.
Run focused checks, all applicable repository checks, and changed entry points.
Restore regressions and finish no active or failed work.
Report Phase 1, removed code, LOC change, retained structures, separate defects, and observed checks.
