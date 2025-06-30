# Project-Specific Guidelines for Gemini

This `GEMINI.md` file provides context and preferences for the Gemini AI agent when working on the `spaceship-bridge-game` project.

## General Preferences:

*   **Texture Format:** Prefer `.png` for image textures. If generating images, convert them to PNG.
*   **Asset Location:** All game assets (textures, models, etc.) should be placed in the `assets/` directory, with subdirectories for organization (e.g., `assets/textures/`).
*   **Code Style:** Adhere to standard Rust formatting (`cargo fmt`) and Bevy's idiomatic patterns.
*   **Physics Debugging:** Use `RapierDebugRenderPlugin` only when explicitly requested for debugging purposes. Keep it disabled for general development and commits.
*   **Commit Messages:** Aim for clear, concise, and descriptive commit messages.

## Project Structure:

*   `src/main.rs`: Main application logic, setup, and core systems.
*   `assets/`: Directory for all game resources.
    *   `assets/textures/`: Image textures.

## Current Development Focus:

*   Improving visual fidelity (textures, lighting).
*   Ensuring robust physics and collision detection.
*   Enhancing environmental details.
