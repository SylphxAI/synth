#!/usr/bin/env bash
# Synth parser/markdown-wasm + parser/javascript-wasm differential parity — TS live oracle vs native Rust parsers.
# Fail-closed: requires bun (no SKIP-as-pass). See PARITY-VERIFICATION-STANDARD.md, DECISION-001 / rej-010.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SCRATCH="${SCRATCH_DIR:-/tmp/synth-differential}"
mkdir -p "$SCRATCH"
LOG="$SCRATCH/differential.log"
ARTIFACT="$SCRATCH/verification.json"
ORACLE_JSON="$SCRATCH/oracle.json"
SLICE_FILTER="all"
: >"$LOG"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --slice)
      SLICE_FILTER="${2:-}"
      shift 2
      ;;
    *)
      echo "::error::unknown argument: $1" | tee -a "$LOG"
      exit 1
      ;;
  esac
done

case "$SLICE_FILTER" in
  all|parser/markdown-wasm|parser/javascript-wasm) ;;
  *)
    echo "::error::invalid --slice value: $SLICE_FILTER" | tee -a "$LOG"
    exit 1
    ;;
esac

cd "$REPO_ROOT"

if ! command -v bun >/dev/null 2>&1; then
  echo "::error::bun required for synth differential parity — no SKIP-as-pass" | tee -a "$LOG"
  exit 1
fi

echo "=== synth markdown-wasm + javascript-wasm differential parity $(date -Iseconds) slice=$SLICE_FILTER ===" | tee -a "$LOG"

echo "--- check-no-ts-parser gate ---" | tee -a "$LOG"
bash "$REPO_ROOT/scripts/check-no-ts-parser.sh" 2>&1 | tee -a "$LOG"

echo "--- check-no-ts-js-parser gate ---" | tee -a "$LOG"
bash "$REPO_ROOT/scripts/check-no-ts-js-parser.sh" 2>&1 | tee -a "$LOG"

echo "--- TS live-parser oracle (golden corpora) ---" | tee -a "$LOG"
bun run "$REPO_ROOT/scripts/differential/synth-oracle.ts" >"$ORACLE_JSON" 2>>"$LOG"

run_rust_slice_test() {
  local label="$1"
  local crate="$2"
  local test_name="$3"
  echo "--- Rust bounded slice: $label ---" | tee -a "$LOG"
  SYNTH_ORACLE_JSON="$ORACLE_JSON" \
    cargo test -p "$crate" --lib "$test_name" -- --nocapture 2>&1 | tee -a "$LOG"
}

case "$SLICE_FILTER" in
  parser/markdown-wasm)
    run_rust_slice_test "parser/markdown-wasm" synth-wasm-md parser_markdown_wasm_differential_matches_ts_oracle
    ;;
  parser/javascript-wasm)
    run_rust_slice_test "parser/javascript-wasm" synth-wasm-js parser_javascript_wasm_differential_matches_ts_oracle
    ;;
  all)
    run_rust_slice_test "parser/markdown-wasm" synth-wasm-md parser_markdown_wasm_differential_matches_ts_oracle
    run_rust_slice_test "parser/javascript-wasm" synth-wasm-js parser_javascript_wasm_differential_matches_ts_oracle
    echo "--- existing golden parity gates ---" | tee -a "$LOG"
    cargo test -p synth-wasm-md golden_fixtures_match_ts_baseline 2>&1 | tee -a "$LOG"
    cargo test -p synth-wasm-js golden_fixtures_match_ts_baseline 2>&1 | tee -a "$LOG"
    ;;
esac

CANDIDATE_SHA="${CANDIDATE_SHA:-$(git -C "$REPO_ROOT" rev-parse HEAD 2>/dev/null || echo unknown)}"
BASELINE_TS_SHA="$(git -C "$REPO_ROOT" log -1 --format=%H -- packages/synth-js/src/parser.ts packages/synth-md/src/parser.ts scripts/differential test/fixtures 2>/dev/null || echo unknown)"
RUST_SHA="$CANDIDATE_SHA"
BEHAVIOR_SPEC_HASH="$(sha256sum "$REPO_ROOT/scripts/differential/fixtures/synth-corpus.json" "$REPO_ROOT/test/fixtures/javascript-parity/golden.json" "$REPO_ROOT/test/fixtures/markdown-parity/golden.json" 2>/dev/null | awk '{print $1}' | sha256sum | awk '{print $1}' || echo missing)"
FIXTURE_CORPUS_HASH="$(jq -r '.fixtureCorpusHash' "$ORACLE_JSON")"
CASE_COUNT="$(jq '.cases | length' "$ORACLE_JSON")"
MARKDOWN_CASE_COUNT="$(jq '[.cases[] | select(.slice == "parser/markdown-wasm")] | length' "$ORACLE_JSON")"
JAVASCRIPT_CASE_COUNT="$(jq '[.cases[] | select(.slice == "parser/javascript-wasm")] | length' "$ORACLE_JSON")"

jq -n \
  --arg verifiedAt "$(date -Iseconds)" \
  --arg candidateSha "$CANDIDATE_SHA" \
  --arg baselineTsSha "$BASELINE_TS_SHA" \
  --arg rustCandidateSha "$RUST_SHA" \
  --arg behaviorSpecHash "$BEHAVIOR_SPEC_HASH" \
  --arg fixtureCorpusHash "$FIXTURE_CORPUS_HASH" \
  --arg sliceFilter "$SLICE_FILTER" \
  --argjson caseCount "$CASE_COUNT" \
  --argjson markdownCaseCount "$MARKDOWN_CASE_COUNT" \
  --argjson javascriptCaseCount "$JAVASCRIPT_CASE_COUNT" \
  '{
    schemaVersion: 2,
    slice: (if $sliceFilter == "all" then "parser/markdown-wasm|parser/javascript-wasm" else $sliceFilter end),
    sliceFilter: $sliceFilter,
    status: "differential_green",
    verifiedAt: $verifiedAt,
    lastComparedMainSha: $candidateSha,
    mergeGroupSha: $candidateSha,
    baselineTsSha: $baselineTsSha,
    rustCandidateSha: $rustCandidateSha,
    behaviorSpecHash: $behaviorSpecHash,
    fixtureCorpusHash: $fixtureCorpusHash,
    caseCount: $caseCount,
    markdownCaseCount: $markdownCaseCount,
    javascriptCaseCount: $javascriptCaseCount,
    harness: "scripts/run-synth-differential.sh",
    differentialTest: "crates/wasm-md/src/parser_v2.rs#parser_markdown_wasm_differential_matches_ts_oracle; crates/wasm-js/src/lib.rs#parser_javascript_wasm_differential_matches_ts_oracle",
    boundedSlices: {
      "parser/markdown-wasm": "crates/wasm-md/src/parser_v2.rs#parser_markdown_wasm_differential_matches_ts_oracle",
      "parser/javascript-wasm": "crates/wasm-js/src/lib.rs#parser_javascript_wasm_differential_matches_ts_oracle"
    },
    oracle: "scripts/differential/synth-oracle.ts",
    gate: "scripts/check-no-ts-parser.sh; scripts/check-no-ts-js-parser.sh"
  }' >"$ARTIFACT"

echo "synth-differential: OK (cases=$CASE_COUNT markdown=$MARKDOWN_CASE_COUNT javascript=$JAVASCRIPT_CASE_COUNT corpus=$FIXTURE_CORPUS_HASH)" | tee -a "$LOG"
echo "verification artifact: $ARTIFACT" | tee -a "$LOG"