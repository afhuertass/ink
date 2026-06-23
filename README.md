# Narrative Framework

A declarative framework for story-driven and text games.

This repository is a **fork of [inkle/ink](https://github.com/inkle/ink)**, extended with a structured directive system and a rewritten Rust compiler. CODE is lARGELY AI DRIVEN at this stage, I'm just testing the limits and ideas, nowhere near to be useful for something (other than burning my precious tokens)

## Purpose

Narrative bridges the gap between structured narrative scripting (`ink`) and game engine implementation (`Godot`, `Unity`). It extends `ink` with a structured directive system, allowing writers to reference game scene elements — assets, sounds, actions, events — directly within their stories.

### The Philosophy

Narrative defines a clear boundary between writing and programming:

- **Programmer Workflow (`.inkdef.yaml`)**: Programmers define the "API" for the narrative — declaring assets, available actions, game state variables, and events.
- **Writer Workflow (`.ink`)**: Writers reference these definitions via validated `@` directives in their ink stories, ensuring that narrative scripts stay in sync with game engine capabilities.

This setup provides compile-time validation of game-side references, autocompletion for writers, and clean, engine-agnostic compilation.

## Roadmap

| Phase | Description                                | Status      |
| ----- | ------------------------------------------ | ----------- |
| 1     | Rust ink compiler (compatible JSON output) | ✅ Complete |
| 2     | Definitions parser (`.inkdef.yaml`)        | ✅ Complete |
| 3     | Directives (`@` prefix) + validation       | Planned     |
| 4     | LSP with validation + autocompletion       | Planned     |
| 5     | Godot SDK                                  | Planned     |

---

# Original Ink Documentation

[Ink](http://www.inklestudios.com/ink) is [inkle](http://www.inklestudios.com/)'s scripting language for writing interactive narrative, both for text-centric games as well as more graphical games that contain highly branching stories. It's designed to be easy to learn, but with powerful enough features to allow an advanced level of structuring.

## Getting started

**Download [Inky, our ink editor](https://github.com/inkle/inky), and the follow either:**

- [The basics tutorial](https://www.inklestudios.com/ink/web-tutorial/) if you're non-technical and/or if you'd like to use ink to make a web-based interactive fiction game
- [The full tutorial](https://github.com/inkle/ink/blob/master/Documentation/WritingWithInk.md) if you want to see everything that ink has to offer.

## License

**ink** is released under the [MIT license](https://github.com/inkle/ink/blob/master/LICENSE.txt).
