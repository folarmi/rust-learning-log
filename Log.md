# Log

One line a day. What you did, what clicked, what didn't.

## Week 1: setup and the basics

- **Day 1 (Mon 12 Oct):** Installed Rust, set up this repo, Book ch1.
- **Day 2 (Tue 13 Oct):** Started ch3. Variables are immutable by default??
- **Day 3 (Wed 14 Oct):** Shadowing with `let x = x + 1;` finally makes sense.
- **Day 4 (Thu 15 Oct):** Shadowing (`let x = ...` again) creates a new variable, so the type can change. `mut` changes the value of the same variable, so the type can't.
- **Day 5 (Fri 16 Oct):** Every time an if produces a value, every arm has to produce the same type.
- **Day 6 (Sat 17 Oct):** Completed Chapter 2, built the temperature converter as well.
- **Day 7 (Sun 18 Oct):** Built the Fibonacci generator and Twelve Days of Christmas printer.

## Week 2: Ownership

- **Day 8 (Mon 19 Oct):** Started learning about ownership.
- **Day 9 (Tue 20 Oct):** Completed references and borrowing. Every value in Rust has an owner, there can only be one owner at a time, and when the owner goes out of scope, the value is dropped.
- **Day 10 (Wed 21 Oct):** Started learning about structs.
- **Day 11 (Thu 22 Oct):** Learnt about associated functions and methods.
- **Day 12 (Fri 23 Oct):** Concluded with enums, match, `Option<T>`, `if let` and `let else` statements.
- **Day 13 (Sat 24 Oct):** Worked on the Rustlings exercises for chapters 5 to 8.
- **Day 14 (Sun 25 Oct):** Started the modules chapter

## Week 3: Modules and collections

- **Day 15 (Mon 26 Oct):** Chapter 7: modules, paths, `pub`, `use` and `super`. A file isn't part of the program until its parent declares it with `mod`. Privacy rule: children can use their parents' stuff, but parents can only use their children's stuff if it's marked `pub`. Got it backwards at first (5/8 on the quiz).
- **Day 16 (Tue 27 Oct):** Started the collections chapter and studied about Vectors
- **Day 17 (Wed 28 Oct):** Worked on chapter 8 and 9 of the rustlings book
