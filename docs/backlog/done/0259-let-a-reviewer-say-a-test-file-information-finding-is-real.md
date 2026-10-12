# Let a reviewer say a test-file information finding is real, so it counts (sv review step, ADR-091)

**Status:** done, 11 October 2026

Built 11 October 2026 (ADR-093): a sealed `[[finding-review]]` entry with the verdict `counts`, for a test-file information finding a person has read and found real. It counts as a finding, and is refused for any other finding. Breaks: the refusal, and the counted fingerprint in the split, each fail a test.
