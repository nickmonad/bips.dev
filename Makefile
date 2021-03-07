.PHONY: bips
bips:
	git submodule init && git submodule update --recursive --remote

.PHONY: serve
serve:
	cd web && zola serve && cd ..

.PHONY: build
build:
	cd web && zola build && cd ..
