#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

PROD ?= 0

build_dir := build

top_level_dir := ..
code_dir := $(top_level_dir)/code
code_gh_url := coliasgroup/seL4-rust-tutorial-code
manual_url := https://sel4.systems/Info/Docs/seL4-manual-13.0.0.pdf

mdbook_env := \
	THIS_MDBOOK_TOP_LEVEL_LOCAL_ROOT=$(abspath $(top_level_dir)) \
	THIS_MDBOOK_CODE_LAST_STEP_REV=$$(cd $(code_dir) && git rev-parse HEAD) \
	THIS_MDBOOK_CODE_GITHUB_ROOT=$(code_gh_url) \
	THIS_MDBOOK_MANUAL_URL=$(manual_url) \
	PATH=preprocessor/target/debug:$$PATH

.PHONY: none
none:

.PHONY: clean
clean: clean-preprocessor clean-book

preprocessor_manifest_path_args := --manifest-path preprocessor/Cargo.toml

.PHONY: build-preprocessor
build-preprocessor:
	cargo build $(preprocessor_manifest_path_args)

.PHONY: clean-preprocessor
clean-preprocessor:
	cargo clean $(preprocessor_manifest_path_args)

.PHONY: build-book-mdbook
build-book:
	$(mdbook_env) \
		mdbook build

.PHONY: clean-book
clean-book:
	rm -rf $(build_dir)

.PHONY: build
build:
	$(MAKE) build-preprocessor
	$(MAKE) build-book

.PHONY: check
check:
	linkchecker $(build_dir)/index.html --no-follow-url '.*/rustdoc/.*'

.PHONY: open
open:
	open $(build_dir)/index.html

.PHONY: serve
serve:
	$(mdbook_env) \
		mdbook serve
