#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

SITE_URL ?= /

RUSTDOC_URL ?= file://$(abspath ../code/build/rustdoc)

ifneq ($(RUSTDOC_PATH),)
rustdoc_location_kind := path
rustdoc_location_value := $(RUSTDOC_PATH)
else
rustdoc_location_kind := url
rustdoc_location_value := $(RUSTDOC_URL)
endif

build_dir := build
index_html := $(build_dir)/index.html

top_level_dir := ..
code_dir := $(top_level_dir)/code
code_gh_url := seL4/seL4-rust-tutorial-code
manual_url := https://sel4.systems/Info/Docs/seL4-manual-13.0.0.pdf

run_mdbook := \
	X_PREPROCESSOR_TOP_LEVEL_LOCAL_ROOT=$(abspath $(top_level_dir)) \
	X_PREPROCESSOR_CODE_LAST_STEP_REV=$$(cd $(code_dir) && git rev-parse HEAD) \
	X_PREPROCESSOR_CODE_GITHUB_ROOT=$(code_gh_url) \
	X_PREPROCESSOR_RUSTDOC_LOCATION_KIND=$(rustdoc_location_kind) \
	X_PREPROCESSOR_RUSTDOC_LOCATION_VALUE=$(rustdoc_location_value) \
	X_PREPROCESSOR_MANUAL_URL=$(manual_url) \
	MDBOOK_OUTPUT__HTML__SITE_URL=$(SITE_URL) \
		cargo run -p x-wrapper --

.PHONY: none
none:

.PHONY: clean
clean:
	rm -rf target $(build_dir)

.PHONY: build
build:
	$(run_mdbook) build

.PHONY: serve
serve:
	$(run_mdbook) serve

.PHONY: open
open:
	open $(index_html)

.PHONY: check-html
check-html:
	html5validator \
		--root $(build_dir) \
		--ignore \
			'Element "head" is missing a required instance of child element "title".' \
			'Start tag "a" seen but an element of the same type was already open.' \
			'Stray end tag "a".'

.PHONY: check-links
check-links:
	linkchecker $(index_html) --no-follow-url ".*/rustdoc/.*"

.PHONY: check
check: check-links check-html
