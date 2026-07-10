#!/usr/bin/env bash
# ADR-168 S4 gate: @sylphx/synth-js default parse()/parseAsync() must route to Rust WASM.
# Allowed: TS JSParser for explicit opt-in (plugins, useTsParser, SYNTH_JS_AUTHORITY=ts).
# Forbidden: unconditional TS fallback on baseline default options.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PARSER="${ROOT}/packages/synth-js/src/parser.ts"
WASM_AUTH="${ROOT}/packages/synth-js/src/wasm-authority.ts"
INDEX="${ROOT}/packages/synth-js/src/index.ts"
WASM_PARSE_SYNC="${ROOT}/packages/synth-wasm-js/src/parse-sync.ts"
WASM_INIT_SYNC="${ROOT}/packages/synth-wasm-js/src/init-sync.ts"
WASM_TREE_BRIDGE="${ROOT}/packages/synth-wasm-js/src/tree-bridge.ts"
RUST_PARSER="${ROOT}/crates/wasm-js/src/parser.rs"
RUST_LIB="${ROOT}/crates/wasm-js/src/lib.rs"
GOLDEN="${ROOT}/test/fixtures/javascript-parity/golden.json"
PARITY_TEST="${ROOT}/packages/synth-wasm-js/test/parity-golden.test.ts"
WASM_TEST="${ROOT}/packages/synth-js/src/wasm-authority.test.ts"
GATE_TEST="${ROOT}/test/check-no-ts-js-parser.node-test.mjs"
GOLDEN_PARITY_TEST="${ROOT}/test/javascript-parity-golden.node-test.mjs"
LEDGER="${ROOT}/docs/specs/synth-migration-ledger.json"
CI_WORKFLOW="${ROOT}/.github/workflows/ci.yml"
PACKAGE_JSON="${ROOT}/package.json"

violations=0

report_violation() {
  echo "VIOLATION: $*"
  violations=$((violations + 1))
}

echo "=== check-no-ts-js-parser $(date -u +%Y-%m-%dT%H:%M:%SZ) ==="

if [[ ! -f "${PARSER}" ]]; then
  report_violation "missing packages/synth-js/src/parser.ts"
fi

if [[ ! -f "${WASM_AUTH}" ]]; then
  report_violation "missing packages/synth-js/src/wasm-authority.ts"
fi

if [[ ! -f "${INDEX}" ]]; then
  report_violation "missing packages/synth-js/src/index.ts"
fi

if [[ ! -f "${WASM_PARSE_SYNC}" ]]; then
  report_violation "missing packages/synth-wasm-js/src/parse-sync.ts"
fi

if [[ ! -f "${WASM_INIT_SYNC}" ]]; then
  report_violation "missing packages/synth-wasm-js/src/init-sync.ts"
fi

if [[ ! -f "${WASM_TREE_BRIDGE}" ]]; then
  report_violation "missing packages/synth-wasm-js/src/tree-bridge.ts"
fi

if [[ ! -f "${RUST_PARSER}" ]]; then
  report_violation "missing crates/wasm-js/src/parser.rs"
fi

if [[ ! -f "${RUST_LIB}" ]]; then
  report_violation "missing crates/wasm-js/src/lib.rs"
fi

if [[ ! -f "${GOLDEN}" ]]; then
  report_violation "missing test/fixtures/javascript-parity/golden.json"
fi

if [[ ! -f "${PARITY_TEST}" ]]; then
  report_violation "missing packages/synth-wasm-js/test/parity-golden.test.ts"
fi

if [[ ! -f "${WASM_TEST}" ]]; then
  report_violation "missing packages/synth-js/src/wasm-authority.test.ts"
fi

if [[ ! -f "${GATE_TEST}" ]]; then
  report_violation "missing test/check-no-ts-js-parser.node-test.mjs"
fi

if [[ ! -f "${GOLDEN_PARITY_TEST}" ]]; then
  report_violation "missing test/javascript-parity-golden.node-test.mjs"
fi

if [[ ! -f "${LEDGER}" ]]; then
  report_violation "missing docs/specs/synth-migration-ledger.json"
fi

if [[ ! -f "${CI_WORKFLOW}" ]]; then
  report_violation "missing .github/workflows/ci.yml"
fi

if [[ ! -f "${PACKAGE_JSON}" ]]; then
  report_violation "missing package.json"
fi

if [[ -f "${LEDGER}" ]]; then
  node - "${LEDGER}" <<'NODE'
