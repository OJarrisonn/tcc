# Patch Hub

`patch-hub` is a text user interface (TUI) application that is ran on the terminal.
It's a helper for kernel developers to view patches using [lore](https://lore.kernel.org).
Currently, it's a mostly single-threaded application written in Rust. Due to Rust
rigorous type system and _don't pay for what you don't use_ philosophy, `patch-hub`
,as is, is hitting the flexibility limits. Which means that huge refactorings
are needed in order to create new features.

The best example is the popup system. Currently, popups are limited to only displaying
data. They sadly can't modify data due to some architectural decisions and Rust
type system restrictions.

In Rust, every value must have one and only owner. There's an `app` variable that
holds both the configurations and the popups to be displayed, together with the
application state information. To modify the app, we need to take a `&mut`
(mutable reference) to it. But for a popup to modify data in the app it needs
this `&mut` (remember that the popup is owned by the app). There's a tricky rule
in Rust that prohibits you to have two simultaneous `&mut` to the same piece of data
or even have `&` (immutable references) and `&mut` at once. This means that we cannot
Take the popups from the app and at the same time pass them a `&mut` to the app.

Those Rust rules might seem restrictive, but that's what makes Rust safe. Rust
won't absolutely prohibits you from doing modifications from values. But you need
to explicitly use safe abstractions to do so.

## Modules

Currently `patch-hub` is broken up into the following modules:

- `lore`: interface to interact with the lore API
- `ui`: draw to the terminal
- `cli`: CLI arg parse for patch-hub binary
- `app`: manages patch-hub data and state
- `handler`: handle user input

## Actors

Not every module translates directly to an actor, since actors should be responsible
or one thing and one thing only. So this is a rough idea of the actors that we
would build.

- **Lore**(`lore`): interface to interact with the lore API
- **View**(`ui`): draws to the terminal
- **App**(`app`): manages app state
- **Config**(`app::config`): manages app configuration
- **Controller**(`handler`): handles user input
- **File**(spread across different locations): manages file IO
- **Logger**(`app::logging`): logs messages

Also, keep in mind that we won't stick to the "everything is an actor" philosophy.
Some pieces of the `patch-hub` code will be just reorganized in different folders
but still be just regular library structs, functions, etc. outside of the actor model.
