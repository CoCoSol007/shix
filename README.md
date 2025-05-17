# Shix
A shit programming language
> ⚠️ I created this programming language to practice building a Chumsky parser in Rust. It is not intended for deployment.

You can find some examples of Shix code [here](examples).

## Getting Started

```bash
git clone https://github.com/cocosol007/shix.git
cd shix

cargo run --release link/to/your/file.shix
```
## ToDo

- [x] AST 
- [x] Eval statments
- [x] Parser
- [x] basic errors messages 
- [ ] better errors messages with ariadne  

## Overview

**Shix** is a lightweight stack-based scripting language designed for simple arithmetic, stack operations, and basic flow control using jump commands. This guide will walk you through the core syntax and features of Shix.

---

## Features

### ➤ Arithmetic Operations

Shix supports basic mathematical expressions:

```shix
1 + 2
2 - 1
2 * (7 + (-2))
```

To **evaluate and view** the result, use the `print:` keyword:

```shix
print: 1 + 2       // 3
print: 2 - 1       // 1
print: 2 * (7 + (-2)) // 10
```

---

## Stack Operations

Shix uses a stack to manage values. The stack follows a Last-In, First-Out (LIFO) structure.

### ➤ `push:`

Adds a value to the top of the stack.

```shix
push: 1        // Stack: [1]
push: 2        // Stack: [2, 1]
```

### ➤ `read`

Reads (peeks) the top value of the stack without removing it.

```shix
push: 1        // Stack: [1]
push: 2       // Stack: [2, 1]
print: read    // 2
```

### ➤ `pop`

Removes and returns the top value of the stack.

```shix
push: 1        // Stack: [1]
push: 2       // Stack: [2, 1]
print: pop     // 2, Stack: [1]
print: pop     // 1, Stack: []
```

### ➤ `swap`

Swaps the top two values of the stack.

```shix
push: 1        // Stack: [1]
push: 2       // Stack: [2, 1]
push: pop + 1  // Stack: [3, 1]
swap           // Stack: [1, 3]
```

### ➤ `over:<index>`

Duplicates the value at the specified stack index and pushes it to the top.

```shix
push: 2       // Stack: [2]
push: 1        // Stack: [1, 2]
over: 0        // Duplicates top value — Stack: [1, 1, 2]
over: 2        // Duplicates index 2 (third) value — Stack: [2, 1, 1, 2]
```

### ➤ `clear`

Clears all values from the stack.

```shix
clear          // Stack: []
```

---

## Flow Control

Shix includes several **conditional and unconditional jump** commands for controlling execution flow.

> ⚠️ **Line Counting Note**: Only **executable lines** (excluding comments and blank lines) are counted when referencing line numbers for jumps.

### ➤ `jumpZ:<value>, <line>`

Jumps to a specified line **only if the value is zero**.

```shix
push: -1
push: pop + 1
jumpZ: read, 1    // jumps
```

### ➤ `jumpNZ:<value>, <line>`

Jumps if the value is **non-zero**.

```shix
push: -1
push: pop + 1
jumpNZ: read, 1    // no jump
```

### ➤ `jumpN:<value>, <line>`

Jumps if the value is **negative**.

```shix
push: -10
push: pop + 1
jumpN: read, 1    // jumps few times
```

### ➤ `jumpP:<value>, <line>`

Jumps if the value is **positive**.

```shix
push: 10
push: pop - 1
jumpP: read, 1    // jumps few times
```

---
## Summary

Shix is simple and expressive for stack-based operations and control flow. Remember:

* Use `print:` to evaluate and display results.
* Stack commands are core to manipulating data.
* Use conditional jumps for program logic.
* **Only executable lines** (not comments or blank lines) are counted when using jump commands.
