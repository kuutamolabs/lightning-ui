# Lint rust
lint:
  cargo clippy --all-targets --all-features -- -D warnings
fix:
  treefmt
  cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged  -- -D warnings
serve:
  trunk serve --public-url /lightning-ui

test:
  cargo test --all --all-features
# Continously run cargo check as code changes
watch:
  cargo watch
