# Code Style Conventions

This document outlines important code style rules we want to enforce in this
project. At some point we might enforce these rules with tests.

This guide will grow over time. To propose new rules, simply make a pull
request against this document.

## Automatic Formatting

Do what `cargo fmt` does. This is the most important rule and supersedes all
other rules. Other rules exist only to catch things that `cargo fmt` doesn't
catch. If a `cargo fmt` rule conflicts with one of the other rules below, then
`cargo fmt` wins.

## Inline Assembly

Inline assembly is necessary in this project. However, inline assembly
statements are also by nature difficult to comprehend. Enforcing a strict,
consistent style on these statements will hopefully make them easier to
understand in spite of their difficulty.

1. Every `asm!()` statement **MUST** have a Rust comment before the statement
   (or its surrounding `unsafe` block) explaining what it does.

2. Inline assembly **MUST** format each assembly instruction as a separate
   string. If more than one instruction is used, each instruction string
   **MUST** appear on a new line.

3. Each assembly instruction string **MAY** have a Rust comment at the end of
   the line explaing what it does. These comments **SHOULD** be vertically
   aligned if practical.

4. Assembly comments **MUST NOT** be used.

5. Instruction operands **SHOULD** be vertically aligned if practical.

6. Long `asm!()` statements containing many instructions **SHOULD** be avoided.
   Find a way to break them up into smaller chunks if at all possible.

7. In longer `asm!()` statements, instruction strings **SHOULD** be logically
   grouped with an empty line between them. Each group **MAY** have a Rust
   comment explaining what the group does.

8. Where practical, `asm!()` statements **SHOULD** be encapsulated by their own
   function.

9. An empty line **SHOULD** be placed between instruction strings and
   arguments in a multi-line `asm!()` statement.

### Example

```rust
// This is a single line asm.
asm!("ud2", options(noreturn));

// This is multi-line asm.
// Instruction operands and comments are aligned vertically.
asm!(
    // Move inputs into the correct registers
    "mov     r10,    {0}",
    "mov     r11,    {1}",

    // Perform summation
    "add     r10,    r11",        // Add the inputs
    "add     r10,    {CONSTANT}", // Add the constant

    CONSTANT = const 42usize,
    in(reg) 7,
    in(reg) 12,
    out("r10") sum,
    out("r11") _,
);
```