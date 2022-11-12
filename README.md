# Tiny Expr Parser

A simple parser without externally dependences

## Feature

- Format
- Evaluate

## Usage

```rust
#[test]
fn smoke() {
    let expr = "1*2+(3/(4+(-5)))";
    let ast = build_ast(expr).unwrap();

    assert_eq!(-1, eval(&ast));
    assert_eq!("1 * 2 + 3 / (4 + (-5))", format(&ast));
}
```

## DFA

### Defination

**DFA = ( StateSet, InputSet, transition_fn, start, TerminatorSet )**

- StateSet = { START, OPERATOR, ZERO, NUM }
- InputSet = { operator, whitespace, 0, 1-9 }
- start = START
- TerminatorSet = { OPERATOR, ZERO, NUM }

### Transition Graph

### Transition Table

|          | op  | ws  | 0   | 1-9 |
|----------|-----|-----|-----|-----|
| ERR      | E   | E   | E   | E   |
| START    | 2   | 1   | 3   | 4   |
| OPERATOR | 2   | 1   | 3   | 4   |
| ZERO     | 2   | 1   | E   | E   |
| NUM      | 2   | 1   | 4   | 4   |

## Grammar

primitive grammar

```text
<expr> ::= <add> 
         | <mul> 
         | <literal>
         | "(" <expr> ")" 
         ;
      
<add>  ::= <expr> ("+" | "-") <expr> ;

<mul>  ::= <expr> ("*" | "/") <expr> ;
```

introduce operator precedence

```text
<expr>   ::= <expr> ("+" | "-") <add>
           | <add>
           ;

<add>    ::= <add> ("*" | "/") <mul> 
           | <factor>
           ;

<factor> ::= "(" <expr> ")"
           | <literal>
           ;
```

eliminate left recursion

```text
<expr>   ::= <add> <expr'> ;
<expr'>  ::= ("+" | "-") <add> <expr'> 
           | <empty>
           ;

<add>    ::= <factor> <add'> ;
<add'>   ::= ("*" | "/") <mul> <add'> 
           | <empty>
           ;

<factor> ::= "(" <expr> ")"
           | <literal>
           ;
```

simplified

```text
<expr>   ::= <add> (("+" | "-") <add>)* ;

<add>    ::= <mul> (("*" | "/") <mul>)* ;

<factor> ::= "(" <expr> ")"
           | <literal>
           ;
```
