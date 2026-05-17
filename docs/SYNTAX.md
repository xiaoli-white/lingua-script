# LinguaScript Syntax Specification

## Overview

LinguaScript is a narrative-style programming language designed to read like natural English prose while maintaining programming rigor. It minimizes symbolic punctuation (`()`, `{}`, `;`, `=`) in favor of natural language connectives.

**Key design principles:**
- Every statement ends with a period `.`
- Colons `:` open code blocks, `end.` closes them
- Keywords are English words; symbolic operators are kept minimal

---

## Lexical Structure

### Comments

| Style | Syntax | Example |
|-------|--------|---------|
| Line comment | `// text` | `// this is ignored` |
| Narrative comment | `note that text.` | `note that this is also ignored.` |

### Identifiers

| Style | Rules | Example |
|-------|-------|---------|
| Standard | Letters, digits, underscores | `my_var`, `score`, `_temp` |
| Multi-word | Space-separated words (none can be keywords) | `take damage`, `player name` |
| Backtick-quoted | Anything between backticks | `` `weird-name!` `` |

### Literals

**Numbers** — Arabic numerals, English words, or mixed:

| Form | Examples |
|------|----------|
| Arabic | `42`, `3.14`, `-7` |
| English words | `zero`, `one`, `ten`, `hundred`, `thousand`, `million`, `billion`, `trillion` |
| Decimal in words | `three point one four`, `zero point five` |
| Special | `half` (= 0.5) |
| Mixed | `35 thousand`, `2 hundred` |

**Strings:**

| Form | Syntax | Example |
|------|--------|---------|
| Single-line | `"..."` | `"hello world"` |
| Multi-line | `"""..."""` | `"""line one\nline two"""` |
| Escapes | `\"`, `\\`, `\n`, `\t` | `"say \"hi\""` |

**Other literals:**

| Type | Values |
|------|--------|
| Boolean | `true`, `false` |
| Null | `null`, `empty` |

### Symbolic Operators

| Symbol | Meaning |
|--------|---------|
| `+` | Addition / string concatenation |
| `-` | Subtraction / negation |
| `*` | Multiplication |
| `/` | Division |
| `//` | Start of line comment |
| `:` | Opens a code block |
| `.` | Ends a statement |
| `,` | Separates arguments or list elements |
| `(` `)` | Groups expressions or holds function call arguments |

### Keywords

```
a, accessed, added, an, and, ask, assigned, attempt, at, be, becomes, beware, by, can, capitalize,
case, chapter, containing, convert, create, define, destroy, divided, each,
empty, end, equal, execute, exit, extends, fails, false, for, fresh, from,
greater, has, here, if, implements, in, incase, instantiate, interface,
is, isnt, it, leave, less, let, list, make, map, minus, multiplied, note, not, null,
of, on, or, otherwise, plus, product, public, raise, read, regardless, refer,
remainder, repeat, return, root, run, save, say, skip, square, start, stop,
subtracted, sum, super, than, that, the, times, to, true, type, using, when,
which, while, with, write
```

---

## Statements

Every statement ends with a period `.`. Blocks are opened with `:` and closed with `end.`.

### Variables

**Define a variable (narrative style):**
```
name is value.
```
Example: `score is zero.`

**Define a variable (declarative style):**
```
let name be value.
```
Example: `let health be one hundred.`

**Reassign a variable:**
```
name becomes value.
```
Example: `score becomes one hundred.`

---

### Output and Input

**Print to console:**
```
say expression.
```
Example: `say "Hello".` / `say score + 10.`

**Prompt user and save input:**
```
ask prompt and save to variable.
```
Example: `ask "Enter name:" and save to user_name.`

**Prompt user without saving (pushes to stack):**
```
ask prompt.
```
Example: `ask "Continue? (y/n)".`

**Read from stdin without prompt:**
```
ask and save to variable.
```
Example: `ask and save to line.`

**Read from stdin without saving:**
```
ask.
```

---

### File Operations

**Read a file:**
```
read filename and save to variable.
```
Example: `read "data.txt" and save to content.`

**Write to a file:**
```
write content to filename.
```
Example: `write "hello" to "output.txt".`

---

### Conditional

```
when condition:
    statements
otherwise:
    statements
end.
```

The `otherwise` block is optional.

**Example:**
```
when score is greater than zero:
    say "alive".
otherwise:
    say "dead".
end.
```

