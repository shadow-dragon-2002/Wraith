> SUPERSEDED — this file had incorrect workflow info. See below for the real flow.

# Wraith — Matt Pocock Skill Workflow (Accurate)

## Step 0 — Run this first, once
```
/setup-matt-pocock-skills
```
Scaffolds `docs/agents/` (issue tracker, triage labels, domain layout) and adds the
`## Agent skills` block to `CLAUDE.md`. All other skills depend on this config existing.

---

## The Flow

```
/grill-with-docs  →  /to-prd  →  /to-issues  →  /tdd
                                                     ↑
                                           repeat per issue
```

**`/improve-codebase-architecture`** — run between major milestones and before final release.

---

## What Each Skill Actually Does

**`/grill-with-docs`**
Interviews you interactively about your plan, one question at a time. Explores the codebase itself — you don't feed it a file. Creates/updates `CONTEXT.md` (glossary) and `docs/adr/` inline as decisions crystallise. Run at the start of any significant new work.

**`/to-prd`**
Synthesises the current conversation into a PRD and **publishes it as a GitHub Issue** with `needs-triage` label. Does NOT write a local file. Does NOT interview you — it uses conversation context it already has.

**`/to-issues`**
Takes a PRD issue (by number or URL) and breaks it into individual **vertical-slice GitHub Issues**, each independently implementable. Publishes them directly to GitHub Issues. Quizzes you on granularity before publishing.

**`/tdd`**
Red-green-refactor loop. One test → one implementation → repeat. Does NOT consume a test spec file — it does TDD naturally using the issue and `CONTEXT.md` vocabulary. Warns you not to write all tests first.

**`/improve-codebase-architecture`**
Explores `src/` organically, reads `CONTEXT.md` + `docs/adr/`, surfaces "deepening opportunities" (shallow→deep module refactors). Updates `CONTEXT.md` inline. Offers ADRs when warranted.

---

## Key Files These Skills Read/Write

| File | Purpose |
|------|---------|
| `CONTEXT.md` | Domain glossary — terms only, no implementation details |
| `docs/adr/0001-slug.md` | Individual ADRs — one decision per file |
| `docs/agents/issue-tracker.md` | Which issue tracker + how to create issues |
| `docs/agents/triage-labels.md` | Label vocabulary mapping |
| `docs/agents/domain.md` | Where CONTEXT.md and docs/adr/ live |

---

## Note on `/domain-modeling` and `/codebase-design`
These are **not** Matt Pocock skills. They don't exist in his repo. `/grill-with-docs` covers domain modelling inline, and `/improve-codebase-architecture` covers codebase design review.
