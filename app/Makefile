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
# What the device runs is built for this target (make nostd).
NOSTD_TARGET ?= aarch64-unknown-none
SBOM_DIR := build/sbom

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
# directory: in the dev container, one named for this clone (see below).
SCREEN_COMPILER_DIR ?= ../screen-compiler
_sc_target = $(if $(NOWAYOS_CONTAINER),$(CARGO_TARGET_DIR)-screen-compiler,$(abspath $(SCREEN_COMPILER_DIR))/target)
_sc = $(_sc_target)/debug/screen-compiler
_target = $(if $(CARGO_TARGET_DIR),$(CARGO_TARGET_DIR),target)

# The build cache in the dev container is named for the folder this clone is
# in, so two applications on one machine never share one.
CONTAINER_TARGET_DIR ?= /cache/target/$(notdir $(abspath $(CURDIR)/..))

-include ../nowayos/tools/make/container.mk
-include ../nowayos/tools/make/system.mk

.DEFAULT_GOAL := help
.PHONY: help dev preview screens new-screen rename fmt lint test doc check \
	nostd deny sbom secrets pins ci ci-local \
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
	@echo "  make ci                  every check NoWayOS holds its own repositories to: the ones above"
	@echo "                           and the security checks below. A few minutes"
	@echo ""
	@echo "  The security checks, each on its own:"
	@echo "  make secrets             no key or credential in the tree or in any commit"
	@echo "  make deny                every dependency's licence, source and advisories; no banned crate"
	@echo "  make pds-deny            the same for the protection domains, and their git pins"
	@echo "  make unsafe-audit        no unsafe code anywhere in the application"
	@echo "  make nostd               what the device runs builds without the standard library"
	@echo "  make std-crates-check    the protection domains hold only the standard-library crates NoWayOS lists"
	@echo "  make pins                NoWayOS and Screen Compiler are the commits the .rev files name"
	@echo "  make sbom                a bill of materials for each workspace, in $(SBOM_DIR)/"
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

doc:
	RUSTDOCFLAGS="-D warnings" $(CARGO) doc --workspace --no-deps $(LOCKED)

# What the device runs, built as it ships (the release profile: a panic
# aborts, overflow checks on) with no standard library.
nostd:
	$(CARGO) build $(LOCKED) --release --target $(NOSTD_TARGET) --workspace --exclude $(DEV_PROGRAM)

# NoWayOS's baseline first - both workspaces keep its lints, profiles and
# dependency rules, and every local crate is this repository's or NoWayOS's -
# then cargo-deny (deny.toml; the protection domains': make pds-deny).
deny:
	$(PYTHON) ../nowayos/tools/check-baseline . pds
	$(CARGO) deny $(LOCKED) check

# A CycloneDX bill of materials for each workspace, by NoWayOS's generator:
# what the device is built from is marked required; tests, build-time tools
# and the dev window are marked excluded, each with its reason.
sbom:
	@mkdir -p $(SBOM_DIR)
	$(PYTHON) ../nowayos/tools/sbom --workspace . --name $(SYSTEM) --target $(NOSTD_TARGET) \
		--host-only $(DEV_PROGRAM) --cargo $(CARGO) --out $(SBOM_DIR)/$(SYSTEM).cdx.json
	$(PYTHON) ../nowayos/tools/sbom --workspace pds --name $(SYSTEM)-pds --kind application --target aarch64-sel4 \
		--std-crates $(PDS_STD_CRATES) --toolchain $(PDS_TOOLCHAIN) \
		--std-library "$$(rustc +$(PDS_TOOLCHAIN) --print sysroot)/lib/rustlib/src/rust/library" \
		--cargo $(CARGO) --out $(SBOM_DIR)/$(SYSTEM)-pds.cdx.json

# No key or credential in the tree, in any version of any file or in any
# commit message, by NoWayOS's scanner, offline. A finding names its file and
# line, never the text; a reviewed false positive goes in secrets-allow.toml.
# It reads the whole history: a shallow clone fails.
secrets:
	$(PYTHON) ../nowayos/tools/secrets-scan --repo .. --allow ../secrets-allow.toml

# NoWayOS and Screen Compiler are the commits nowayos.rev and
# screen-compiler.rev name, with no local changes (tools/check-pins).
pins:
	@sh tools/check-pins ..

ci: ci-local
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
# The secrets scan runs with no network at all.
secrets: override CONTAINER_RUN_ARGS += --network=none
fmt lint test doc dev preview screens new-screen rename check nostd deny sbom secrets pins ci:
	@$(IN_CONTAINER) make --no-print-directory $@ $(_given)
else
fmt lint test doc dev preview screens new-screen rename check nostd deny sbom secrets pins ci:
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
	@# What was built under the old name would still be in the protection
	@# domains' build output, where make std-crates-check would find it.
	@$(MAKE) --no-print-directory system-clean >/dev/null

# Everything worth running before a change is shared.
check-local:
	@$(MAKE) --no-print-directory lint LOCKED=--locked
	@$(MAKE) --no-print-directory test LOCKED=--locked
	@$(MAKE) --no-print-directory screens-local
	@$(MAKE) --no-print-directory qemu-test
	@echo "check: PASS"

# Every check NoWayOS holds its own repositories to, in the order they run
# there: the pins before anything is built, then the application's
# workspace, then the protection domains and the system. Defence Agent
# (github.com/NoWayOS/defence-agent) lists each step and the gate it is.
# `override`: --locked even when LOCKED is given on the command line.
ci-local: override LOCKED := --locked
ci-local: pins secrets lint test doc nostd deny sbom screens-local \
	pds-lint pds-deny unsafe-audit qemu-test-all std-crates-check
	@echo "ci: PASS"
