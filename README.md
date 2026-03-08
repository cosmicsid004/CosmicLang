# CosmicLang

An experimental programming language focused on concise, expressive syntax

## Description
It's got a lexer, parser, and evaluator the whole compiler pipeline and runs as a REPL in your terminal.
Still very much a work in progress, but it already works for basic math, variables, and printing output.

## Usage

### What it can do right now

```javascript
cosmic >> x = 10
cosmic >> x + 5
cosmic >> publish(x * 2)
20
cosmic >> (x + 2) * 3
```

### Current Version (v1.0)
- Math expressions with correct operator precedence (2 + 3 * 4 = 14, not 18)
- Variable assignment and reuse
- Negative numbers and nested parens
- 'publish(expr)' to print output
- 'vars' to see all stored variables
- 'clear' to clear the screen
- 'quit' / 'exit' to leave
## How it's built

The compiler pipeline has 3 stages:

```
raw text ->  [Lexer]  ->  tokens  ->  [Parser]  ->  AST  ->  [Evaluator]  ->  result
```

**Lexer** (`lexer.rs`) -> turns raw text into a list of tokens. So `x = 10 + 6` becomes `[Ident(x), Equal, Number(10), Plus, Number(6)]`.

**Parser** (`parser.rs`) ->  turns tokens into an AST (Abstract Syntax Tree) using a recursive descent parser. This is what gets operator precedence right.

**Evaluator** (`evaluator.rs`) ->  walks the AST and computes the final value. Also holds a global variable store (`HashMap<String, Value>`).

## Running it

```bash
git clone https://github.com/yourusername/cosmiclang
cd cosmiclang
cargo run
```

That's it. No dependencies outside of Rust's standard library.

---

## What's coming next

- [ ] Comparison operators (`==`, `>`, `<`)
- [ ] `if / else` blocks
- [ ] String literals (`"hello"`)
- [ ] `while` loops
- [ ] Functions with `fn name(args) { body }`
- [ ] Better error messages with line numbers
- [ ] Run `.cosmic` files directly

---
