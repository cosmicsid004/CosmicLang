<p align="center">
  <img src="owl.png" width="180" alt="CosmicLang Logo"/>
</p>
<h1 align="center">CosmicLang</h1>
<p align="center">
<i>An edge-computing focused programming language built for constrained, latency-sensitive environments.</i>
</p>

## Description
CosmicLang has a full compiler pipeline lexer, parser, and evaluator and runs as both a REPL and a file compiler. It supports multi-line programs, blocks, conditionals, string operations, and math. Still a work in progress, but growing fast.

## Usage

### REPL
```javascript
cosmic >> x = 10
cosmic >> x + 5
cosmic >> publish(x * 2)
20
cosmic >> (x + 2) * 3
```

### File Mode
```bash
cargo run hello.cosmic
```

### Example `.cosmic` program
```javascript
x = 10
y = 20

result = x > y ? {
    publish("x is bigger")
} : {
    publish("y is bigger")
}

name = "Cosmic"
lang = "Lang"
full = name + lang
publish(full)
```

## What it can do right now (v2.0)

- Math expressions with correct operator precedence (`2 + 3 * 4 = 14`, not 18)
- Variable assignment and reuse across lines
- Negative numbers and nested parentheses
- `publish(expr)` to print output
- String concatenation with `+`
- Comparison operators `>`, `<`, `>=`, `<=`, `==`, `!=`
- Ternary conditionals `condition ? { } : { }`
- **Multi-line block support** blocks spanning multiple lines work correctly
- **Full file compilation** entire `.cosmic` file parsed as one program
- `vars` to inspect stored variables (REPL)
- `clear` to clear the screen (REPL)
- `quit` / `exit` to leave (REPL)

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
git clone https://github.com/cosmicsid004/cosmiclang
cd cosmiclang
cargo run
```

That's it. No dependencies outside of Rust's standard library.

---

## What's coming next

- [ ] `if / else` blocks
- [ ] `while` loops
- [ ] Functions with `fn name(args) { body }`
- [ ] Better error messages with line numbers
- [ ] Run `.cosmic` files directly

---