const [ledgerPath] = process.argv.slice(2);
const ledger = JSON.parse(require("node:fs").readFileSync(ledgerPath, "utf8"));
const jsWasm = ledger.capabilities.find((cap) => cap.id === "parser/javascript-wasm");
const jsTs = ledger.capabilities.find((cap) => cap.id === "parser/javascript-ts");
if (!jsWasm) {
  console.error("[check-no-ts-js-parser] missing capability parser/javascript-wasm");
  process.exit(1);
}
if (!jsTs) {
  console.error("[check-no-ts-js-parser] missing capability parser/javascript-ts");
  process.exit(1);
}
if (jsTs.state !== "parity_proven") {
  console.error(
    `[check-no-ts-js-parser] parser/javascript-ts is ${jsTs.state}; golden parity must be parity_proven before authority_rust`
  );
  process.exit(1);
}
if (jsWasm.state !== "authority_rust") {
  console.error(
    `[check-no-ts-js-parser] parser/javascript-wasm is ${jsWasm.state}; expected authority_rust`
  );
  process.exit(1);
}
if (!jsWasm.notes?.includes("S4")) {
  console.error("[check-no-ts-js-parser] parser/javascript-wasm notes must document S4 authority routing");
  process.exit(1);
}
if (!jsWasm.parityTest?.includes("test/fixtures/javascript-parity/golden.json")) {
  console.error("[check-no-ts-js-parser] parser/javascript-wasm parityTest must include golden.json");
  process.exit(1);
}
const s3 = ledger.slices?.S3;
if (!s3 || s3.status !== "shipped") {
  console.error("[check-no-ts-js-parser] slices.S3 must be shipped before javascript-wasm authority_rust");
  process.exit(1);
}
const s4 = ledger.slices?.S4;
if (!s4 || s4.status !== "shipped") {
  console.error("[check-no-ts-js-parser] slices.S4 must be shipped for javascript-wasm authority_rust");
  process.exit(1);
}
NODE
fi

if [[ -f "${GOLDEN}" ]]; then
  node - "${GOLDEN}" <<'NODE'
const [goldenPath] = process.argv.slice(2);
const golden = JSON.parse(require("node:fs").readFileSync(goldenPath, "utf8"));
const count = Object.keys(golden).length;
if (count < 10) {
  console.error(`[check-no-ts-js-parser] golden.json must contain >= 10 fixtures (found ${count})`);
  process.exit(1);
}
NODE
fi

if [[ -f "${PACKAGE_JSON}" ]]; then
  if ! grep -q 'check:no-ts-js-parser' "${PACKAGE_JSON}"; then
    report_violation "package.json validate must include check:no-ts-js-parser"
  fi
fi

if [[ -f "${CI_WORKFLOW}" ]]; then
  if ! grep -q 'check-no-ts-js-parser.sh' "${CI_WORKFLOW}"; then
    report_violation "ci.yml must run scripts/check-no-ts-js-parser.sh"
  fi
fi

if [[ -f "${RUST_LIB}" ]]; then
  if ! grep -q 'parseBinary' "${RUST_LIB}"; then
    report_violation "crates/wasm-js/src/lib.rs must expose parseBinary authority surface"
  fi

  if ! grep -q 'golden_fixtures_match_ts_baseline' "${RUST_LIB}"; then
    report_violation "crates/wasm-js/src/lib.rs must include golden_fixtures_match_ts_baseline parity test"
  fi

  if ! grep -q 'javascript-parity/golden.json' "${RUST_LIB}"; then
    report_violation "crates/wasm-js/src/lib.rs must load javascript-parity/golden.json for parity gate"
  fi
fi

if [[ -f "${WASM_PARSE_SYNC}" ]]; then
  if ! grep -q 'parseBinary' "${WASM_PARSE_SYNC}"; then
    report_violation "parse-sync.ts must delegate to Rust parseBinary"
  fi

  if ! grep -q 'binaryToTree' "${WASM_PARSE_SYNC}"; then
    report_violation "parse-sync.ts must bridge WASM binary output via binaryToTree"
  fi

  if ! grep -q 'initWasmSync' "${WASM_PARSE_SYNC}"; then
    report_violation "parse-sync.ts must initialize WASM via initWasmSync"
  fi
fi

if [[ -f "${WASM_INIT_SYNC}" ]]; then
  if ! grep -q 'initSync' "${WASM_INIT_SYNC}"; then
    report_violation "init-sync.ts must call wasm initSync for authority routing"
  fi
fi

