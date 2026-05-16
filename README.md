# LinguaScript

A narrative programming language where code reads like English prose.

## Philosophy

LinguaScript aims to transform code into a fluent English narrative while maintaining the rigor of programming logic. It rejects symbolic pollution — minimizing `()`, `{}`, `;`, `=` in favor of natural language connectors.

## Quick Start

```bash
cargo run -- examples/hello.ls
```

Output:
```
Hello LinguaScript!
100
Hello World
```

## Installation

Requires Rust 1.70+.

```bash
git clone https://github.com/yourname/lingua-script.git
cd lingua-script
cargo build --release
```

The binary will be at `target/release/lingua-script`.

## Language Guide

### Basics

```linguascript
say "Hello World".
score is one hundred.
say score.
let name be "Alice".
say "Hello " + name.
```

### Numbers

Integers, decimals, and large numbers are written as English words:

```linguascript
let a be one hundred.
let b be two thousand five hundred.
let c be three point one four.
let d be half.
let e be 35 thousand.
```

### Variables

```linguascript
let x be ten.
score is zero.
score becomes one hundred.
```

### Arithmetic

```linguascript
let sum be ten added to five.
let diff be ten subtracted from five.
let prod be ten multiplied by five.
let quot be ten divided by five.
let rem be remainder of ten divided by three.
let sq be square of five.
```

### Comparison

```linguascript
when ten is greater than five:
    say "yes".
end.

when five is less than ten:
    say "yes".
end.

when ten is equal to ten:
    say "yes".
end.

when five is not equal to ten:
    say "yes".
end.

when ten isnt five:
    say "yes".
end.

when ten is greater than or equal to five:
    say "yes".
end.
```

### Logic

```linguascript
when true and true:
    say "both".
end.

when true or false:
    say "either".
end.

when not false:
    say "negated".
end.
```

### Control Flow

```linguascript
when x is greater than y:
    say "greater".
otherwise:
    say "not greater".
end.

repeat three times:
    say "loop".
end.

let i be zero.
while i is less than three:
    say i.
    i becomes i added to one.
end.

let items be a list containing "a", "b", "c".
for each item in items:
    say item.
end.
```

### Lists

```linguascript
let fruits be a list containing "apple", "banana".
add "cherry" to fruits.
remove "apple" from fruits.
say fruits.
```

### Maps

```linguascript
let config be a map with "volume" as 80, "difficulty" as "Hard".
say config.
```

### Type Query and Conversion

```linguascript
let t be type of 42.
say t.

convert "50" to number and save to n.
convert 42 to string and save to s.
```

### Functions

```linguascript
to greet with name:
    say "Hello, " + name + "!".
end.

to add with a, b:
    return a added to b.
end.

run greet with "Alice" and save to _.
run add with 3, 4 and save to result.
say result.
```

### Classes and Objects

```linguascript
define a Counter:
    it has value which is 0.

    on create with initial:
        value becomes initial.
    end.

    to increment with amount:
        value becomes value added to amount.
    end.

    to get_value:
        return value.
    end.

    make increment public.
    make get_value public.
end.

let c be instantiate Counter with 10.
let v be get_value using c.
say v.

increment using c with 5.
```

### Fresh Instances

```linguascript
let c be fresh Counter with 100.
```

### Operator Overloading

Define custom behavior for operators using `when` inside a class:

```linguascript
define a Vec:
    it has x which is 0.

    on create with n:
        x becomes n.
    end.

    when added to with other:
        return Vec with x added to other.
    end.

    when multiplied by with other:
        return Vec with x multiplied by other.
    end.

    when negated:
        return Vec with 0 minus x.
    end.

    when equals with other:
        return x is equal to other.
    end.

    when greater than with other:
        return x is greater than other.
    end.
end.

let a be instantiate Vec with 3.
let b be instantiate Vec with 5.
let c be a added to b.     "calls when added to with other"

let d be -a.               "calls when negated"
```

Method names support multi-word identifiers (joined with `_`):

| Operator Syntax        | Internal Method Name     |
|------------------------|--------------------------|
| `when added to`        | `added_to`               |
| `when subtracted by`   | `subtracted_by`          |
| `when multiplied by`   | `multiplied_by`          |
| `when divided by`      | `divided_by`             |
| `when remainder of`    | `remainder_of`           |
| `when negated`         | `negated`                |
| `when inverted`        | `inverted`               |
| `when equals`          | `equals`                 |
| `when not equals`      | `not_equals`             |
| `when greater than`    | `greater_than`           |
| `when less than`       | `less_than`              |
| `when greater than or equal to` | `greater_than_or_equal_to` |
| `when less than or equal to`    | `less_than_or_equal_to`  |

### Exception Handling

```linguascript
beware:
    say "trying...".
    raise "something went wrong".
in case of error:
    say "caught".
regardless:
    say "finally".
end.

attempt to:
    say "attempting".
    raise "fail".
if it fails:
    say "handled".
regardless:
    say "cleanup".
end.
```

