#!/bin/sh
# The drafts trial, group A (backlog 0257), by docs/prompts/drafts-trial/protocol.md, decided before any build.
#
#     sh run_drafts_trial.sh controls   # 10 builds a model with no draft, each checked; then the scorer chooses
#     sh run_drafts_trial.sh drafts     # 10 builds for each draft the controls chose, each checked; then the verdicts
#
# Needs, set by the person running it: SV (the sv release build), SV_REPO (a copy of the repository at that build's
# commit), SV_KEY_ENV_FILE (the .env holding the Anthropic key, read only by the key helper). The builds are made under
# ~/sv-drafts, since Colima shares only the home folder. Models by exact id: Haiku 5.5, never 4.5 (the owner, 12 October
# 2026), and Sonnet 5.5.
set -u
export CLAUDE_BIN="$HOME/Library/Application Support/Claude/claude-code/2.1.295/9d6405864fdc/claude.app/Contents/MacOS/claude"
export KEY_HELPER="$SV_REPO/docs/prompts/loop-pilot/key_helper.sh"
: "${SV:?}" "${SV_REPO:?}" "${SV_KEY_ENV_FILE:?}"
O=~/sv-drafts
T="$SV_REPO/docs/prompts/loop-pilot/loop_trial.py"
S="$SV_REPO/docs/prompts/drafts-trial/score_drafts_trial.py"
CHECK="$SV_REPO/tools/prompt_trial.py"
BUDGET=25        # dollars, the estimate; passing it warns and goes on (the owner, 12 October 2026)
mkdir -p "$O" && cp "$SV_REPO/docs/prompts/trial/policy.toml" "$O/"

model_id() { case $1 in haiku) echo claude-haiku-5-5;; sonnet) echo claude-sonnet-5-5;; esac; }
spent() {
  python3 -c "
import json,glob
print(round(sum(json.loads(l).get('total_cost_usd') or 0 for f in glob.glob('$O/*.jsonl') for l in open(f) if '\"type\":\"result\"' in l),2))"
}
# Before each batch of ten: say so when what is spent, and ten more builds at $0.60 each, would pass the estimate. It
# warns and goes on: the owner, 12 October 2026, "builds shouldn't be canceled if they have to go over the estimated
# cost". Each build is capped at $5 by the Claude program, against a build that runs away, not against the estimate.
guard() {
  now=$(spent)
  python3 -c "import sys; sys.exit(0 if $now + 10 * 0.60 <= $BUDGET else 1)" \
    || echo "NOTE: \$$now spent; ten more may pass the \$$BUDGET estimate. Going on, as the owner asked."
}
batch() {   # batch MODEL ARM_ARGS LABEL: ten builds at once, then each checked with sv report --run
  model=$1; args=$2; label=$3
  guard
  for n in 1 2 3 4 5 6 7 8 9 10; do
    python3 "$T" "$O" none "$model" "$n" --model-id "$(model_id "$model")" --budget 5 --api --with-spec $args &
  done
  wait
  echo "key-in-transcripts: $(cat "$O"/*.jsonl | grep -c 'sk-ant-')"
  (cd "$O" && SV="$SV" python3 "$CHECK" "$O" $(ls | grep -E "^$model-$label-[0-9]+\$") > /dev/null)
  echo "spent so far: \$$(spent)"
}

case "${1:-}" in
  controls)
    for m in haiku sonnet; do
      batch "$m" "" none
      # The first ten builds' cost check of the library trial: an average above $0.60 is said, and the trial goes on.
      avg=$(python3 -c "print(round($(spent) / 10, 3))")
      [ "$m" = haiku ] && { python3 -c "import sys; sys.exit(0 if $avg <= 0.60 else 1)" \
        || echo "NOTE: the first ten builds averaged \$$avg, above the \$0.60 expected. Going on, as the owner asked."; }
    done
    python3 "$S" controls "$O" --json "$O/controls.json" > /dev/null
    echo "CONTROLS DONE"
    ;;
  drafts)
    python3 -c "
import json
for d, m in json.load(open('$O/controls.json'))['choice'].items():
    if m: print(d, m)" | while read -r d m; do
      batch "$m" "--draft $d" "d$d"
    done
    python3 "$S" verdicts "$O" --json "$O/verdicts.json" > /dev/null
    echo "DRAFTS DONE"
    ;;
  *) echo "say controls or drafts"; exit 2 ;;
esac
