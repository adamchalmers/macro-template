# Rust proc-macro template

Goal is to make the macro

 - testable
 - emit useful diagnostics to Rust Analyzer or other LSPs, even if the person calling the macro did something wrong.

Structure based on

 - Code from Oxide Computer's macros like Dropshot and Daft
 - This writeup: https://www.schneems.com/2025/03/26/a-daft-procmacro-trick-how-to-emit-partialcode-errors/
