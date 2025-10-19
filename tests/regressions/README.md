# Fuzz Regression Tests

This folder stores regression scenarios recovered from fuzz artifacts (crash-*, 	imeout-*, leak-*).

How to add a new test:

1. Copy 	emplate.rs, rename it following the crash_YYYYMMDD_HHMM.rs pattern.
2. Replace the placeholder sequence with the actual operations from the artifact.
3. Remove #[ignore] once the test is stable and should run by default.
4. Add links to the original artifact and the tracking issue/PR in comments.

Keep each regression test minimal and deterministic.
