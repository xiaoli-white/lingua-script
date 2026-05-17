# LinguaScript Syntax Specification

## Overview

LinguaScript is a narrative-style programming language designed to read like natural English prose while maintaining programming rigor. It minimizes symbolic punctuation (`()`, `{}`, `;`, `=`) in favor of natural language connectives.

## Lexical Structure

### Comments

- Line comments: `// comment text`
- Narrative comments: `note that comment text.`

### Identifiers

- Standard identifiers: alphanumeric characters and underscores (e.g., `my_var`, `score`)
- Multi-word identifiers: words separated by spaces, parsed as a single identifier if none of the words are keywords (e.g., `take damage`)
- Backtick-quoted identifiers: `` `identifier with special chars` ``
- Keywords can be used as identifiers via backtick quoting

### Literals

#### Numbers

- Arabic numerals: `42`, `3.14`
- English number words: `zero`, `one`, `two`, ..., `ten`, `hundred`, `thousand`, `million`, `billion`, `trillion`
- Special words: `half`
- Decimal points: `point` (e.g., `three point one four`)
- Mixed notation allowed: `35 thousand`

#### Strings

- Single-line: `"hello world"`
- Multi-line: `"""line one\nline two"""`
- Escape sequences: `\"`, `\\`, `\n`, `\t`

#### Booleans

- `true`, `false`

#### Null

- `null`, `empty`

### Operators (Symbolic)

| Symbol | Meaning |
|--------|---------|
| `+` | Addition / String concatenation |
| `-` | Subtraction / Negation |
| `*` | Multiplication |
| `/` | Division |
| `//` | Line comment start |
| `:` | Block opener |
| `.` | Statement terminator |
| `,` | Argument / element separator |
| `(` `)` | Expression grouping, function call args |

### Keywords

```
is, isnt, be, becomes, let, true, false
when, otherwise, end
repeat, times, for, each, in, while
start, here, stop, exit, with
say, ask, and, save, to
read, write
define, a, an, it, has, which, on, create, destroy, make, public
instantiate, fresh
note, that
refer, from, chapter
beware, incase, of, regardless
attempt, if, fails
raise, return
run, execute
convert, type
added, subtracted, multiplied, divided
plus, minus
remainder, square, root
sum, product, as
not, or
greater, less, equal
using
empty, null
list, containing, map
add, remove
the, by, than
capitalize, extends, input
interface, can, implements
super
```

## Grammar

### Program Structure

```
program     → statement* EOF
statement   → let_stmt | var_def | assignment
            | when_stmt | repeat_stmt | foreach_stmt | while_stmt
            | start_stmt | stop_stmt | exit_stmt
            | say_stmt | ask_stmt | read_stmt | write_stmt
            | func_def | func_call_stmt
            | class_def | interface_def
            | beware_stmt | attempt_stmt | raise_stmt | return_stmt
            | refer_stmt | chapter_stmt
            | add_remove_stmt | convert_stmt | make_public_stmt
            | note_stmt | expression_stmt
```

Statements are terminated by `.` (dot).

### Variable Definition

```
var_def     → name "is" expression "."
let_stmt    → "let" name "be" expression "."
assignment  → name "becomes" expression "."
```

Examples:
```
score is zero.
let health be one hundred.
score becomes one hundred.
```

### Expressions

```
expression  → or_expr
or_expr     → and_expr ("or" and_expr)*
and_expr    → comparison ("and" comparison)*
comparison  → addition (("is" | "isnt" | "not") comp_op addition)?
comp_op     → "greater" "than" ("or" "equal" "to")?
            | "less" "than" ("or" "equal" "to")?
            | "equal" "to"
            | "not" "equal" "to"
addition    → multiplication (("+" | "-" | "added" "to" | "subtracted" ("from" | "by")) multiplication)*
multiplication → unary (("*" | "/" | "multiplied" "by" | "divided" "by") unary)*
```

#### Unary Expressions