if [[ -f "${WASM_AUTH}" ]]; then
  if ! grep -q "@sylphx/synth-wasm-js" "${WASM_AUTH}"; then
    report_violation "wasm-authority.ts must import @sylphx/synth-wasm-js"
  fi

  if ! grep -q 'parseSync as parseWasmSync' "${WASM_AUTH}"; then
    report_violation "wasm-authority.ts must alias parseSync for sync authority routing"
  fi

  if ! grep -q 'parse as parseWasmAsync' "${WASM_AUTH}"; then
    report_violation "wasm-authority.ts must alias parse for async authority routing"
  fi

  if ! grep -q 'initWasm' "${WASM_AUTH}"; then
    report_violation "wasm-authority.ts must import initWasm for async authority routing"
  fi

  if ! grep -q "return override !== 'ts'" "${WASM_AUTH}"; then
    report_violation "wasm-authority.ts must default to WASM (override !== 'ts')"
  fi

  if ! grep -q "useTsParser === true" "${WASM_AUTH}"; then
    report_violation "wasm-authority.ts must require explicit useTsParser opt-in for TS path"
  fi

  if ! grep -q 'parseWasmSync' "${WASM_AUTH}"; then
    report_violation "wasm-authority.ts parseViaWasmAuthority must call parseWasmSync"
  fi

  if ! grep -q 'await initWasm()' "${WASM_AUTH}"; then
    report_violation "wasm-authority.ts parseViaWasmAuthorityAsync must await initWasm()"
  fi

  if ! grep -q 'parseWasmAsync' "${WASM_AUTH}"; then
    report_violation "wasm-authority.ts parseViaWasmAuthorityAsync must call parseWasmAsync"
  fi
fi

if [[ -f "${INDEX}" ]]; then
  if ! grep -q 'isWasmAuthorityEligible' "${INDEX}"; then
    report_violation "index.ts must export isWasmAuthorityEligible"
  fi

  if ! grep -q 'parseViaWasmAuthority' "${INDEX}"; then
    report_violation "index.ts must export parseViaWasmAuthority helpers"
  fi
fi

if [[ -f "${PARSER}" ]]; then
  gate_count="$(grep -c "if (isWasmAuthorityEligible(options))" "${PARSER}" || true)"
  if [[ "${gate_count}" -lt 2 ]]; then
    report_violation "parser.ts must gate both parse() and parseAsync() on isWasmAuthorityEligible (found ${gate_count})"
  fi

  parse_block="$(sed -n '/^export function parse(code/,/^export async function parseAsync/p' "${PARSER}" | sed '$d')"
  parse_async_block="$(sed -n '/^export async function parseAsync(code/,$p' "${PARSER}")"

  if [[ -z "${parse_block}" ]]; then
    report_violation "parser.ts export parse() block not found"
  else
    if ! grep -q 'isWasmAuthorityEligible(options)' <<<"${parse_block}"; then
      report_violation "parser.ts parse() must gate on isWasmAuthorityEligible(options)"
    fi

    if ! grep -q 'parseViaWasmAuthority(code)' <<<"${parse_block}"; then
      report_violation "parser.ts parse() must delegate baseline sync path to parseViaWasmAuthority"
    fi

    if grep -q 'new JSParser()' <<<"${parse_block}"; then
      gate_line="$(grep -n 'isWasmAuthorityEligible(options)' <<<"${parse_block}" | head -n1 | cut -d: -f1)"
      parser_line="$(grep -n 'new JSParser()' <<<"${parse_block}" | head -n1 | cut -d: -f1)"
      if [[ -n "${gate_line}" && -n "${parser_line}" && "${parser_line}" -lt "${gate_line}" ]]; then
        report_violation "parser.ts parse() must not construct JSParser before isWasmAuthorityEligible gate"
      fi
    fi
  fi

  if [[ -z "${parse_async_block}" ]]; then
    report_violation "parser.ts export parseAsync() block not found"
  else
    if ! grep -q 'isWasmAuthorityEligible(options)' <<<"${parse_async_block}"; then
      report_violation "parser.ts parseAsync() must gate on isWasmAuthorityEligible(options)"
    fi

    if ! grep -q 'parseViaWasmAuthorityAsync(code)' <<<"${parse_async_block}"; then
      report_violation "parser.ts parseAsync() must delegate baseline async path to parseViaWasmAuthorityAsync"
    fi

    if grep -q 'new JSParser()' <<<"${parse_async_block}"; then
      gate_line="$(grep -n 'isWasmAuthorityEligible(options)' <<<"${parse_async_block}" | head -n1 | cut -d: -f1)"
      parser_line="$(grep -n 'new JSParser()' <<<"${parse_async_block}" | head -n1 | cut -d: -f1)"
      if [[ -n "${gate_line}" && -n "${parser_line}" && "${parser_line}" -lt "${gate_line}" ]]; then
        report_violation "parser.ts parseAsync() must not construct JSParser before isWasmAuthorityEligible gate"
      fi
    fi
  fi
fi

if [[ "${violations}" -gt 0 ]]; then
  echo ""
  echo "FAIL: ${violations} default-path TS JS parser authority violation(s)."
  echo "Authority: crates/wasm-js via packages/synth-js/src/wasm-authority.ts."
  exit 1
fi

echo "PASS: @sylphx/synth-js baseline parse() routes through Rust WASM authority."