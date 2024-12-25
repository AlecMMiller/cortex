#!/usr/bin/env bash
bin=$(cargo test --no-run --message-format=json | jq -r '.executable')
rust-lldb $bin <<- EOF
breakpoint set --name database::entity_test::reference
EOF
