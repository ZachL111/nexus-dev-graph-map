# Nexus Dev Graph Map Walkthrough

The fixture is intentionally compact, so the review starts with the cases that pull farthest apart.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 165 | ship |
| stress | diagnostic quality | 217 | ship |
| edge | review cost | 194 | ship |
| recovery | safe rewrite | 172 | ship |
| stale | change width | 164 | ship |

Start with `stress` and `stale`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

If `stale` becomes less cautious without a clear reason, I would inspect the drag input first.