```
unary       → "-" unary
            | "not" unary
            | "remainder" "of" unary "divided" "by" unary
            | "square" "of" unary
            | ("the" number? "root" | "root") "of" unary
            | "sum" "of" addition
            | "product" "of" multiplication
            | "type" "of" unary
            | "capitalize" unary
            | primary
```

#### Primary Expressions

```
primary     → number | string | "true" | "false" | "null" | "empty"
            | list_literal | map_literal
            | "input"
            | "instantiate" identifier ("with" args)?
            | "fresh" identifier ("with" args)?
            | ("run" | "execute") callable ("with" args)? ("using" object)? ("and" "save" "to" name)?
            | "(" expression ")"
            | identifier postfix*
```

#### Postfix Operations

```
postfix     → "(" args ")"
            | "with" args
            | "using" object ("with" args)?
```

#### List Literal

```
list_literal → ("a" | "an") "list" "containing" (expression ("and" | ",") expression)*
```

#### Map Literal

```
map_literal → ("a" | "an") "map" "with" (key "as" value ("and" | ",") key "as" value)*
key         → string | identifier
```

### Control Flow

#### Conditional

```
when_stmt   → "when" expression ":" body ("otherwise:" body)? "end" "."
body        → statement*
```

Example:
```
when score is greater than zero:
    say "alive".
otherwise:
    say "dead".
end.
```

#### Loops

```
repeat_stmt → "repeat" expression "times" ":" body "end" "."
foreach_stmt → "for" "each" name "in" expression ":" body "end" "."
while_stmt  → "while" expression ":" body "end" "."
```

Examples:
```
repeat three times:
    say "hello".
end.

for each item in items:
    say item.
end.

while count is less than ten:
    count becomes count added to one.
end.
```

#### Program Entry and Exit

```
start_stmt  → "start" "here" ":" body "end" "."
stop_stmt   → "stop" "."
exit_stmt   → "exit" ("with" expression)? "."
```

### Input/Output

```
say_stmt    → "say" expression "."
ask_stmt    → "ask" expression "and" "save" "to" name "."
read_stmt   → "read" expression "and" "save" "to" name "."
write_stmt  → "write" expression "to" expression "."
```

Examples:
```
say "Hello".
ask "Enter name:" and save to user_name.
read "data.txt" and save to content.
write content to "output.txt".
```

### Functions

#### Function Definition

```
func_def    → "to" name ("with" params)? ":" body "end" "."
params      → identifier ("," identifier)*
```

Example:
```
to greet with name:
    say "Hello, " + name.
end.

to add with a, b:
    return a added to b.
end.
```

#### Function Call (Statement Form)

```
func_call_stmt → ("run" | "execute") callable ("with" args)? ("using" object)? ("and" "save" "to" name)? "."
callable    → identifier | super_expr | "(" expression ")"
args        → expression ("," expression)*
```

Example:
```
run greet with "Alice" and save to result.
execute calculate with 10, 20 using calculator.
```

#### Return

```
return_stmt → "return" expression? "."
```

### Classes

#### Class Definition

```
class_def   → "define" "a" name class_inheritance? ":" class_body "end" "."
class_inheritance → ("extends" identifier)? ("implements" identifier ("," identifier)*)?
class_body  → (field | constructor | destructor | method | make_public)*
field       → "it" "has" name "which" "is" expression "."
constructor → "on" "create" ("with" params)? ":" body "end" "."
destructor  → "on" "destroy" ":" body "end" "."
method      → "to" name ("with" params)? ":" body "end" "."
            | "when" method_name ("with" params)? ":" body "end" "."
make_public → "make" name "public" "."
```

Example:
```
define a Player extends Entity implements Damageable, Serializable:
    it has name which is "Hero".
    it has hp which is 100.

    on create with player_name:
        name becomes player_name.
    end.

    on destroy:
        say name + " is destroyed.".
    end.

    to take_damage with amount:
        hp becomes hp subtracted by amount.
    end.

    make take_damage public.
end.
```

#### Instantiation

```
instantiate → "instantiate" identifier ("with" args)?
fresh       → "fresh" identifier ("with" args)?
```

