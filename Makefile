.PHONY: submodule
submodule:
	git submodule init && git submodule update --recursive --remote
