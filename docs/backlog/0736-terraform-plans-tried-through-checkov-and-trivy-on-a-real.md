# Terraform plans tried through Checkov and Trivy on a real plan

**Status:** open

Filed 22 November 2026, at the owner's word. ADR-157, part 3 ("As built", "Not done: Terraform plans tried through the
tools"): `cv` hands Checkov the Terraform plans it read, and Trivy the folder, but neither was tried on a real plan.
Part 3 found that Checkov given a plan it cannot parse says nothing and exits 0, so a plan it reads wrongly could look
clean.

- **To do:** make a real plan (`terraform show -json`) of a small, safe configuration with a known unsafe setting, in a
  test fixture, with no provider credentials and nothing applied; run `cv check` on it with both tools installed; and
  record what each tool found and what it missed.
- **Check:** a test, when the tools are installed, that each tool reports the known setting; and that a plan neither
  tool can read is said as not assessed, never as a pass.
- **Not here:** installing Terraform without asking.
