---
name: aposlop-deslop-tests
description: Manually remove tests without distinct behavioral value, then simplify production seams used only by them. Invoke only through the aposlop-deslop-tests skill command.
user-invocable: true
disable-model-invocation: true
---

# Deslop tests

Delete tests without distinct behavioral protection, then delete production seams used only by them.
Useful tests fail on plausible regressions and survive behavior-preserving refactors.

## Procedure

1. Read repository rules and test conventions.
2. Use the supplied scope, or the complete repository when none is supplied.
3. Run the smallest relevant baseline and record existing failures.
4. Map each test to its observable contract and production path.

Remove tests that only:

- Omit meaningful assertions, assert tautologies, or repeat fixtures.
- Assert source structure, private symbols, internal calls, wiring, order, or implementation snapshots.
- Duplicate another test's behavior, boundary, and failure mode.
- Recheck compiler, library, framework, or dependency behavior.
- Test private helpers already covered through public behavior.
- Require updates after behavior-preserving source changes.
- Remain skipped, obsolete, or unreachable.

Keep one distinct test for:

- Public output, side effects, or defect regressions.
- Boundaries, invariants, states, precedence, errors, or recovery.
- Security or accessibility.
- Schemas, wire formats, deterministic output, or integration behavior.

Exact source is behavior only when generated, formatted, or analyzed source is the contract.
Mock interactions matter only when the interaction is required behavior, such as exactly-once writes.
Age, speed, size, difficulty, coverage, test count, and shared setup do not determine value.

5. Delete confirmed tests and their unused imports, fixtures, mocks, snapshots, and helpers.
6. Keep shared support with remaining consumers.
7. Run focused retained tests after each removal group.
8. Do not replace weak tests or weaken valuable assertions.

## Shake production

After test cleanup, find test-only visibility, state exposure, injection, interfaces, wrappers, factories, hooks, options, branches, switches, and adapters.
Find every production caller.
Remove or inline only seams with no production or public contract.
Preserve real external and domain boundaries.
Require approval before removing public APIs.
Do not force a production change.

## Verify

Run focused tests, repository checks, and real changed paths.
Do not weaken retained tests to pass.
Report removed tests, support, seams, retained candidates, reasons, and observed checks.
