# The application's Makefile. Run `make` here, or from the folder above, for
# the list of targets.
#
# Every target runs in NoWayOS's dev container (Podman; ENGINE=docker for
# Docker), so this machine needs no Rust, no seL4 tools and no QEMU. The
# container, the checks and the system targets (qemu, qemu-test, image) are
# NoWayOS's own make fragments, from the clone `make setup` fetched.
#
# Written for GNU Make 3.81 (the version macOS ships) and later.

CARGO ?= cargo
# Never let a cargo, rustc or rustup call made from here install anything.
RUSTUP_AUTO_INSTALL ?= 0
export RUSTUP_AUTO_INSTALL
LOCKED ?=
PYTHON ?= python3

# This repository's own system (systems/) and protection domains (pds/),
# built and checked by NoWayOS's fragments exactly as NoWayOS's own are
# (nowayos/tools/make/system.mk, "Product repos").
SYSTEM_REPO := $(CURDIR)
SYSTEM ?= starter
PDS_LIB_CRATES :=
UNSAFE_ALLOWLIST :=

# The crate whose tsx/ folder holds the screens, and the program make dev runs.
SCREENS ?= starter-app
DEV_PROGRAM ?= starter-dev
DEV_PORT ?= 8017

# Screen Compiler, from the clone beside this folder, built in its own target
# directory.
SCREEN_COMPILER_DIR ?= ../screen-compiler
_sc_target = $(if $(NOWAYOS_CONTAINER),/cache/target/screen-compiler,$(abspath $(SCREEN_COMPILER_DIR))/target)
_sc = $(_sc_target)/debug/screen-compiler
_target = $(if $(CARGO_TARGET_DIR),$(CARGO_TARGET_DIR),target)

-include ../nowayos/tools/make/container.mk
-include ../nowayos/tools/make/system.mk

.DEFAULT_GOAL := help
.PHONY: help dev preview screens new-screen rename fmt lint test check \
	dev-local preview-local screens-local new-screen-local rename-local check-local screen-compiler

help:
	@echo "Your application on NoWayOS (every target runs in the dev container)"
	@echo ""
	@echo "  make dev                 the application in a browser window on this machine,"
	@echo "                           http://localhost:$(DEV_PORT)/ - save a screen (tsx/) or the logic (src/)"
	@echo "                           and it is showing again about a second later. SCREEN=NAME opens it"
	@echo "                           on that screen. Ctrl-C stops it"
	@echo "  make preview FILE=$(SCREENS)/tsx/welcome.tsx"
	@echo "                           one screen as a picture in build/preview/ (WATCH=1: again on each save)"
	@echo "  make new-screen NAME=thanks   a new screen from a template, converted and ready to wire in"
	@echo "  make rename NAME=my-product   make the starter yours: its crates, programs and system renamed"
	@echo ""
	@echo "  make screens             check every screen converts as checked in (BLESS=1 rewrites src/generated)"
	@echo "  make test                the application's tests, on the host: keys in, screens out"
	@echo "  make lint                rustfmt and clippy, warnings denied"
	@echo "  make fmt                 format the code"
	@echo ""
	@echo "  make qemu VNC=1          boot it on seL4 in QEMU; the screen at vnc://localhost:5900"
	@echo "                           (quit: Ctrl-A, then x)"
	@echo "  make qemu-test           boot it headless, play systems/$(SYSTEM).qmp, check systems/$(SYSTEM).expect"
	@echo "  make check               lint, test, screens and qemu-test: run it before you share a change"
	@echo "  make shell               a shell in the dev container"

# What each target is given, passed on into the container.
_given = CARGO=cargo $(foreach name,FILE OUT WATCH ARGS NAME SCREEN BLESS DEV_PORT LOCKED,$(if $($(name)),$(name)='$($(name))'))

ifneq ($(NOWAYOS_CONTAINER)$(filter host,$(IN)),)
# In the dev container (or on the host, with IN=host): run directly.
fmt:
	$(CARGO) fmt

lint:
	$(CARGO) fmt -- --check
	$(CARGO) clippy --workspace --all-targets $(LOCKED) -- -D warnings

test:
	$(CARGO) test --workspace $(LOCKED)

