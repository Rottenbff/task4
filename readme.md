# Task 4: Rick and Morty - Provably Fair Game

This is a console application that implements a game based on the Monty Hall problem, developed for Task #4 of the iLearning/Itransition training program. The application is written in Rust and demonstrates a modular, extensible architecture.

## Key Features

- **Provably Fair Randomness:** Implements a collaborative, cryptographically secure random number generation protocol using HMAC-SHA3 to ensure the computer cannot cheat.
- **Pluggable Architecture:** The game logic is separated from the "Morty" strategy. Two implementations are provided:
  - `ClassicMorty`: Follows the standard Monty Hall problem, using the fair protocol to make random choices.
  - `LazyMorty`: A deterministic version that follows the rules with the simplest possible logic.
- **Statistical Analysis:** Tracks gameplay statistics and displays a summary table at the end of the game, including both experimental and theoretical probabilities.
- **Robust Error Handling:** Professional command-line argument parsing and validation for all user inputs.

## Prerequisites

- The Rust toolchain (including `cargo`) is required to build and run the project. It can be installed via [rustup](https://rustup.rs/).

## Building and Running

1.  **Clone the repository:**
    ```bash
    git clone
    cd task4
    ```

2.  **Build the release version:**
    ```bash
    cargo build --release
    ```
    The executable will be located at `./target/release/rng_game`.

3.  **Run the game:**

    **Example with `ClassicMorty`:**
    ```bash
    ./target/release/rng_game 3 ClassicMorty
    ```

    **Example with `LazyMorty` and more boxes:**
    ```bash
    ./target/release/rng_game 10 LazyMorty
    ```

    **Example of an invalid command:**
    ```bash
    ./target/release/rng_game 2 ClassicMorty
    ```

## Technologies Used

- **Language:** Rust
- **Key Crates:**
  - `clap`: For command-line argument parsing.
  - `rand`: For cryptographically secure random number generation.
  - `hmac` & `sha3`: For the HMAC-SHA3 implementation.
  - `comfy-table`: For displaying the final statistics table.
