---
name: aposlop-deslop-tests
description: Manually audit tests that reassert source structure, duplicate covered behavior, or provide no meaningful regression protection. Remove confirmed low-value tests, then simplify production seams that only those tests required. Invoke only through the aposlop-deslop-tests skill command.
user-invocable: true
disable-model-invocation: true
---

# Deslop tests

Remove tests that do not protect observable behavior.
Then simplify production code that exists only to support those tests.

Tests must justify their maintenance cost.
A useful test fails after a plausible behavior regression.
It should survive an internal refactor that preserves behavior.

## Establish the scope

1. Read the repository instructions and test conventions.
2. Use the scope supplied with the skill command.
3. Audit the complete repository when the command supplies no scope.
4. Run the smallest relevant baseline test command.
5. Record existing failures without attributing them to this sweep.
6. Map each test to its observable contract and production path.

Do not use coverage percentage or test count as proof of value.
Trace the behavior that each candidate claims to protect.

## Find low-value tests

A test is a removal candidate when it has one or more of these defects:

- It has no meaningful assertion.
- Its assertion is a tautology or only repeats its fixture.
- It reads source text only to assert tokens, names, line structure, or private symbols.
- It asserts internal calls, mock wiring, or implementation order instead of an observable result.
- It duplicates an existing test with the same behavior, boundary, and failure mode.
- It tests compiler, standard-library, framework, or dependency behavior without adding a project contract.
- It tests a private helper whose behavior is already covered through the owning public path.
- Its snapshot mirrors implementation details and changes after behavior-preserving edits.
- It is permanently skipped, obsolete, or unreachable.

Maintenance churn is strong evidence of an implementation test.
A test that must change with behavior-preserving source edits does not protect behavior.

Source text can be an observable result for generators, formatters, analyzers, and stable output contracts.
Keep source assertions when exact source output is the product contract.

Mock interactions can also be observable at an external boundary.
Keep tests for required effects such as exactly-once writes, transactions, retries, or audit events.

## Preserve valuable tests

Keep a test when it protects at least one distinct contract:

- Public output or externally visible side effects.
- A plausible defect regression.
- A boundary value or domain invariant.
- A state transition or precedence rule.
- A real failure path or recovery rule.
- A security or accessibility property.
- A stable schema, wire format, or deterministic report.
- Integration behavior across owned boundaries.

Do not delete a test only because it is old, slow, large, or difficult.
Repair it when the protected behavior remains valuable.

Shared setup does not make two tests duplicates.
Compare their inputs, observable assertions, boundaries, and failure modes.

## Remove confirmed waste

1. Identify the retained test that protects each duplicated contract.
2. Remove only candidates with no distinct behavioral protection.
3. Remove unused test imports, fixtures, mocks, snapshots, and helpers.
4. Keep shared test support that still has a real consumer.
5. Run the focused retained tests after each coherent removal group.

Do not replace one low-value test with another implementation assertion.
Do not weaken a valuable assertion to make deletion appear safe.

## Shake the production code

Inspect production code after the test cleanup.
Find seams that only the removed tests required.

Common test-only seams include:

- Unnecessary public visibility.
- Dependency injection with no production variant.
- One-use interfaces, traits, wrappers, factories, or hooks.
- Constructor options that production callers never select.
- Branches, feature switches, or adapters used only by deleted tests.
- Indirection that exposes private state for assertions.

Find every production caller before you simplify a seam.
Do not infer production use from test references alone.

Remove or inline a seam only when no production contract requires it.
Restore the simplest natural ownership, visibility, and control flow.
Preserve architecture boundaries that isolate real external systems or domain responsibilities.

Do not remove a public API without an approved breaking change.
Do not force a production simplification when none is safe.
A test-only cleanup can be the correct complete result.

## Verify the result

Run the focused tests for each changed behavior.
Run the repository's applicable completion checks.
Exercise the real behavior when production code changes.
Confirm that retained tests still detect the contracts assigned to them.

Report:

- Removed tests and why they had no distinct behavioral value.
- Removed test support and production seams.
- Candidates retained because they protect distinct behavior.
- Verification commands and observed results.
