---
name: aposlop-test-changes
description: Checklist for adding or changing tests, read before writing the first test. Use it for a regression test after a bug fix, tests for a new feature, flag or option, and any unit or integration test a change needs, including when the user only says to add tests or to run the relevant tests as part of a code change. It keeps the test set to the fewest tests that protect visible behavior, with no duplicate cases or coverage padding, checked through public entry points.
---

# Aposlop test changes

Write the fewest tests that protect real behavior.
A good test fails when visible behavior breaks.
It stays unchanged when internal code changes.

## Before writing a test

1. Read the changed behavior, public entry point, existing tests, and project test rules.
2. State the input, action, and expected result.
3. Find tests that already protect that result.
4. Name the likely bug that the new test must catch.

Add no test when an existing test catches that bug.

## Choose what to test

Add a test only for a distinct behavior:

- A bug that could return.
- A boundary where the result changes.
- A rule about state, order, priority, or errors.
- A visible output or side effect.
- A security, access, storage, or data format rule.

Use the smallest test level that proves the behavior through a public entry point.
Use an integration test only when the result depends on an external boundary.

## Check results, not code

Check only what a caller can see:

- A returned value.
- Public state.
- A visible side effect.
- An error type and its useful data.
- Exact output when its format is a contract.

Do not check:

- Private functions, fields, branches, or algorithms.
- Internal calls, wiring, or forwarding.
- Call order unless that order is promised.
- Mock calls unless that interaction is promised.
- Snapshots unless the full output is promised.
- Language, framework, or library behavior.
- Only that a result exists or no error occurred.

Keep related checks together when they describe one result.
Split a test only when each failure explains a different broken rule.

## Prevent duplicate tests

Do not write one test for every branch, parameter, or method.
Use one input for each distinct rule.
Test a boundary only where behavior changes.
Use a table only when each row protects a different rule or boundary.
Do not test the same behavior at unit and integration levels.

Extend an existing test when the action and result are the same.
Use existing setup code.
Keep one-use setup inside its test.

## Keep tests reliable

Use fixed inputs.
Control time, randomness, processes, network access, and concurrent work.
Do not depend on test order, repository files, or shared state.
Use real collaborators when they are fast and stable.
Fake only slow, destructive, or unstable external systems.

For a bug fix, run the new test against the old behavior when practical.
Observe the expected failure.
Apply the fix.
Run the focused test.
Run the applicable test suite.

Remove test code that the change makes unused.
Report the protected behavior and the checks you ran.
Do not report success by counting tests.
