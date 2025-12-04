[![Documentation on GitHub Pages](https://github.com/marcel-scherzinger/scratch-test-model/actions/workflows/deploy-docs-to-pages.yml/badge.svg)](https://marcel-scherzinger.github.io/scratch-test-model)

# About

The block-oriented programming language [Scratch](https://scratch.mit.edu/)
is used by many institutions to teach beginners how programming concepts work.
The `scratch-test` project is planned as a way to unit-test submissions of
learners for defined exercises.

This repository contains the `model` for parsing Scratch files.
Other crates like the interpreter build on top of it.

# Limitations

This project is assumed to be used for **algorithmic exercises**, so the focus is
on control structures, input, output, variables and lists.

- Sounds, movements, colors, etc. are not planned.
- As Scratch gives *no guarantees* about the execution order of parallel programs
  this project disallows them completly.
  The usage of parallelism in a file can lead to the interpreter rejecting it.
  _(Just stick to a single green-flag event and you're fine.)_
- Scratch often tries to do _something_ to avoid exceptions or fatal errors.
  Some expressions don't evaluate to a value programmers would be expect based on their
  knowledge from other languages. This project tries to model them but there is still a
  chance of differences in behaviour, especially when it comes to numbers.