Examples:
```
let warrior be instantiate Player with "Arthur".
let enemy be fresh Player with "Goblin".
```

#### Method Call

```
method_call → name "using" object ("with" args)?
```

Example:
```
take_damage using warrior with 30.
let hp be get_hp using warrior.
```

#### Super Call

```
super_call  → "super" "of" name ("with" args)?
super_access → "super" postfix*
```

### Interfaces

```
interface_def → "define" "a" "interface" name ("extends" identifier ("," identifier)*)? ":" interface_body "end" "."
interface_body → ("can" name ("with" params)? ".")*
```

Example:
```
define a interface Damageable:
    can take_damage with amount.
    can heal with amount.
end.
```

### Exception Handling

#### Beware Style (Typed Catch)

```
beware_stmt → "beware" ":" body ("in" "incase" "of" identifier ":" catch_body)? ("regardless" ":" finally_body)? "end" "."
```

Example:
```
beware:
    raise "something went wrong".
in case of error:
    say "caught error".
regardless:
    say "cleanup".
end.
```

#### Attempt Style (Generic Catch)

```
attempt_stmt → "attempt" "to" ":" body ("if" "it" "fails" ":" catch_body)? ("regardless" ":" finally_body)? "end" "."
```

Example:
```
attempt to:
    raise "fail".
if it fails:
    say "caught".
regardless:
    say "done".
end.
```

#### Raise

```
raise_stmt  → "raise" expression "."
```

### Modules

#### Chapter

```
chapter_stmt → "chapter" name "." statement*
```

A chapter groups statements until the next chapter or EOF.

#### Import (Refer)

```
refer_stmt  → "refer" "to" (name ("," name)* "from")? module_path ("as" name)? "."
module_path → name ("of" name)*
```

Examples:
```
refer to math_utils.
refer to max, min from math_utils.
refer to helper functions of utils.
refer to utils as u.
```

#### Export

```
make_public_stmt → "make" name "public" "."
```

### Type Operations

```
type_query  → "type" "of" expression
convert_stmt → "convert" expression "to" identifier ("and" "save" "to" name)? "."
```

Examples:
```
let t be type of 42.
convert "50" to number and save to n.
convert 42 to string and save to s.
```

### List Operations

```
add_remove_stmt → "add" expression "to" identifier "."
                | "remove" expression "from" identifier "."
```

Examples:
```
add "cherry" to fruits.
remove "banana" from fruits.
```

### Miscellaneous

#### Note (Comment)

```
note_stmt   → "note" "that" ... "."
```

Everything between `note that` and the next `.` is ignored.

#### Capitalize

```
capitalize_expr → "capitalize" expression
```

Returns the capitalized form of a string expression.

#### Input Expression

```
input_expr  → "input"
```

Reads a line from standard input as an expression.

## Operator Precedence (Lowest to Highest)

1. `or`
2. `and`
3. Comparison: `is greater than`, `is less than`, `is equal to`, `isnt`, `is not equal to`
4. Addition/Subtraction: `+`, `-`, `added to`, `subtracted from/by`
5. Multiplication/Division: `*`, `/`, `multiplied by`, `divided by`
6. Unary: `-`, `not`, `remainder of`, `square of`, `root of`, `sum of`, `product of`, `type of`, `capitalize`
7. Primary: literals, identifiers, parenthesized expressions
8. Postfix: function calls `()`, `with`, method calls `using`

## Data Types

| Type | Description |
|------|-------------|
| `number` | 64-bit floating point (f64) |
| `string` | UTF-8 string |
| `bool` | Boolean (`true`/`false`) |
| `null` | Null value |
| `list` | Ordered collection of values |
| `map` | Key-value store with string keys |
| `function` | First-class function with closures |
| `class` | Class definition |
| `instance` | Object instance |
| `native_function` | Built-in function |

## Truthiness

| Value | Truthy? |
|-------|---------|
| `null` | false |
| `false` | false |
| `0` (number) | false |
| `""` (empty string) | false |
| `[]` (empty list) | false |
| `{}` (empty map) | false |
| All other values | true |
