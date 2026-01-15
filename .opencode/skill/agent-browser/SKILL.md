---
name: agent-browser
description: Use the Vercel `agent-browser` CLI for browser automation (navigate, click, fill, screenshots, accessible snapshots with refs). Prefer refs from `snapshot -i --json` for reliable element targeting.
---

# Agent Browser Skill

Use the Vercel `agent-browser` CLI (https://github.com/vercel-labs/agent-browser) for web automation.

This tool runs as a daemon and persists browser state across commands via **sessions**, so you can iterate with small, deterministic steps.

## Installation

If `agent-browser` is not available:

```bash
pnpm dlx agent-browser@latest --help
```

If you need a globally installed binary:

```bash
npm install -g agent-browser
agent-browser install
```

## Core Workflow (Recommended)

1. Navigate:

```bash
agent-browser open "https://example.com"
```

2. Get an accessibility snapshot with stable refs (best for AI):

```bash
agent-browser snapshot -i --json
```

3. Interact using refs from the snapshot:

```bash
agent-browser click @e2
agent-browser fill @e3 "test@example.com"
```

4. Re-snapshot any time the page changes:

```bash
agent-browser snapshot -i --json
```

## Sessions (Persistent State)

Use sessions to keep cookies/storage isolated between workflows.

```bash
agent-browser --session login-flow open "https://example.com"
agent-browser --session login-flow snapshot -i --json
```

You can also set `AGENT_BROWSER_SESSION`:

```bash
AGENT_BROWSER_SESSION=login-flow agent-browser click @e2
```

## Common Commands

- Snapshot / discovery: `agent-browser snapshot -i --json`
- Click: `agent-browser click <selector|@ref>`
- Fill: `agent-browser fill <selector|@ref> "text"`
- Extract text: `agent-browser get text <selector|@ref> --json`
- Screenshot: `agent-browser screenshot page.png` (add `--full` for full page)
- Wait: `agent-browser wait --text "Welcome"` or `agent-browser wait --url "**/dash"`

## Best Practices

- Prefer `snapshot -i` + `@ref` targets over brittle CSS selectors.
- After each meaningful action, re-run a snapshot and/or query `get url` / `get title` to confirm state.
- Keep command steps small and inspect outputs before proceeding.
- Always close the browser session when finished with `agent-browser close`.
- When debugging, use `--headed` to show a visible browser window.

## Notes

- For full command reference, run `agent-browser --help`.