---

### Loops

**Repeat N times:**
```
repeat count times:
    statements
end.
```
Example:
```
repeat three times:
    say "hello".
end.
```

**For-each loop:**
```
for each item in collection:
    statements
end.
```
Example:
```
for each fruit in fruits:
    say fruit.
end.
```

**While loop:**
```
while condition:
    statements
end.
```
Example:
```
while count is less than ten:
    count becomes count added to one.
end.
```

**Leave loop (break):**
```
leave.
```
Example:
```
while true:
    when done is equal to true:
        leave.
    end.
end.
```

**Skip to next iteration (continue):**
```
skip.
```
Example:
```
for each item in items:
    when item is equal to "skip_me":
        skip.
    end.
    say item.
end.
```

---

### Program Entry and Exit

**Entry point:**
```
start here:
    statements
end.
```

**Stop execution:**
```
stop.
```

**Exit with status code:**
```
exit with code.
```
Example: `exit with 1.`

---

### Functions

**Define a function:**
```
to name with param1, param2, ...:
    statements
end.
```
Parameters are optional.

**Example:**
```
to greet with name:
    say "Hello, " + name.
end.

to add with a, b:
    return a added to b.
end.
```

**Call a function and save result:**
```
run function_name with arg1, arg2, ... and save to variable.
execute function_name with arg1, arg2, ... and save to variable.
```
The `with` arguments and `and save to` clause are optional.

**Example:**
```
run greet with "Alice" and save to result.
execute calculate with 10, 20.
```

**Call a method on an object:**
```
method_name using object with arg1, arg2, ...
```
Example: `take_damage using warrior with 30.`

**Return from a function:**
```
return expression.
```
or simply:
```
return.
```

---

### Classes

**Define a class:**
```
define a ClassName:
    it has field_name which is default_value.
    
    on create with param1, param2, ...:
        constructor body
    end.
    
    on destroy:
        destructor body
    end.
    
    to method_name with param1, param2, ...:
        method body
    end.
    
    make member_name public.
end.
```

**With inheritance and interfaces:**
```
define a ClassName extends ParentClass implements Interface1, Interface2:
    ...
end.
```

**Full example:**
```
define a Player:
    it has name which is "Hero".
    it has hp which is 100.

    on create with player_name:
        name becomes player_name.
    end.

    on destroy:
        say name + " is gone.".
    end.

    to take_damage with amount:
        hp becomes hp subtracted by amount.
        when hp is less than zero:
            hp becomes zero.
        end.
    end.

    to get_hp:
        return hp.
    end.

    make take_damage public.
    make get_hp public.
end.
```

**Instantiate a class:**
```
let variable be instantiate ClassName with arg1, arg2, ...
let variable be fresh ClassName with arg1, arg2, ...
```
The `with` arguments are optional. `instantiate` binds to a named variable; `fresh` creates an anonymous instance (often used inline).

**Examples:**
```
let warrior be instantiate Player with "Arthur".
let enemy be fresh Player with "Goblin".
say get_hp using fresh Player.
```

**Call a method:**
```
method_name using object with arg1, arg2, ...
```
Example: `take_damage using warrior with 30.`

**Access parent class members:**
```
super of method_name with args
super with args
super.method_name(args)
```

---

### Interfaces

**Define an interface:**
```
define a interface InterfaceName extends ParentInterface:
    can method_name with param1, param2, ...
    can another_method
end.
```

**Example:**
```
define a interface Damageable:
    can take_damage with amount.
    can heal with amount.
end.
```

---

### Exception Handling

**Style 1 — beware (typed catch):**
```
beware:
    risky code
in case of ErrorType:
    error handling
regardless:
    cleanup code
end.
```

The `in case of` and `regardless` blocks are both optional.

**Example:**
```
beware:
    raise "something went wrong".
in case of error:
    say "caught error".
regardless:
    say "cleanup".
end.
```

**Style 2 — attempt (generic catch):**
```
attempt to:
    risky code
if it fails:
    error handling
regardless:
    cleanup code
end.
```

**Example:**
```
attempt to:
    raise "fail".
if it fails:
    say "caught".
regardless:
    say "done".
end.
```

**Raise an exception:**
```
raise expression.
```
Example: `raise "invalid input".`

---

### Modules

**Define a chapter (code section):**
```
chapter chapter_name.
statements...
```
A chapter groups all statements until the next `chapter` keyword or end of file.

