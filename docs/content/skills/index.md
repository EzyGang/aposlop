# Agent skills

Aposlop includes optional guides for coding agents.
The skills are separate from the CLI and do not scan code themselves.

## Install

Use the Aposlop CLI:

```bash
aposlop install-skills
```

You can also use either package runner:

```bash
npx skills@latest add EzyGang/aposlop
pnpm dlx skills@latest add EzyGang/aposlop
```

The installer lets you choose skills and target agents.

## Choose a skill

| Skill | Purpose | Activation |
| --- | --- | --- |
| [`aposlop`](aposlop.md) | Set up, run, and read Aposlop | Automatic |
| [`aposlop-code-changes`](code-changes.md) | Make small, complete code changes | Automatic |
| [`aposlop-test-changes`](test-changes.md) | Add a few tests that protect behavior | Automatic |
| [`aposlop-deslop-tests`](deslop-tests.md) | Remove tests without a distinct purpose | Manual |
| [`aposlop-deslop-turbo`](deslop-turbo.md) | Reduce tests and production code | Manual |

Automatic skills start when the request matches their purpose.
Start a manual skill with its slash command when the agent supports slash commands.
Otherwise, select the skill by name through the agent interface.
