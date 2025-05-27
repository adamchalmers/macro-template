# Rust proc-macro template

Rust macros are a bit tricky to structure correctly. Two common problems are:

 - How do you test them?
 - How do you ensure that, when users get something wrong, your macro still emits useful information to Rust Analyzer or other LSPs? For more on this problem watch [this EuroRust talk](https://www.youtube.com/watch?v=JceLEyphDXU)

This is the rough template we use at [zoo.dev] for our proc-macro. It's what I'd suggest for others getting started with Rust macros. PRs witih suggestions are very welcome.

# Acknowledgements

Big thank you to:

 - Oxide Computer, whose macros like Dropshot and Daft provided me with great examples
 - This [writeup](https://www.schneems.com/2025/03/26/a-daft-procmacro-trick-how-to-emit-partialcode-errors/) from Richard Schneeman analyzing the above Daft crate
 - Lukas Wirth of Rust Analyzer who spoke about the problems that macros can cause for RA.
