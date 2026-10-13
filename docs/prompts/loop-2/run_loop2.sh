#!/bin/sh
# The second loop trial (backlog 0257 and 0006 part 21, ADR-109), by docs/prompts/loop-2/protocol.md, decided before any
# build.
#
#     sh run_loop2.sh outside   # the drafts trial's twenty controls checked again with the outside tools (no model money)
#     sh run_loop2.sh a         # the club brief, loop arm: 10 builds a model, each checked twice
#     sh run_loop2.sh b1        # the accounts brief, no sv while building
#     sh run_loop2.sh b2        # the accounts brief, loop arm
#     sh run_loop2.sh c         # the accounts brief offering none of the three changes, loop arm, scripted review: 5 a model
#     sh run_loop2.sh score     # score_loop2.py over everything
#
# Needs, set by the person running it: SV (the sv release build), SV_REPO (a copy of the repository at that build's
# commit), SV_KEY_ENV_FILE (the .env holding the Anthropic key, read only by the key helper). The builds are made under
# ~/sv-loop2, since Colima shares only the home folder. Models by exact id: Haiku 5.5, never 4.5 (the owner, 12 October
# 2026), and Sonnet 5.5.
set -u
export CLAUDE_BIN="$HOME/Library/Application Support/Claude/claude-code/2.1.295/9d6405864fdc/claude.app/Contents/MacOS/claude"
export KEY_HELPER="$SV_REPO/docs/prompts/loop-pilot/key_helper.sh"
# sv reads its data from the same fixed copy, not from wherever it was compiled, which may change during the run.
export SV_DATA_DIR="$SV_REPO/data"
: "${SV:?}" "${SV_REPO:?}" "${SV_KEY_ENV_FILE:?}"
O=~/sv-loop2
DRAFTS=~/sv-drafts
# The trial's own settings and data folders: its signing key, its history (switched on), and any crash file, so the
# owner's are never read or touched. And sv's own log, for every check (ADR-100).
export XDG_CONFIG_HOME="$O/config" XDG_DATA_HOME="$O/data" SV_LOG="$O/sv.log"
T="$SV_REPO/docs/prompts/loop-pilot/loop_trial.py"
CHECK="$SV_REPO/tools/prompt_trial.py"
L2="$SV_REPO/docs/prompts/loop-2"
BUDGET=50        # dollars, the owner's; passing it is said and the trial goes on (the owner, 12 October 2026)
mkdir -p "$O" "$XDG_CONFIG_HOME/stackvet" "$XDG_DATA_HOME" && cp "$SV_REPO/docs/prompts/trial/policy.toml" "$O/"
touch "$XDG_CONFIG_HOME/stackvet/keep-history"

model_id() { case $1 in haiku) echo claude-haiku-5-5;; sonnet) echo claude-sonnet-5-5;; esac; }
spent() {
  python3 -c "
import json,glob
print(round(sum(json.loads(l).get('total_cost_usd') or 0 for f in glob.glob('$O/*.jsonl') for l in open(f, encoding='utf-8', errors='replace') if '\"type\":\"result\"' in l),2))"
}
note_budget() {
  now=$(spent)
  python3 -c "import sys; sys.exit(0 if $now <= $BUDGET else 1)" \
    || echo "NOTE: \$$now spent, past the \$$BUDGET budget. Going on, as the owner asked."
}
# batch MODEL ARM LABEL BRIEF COUNT [CHECK_ENV]: COUNT builds at once, then each checked by prompt_trial.py, twice.
batch() {
  model=$1; arm=$2; label=$3; brief=$4; count=$5; extra=${6:-}
  n=1
  while [ "$n" -le "$count" ]; do
    python3 "$T" "$O" "$arm" "$model" "$n" --label "$label" --brief "docs/prompts/loop-2/$brief" \
      --model-id "$(model_id "$model")" --budget 5 --api --with-spec --allow-git &
    n=$((n + 1))
  done
  wait
  echo "key-in-transcripts: $(cat "$O"/*.jsonl | grep -c 'sk-ant-')"
  # $extra is one NAME=VALUE whose value may hold a space (the USB drive's name): passed whole, never split. Split, it
  # stopped every check of phase C on 13 October 2026 (protocol, amendment 1).
  (cd "$O" && env SV="$SV" PT_TOOLS=1 ${extra:+"$extra"} python3 "$CHECK" "$O" $(ls | grep -E "^$model-$label-[0-9]+\$") > /dev/null)
  echo "$model $label checked; spent so far: \$$(spent)"
  note_budget
}
phase() {   # phase ARM LABEL BRIEF COUNT [CHECK_ENV]: both models
  for m in haiku sonnet; do batch "$m" "$@"; done
  echo "PHASE $2 DONE"
}

case "${1:-}" in
  outside)
    # The club brief's arm without sv is the drafts trial's controls (protocol, "The builds"): their copies, already
    # checked with --run, checked again with the outside tools only.
    for d in "$DRAFTS"/runs/*-none-*; do
      case "$d" in *-out|*.log|*.summary) continue;; esac
      "$SV" report "$d" --tools --keep-tool-output --out "$d-tools-out" > "$d.tools.log" 2>&1
      echo "$(basename "$d"): outside tools exit $?"
    done
    echo "PHASE outside DONE"
    ;;
  a)  phase loop a club-brief.md 10 ;;
  b1) phase none b1 accounts-brief.md 10 ;;
  b2) phase loop b2 accounts-brief.md 10 ;;
  c)  phase loop c accounts-none-brief.md 5 "PT_REVIEW=$L2/scripted_review.py" ;;
  score)
    # The history's test (protocol, measure 10): each build's two reports compared.
    for d in "$O"/runs/*-tools-out; do
      b=${d%-tools-out}
      "$SV" compare "$b-out" "$d" > "$b.compare.txt" 2>&1
    done
    python3 "$L2/score_loop2.py" "$O" --drafts "$DRAFTS" --json "$O/loop2-scores.json"
    python3 "$SV_REPO/docs/prompts/loop-pilot/loop_measures.py" "$O" $(cd "$O" && ls *.jsonl | sed 's/\.jsonl$//') \
      --json "$O/loop-measures.json" > /dev/null
    echo "SCORED"
    ;;
  *) echo "say outside, a, b1, b2, c, or score"; exit 2 ;;
esac