dev: dev-local
preview: preview-local
screens: screens-local
new-screen: new-screen-local
rename: rename-local
check: check-local
else ifdef IN_CONTAINER
# On the host: the same target, in the dev container. make dev's window is
# published on this machine's loopback address only.
dev: override CONTAINER_RUN_ARGS += -p 127.0.0.1:$(DEV_PORT):$(DEV_PORT)
fmt lint test dev preview screens new-screen rename check:
	@$(IN_CONTAINER) make --no-print-directory $@ $(_given)
else
fmt lint test dev preview screens new-screen rename check:
	@echo "error: NoWayOS is not here yet: run make setup in the folder above" >&2
	@exit 1
endif

# Screen Compiler itself, built from the clone beside this folder.
screen-compiler:
	@test -f $(SCREEN_COMPILER_DIR)/Cargo.toml || { echo "error: Screen Compiler is not here yet: run make setup in the folder above" >&2; exit 1; }
	@cd $(SCREEN_COMPILER_DIR) && CARGO_TARGET_DIR=$(_sc_target) $(CARGO) build --locked --quiet -p screen-compiler-cli

# Every screen checked and converted, and compared with src/generated - or,
# with BLESS=1, written there (Screen Compiler's tools/screens).
screens-local: screen-compiler
	@$(PYTHON) $(SCREEN_COMPILER_DIR)/tools/screens --screen-compiler $(_sc) --screens $(SCREENS) $(if $(BLESS),--bless)

# One screen as pictures, by Screen Compiler's make preview.
OUT ?= build/preview
preview-local:
	@test -n "$(FILE)" || { echo "usage: make preview FILE=$(SCREENS)/tsx/welcome.tsx [WATCH=1] [ARGS='--show GROUP --slot NAME=TEXT --size 1920x1080']" >&2; exit 1; }
	@mkdir -p build
	@CARGO_TARGET_DIR=$(_sc_target) $(MAKE) --no-print-directory -C $(SCREEN_COMPILER_DIR) preview CARGO=$(CARGO) \
		FILE="$(abspath $(FILE))" OUT="$(if $(filter /%,$(OUT)),$(OUT),../$(notdir $(CURDIR))/$(OUT))" \
		PICTURES="$(wildcard $(abspath $(SCREENS))/pictures.names)" WATCH='$(WATCH)' ARGS='$(ARGS)'

# The application in a browser window, kept running by NoWayOS's
# tools/dev-window: on a save it checks and converts the screens, builds
# again and starts the program again where it was.
dev-local: screen-compiler
	@$(PYTHON) ../nowayos/tools/dev-window \
		--build "$(CARGO) build --quiet --profile dev-window -p $(DEV_PROGRAM)" \
		--watch $(SCREENS) --watch $(DEV_PROGRAM) \
		--check "$(_sc) check {file}" \
		--screens "$(PYTHON) $(SCREEN_COMPILER_DIR)/tools/screens --screen-compiler $(_sc) --bless --screens {crate}" \
		--port $(DEV_PORT) $(if $(NOWAYOS_CONTAINER),--all-addresses) --session build/dev/session \
		-- $(_target)/dev-window/$(DEV_PROGRAM) $(if $(SCREEN),--screen $(SCREEN))

# A new screen from a template, converted (tools/new-screen).
new-screen-local: screen-compiler
	@test -n "$(NAME)" || { echo "usage: make new-screen NAME=thanks" >&2; exit 1; }
	@$(PYTHON) tools/new-screen --screens $(SCREENS) "$(NAME)"
	@$(PYTHON) $(SCREEN_COMPILER_DIR)/tools/screens --screen-compiler $(_sc) --screens $(SCREENS) --bless >/dev/null
	@echo "  see it:       make preview FILE=$(SCREENS)/tsx/$(NAME).tsx"
	@echo "  walk into it: make dev SCREEN=$(NAME)"

# The starter made yours (tools/rename).
rename-local:
	@test -n "$(NAME)" || { echo "usage: make rename NAME=my-product" >&2; exit 1; }
	@$(PYTHON) tools/rename "$(NAME)"

# Everything worth running before a change is shared.
check-local:
	@$(MAKE) --no-print-directory lint LOCKED=--locked
	@$(MAKE) --no-print-directory test LOCKED=--locked
	@$(MAKE) --no-print-directory screens-local
	@$(MAKE) --no-print-directory qemu-test
	@echo "check: PASS"
