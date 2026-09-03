---
name: aposlop-deslop-turbo
description: Run a manual, sequential repository simplification. Complete aposlop-deslop-tests first. Then apply aposlop-code-changes across all applicable production logic to remove the most code possible without changing behavior. Invoke only through the aposlop-deslop-turbo skill command.
compatibility: Requires the aposlop-deslop-tests and aposlop-code-changes skills.
user-invocable: true
disable-model-invocation: true
---

# Deslop turbo

Remove the maximum safe amount of code while preserving existing behavior.
Complete the test cleanup before you simplify the full application.

One turbo iteration has two ordered phases.
Do not overlap or parallelize these phases.

## Set the scope

Use the scope supplied with the turbo skill command.
Use the complete repository when the command supplies no scope.

Apply the same scope to both phases.
Include all production logic reachable from a narrower scope and its shared owners.
Exclude generated code, vendored code, dependencies, and build output.

## Phase 1: Deslop the tests

Invoke `aposlop-deslop-tests` with the turbo scope.
Complete its full workflow, including its production-seam cleanup and verification.

Wait until all Phase 1 edits and checks finish.
Do not start Phase 2 while any Phase 1 work remains active.
Do not start Phase 2 when Phase 1 has a failure or unresolved blocker.

A Phase 1 result with no safe deletion is still complete.
Continue only after the test-deslop iteration reports completion.

If the client cannot invoke a nested skill, load the installed `aposlop-deslop-tests` instructions.
Follow those instructions without copying or weakening them.

## Phase 2: Deslop the application

Load and apply `aposlop-code-changes` across every applicable production module.
Do not limit this phase to production seams found during Phase 1.

If the client cannot invoke a nested skill, load the installed `aposlop-code-changes` instructions.
Follow its solution checklist for every candidate simplification.

### Establish the behavior boundary

Read the surviving tests, public documentation, configuration, interfaces, and runtime entry points.
Trace each applicable path from input to observable result.
Run focused baseline scenarios when the surviving tests do not establish current behavior.

Treat tests as evidence, not as the complete specification.
Preserve these behaviors:

- Public APIs and accepted inputs.
- Output values, formats, and deterministic ordering.
- Errors, diagnostics, and exit behavior.
- Filesystem, network, process, and other visible side effects.
- Configuration defaults and precedence.
- Security and accessibility properties.
- Required concurrency and performance characteristics.

Do not remove behavior when its purpose is unclear.
Find its callers and contract first.

### Sweep all production logic

Inspect each applicable production module and every affected caller.
Prefer these reductions when behavior remains identical:

1. Delete unreachable or unused code after checking all references.
2. Delete pass-through wrappers and one-use indirection.
3. Reuse an existing owner instead of keeping duplicate logic.
4. Replace custom code with the standard library or an existing platform feature.
5. Reuse an installed dependency when it already owns the behavior.
6. Evaluate an external dependency as required by the code-change solution checklist.
7. Collapse redundant branches, states, conversions, and temporary values.
8. Remove unnecessary allocations, copies, clones, and intermediate collections.
9. Reduce visibility that no production contract requires.
10. Delete options, adapters, and abstractions with no remaining production variant.

Remove code instead of moving it behind a new abstraction.
Do not create shared code only to reduce the line count.
Keep distinct behavior separate when a shared abstraction would add conditions or coupling.

### Protect maintainability

Line reduction is the optimization target after behavior preservation.
It does not justify compressed, obscure, or fragile code.

Do not:

- Combine unrelated responsibilities.
- Replace clear operations with code golf.
- Rename symbols only to shorten lines.
- Remove trust-boundary validation.
- Remove error handling that prevents data loss.
- Weaken security or accessibility.
- Change a public API without an approved breaking change.
- Change tests only to accept altered behavior.

Select boring code when two reductions remove the same amount.
Select the edge-case-correct implementation when two options have similar size.

### Complete the sweep

Review every applicable production module at least once.
Revisit direct callers after each shared-owner simplification.
Continue until the full scope has no remaining safe reduction.
Do not force a change in code that already has the smallest clear form.

Record production line counts before and after Phase 2 when repository tooling supports a reliable comparison.
Use the same scope and counting method for both values.

## Verify the turbo iteration

Run focused checks after each coherent behavior-preserving change.
Run the repository's complete applicable validation suite after the sweep.
Exercise real entry points for changed application paths.
Restore behavior when a check or smoke scenario exposes a regression.

Do not weaken surviving tests to make the turbo iteration pass.
Do not report completion while either phase has active or failed work.

Report:

- The completed test-deslop result.
- Removed production code and obsolete seams.
- Production lines added, deleted, and net removed when measured reliably.
- Structures retained because they protect behavior or clarity.
- Verification commands and observed results.
