# MysticLang Documentation

## Overview

MysticLang is a **unique**, **theme-based programming language** designed around mystic terms and concepts written in **Rust**. It incorporates **object-oriented programming** (OOP) with some **functional features**. The syntax is minimalistic and avoids common programming constructs such as semicolons, access modifiers, and pointers, making it easy to follow yet powerful.

It's type safe and supports automatic memory management using a garbage collector.

## Key Concepts 

1. `rune` (**Class**)  
   Defines a class, with support for **inheritance**. It can have both data members (`bind`, `seal`) and methods (`spell`).

2. `spell` (**Function**)  
   Defines a function. Functions can return a type or be void (implicitly returning null).

3. `bind` (**Variable Declaration**)  
   Declares a variable of a specific type.

4. `seal` (**Constant Declaration**)  
   Defines a constant whose value cannot be modified once set.

5. `reveal` / `veil` (**Conditional Statements**)  
   Equivalent to if and else. Controls program flow based on conditions.

6. `invoke` (**For Loop**)  
   Used for iterating over a range, equivalent to a for loop.

7. `linger` (**While Loop**)  
   A basic while loop.

8. `divine`/`sigil` (**Switch/Case Statements**)  
   Provides branching based on specific conditions.

9. `chant` (**Return Statement**)  
   Returns a value from a function.

10. `call` (**Import Statement**)  
    Used to **import modules** into the current file. For example, 
    `call io.myst as io` imports the io module.

11. `summon` (**Intializer**)  
    Used to intialize a class instance.

12. `self` (**The `self` object**)  
    Used to intialize a class.

13. `draws` (**Inheritance**)  
    Used to inherit classes.

14. `shatter` (**break**) 
    Used to break a loop
    
15. `phase` (**continue**)
    Used to continue 

## Language Specification

### Comments 

```
# single line comment

###
multi line comment
###
```

### Variable Declarations

```
bind variable_name: type
```

- Declares a variable of a specific type (`int`, `str`, `float`, `bool` etc.).
- Variables can hold any data type and can be reassigned later.

### Constant Declarations

```
seal constant_name: type = value
```

- Declares a constant whose value cannot be changed once set.

### Class Declaration

```
rune ClassName {
    spell summon(self) {
        # Code block
    }
}
```

- Defines a class with methods (`spell`) and properties (`bind`, `seal`)
- Can inherit from other classes by specifying `rune ChildClass draws ParentClass`.

### Functions

```
spell function_name(a: int, b: float): return_type {
    # Code block
    chant return_value
}
```

- Defines a function. Functions can return a value using `chant`, or implicitly return `null`.
- Parameters are defined in parentheses and are typed.

### Conditionals

```
reveal condition {
    # Code if condition is true
} veil {
    # Code if condition is false
}
```

- Conditional statement similar to if/else.

### Loops 

`invoke` (**for loop**)

```
invoke i in start..end , 2 {
    # Code block
}
```

- Loops from start to end (inclusive).

`linger` (**while loop**)

```
linger condition {
    # Code block
}
```

- Loops until the condition is false.

### Switch/Case 

```
divine variable_name {
    sigil value {
        # Code block if value matches
    }
    sigil default {
        # Code block for default case
    }
}
```

- A switch-case style construct for branching based on specific values.

### Return 

```
chant return_value
```

## `io` Module (from `myst.io`)

The `io` module provides basic input and output functionality in MysticLang. It is imported using the `call` keyword:

```
call io.myst as io
```

### Functions

1. `io.echo(value: any)`  
   Prints a value to the output without a newline.

   ```
   io.echo("Hello")
   ```

2. `io.echoln(value: any)`  
   Prints a value to the output with a newline.

   ```
   io.echoln("MysticLang")
   ```

3. `io.whisper(): str`  
   Reads a line of input from the user and returns it as a str.

   ```
   bind name: str = io.whisper()
   ```

## Sample MysticLang Code

```
call io.myst as io

rune Account {
    bind owner: str
    bind balance: float

    spell summon(self, owner: str, balance: float) {
        self.owner = owner
        self.balance = balance
    }

    spell deposit(self, amount: float): str {
        reveal amount > 0 {
            self.balance = self.balance + amount
            chant "Deposit successful"
        } veil {
            chant "Invalid deposit amount"
        }
    }

    spell withdraw(self, amount: float): str {
        reveal amount <= self.balance {
            self.balance = self.balance - amount
            chant "Withdrawal successful"
        } veil {
            chant "Insufficient balance"
        }
    }

    spell get_balance(self): float {
        chant self.balance
    }
}

rune SavingsAccount draws Account {
    bind interest_rate: float

    spell summon(self, owner: str, balance: float, interest_rate: float) {
        self.owner = owner
        self.balance = balance
        self.interest_rate = interest_rate
    }

    spell apply_interest(self): str {
        bind interest: float = self.balance * self.interest_rate / 100.0
        self.balance = self.balance + interest
        chant "Interest applied"
    }
}

spell main(): str {
    io.echoln("Enter your name:")
    bind name: str = io.whisper()

    bind account: SavingsAccount = SavingsAccount(name, 1000.0, 5.0)
    io.echoln("Account created for " + name)

    invoke i in 1..3 {
        io.echoln("Choose an action: 1=Deposit 2=Withdraw 3=Check Balance 4=Apply Interest")
        bind action: str = io.whisper()

        divine action {
            sigil "1" {
                io.echoln("Enter amount to deposit:")
                bind amt: str = io.whisper()
                chant account.deposit(amt.to_float())
            }
            sigil "2" {
                io.echoln("Enter amount to withdraw:")
                bind amt: str = io.whisper()
                chant account.withdraw(amt.to_float())
            }
            sigil "3" {
                bind bal: float = account.get_balance()
                io.echoln("Current Balance: " + bal.to_str())
            }
            sigil "4" {
                chant account.apply_interest()
            }
            sigil default {
                io.echoln("Invalid action")
            }
        }
    }

    chant "Session Ended"
}
```

## Walkthrough: MysticLang Banking System

1. `rune Account`
   - Base class with `owner` and `balance`
   - Provides `deposit`, `withdraw`, and `get_balance` methods
   - Enforces rules using `reveal`/`veil` conditionals

2. `rune SavingsAccount draws Account`
   - Inherits from `Account`
   - Adds `interest_rate` and method `apply_interest` to compute and apply interest

3. `main` Function
   - Takes user input using io.whisper
   - Creates a SavingsAccount with default values
   - Uses a loop (invoke) to allow 3 operations
   - Handles user input with divine/sigil for command selection

4. Real-World Use
   - Simulates an actual banking use-case
   - Demonstrates encapsulation and data handling
   - Shows clear control flow, OOP structure, and function interaction
   - Uses type safety and no semicolons per MysticLang design