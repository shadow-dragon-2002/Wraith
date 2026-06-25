# Issue Tracker

Issues live in **GitHub Issues** on the `nightraven/wraith` repository.

## Creating issues

Use the `gh` CLI:

```bash
gh issue create --title "..." --body "..." --label "needs-triage" --repo nightraven/wraith
```

## Reading issues

```bash
gh issue view <number> --repo nightraven/wraith
gh issue list --repo nightraven/wraith
```

## Closing / commenting

```bash
gh issue close <number> --repo nightraven/wraith
gh issue comment <number> --body "..." --repo nightraven/wraith
```
