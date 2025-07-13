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

.PHONY: book
book:
	cd $(book_dir) && \
		nix-shell \
			--run '$(MAKE) build && $(MAKE) check'

.PHONY: ci-in-container
ci-in-container: check-each-step
	$(MAKE) -C $(code_dir) exported-rustdoc

.PHONY: ci
ci:
	$(MAKE) step-list
	$(MAKE) -C docker ci-in-container
	$(MAKE) book

book_for_docsite_dir := $(build_dir)/book-for-docsite

src_rustdoc_dir := $(code_dir)/build/exported-rustdoc
dst_rustdoc_dir := $(book_for_docsite_dir)/rustdoc

.PHONY: book-for-docsite
book-for-docsite:
	cd $(code_dir) && $(MAKE) exported-rustdoc
	cd $(book_dir) && $(MAKE) build
	rm -rf $(book_for_docsite_dir)
	mkdir -p $(dir $(book_for_docsite_dir))
	cp -r $(book_dir)/build $(book_for_docsite_dir)
	cp -r $(src_rustdoc_dir) $(dst_rustdoc_dir)
