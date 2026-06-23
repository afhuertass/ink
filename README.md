# Narrative Framework

A declarative framework for story-driven and text games.

This repository is a **fork of [inkle/ink](https://github.com/inkle/ink)**, extended with a structured directive system and a rewritten Rust compiler. The code is ABSOLUTELY AI DRIVEN AT THIS POINT. can't really speak of the quality of usefulness at the moment. I'm running this as an experiment to test ideas and so on. I hope in the future I can polish and improve the code and architecture.

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
| 3     | Directives (`@` prefix) + validation       | ✅ Complete |
| 4     | LSP with validation + autocompletion       | ✅ Complete |
| 5     | Godot SDK (Scaffolded & Ready)             | ✅ Complete |

---

# Original Ink Documentation

[Ink](http://www.inklestudios.com/ink) is [inkle](http://www.inklestudios.com/)'s scripting language for writing interactive narrative, both for text-centric games as well as more graphical games that contain highly branching stories. It's designed to be easy to learn, but with powerful enough features to allow an advanced level of structuring.

## Getting started

**Download [Inky, our ink editor](https://github.com/inkle/inky), and the follow either:**

- [The basics tutorial](https://www.inklestudios.com/ink/web-tutorial/) if you're non-technical and/or if you'd like to use ink to make a web-based interactive fiction game
- [The full tutorial](https://github.com/inkle/ink/blob/master/Documentation/WritingWithInk.md) if you want to see everything that ink has to offer.

## License

**ink** is released under the [MIT license](https://github.com/inkle/ink/blob/master/LICENSE.txt).