**Import a module:**
```
refer to module_name.
```

**Import specific symbols:**
```
refer to symbol1, symbol2, ... from module_name.
```

**Import with nested path:**
```
refer to symbol from module of submodule.
```

**Import with alias:**
```
refer to module_name as alias.
```

**Examples:**
```
refer to math_utils.
refer to max, min from math_utils.
refer to helper functions of utils.
refer to utils as u.
```

**Export a member (inside a class or chapter):**
```
make name public.
```

---

### Type Operations

**Query the type of a value:**
```
type of expression
```
Example: `let t be type of 42.` (returns `"number"`)

**Convert a value to another type:**
```
convert expression to type_name and save to variable.
```
Example:
```
convert "50" to number and save to n.
convert 42 to string and save to s.
```

---

### Lists

**Create a list:**
```
let name be a list containing item1, item2, ...
```
Example: `let fruits be a list containing "apple", "banana".`
Also supports `and`: `let fruits be a list containing "apple" and "banana".`

**Access an element:**
```
list at index
```
Example: `say fruits at 0.` / `let first be fruits at zero.`

**Assign an element:**
```
list at index becomes value.
```
Example: `fruits at 0 becomes "orange".`

**Add an element:**
```
add element to list_name.
```
Example: `add "cherry" to fruits.`

**Remove an element:**
```
remove element from list_name.
```
Example: `remove "banana" from fruits.`

---

### Maps

**Create a map:**
```
let name be a map with "key1" as value1, "key2" as value2, ...
```
Keys can be strings or identifiers. Also supports `and`.

**Example:**
```
let cfg be a map with "volume" as 80, "difficulty" as "Hard".
```

**Access a value:**
```
map at key
```
Example: `say cfg at "volume".` / `let vol be cfg at "volume".`

**Assign a value:**
```
map at key becomes value.
```
Example: `cfg at "volume" becomes 100.`

---

### Object Member Access

**Access a member:**
```
member of object
```
Example: `say name of player.` / `let hp be hp of player.`

**Assign a member:**
```
member of object becomes value.
```
Example: `hp of player becomes 50.`

If the object defines `when accessed with key:` or `when assigned with key, value:`, those methods are called instead of direct field access.

---

### Index and Member Operator Overloading

Custom objects can behave like lists or maps by defining special `when` methods inside a class:

| Operation | Class Definition | Internal Method |
|-----------|-----------------|-----------------|
| `obj at index` read | `when accessed at with index:` | `accessed_at` |
| `obj at index` write | `when assigned at with index, value:` | `assigned_at` |
| `member of obj` read | `when accessed with key:` | `accessed` |
| `member of obj` write | `when assigned with key, value:` | `assigned` |

**Example — custom vector with index access:**
```linguascript
define a Vector:
    it has x which is 0.
    it has y which is 0.
    it has z which is 0.

    on create with vx, vy, vz:
        x becomes vx.
        y becomes vy.
        z becomes vz.
    end.

    when accessed at with index:
        when index is equal to 0:
            return x.
        end.
        when index is equal to 1:
            return y.
        end.
        when index is equal to 2:
            return z.
        end.
        raise "index out of range".
    end.

    when assigned at with index, value:
        when index is equal to 0:
            x becomes value.
        end.
        when index is equal to 1:
            y becomes value.
        end.
        when index is equal to 2:
            z becomes value.
        end.
    end.

    make accessed_at public.
    make assigned_at public.
end.

let v be instantiate Vector with 1, 2, 3.
say v at 0.          // 1
say v at 1.          // 2
v at 0 becomes 10.
say v at 0.          // 10
```

**Example — dynamic object with member access:**
```linguascript
define a DynamicObject:
    it has storage which is a map with "name" as "default".

    when accessed with key:
        return storage at key.
    end.

    when assigned with key, value:
        storage at key becomes value.
    end.

    make accessed public.
    make assigned public.
end.

let obj be instantiate DynamicObject.
say name of obj.             // "default"
name of obj becomes "Alice".
say name of obj.             // "Alice"
```

### Fallback Behavior

| Operation | With Overload | Without Overload |
|-----------|--------------|-----------------|
| `obj at index` read | calls `accessed_at` | list/map normal indexing, instance returns null |
| `obj at index becomes val` | calls `assigned_at` | list/map normal assignment, instance no-op |
| `member of obj` read | calls `accessed` | falls back to `it has` field read |
| `member of obj becomes val` | calls `assigned` | falls back to `it has` field write |

