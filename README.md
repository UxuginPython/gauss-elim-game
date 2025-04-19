# Gaussian Elimination Game
**A Gaussian elimination puzzle game built using GTK 4.** Written as a challenge from my precalculus teacher. BSD licensed.
## What is Gaussian elimination?
Gaussian elimination is a method of solving linear systems of equations named after mathematician Carl Friedrich Gauss. It arranges the coefficients and solutions of the equations into a matrix and then allows three operations: swapping two rows, scaling a row, and adding a multiple of a row to another. These operations are performed until the coefficients form the identity matrix (called reduced row echelon form) if a unique solution exists.
## Installation
Before you install this crate, you need (obviously) the Rust toolchain installed as well as the GTK 4 build essentials. See the [gtk-rs documentation](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html) for how to install these. Then, it's just a regular `cargo install gauss-elim-game`.
## How to Play
- To swap two rows, drag from the circle to the left of one to the circle of the other.
- To scale a row to make a coefficient 1, click the coefficient.
- To add a multiple of a row to another row to make a coefficient 0, drag from the row's circle to the coefficient.
- Click "Hint" for a suggestion for what to do.
- Click "New" to generate a new random system.
