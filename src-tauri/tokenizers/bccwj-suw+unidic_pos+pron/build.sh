#!/bin/bash
set -e

DIRNAME="$(dirname $0)"

mkdir -p "${DIRNAME}/../work"

find "${DIRNAME}/../../resources" -name "*.xml" \
    | sort \
    | xargs cargo run --manifest-path "${DIRNAME}/../../convert_bccwj_xml/Cargo.toml" --release -- \
        --attr pos \
        --attr pron \
  > "${DIRNAME}/../work/bccwj-suw_pos+pron.txt"

cargo run --manifest-path "${DIRNAME}/../../convert_unidic_csv/Cargo.toml" --release -- \
    --tag "{4}-{5}-{6}-{7}|{4}-{5}-{6}|{4}-{5}|{4}" \
    --tag "{13}|" \
    "${DIRNAME}/../../resources/lex_3_1.csv" \
  > "${DIRNAME}/../work/unidic_pos+pron.txt"

cargo run --manifest-path "${DIRNAME}/../../vaporetto/train/Cargo.toml" --release -- \
    --tok "${DIRNAME}/../work/bccwj-suw_pos+pron.txt" \
    --dict "${DIRNAME}/../work/unidic_pos+pron.txt" \
    --solver 5 \
    --model "${DIRNAME}/bccwj-suw+unidic_pos+pron.model.zst"

tar cJf "${DIRNAME}/../bccwj-suw+unidic_pos+pron.tar.xz" -C "${DIRNAME}/.." "bccwj-suw+unidic_pos+pron"
