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

.PHONY: deep-clean
deep-clean: clean
	$(MAKE) -C $(book_dir) clean
	$(MAKE) -C $(code_dir) clean

.PHONY: check-licenses
check-licenses:
	reuse lint
	cd $(code_dir) && reuse lint

.PHONY: checkout-last-step
checkout-last-step:
	set -eu; \
	rev=$$(tail -n 1 $(step_list)); \
	cd $(code_dir); \
	git checkout $$rev

.PHONY: step-list
step-list: $(step_list)

.PHONY: $(step_list)
$(step_list): | $(build_dir)
	cd $(preprocessor_dir) && \
		nix-shell ../shell.nix \
			--run 'cargo run --bin show-steps -- $(abspath $(top_level_dir))' \
				> $(abspath $@)

.PHONY: check-each-step
check-each-step:
	set -eu; \
	cd code; \
	for rev in $$(cat $(abspath $(step_list))); do \
		git checkout $$rev; \
		git log --format=%B -n 1 HEAD | cat; \
		$(MAKE) check-step; \
	done

assembled_dir := $(build_dir)/assembled

.PHONY: assemble
assemble:
	$(MAKE) -C docker exported-rustdoc
	cd $(book_dir) && \
		RUSTDOC_PATH=rustdoc \
			nix-shell --run '$(MAKE) build'
	rm -rf $(assembled_dir)
	mkdir -p $(dir $(assembled_dir))
	cp -r $(book_dir)/build $(assembled_dir)
	cp -r $(code_dir)/build/exported-rustdoc $(assembled_dir)/rustdoc
	nix-shell $(book_dir)/shell.nix \
		--run 'linkchecker $(assembled_dir)/index.html --no-follow-url ".*/rustdoc/.*"'

.PHONY: ci
ci:
	$(MAKE) step-list
	$(MAKE) -C docker check-each-step
	$(MAKE) assemble
