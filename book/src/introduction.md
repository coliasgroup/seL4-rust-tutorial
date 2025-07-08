<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Tutorial: Using Rust in seL4 Userspace

Support for Rust in seL4 userspace has been an official seL4 Foundation project since November 2023:

<https://github.com/seL4/rust-sel4>

The exports of this project covered in this tutorial are:
- Rust bindings for the seL4 API
- A runtime for root tasks
- Rust bindings for the seL4 Microkit API
- A runtime for [seL4 Microkit](https://github.com/seL4/microkit) protection domains
- Custom rustc [target specifications](https://docs.rust-embedded.org/embedonomicon/custom-target.html) for seL4 userspace

[Part I](#the-root-task) covers the Rust bindings for the seL4 API and the runtime for root tasks.
Familiarity with the seL4 API isn't necessarily assumed or required, but this text doesn't introduce its elements in as much detail as the {{#manual_link [seL4 Manual]}}.

[Part II](#sel4-microkit) is much shorter, and covers the Rust language runtime for seL4 Microkit protection domains and implementation of the Microkit API.
This part does assume that the reader is familiar with the basics of the Microkit framework and API, or is using a companion resource to learn about the Microkit in parallel.
