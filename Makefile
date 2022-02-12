watch:
	cargo watch --ignore 'logs/*' -x 'check --'

watch:
	cargo watch --ignore 'logs/*' -x 'run --'

test:
	cargo test

encode:
	pbpaste | jq '.imp[0] | .native.request' -rc | tr -d "\n" | jq -Rs
