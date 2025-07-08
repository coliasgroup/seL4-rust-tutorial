#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

top_level_dir := .
code_dir := $(top_level_dir)/code
book_dir := $(top_level_dir)/book
preprocessor_dir := $(book_dir)/preprocessor

build_dir := build
step_list := $(build_dir)/steps.txt

.PHONY: none
none:

$(build_dir):
	mkdir -p $(build_dir)

.PHONY: clean
clean:
	rm -rf $(build_dir)

.PHONY: ci-in-container
ci-in-container:
	set -eu; \
	cd code; \
	for rev in $$(cat $(abspath $(step_list))); do \
		git checkout $$rev; \
		git log --format=%B -n 1 HEAD | cat; \
		$(MAKE) check-step; \
	done
	$(MAKE) -C $(code_dir) rustdoc
	$(MAKE) -C $(code_dir) prune-rustdoc

.PHONY: step-list
step-list: $(step_list)

.PHONY: $(step_list)
$(step_list): | $(build_dir)
	cd $(preprocessor_dir) && nix-shell ../shell.nix --run 'cargo run --bin show-steps -- $(abspath $(top_level_dir))' > $(abspath $@)

.PHONY: build-book
build-book:
	cd $(book_dir) && nix-shell --run '$(MAKE) build-preprocessor && $(MAKE) build && $(MAKE) check'

.PHONY: ci
ci:
	$(MAKE) step-list
	$(MAKE) -C docker ci-in-container
	$(MAKE) build-book

.PHONY: deep-clean
deep-clean: clean
	$(MAKE) -C $(book_dir) clean
	$(MAKE) -C $(code_dir) clean-all

.PHONY: checkout-last-step
checkout-last-step:
	set -eu; \
	rev=$$(tail -n 1 $(step_list)); \
	cd $(code_dir); \
	git checkout $$rev

.PHONY: check-licenses
check-licenses:
	reuse lint
	cd $(code_dir) && reuse lint
