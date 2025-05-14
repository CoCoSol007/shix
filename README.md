# Shix
A shit programming language
> ⚠️ I created this programming language to practice building a Chumsky parser in Rust. It is not intended for deployment.

You can find some examples of Shix code [here](examples).

## Overview

**Shix** is a lightweight stack-based scripting language designed for simple arithmetic, stack operations, and basic flow control using jump commands. This guide will walk you through the core syntax and features of Shix.

---

## Features

### ➤ Arithmetic Operations

Shix supports basic mathematical expressions:

```shix
1 + 2
2 - 1
2.5 * (7 + (-2))
1 / 2 + 1
70 % 4
```

To **evaluate and view** the result, use the `print:` keyword:

```shix
print: 1 + 2       // 3
print: 2 - 1       // 1
print: 2.5 * (7 + (-2)) // 12.5
print: 1 / 2 + 1   // 1.5
print: 70 % 4      // 2
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
print: read    // 2
```

### ➤ `pop`

Removes and returns the top value of the stack.

```shix
print: pop     // 2, Stack: [1]
```

### ➤ `swap`

Swaps the top two values of the stack.

```shix
push: read     // Stack: [1, 1]
push: pop + 1  // Stack: [2, 1]
swap           // Stack: [1, 2]
```

### ➤ `over:<index>`

Duplicates the value at the specified stack index and pushes it to the top.

```shix
over: 0        // Duplicates top value — Stack: [1, 1, 2]
over: 2        // Duplicates third value — Stack: [2, 1, 1, 2]
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

### ➤ `jump:<line>`

Unconditionally jumps to a specified executable line.

```shix
jump:22
print: 0       // This line is skipped
print: 1       // This line is executed
```

### ➤ `jumpZ:<value>, <line>`

Jumps to a specified line **only if the value is zero**.

```shix
push: 1
jumpZ: read, 26    // Does not jump (1 ≠ 0)

push: pop - 1      // Stack: [0]
jumpZ: read, 26    // Jumps (0 == 0)
```

### ➤ `jumpNZ:<value>, <line>`

Jumps if the value is **non-zero**.

```shix
jumpNZ: read, 31   // Jumps if value ≠ 0
```

### ➤ `jumpN:<value>, <line>`

Jumps if the value is **negative**.

```shix
print: read
jumpN: read, 35    // Jumps if value < 0
```

### ➤ `jumpP:<value>, <line>`

Jumps if the value is **positive**.

```shix
print: read
jumpP: read, 41    // Jumps if value > 0
```

---
## Summary

Shix is simple and expressive for stack-based operations and control flow. Remember:

* Use `print:` to evaluate and display results.
* Stack commands are core to manipulating data.
* Use conditional jumps for program logic.
* **Only executable lines** (not comments or blank lines) are counted when using jump commands.
