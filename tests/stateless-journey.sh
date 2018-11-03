#!/usr/bin/env bash
set -eu

exe=${1:?First argument must be the executable to test}

root="$(cd "${0%/*}" && pwd)"
# shellcheck disable=1090
source "$root/utilities.sh"
snapshot="$root/snapshots"
fixture="$root/fixtures"

SUCCESSFULLY=0

(with "input from stdin"
  (when "the input is well formed"
    (with "small padding"
      it "produces the expected output" && {
        WITH_SNAPSHOT="$snapshot/success-input-file-produces-correct-output" \
        expect_run ${SUCCESSFULLY} "$exe" < "$fixture/valid.input"
      }
    )
    (with "medium padding"
      it "produces the expected output (we know it's correct, despite nonesense)" && {
        WITH_SNAPSHOT="$snapshot/success-input-file-produces-correct-output-large" \
        expect_run ${SUCCESSFULLY} "$exe" < "$fixture/valid-medium.input"
      }
    )
    (with "huge padding"
      it "produces the expected output" && {
        WITH_SNAPSHOT="$snapshot/success-input-file-produces-correct-output-huge" \
        expect_run ${SUCCESSFULLY} "$exe" < "$fixture/valid-big.input"
      }
    )
  )
)
