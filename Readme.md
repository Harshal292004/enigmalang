# EnigmaLang Documentation 

## Overview

EnigmaLang is a two-tiered language suite designed for building a custom operating system (EnigmaOS) from the ground up. It serves both low-level systems programming and high-level user space development.

1. **EnigmaCore**: A Rust-inspired, kernel-safe systems programming language for low-level development (written in Rust).
2. **EnigmaPlusPlus (EnigmaFull)**: A high-level, object-oriented language designed for writing user space applications, bootstrapped from EnigmaCore.

The goal is to understand operating systems at a deep level by writing everything from scratch, including the language itself.

---

## EnigmaCore Syntax Summary

### Comments

* Single line comments using `#`
* No multi-line comments

### Module Imports

```en
get module std.io as io
get module math as m
```

### Variable Declarations

```en
int x := 4             # Immutable
mut int y := 9         # Mutable
```

### Functions

```en
@sum(int a, int b)::int -> a + b;

@diff(int a, int b)::int {
    return a - b
}

@mutate(byte buf)::byte {
    buf = buf + 1
    buf;
}

@mutate_mut(&mut byte buf)::unit {
    buf = buf + 1
}

@log_event(int trace_id%id, string msg)::unit {
    # implementation
}
```

### Anonymous Functions / Expressions

```en
int foo := {
    int u = 8
    int y = 9
    u + y
}
```

### Conditionals

```en
int foo := if 9 == 9 {
    9
} else {
    0
}
```

### Loops

```en
for i in 1..2 {
    print(i + 1)
    print(i)
}

while i < 1 {
    i++
}

loop {
    if i > 10 {
        break i
    }
}
```

### Data Types

#### Records

```en
record human {
    name: string
    age: int
    karma: int
    health: Option[int]
}
```

#### Implementations

```en
implement human {
    pub @new(string name%name, int age%age, int karma%karma)::human {
        human { name: name, age: age, karma: karma };
    }

    pub @speak(self) {
        print("Sab golmal hai...")
    }

    pub @cough(mut self) {
        self::health -= 10
    }
}
```

#### Unions

```en
union status[T] {
    Ok(T)
    Err(string)
}
```

### Protocols (Traits / Interfaces)

```en
live protoc {
    @eat(self)::string
    @socialize()
    @read()
}

implement live for human {
    @eat(self)::string {
        "Mai ${self::name} hun"
    }
    @socialize() {
        print("Aur bhai!!")
    }
    @read() {
        print("Ek kirayedar ne...")
    }
}
```

### Tuples

```en
(int, string, byte) x := (42, "hello", 0xFF)
(int a, string b, byte c) $= x

@coords()::(int, int) {
    return (1, 2)
}
```

### Generics

```en
@add[T](T lis)::T {
    return lis
}

record human[T] {
    program: T
}
```

### Option / Result for Error Handling

```en
union Option[T] {
    Some(T),
    None
}

implement Option[T] {
    pub @extract[T](self) -> T {
        match self {
            Option::Some(T): T;,
            Option::None: exit("")
        }
    }
}

union Result[T, E] {
    Ok(T),
    Err(E)
}
```

### Pattern Matching

```en
match drink {
    case Option::Some("lemonade"): print("Yuck!"),
    case Option::Some("inner"): print("Nice."),
    case Option::None: print("No drink")
}

match drink {
    case Option::Some("lemonade"): print("Yuck!"),
    case Option::Some("inner"): print("Nice."),
    case _: print("Unhandled")
}
```

### Unpacking with `?`

```en
# T ? => extract or exit if None
```

### References and Raw Pointers

```en
int x := 4
ref int y := ref x
int val := deref y

raw_ref int val := raw_ref x

unsafe {
    int val := deref val
}
```

### Inline Assembly

```en
asm {
    "mov eax , 1"
    "int 0x80"
}
```

---

## Goals

* Build minimal compiler & runtime for EnigmaCore in Rust
* Build EnigmaOS kernel in EnigmaCore

---

## File Structure (WIP)

```sh
enigmalang/
├── decisions.md
├── enigma-core/          # Rust implementation of EnigmaCore
│   ├── lexer/
│   ├── parser/
│   ├── checker/
│   └── main.rs
├── enigma-full/          # Future bootstrapped language
├── syntax/
│   └── syntax.en         # EnigmaCore syntax examples
```
---


This is a programming language  experiment with educational intent. 