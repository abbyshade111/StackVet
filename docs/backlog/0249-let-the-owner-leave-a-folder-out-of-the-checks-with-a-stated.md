# Let the owner leave a folder out of the checks, with a stated reason shown in the report

**Status:** claimed by backlog-org-0249, 12 October 2026

From the same report as backlog 0248, where the owner's own test folder was the source of most of the noise. Proposal:
a setting, in the app's `securevibe.toml`, that names a folder to leave out of the checks, with the owner's reason for it,
which the report prints beside the folder. It is never silent: the report lists each folder left out and its reason, and
a secret in a left-out folder is still found by the secrets scan unless the owner says otherwise in the same setting.
Open question for the owner: should a folder left out still be checked for real keys (recommended), or not at all?

**Record, `Status: proposed`: ADR-099** (12 October 2026), with the owner's answer of the same day.
