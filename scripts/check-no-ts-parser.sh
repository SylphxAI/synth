#!/usr/bin/env bash
# ADR-168 S3 gate: @sylphx/synth-md default parse()/parseAsync() must route to Rust WASM.
# Allowed: TS Parser class for explicit opt-in (plugins, batch tokenizer, useTsParser, SYNTH_MD_AUTHORITY=ts).
# Forbidden: unconditional TS fallback on baseline default options.
# Temporary: delete after Markdown parsing is ts_deleted, exact-candidate WASM
# parity and package install smoke are green, and registry readback binds it.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PARSER="${ROOT}/packages/synth-md/src/parser.ts"
WASM_AUTH="${ROOT}/packages/synth-md/src/wasm-authority.ts"
INDEX="${ROOT}/packages/synth-md/src/index.ts"
WASM_PARSE_SYNC="${ROOT}/packages/synth-wasm-md/src/parse-sync.ts"
WASM_INIT_SYNC="${ROOT}/packages/synth-wasm-md/src/init-sync.ts"
RUST_PARSER="${ROOT}/crates/wasm-md/src/parser_v2.rs"
RUST_LIB="${ROOT}/crates/wasm-md/src/lib.rs"
GOLDEN="${ROOT}/test/fixtures/markdown-parity/golden.json"
PARITY_TEST="${ROOT}/packages/synth-wasm-md/test/parity-golden.test.ts"
WASM_TEST="${ROOT}/packages/synth-md/src/wasm-authority.test.ts"
GATE_TEST="${ROOT}/test/check-no-ts-parser.node-test.mjs"
# SSOT: migration-ledger.json (repoLedgerPath). Legacy synth-migration-ledger.json is a projection.
LEDGER="${ROOT}/docs/specs/migration-ledger.json"
LEGACY_LEDGER="${ROOT}/docs/specs/synth-migration-ledger.json"
CI_WORKFLOW="${ROOT}/.github/workflows/ci.yml"
PACKAGE_JSON="${ROOT}/package.json"

violations=0

report_violation() {
  echo "VIOLATION: $*"
  violations=$((violations + 1))
}

echo "=== check-no-ts-parser $(date -u +%Y-%m-%dT%H:%M:%SZ) ==="

if [[ ! -f "${PARSER}" ]]; then
  report_violation "missing packages/synth-md/src/parser.ts"
fi

if [[ ! -f "${WASM_AUTH}" ]]; then
  report_violation "missing packages/synth-md/src/wasm-authority.ts"
fi

if [[ ! -f "${INDEX}" ]]; then
  report_violation "missing packages/synth-md/src/index.ts"
fi

if [[ ! -f "${WASM_PARSE_SYNC}" ]]; then
  report_violation "missing packages/synth-wasm-md/src/parse-sync.ts"
fi

if [[ ! -f "${WASM_INIT_SYNC}" ]]; then
  report_violation "missing packages/synth-wasm-md/src/init-sync.ts"
fi

if [[ ! -f "${RUST_PARSER}" ]]; then
  report_violation "missing crates/wasm-md/src/parser_v2.rs"
fi

if [[ ! -f "${RUST_LIB}" ]]; then
  report_violation "missing crates/wasm-md/src/lib.rs"
fi

if [[ ! -f "${GOLDEN}" ]]; then
  report_violation "missing test/fixtures/markdown-parity/golden.json"
fi

if [[ ! -f "${PARITY_TEST}" ]]; then
  report_violation "missing packages/synth-wasm-md/test/parity-golden.test.ts"
fi

if [[ ! -f "${WASM_TEST}" ]]; then
  report_violation "missing packages/synth-md/src/wasm-authority.test.ts"
fi

if [[ ! -f "${GATE_TEST}" ]]; then
  report_violation "missing test/check-no-ts-parser.node-test.mjs"
fi

if [[ ! -f "${LEDGER}" ]]; then
  report_violation "missing docs/specs/migration-ledger.json (SSOT)"
fi

if [[ ! -f "${LEGACY_LEDGER}" ]]; then
  report_violation "missing docs/specs/synth-migration-ledger.json (projection)"
fi

if [[ ! -f "${CI_WORKFLOW}" ]]; then
  report_violation "missing .github/workflows/ci.yml"
fi

if [[ ! -f "${PACKAGE_JSON}" ]]; then
  report_violation "missing package.json"
fi

if [[ -f "${LEDGER}" ]]; then
  # rej-010 honesty: accept rust_impl|parity_proven|authority_rust|ts_deleted.
  # Do NOT require authority_rust without valid v2 proof (PROOF-GAP-58 demote).
  # Gate still enforces code-level WASM default routing below.
  node - "${LEDGER}" <<'NODE'
