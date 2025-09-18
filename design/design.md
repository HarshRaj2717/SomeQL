> This file is mostly used as a design doc, TODO(s) denote what needs to be done while building this project, other unplanned issues can be found in the Issue tab on GitHub

# \[_TODO_] Global

- Make it thread safe, add locks and stuff
- Improve error handling, maybe make a separate common/errors.rs to consist of all errors and how to show them to end-user at a single place
- Implement indexing (via b-tree this should be fine automatically, dedpends on how you implement)\
- Look into separating out app & core : https://www.youtube.com/watch?v=rUxZ5N77M5E

# main.rs

- just runs a REPL, compiles stuff and executes stuff

# common/

## datatypes.rs

- consists of all the data types that a SomeQL db can have
- also includes all the operations (eg: arithematic operations) that we can do on them

## constants.rs

- just the constants, duh!!!

## errors.rs

- all errors in one place

# compiler/

## compiler.rs

- compiles a text command into different internal code understandable Statement(s)
- \[_TODO_] update the design to use enums properly to store data directly, instead of making the Statement struct, the space allocation can be improved if StatementType itself could store the data ... we will have to rename StatementType for the same though ... move this to a common folder too
- \[_TODO_ | **IMPORTANT**] Fix compiler to use proper lexing/grammar steps, include more complex commands, sub-queries, etc
- \[_TODO_] Need to update DROP command to accept WHERE clause

# \[_TODO_] executor/

## executor.rs

- takes input from compiler.rs
- handles the process of running a compiled Statement
- maintains a database object

## database.rs

- provides a database struct for managing data input/output
- does type checking of queries
- maintains a cursor object

## cursor.rs

- manages the exact location to write to
- maintains a page_manager object

## page_manager.rs

- fix the current page size to 4KB, will dynamically pick up system's page size later
- return the current page to write to
- create new page into the table if needed
- handles structure, offsets, etc of a page
- controls how many rows will there be in a single page
- each page consists of a way to point to the next page (this is to make it easier to move the cursor in case of multiple tables)
- maintains a persistent_storage_manager object

## persistent_storage_manager.rs

- mmap layer, use map, map_copy, and map_mut as needed
- handle file locking mechanism
- provide read-only mapping for isolation and also writable once
- solve the proble of dirty reads/writes here
