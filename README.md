# nexus-dev-graph-map

`nexus-dev-graph-map` is a compact Rust repository for developer tools, centered on this goal: Build a Rust toolkit that studies graph behavior through round-trip fixtures, with lossless normalization checks and no network dependency.

## Why This Exists

I want this repository to be useful as a quick reading exercise: fixtures first, implementation second, verifier last.

## Nexus Dev Graph Map Review Notes

`stress` and `stale` are the cases worth reading first. They show the optimistic and cautious ends of the fixture.

## Capabilities

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/nexus-dev-graph-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `diagnostic quality` and `change width`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Implementation Shape

The fixture data drives the tests. The code stays thin, while `metadata/domain-review.json` and `config/review-profile.json` explain what each case is meant to protect.

The Rust code keeps the review rule close to the tests.

## Local Usage

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Verification

The check exercises the source code and the review fixture. `stress` is the high score at 217; `stale` is the low score at 164.

## Roadmap

The repository is intentionally scoped to local checks. I would expand it by adding adversarial fixtures before adding features.