const [ledgerPath] = process.argv.slice(2);
const ledger = JSON.parse(require("node:fs").readFileSync(ledgerPath, "utf8"));
const entry = ledger.capabilities.find((cap) => cap.id === "parser/markdown-wasm");
const allowed = new Set(["rust_impl", "parity_proven", "authority_rust", "ts_deleted"]);
if (!entry) {
  console.error("[check-no-ts-parser] missing capability parser/markdown-wasm");
  process.exit(1);
}
if (!allowed.has(entry.state)) {
  console.error(
    `[check-no-ts-parser] parser/markdown-wasm is ${entry.state}; expected rust_impl|parity_proven|authority_rust|ts_deleted`
  );
  process.exit(1);
}
if (entry.state === "rust_impl" && entry.promotionHold && entry.promotionHold.active !== true) {
  console.error(
    "[check-no-ts-parser] rust_impl parser/markdown-wasm must keep promotionHold.active=true under rej-010 freeze"
  );
  process.exit(1);
}
if (
  !entry.parityTest?.includes("scripts/check-no-ts-parser.sh") &&
  !entry.parityTest?.includes("golden.json")
) {
  console.error(
    "[check-no-ts-parser] parser/markdown-wasm parityTest must cite check-no-ts-parser or golden fixtures"
  );
  process.exit(1);
}
NODE
fi

if [[ -f "${PACKAGE_JSON}" ]]; then
  if ! grep -q 'check:no-ts-parser' "${PACKAGE_JSON}"; then
    report_violation "package.json validate must include check:no-ts-parser"
  fi
fi

if [[ -f "${CI_WORKFLOW}" ]]; then
  if ! grep -q 'check-no-ts-parser.sh' "${CI_WORKFLOW}"; then
    report_violation "ci.yml must run scripts/check-no-ts-parser.sh"
  fi
fi

if [[ -f "${RUST_LIB}" ]]; then
  if ! grep -q 'MarkdownParserV2' "${RUST_LIB}"; then
    report_violation "crates/wasm-md/src/lib.rs must expose MarkdownParserV2 authority surface"
  fi

  if ! grep -q 'parse_to_json' "${RUST_LIB}"; then
    report_violation "crates/wasm-md/src/lib.rs must expose parseToJson for sync authority routing"
  fi
fi

if [[ -f "${WASM_PARSE_SYNC}" ]]; then
  if ! grep -q 'parseToJson' "${WASM_PARSE_SYNC}"; then
    report_violation "parse-sync.ts must delegate to Rust parseToJson"
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
  if ! grep -q "@sylphx/synth-wasm-md" "${WASM_AUTH}"; then
    report_violation "wasm-authority.ts must import @sylphx/synth-wasm-md"
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

  parse_block="$(sed -n '/^export function parse(markdown/,/^export async function parseAsync/p' "${PARSER}" | sed '$d')"
  parse_async_block="$(sed -n '/^export async function parseAsync(markdown/,$p' "${PARSER}")"

  if [[ -z "${parse_block}" ]]; then
    report_violation "parser.ts export parse() block not found"
  else
    if ! grep -q 'isWasmAuthorityEligible(options)' <<<"${parse_block}"; then
      report_violation "parser.ts parse() must gate on isWasmAuthorityEligible(options)"
    fi

    if ! grep -q 'parseViaWasmAuthority(markdown)' <<<"${parse_block}"; then
      report_violation "parser.ts parse() must delegate baseline sync path to parseViaWasmAuthority"
    fi

    if grep -q 'new Parser()' <<<"${parse_block}"; then
      gate_line="$(grep -n 'isWasmAuthorityEligible(options)' <<<"${parse_block}" | head -n1 | cut -d: -f1)"
      parser_line="$(grep -n 'new Parser()' <<<"${parse_block}" | head -n1 | cut -d: -f1)"
      if [[ -n "${gate_line}" && -n "${parser_line}" && "${parser_line}" -lt "${gate_line}" ]]; then
        report_violation "parser.ts parse() must not construct Parser before isWasmAuthorityEligible gate"
      fi
    fi
  fi

  if [[ -z "${parse_async_block}" ]]; then
    report_violation "parser.ts export parseAsync() block not found"
  else
    if ! grep -q 'isWasmAuthorityEligible(options)' <<<"${parse_async_block}"; then
      report_violation "parser.ts parseAsync() must gate on isWasmAuthorityEligible(options)"
    fi

    if ! grep -q 'parseViaWasmAuthorityAsync(markdown)' <<<"${parse_async_block}"; then
      report_violation "parser.ts parseAsync() must delegate baseline async path to parseViaWasmAuthorityAsync"
    fi

    if grep -q 'new Parser()' <<<"${parse_async_block}"; then
      gate_line="$(grep -n 'isWasmAuthorityEligible(options)' <<<"${parse_async_block}" | head -n1 | cut -d: -f1)"
      parser_line="$(grep -n 'new Parser()' <<<"${parse_async_block}" | head -n1 | cut -d: -f1)"
      if [[ -n "${gate_line}" && -n "${parser_line}" && "${parser_line}" -lt "${gate_line}" ]]; then
        report_violation "parser.ts parseAsync() must not construct Parser before isWasmAuthorityEligible gate"
      fi
    fi
  fi
fi

if [[ "${violations}" -gt 0 ]]; then
  echo ""
  echo "FAIL: ${violations} default-path TS parser authority violation(s)."
  echo "Authority: crates/wasm-md via packages/synth-md/src/wasm-authority.ts."
  exit 1
fi

echo "PASS: @sylphx/synth-md baseline parse() routes through Rust WASM authority."
