test:
  hurl --test --glob "./tests/*.hurl"

test-verbose:
  hurl --test --verbose --glob "./tests/*.hurl"

dev:
  bun wrangler dev

deploy:
  bun wrangler deploy
