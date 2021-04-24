.PHONY: bips
bips:
	git submodule init && git submodule update --recursive --remote
	bash ./web/build/transcode-md.sh

.PHONY: npm
npm:
	cd web && npm install && cd ..

.PHONY: serve
serve: npm
	cd web && zola serve && cd ..

.PHONY: build
build: npm
	cd web && zola build && cd ..

.PHONY: image
image:
	docker build -t ngmiller/bips-dev:latest .

.PHONY: push
push:
	docker push ngmiller/bips-dev:latest
