<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Shared Memory

This chapter covers interacting with shared memory from protection domains written in Rust.
Navigate to and run the example:

```
cd workspaces/microkit/shared-memory
make simulate
```

The example system description specifies two protection domains which share two memory regions:

{{#fragment_with_gh_link "xml" @-11 (workspaces/microkit/shared-memory/)shared-memory.system:7:28}}

The Microkit tool will inject memory region virtual addresses into protection domain images according to the `setvar_vaddr` attribute values.
For example, the virtual address of the mapping of `region_a` into the `client` protection domain will be injected into the `microkit-shared-memory-client.elf` image at the location specified by then `region_a_vaddr` symbol.

In the case of Rust, declaring a symbol that the Microkit tool can patch requires a bit more intentionality than in the C case.
The {{#rustdoc_link microkit sel4_microkit/macro.var.html `sel4_microkit::var!`}} macro is provided to declare such symbols.

The `var!` macro's {{#rustdoc_link microkit src/sel4_microkit_base/symbols.rs.html#55-67 implementation}} is just a few lines of code.
We want to express this symbol as a global variable that does not change at runtime, but which cannot be assumed to have the value we assign it at compile time, and which must not be optimized away.
The near-trivial
{{#rustdoc_link microkit sel4_immutable_cell/struct.ImmutableCell.html `sel4_immutable_cell::ImmutableCell`}} type encapsulates this pattern.
The `#[no_mangle]` attribute instructs the compiler to use the name of the variable as the name of the symbol.
This is the default in C, but not Rust.
We direct the compiler to put this symbol in the `.data` section with `#[link_section = ".data"]` to ensure that space is allocated for it in the ELF file itself, not just the program image it describes.

So far, the example protection domains just store pointers to the shared memory regions in their handler state:

{{#fragment_with_gh_link "rust,ignore" @-11 (workspaces/microkit/shared-memory/)pds/client/src/main.rs:17:31}}

{{#fragment_with_gh_link "rust,ignore" @-11 (workspaces/microkit/shared-memory/)pds/server/src/main.rs:17:46}}

{{#step 11.A}}

Let's assign types to these shared memory regions.
We can define our types in a crate that both the client and server can use:

{{#fragment_with_gh_link "rust,ignore" @-11.A (workspaces/microkit/shared-memory/)pds/common/src/lib.rs:9:18}}

Suppose `region_a: [u8; REGION_A_SIZE]` and `region_b: RegionB`.
You could just turn the virtual addresses we get in our `var!` symbols into pointers and start interacting with the shared memory regions with `unsafe` `ptr::*` operations, but we can leverage the Rust type system to come up with a solution that only requires `unsafe` at initialization time.

{{#step 11.B}}

The
{{#rustdoc_link microkit sel4_shared_memory/index.html `sel4-shared-memory`}}
crate provides a way for you to declare a memory region's type and bounds, along with the memory access operations that can safely be used on it, so that you can access it without `unsafe` code.
That initial declaration is, however, `unsafe`.

The
{{#rustdoc_link microkit sel4_shared_memory/index.html `sel4-shared-memory`}}
is a thin wrapper around the
{{#rustdoc_link microkit sel4_shared_memory/index.html `sel4_abstract_ptr`}}
crate, instantiating its abstract pointer types with memory access operations suitable for memory that is shared with another protection domain.

The
{{#rustdoc_link microkit sel4_microkit/macro.memory_region_symbol.html `sel4_microkit::memory_region_symbol!`}}
macro is like the `sel4_microkit::var!` macro, except specialized for shared memory region virtual address symbols.
For one, the underlying symbol is always of type `usize` and the macro returns a value of type `NonNull<_>`.
`memory_region_symbol!` has a few additional features.
For example, `memory_region_symbol!(foo: *mut [u8] n = BAR)` returns a `NonNull<[u8]>` with a runtime slice length of `BAR`.

See this step's diff for how to put this all together.
