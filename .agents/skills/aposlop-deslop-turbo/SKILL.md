---
name: aposlop-deslop-turbo
description: Manually complete aposlop-deslop-tests, then apply aposlop-code-changes across production logic. Remove maximum LOC without changing behavior. Invoke only through the aposlop-deslop-turbo skill command.
compatibility: Requires the aposlop-deslop-tests and aposlop-code-changes skills.
user-invocable: true
disable-model-invocation: true
---

# Deslop turbo

Run two phases in order.
Never overlap them.
Use the supplied scope for both, or the complete repository when none is supplied.
Include reachable shared owners.
Exclude generated, vendored, dependency, and build output.

## Phase 1: Tests

1. Invoke `aposlop-deslop-tests` with the turbo scope.
2. Wait for all edits and checks.
3. Stop on failure, blocking, or active work.
4. Continue only after the test-deslop iteration completes.

No deletion is a valid result.
If nested invocation is unavailable, load and follow the installed skill unchanged.

## Phase 2: Production

1. Load `aposlop-code-changes`.
2. Apply it to every applicable production module, not only test seams.
3. Trace behavior through surviving tests, public documentation, configuration, interfaces, entry points, paths, and callers.
4. Run a baseline scenario when those sources do not establish behavior.

Tests are evidence, not the complete specification.
Preserve:

- Public APIs and accepted inputs.
- Values, formats, ordering, errors, diagnostics, and exits.
- Visible filesystem, network, process, and other effects.
- Configuration defaults and precedence.
- Security, accessibility, required concurrency, and required performance.

Keep behavior with an unclear contract.

Sweep every module for:

- Unreachable or unused code.
- Pass-through wrappers and one-use indirection.
- Duplicate logic with an existing owner.
- Custom code covered by the standard library, platform, or installed dependencies.
- External dependencies selected by the code-change checklist.
- Redundant branches, states, conversions, temporaries, allocations, copies, clones, and collections.
- Unneeded visibility, options, adapters, and abstractions.

Delete code instead of hiding it behind abstractions.
Do not share distinct behavior when sharing adds conditions or coupling.

LOC reduction never permits code golf, unrelated merging, cosmetic renames, weaker tests, weaker safety, or unapproved API changes.
Preserve trust-boundary validation, data-loss protection, security, and accessibility.
Prefer boring, edge-case-correct code.

Review every module and affected caller until no safe reduction remains.
Do not force changes to minimal code.
Measure production LOC before and after with one scope and method when reliable.

## Verify

Run focused checks after each coherent change.
Run all applicable repository checks and real changed entry points.
Restore behavior after any regression.
Do not finish with active or failed work.
Report Phase 1, removed code, measured LOC change, retained structures, and observed checks.
