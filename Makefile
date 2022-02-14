watch:
	cargo watch --ignore 'logs/*' -x 'check --'

run:
	cargo watch --ignore 'logs/*' -x 'run --'

test:
	cargo test

req:
	curl -sv -d @./resources/req.json localhost:3000/r/video
	curl -sv -d @./resources/req.json localhost:3000/r/nvideo
	curl -sv -d @./resources/req.json localhost:3000/r/nimage
	curl -sv -X GET localhost:3000/r/nvideo

	cat ./resources/req.json | gzip | curl -sv --data-binary @- -H "Content-Encoding: gzip" localhost:3000/r/nimage

encode:
	pbpaste | jq '.imp[0] | .native.request' -rc | tr -d "\n" | jq -Rs
