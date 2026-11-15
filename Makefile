# The starter: an application on NoWayOS, ready to run and to change.
#
#   make setup    once: fetches NoWayOS and Screen Compiler into this folder
#                 and builds the dev container
#   make dev      your application in a browser window, following your edits
#   make          every other target
#
# The application is in app/; this file passes each target on to its
# Makefile. Written for GNU Make 3.81 (the version macOS ships) and later.

.DEFAULT_GOAL := help
TARGETS := dev preview screens new-screen rename fmt lint test doc check ci \
	secrets deny pds-deny pds-lint unsafe-audit nostd std-crates-check pins sbom \
	qemu qemu-test qemu-test-all image pds shell system-clean
.PHONY: help setup $(TARGETS)

help:
	@if [ -f nowayos/tools/make/container.mk ]; then \
		$(MAKE) --no-print-directory -C app help; \
	else \
		echo "Start here:"; \
		echo "  make setup    fetch NoWayOS and Screen Compiler into this folder and build the dev"; \
		echo "                container (Podman and git are all this machine needs)"; \
		echo "then:"; \
		echo "  make dev      your application in a browser window, following your edits"; \
	fi

setup:
	@./setup

$(TARGETS):
	@test -f nowayos/tools/make/container.mk || { \
		echo "error: NoWayOS is not here yet: run make setup first" >&2; exit 1; }
	@$(MAKE) --no-print-directory -C app $@
