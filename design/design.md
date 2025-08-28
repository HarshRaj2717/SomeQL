# Global \[_TODO_]

- Make it thread safe, add locks and stuff
- Fix all build warnings
- Improve error handling, maybe make a separate common/errors.rs to consist of all errors and how to show them to end-user at a single place

# main.rs

- just runs a REPL, compiles stuff and executes stuff

# compiler.rs

- compiles a text command into different internal code understandable Statement(s)
- \[_TODO_] update the design to use enums properly to store data directly, instead of making the Statement struct, the space allocation can be improved if StatementType itself could store the data ... we will have to rename StatementType for the same though

# virtual_machine/

## virtual_machine.rs

- takes input from compiler.rs
- handles the process of running a compiled Statement
- ...need to decide more deeply here

## database.rs

- provides a database struct for managing data input/output
- ...need to decide more deeply here

## data_types.rs

- consists of all the data types that a SomeQL db can have
- also includes all the operations (eg: arithematic operations) that we can do on them

> Further planning left regarding pager.rs, store_persistent.rs, cursor.rs, etc
> Need to think deeper on how to represent the data and it's address internally, including how multiple tables will be represented too (and if to write my own hashing algo for getting data in O(1) time within a table)
