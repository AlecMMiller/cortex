# Cortex - a local-first personal knowledge management app

[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/AlecMMiller/cortex/badge)](https://scorecard.dev/viewer/?uri=github.com/AlecMMiller/cortex)
![build](https://github.com/alecmmiller/cortex/workflows/release/badge.svg?branch=main)
![CodeQL](https://github.com/alecmmiller/cortex/workflows/CodeQL/badge.svg?branch=main)

## What is Cortex

_This project is currently under heavy development and is not yet stable_

Cortex aims to become an app designed to store knowledge in many forms. The primary mode of navigation is via WikiLinks style links between notes. Rather tthan using a traditional tree hierarchy, Cortex organizes notes via tags, which can be children of other tags. For example, in a traditional tree structure, you might have `movies/sci-fi/Star Wars/A New Hope`. But you might also want to categorize it under `archetypes/Hero's Journey/A New Hope`. In a traditional file tree, you have to pick only a single organizational system.

Tags solve this by creating multiple categories that a single item can be a part of, but traditionally tags must be applied manually. However, if the tag `sci-fi movie` is a child of tag `movie`, and `Star Wars` is a child of tag `sci-fi movie`, by applying the tag `Star Wars` to the note A New Hope, it is automatically tagged as `sci-fi movie` and `movie` as well. This also means that it will show up in a list of all items tagged with `movie` or `sci-fi movie`.

With future development, calendar functionality, additional database-based functionality, and free-form canvas content is intended to be included as well.

## Architecture

Cortex is developed using the [Tauri Framework](https://github.com/tauri-apps/tauri) with a Rust backend and a React frontend. The primary editor is based on Meta's [Lexical](https://github.com/facebook/lexical) editor. The primary backend is SQLite with the [Diesel ORM](https://github.com/diesel-rs/diesel). Text content is also indexed with [Tantivy](https://github.com/quickwit-oss/tantivy) for more advanced text searching.

## Languages and Accessibility

Cortex is currently localized in English and (somehwat poor) Japanese. Pull requests for additional languages are welcome.

Effort has been made to ensure Cortex is designed for accesibility, if there are any accesibility issues, please submit an issue so they can be addressed.

## License

Cortex is licensed under the MIT license. Some components, such as fonts, may be licensed under different terms.
