#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

ROOT = $(abspath ..)

real_docker_dir := ../code/docker

.PHONY: none
none:

.PHONY: build run exec rm-container
build run exec rm-container:
	$(MAKE) -C $(real_docker_dir) $@ ROOT=$(ROOT)

.PHONY: exported-rustdoc
exported-rustdoc:
	$(MAKE) -C $(real_docker_dir) oneshot CMD='make $@'

.PHONY: check-each-step
check-each-step:
	$(MAKE) -C $(real_docker_dir) oneshot ROOT=$(ROOT) CMD='make $@'