---

### Miscellaneous

**Capitalize a string:**
```
capitalize expression
```
Example: `say capitalize name.`

**Note (narrative comment):**
```
note that any text here is ignored.
```

---

## Expressions

### Arithmetic

| Operation | Natural Language Form | Symbolic Form |
|-----------|----------------------|---------------|
| Addition | `a added to b` | `a + b` |
| Subtraction | `a minus b` (推荐) | `a - b` |
| Subtraction | `a subtracted by b` | `a - b` |
| Subtraction | `a subtracted from b` (b - a) | — |
| Multiplication | `a multiplied by b` | `a * b` |
| Division | `a divided by b` | `a / b` |
| Modulo | `remainder of a divided by b` | — |
| Square | `square of a` | — |
| Square root | `square root of a` | — |
| Nth root | `the n root of a` | — |
| Sum | `sum of a + b` | — |
| Product | `product of a * b` | — |
| Negation | `-a` | `-a` |

### Comparison

| Operation | Syntax |
|-----------|--------|
| Equal | `a is equal to b` |
| Not equal | `a is not equal to b` / `a isnt b` |
| Greater than | `a is greater than b` |
| Less than | `a is less than b` |
| Greater or equal | `a is greater than or equal to b` |
| Less or equal | `a is less than or equal to b` |

### Logic

| Operation | Syntax |
|-----------|--------|
| AND | `a and b` |
| OR | `a or b` |
| NOT | `not a` |

### Function and Method Calls (Expression Form)

| Form | Syntax |
|------|--------|
| Call with parentheses | `func(arg1, arg2)` |
| Call with `with` | `func with arg1, arg2` |
| Method call | `method using object with args` |
| Instantiate (expression) | `instantiate ClassName with args` |
| Fresh (expression) | `fresh ClassName with args` |
| Run and save | `run func with args and save to var` |

### Indexing

Access list or map elements using the `at` keyword:
```
collection at index
```

**List indexing:**
```
let items be a list containing "a", "b", "c".
say items at 0.        // "a"
let i be one.
say items at i.        // "b"
```

**Map indexing:**
```
let cfg be a map with "volume" as 80, "name" as "test".
say cfg at "volume".   // 80
let key be "name".
say cfg at key.        // "test"
```

Index expressions can be used in any context where a value is expected:
```
let x be items at 0 added to items at 1.
cfg at "count" becomes 100.
```

### Member Access

Access object members using the `of` keyword:
```
member of object
```

**Example:**
```
define a Player:
    it has name which is "Hero".
    it has hp which is 100.
    make name public.
    make hp public.
end.

let p be instantiate Player with "Arthur".
say name of p.         // "Arthur"
hp of p becomes 50.
```

Member access falls back to `it has` field read/write if the object does not define `when accessed with key:` or `when assigned with key, value:`.

---

## Operator Precedence (Lowest to Highest)

| Level | Operators |
|-------|-----------|
| 1 (lowest) | `or` |
| 2 | `and` |
| 3 | `is greater than`, `is less than`, `is equal to`, `isnt`, `is not equal to` |
| 4 | `+`, `-`, `added to`, `subtracted from`, `subtracted by` |
| 5 | `*`, `/`, `multiplied by`, `divided by` |
| 6 | `-` (negation), `not`, `remainder of`, `square of`, `root of`, `sum of`, `product of`, `type of`, `capitalize` |
| 7 | Literals, identifiers, `( expression )` |
| 8 (highest) | `()`, `with`, `using` (function/method calls) |

---

## Data Types

| Type | Description |
|------|-------------|
| `number` | 64-bit floating point |
| `string` | UTF-8 text |
| `bool` | `true` or `false` |
| `null` | Null value |
| `list` | Ordered collection |
| `map` | Key-value store (string keys) |
| `function` | First-class function with closures |
| `class` | Class definition |
| `instance` | Object instance |
| `native_function` | Built-in function |

## Truthiness

| Value | Evaluates as |
|-------|-------------|
| `null` / `empty` | false |
| `false` | false |
| `0` | false |
| `""` (empty string) | false |
| `[]` (empty list) | false |
| `{}` (empty map) | false |
| Everything else | true |
