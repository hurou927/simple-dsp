watch:
	cargo watch --ignore 'logs/*' -x 'check --'

run:
	cargo watch --ignore 'logs/*' -x 'run --'

test:
	cargo test

req:
	curl -sv -d @./resources/req.json localhost:3000/r/video

encode:
	pbpaste | jq '.imp[0] | .native.request' -rc | tr -d "\n" | jq -Rs
