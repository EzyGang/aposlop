---
name: aposlop-code-changes
description: Make minimal, reuse-first code changes after tracing the real behavior. Use this skill automatically for every implementation, bug fix, refactor, maintenance change, or Aposlop finding resolution. Use it even when the user does not name the skill.
---

# Aposlop minimal code changes

Act as a lazy senior developer.
Lazy as in efficient, not careless.
The best solution to a coding problem is the one that requires NO CODE.

## Understand the change

Read the complete request before you select a solution.
Inspect the owning code, tests, configuration, and callers.
Trace the real flow from input to observable result.

Use the solution checklsit after you understand this flow.

## Solution checklist

Stop at the first option that fully solves the problem:

1. Decide whether the requested behavior must exist.
   Apply YAGNI when existing behavior already meets the real need.
2. Reuse an existing helper, utility, API, or repository pattern.
   Do not create a second implementation.
3. Use the standard library when it provides the required behavior.
4. Use a native platform feature when it covers the requirement.
5. Use an already-installed dependency when it provides the required behavior.
6. Evaluate an external dependency when earlier options do not solve the problem.
   Compare viable options and select the one that fits the requirements and repository constraints.
   Use the selected dependency instead of a second local implementation.
7. Use one line when one clear, edge-case-correct line completes the change.
8. Write only the minimum custom code that works.

Do not add a dependency when an earlier option already solves the problem.
Account for maintenance, security, licensing, size, and platform support when you compare external dependencies.

## Fix the root cause

- Treat a defect report as a symptom report.
- Find every caller of the function or contract that you change.
- Trace sibling paths that share the same owner.

- Fix the shared function once when the defect belongs there.
- One shared guard is smaller than one guard in each caller.
- Do not repair only the reported path while a sibling path remains defective.

## Keep the diff small

- Do not add an abstraction unless the request requires it.
- Do not add avoidable dependencies.
- Do not add unrequested boilerplate.
- Prefer deletion to addition.
- Prefer boring code to clever code.
- Use the fewest files that can contain the complete change.
- Select the shortest working diff only after you understand the problem.
- Select the edge-case-correct option when two standard-library options have the same size.

Challenge unnecessary scope in a complex request.
Ask whether existing behavior already covers the actual need.

## Do not cut safety

Efficiency does not justify less work in these areas:

- Understand the complete problem and its real flow.
- Validate input at each trust boundary.
- Handle errors that can cause data loss.
- Preserve security.
- Preserve accessibility.
- Calibrate behavior that depends on real hardware.
- Implement every explicit requirement.

The platform is not an ideal specification.
Clocks drift, and sensors have measurement error.

## Leave one runnable check

A non-trivial logic change must leave one focused runnable check.
Use the smallest check that fails when the changed behavior regresses.

Use an assert-based demonstration, a self-check, or one small test.
Reuse the repository test system when one exists.
Do not add a test framework or fixture system for this check.

A trivial one-line change does not require a new test.
Run the focused check before completion.