### Modules

Flat import all exports from a module:

```linguascript
refer to math.
say sin(0).
say pi.
```

Aliased import with a namespace:

```linguascript
refer to math as m.
say sin using m with 0.
```

Selective import with namespace:

```linguascript
refer to sin, cos from math as trig.
say sin using trig with 0.
```

Module path resolution uses the `of` chain:

```linguascript
refer to random of math.
```

Aliasing with `as`:

```linguascript
refer to math as ma.
refer to sin, cos of math as trig.
```

Using module functions by namespace:

```linguascript
refer to math as m.
say sin using m with 0.
say cos using m with 0.
say pi using m.
```

Standard library modules: `math` (sin, cos, sqrt, abs, floor, ceil, pow, pi, e) and `random` (random, randint, uniform, seed).

Modules are compiled to `.lsbc` bytecode on first import and loaded directly on subsequent runs for faster startup.

### Entry Point

```linguascript
say "this does not run".

start here:
    say "this runs first".
    let x be one hundred.
    say x.
end.

say "this does not run either".
```

### Program Termination

```linguascript
say "done".
stop.
```

## Bytecode

LinguaScript compiles source to a compact bytecode format before execution.

### Exporting Bytecode

Compile a `.ls` file to `.lsbc` bytecode for faster loading:

```bash
lingua-script source.ls -o output.lsbc
```

### Running Bytecode Directly

Execute `.lsbc` files directly without recompilation:

```bash
lingua-script output.lsbc
```

This bypasses the lexer, parser, and compiler stages.

### Import Caching

When a module is imported via `refer to`, the compiler automatically generates a `.lsbc` cache file alongside the source. On subsequent imports, the cached bytecode is loaded directly — no recompilation needed.

### Inspection

Use the `-c` / `--code` flag to inspect the generated bytecode, including the constant pool:

```bash
lingua-script examples/hello.ls -c
```

Output:
```
--- bytecode (7 instrs) ---
constants (3):
  0: String("hello")
  1: Number(1.0)
  2: String("x")

   0: Const(0)
   1: Say
   2: Const(1)
   3: StoreVar(2)
   4: LoadVar(2)
   5: Say
   6: Halt
```

### Bytecode Format

The `.lsbc` binary format is modeled after CPython and JVM designs:

| Section | Description |
|---------|-------------|
| Magic (4B) | `LSBC` identifier |
| Version (4B) | Format version |
| Constant Pool | Deduplicated strings, numbers, booleans, nulls |
| Func Table | Function metadata and code |
| Class Table | Class metadata, methods, and code |
| Main Code | Encoded instruction stream |

Instructions use a compact variable-length encoding (1 byte opcode + operands). Constants and variable names are stored as `u32` indices into the constant pool. A `Const("hello")` is encoded as just 5 bytes instead of the previous ~40+.

### Opcodes

54 opcodes covering: constants, variables, arithmetic, comparison, control flow, functions, classes, collections, I/O, exception handling, type conversion, and stack manipulation.

## Architecture

```
source.ls -> Lexer -> Tokens -> Parser -> AST -> Compiler -> Bytecode -> VM -> Output

              .lsbc file --------> Bytecode --------> VM -> Output (fast path)
```

| Module            | Role                                  |
|-------------------|---------------------------------------|
| `lexer.rs`        | Tokenizes source into keywords, identifiers, literals |
| `parser.rs`       | Recursive-descent parser producing AST |
| `ast.rs`          | Abstract syntax tree node definitions |
| `compiler.rs`     | Compiles AST into bytecode with constant pool |
| `instruction.rs`  | Compact bytecode instruction set with binary encoding |
| `bytecode.rs`     | `.lsbc` file format, constant pool, serialization |
| `vm.rs`           | Stack-based virtual machine with call frames and exception handling |
| `value.rs`        | Runtime value system (Number, String, Bool, List, Map, Func, Class, Instance) |
| `gc.rs`           | Reference-counted garbage collector (`Gc<T>` wrapper) |
| `number.rs`       | English word-to-number parser |

## Memory Management

LinguaScript uses a reference-counted GC built on `Rc<RefCell<T>>`. The `Gc<T>` wrapper in `gc.rs` makes the managed nature of Lists, Maps, and Instances explicit at the type level. Circular references between containers are not automatically collected — the user should avoid creating cycles.

Instances with an `on destroy` method have their destructor queued when the reference count drops to zero, and the VM executes it at a safe point.

## Running Tests

```bash
cargo test
```

74 integration tests cover number parsing, variables, arithmetic, comparisons, logic, control flow, lists, maps, types, functions, classes, inheritance, interfaces, exceptions, methods, module imports, standard library, operator overloading, and GC shared references.

```bash
cargo run -- examples/comprehensive_test.ls
```

## License

MIT
