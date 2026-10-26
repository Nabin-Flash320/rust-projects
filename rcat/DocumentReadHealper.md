# How to Read Rust Documentation

Rust docs (rustdoc-generated, like what you see on [docs.rs](https://docs.rs) or
`doc.rust-lang.org/std`) all follow the same layout. Once you know the layout,
every crate's docs become readable the same way. This guide walks through each
part.

---

## 1. Where docs come from

- **`doc.rust-lang.org/std`** — the standard library docs.
- **`docs.rs/<crate>`** — auto-generated docs for every crate published to
  crates.io. URL pattern: `docs.rs/<crate>/<version>/<crate>/`.
- **`cargo doc --open`** — generates and opens docs for *your own project and
  all its dependencies*, built locally from source. Useful when a crate isn't
  on docs.rs, or you're reading docs for a pinned/patched version.

All three are produced by the same tool, **rustdoc**, so the page structure is
identical everywhere.

### It's all just HTML

There's no magic backend behind any of this. `rustdoc` compiles your doc
comments into a **static site** — plain `.html` files, plus CSS/JS for search
and syntax highlighting. That's true whether it's your machine or docs.rs:

- Run `cargo doc` in your own project and look in `target/doc/` — you'll find
  an `index.html`, one HTML file per struct/enum/trait/function
  (`struct.Foo.html`, `trait.Bar.html`, ...), a `search-index.js` that powers
  the search bar, and a `src/` folder with syntax-highlighted copies of your
  actual source (that's what the `[src]` links point to).
- **docs.rs is just a server that runs `cargo doc` for every crate published
  to crates.io and hosts the resulting `target/doc/` folder for you.** When
  you visit `docs.rs/serde/latest/serde/`, you are looking at literal static
  HTML files, no different from what `cargo doc --open` puts on your own
  disk — same generator, same CSS, same JS search index.
- Because it's static HTML, you can open any generated page in "view source"
  in your browser, or just `cd target/doc && python3 -m http.server` and
  browse it manually without `--open`.

This is worth internalizing because it means: **your own project's docs look
and behave exactly like the official std docs or any crates.io crate's docs**
— same layout, same search, same sidebar — the moment you run `cargo doc`.

---

## 2. The crate root page

When you land on a crate's docs, you're on the **crate root** (`lib.rs`'s doc
comments). It usually contains:

- A prose overview (from `//!` comments at the top of `lib.rs`) — read this
  first, it often has usage examples.
- **Modules** — the crate's internal organization (`http`, `error`, `client`, ...).
- **Structs**, **Enums**, **Traits**, **Functions**, **Macros**, **Type
  Definitions** — top-level items re-exported at the crate root.

Rule of thumb: **start at the crate root, skim the module list, then drill
into the specific item you need** (usually via search, see §8).

---

## 3. Reading a `struct` page

A struct page has these sections, in order:

1. **Signature** — e.g. `pub struct HashMap<K, V, S = RandomState>`. This tells
   you the generic parameters and any defaults.
2. **Fields** (if public) — only shown if the struct's fields are `pub`. Many
   structs hide their fields and expose everything through methods instead —
   that's not a doc gap, it's deliberate encapsulation.
3. **Implementations** — this is the important part, split into subsections:
   - **`impl<K, V, S> HashMap<K, V, S>`** — inherent methods, i.e. methods
     defined directly on the type (`map.insert(...)`, `map.get(...)`). These
     are what you call directly.
   - **Trait Implementations** — every trait this type implements
     (`impl Debug for HashMap`, `impl Clone for HashMap`, `impl IntoIterator
     for HashMap`, ...). This tells you what you can *do* with the type
     generically: can you `{:?}` print it? Can you iterate it in a `for`
     loop? Can you clone it?
   - **Auto Trait Implementations** — compiler-derived markers like `Send`,
     `Sync`, `Unpin`, `UnwindSafe`. You rarely need these unless you're doing
     threading/async work and something won't compile because a type isn't
     `Send`.
   - **Blanket Implementations** — implementations that apply to *huge swaths
     of types automatically*, like `impl<T> From<T> for T` or `impl<T> Any
     for T where T: 'static`. If you see `impl<T: Clone> ToOwned for T`, that
     explains why almost everything has `.to_owned()` even without a direct
     impl.

**Tip:** Use the sidebar / "Implementations" jump links instead of scrolling —
big std types like `Vec` or `String` have dozens of trait impls.

---

## 4. Reading a method signature

This is usually where people get lost. Take:

```rust
pub fn get<Q>(&self, k: &Q) -> Option<&V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
```

Break it down piece by piece:

| Part | Meaning |
|---|---|
| `pub fn get<Q>` | public method, generic over a type `Q` chosen by the caller (often inferred) |
| `(&self, k: &Q)` | takes `self` by shared reference (doesn't consume/mutate the receiver), and a reference to a `Q` |
| `-> Option<&V>` | returns `Some(&V)` if found, `None` if not — a *reference* into the map, not an owned value |
| `where K: Borrow<Q>` | the map's key type `K` must be "borrowable as" `Q` — this is why `map.get("foo")` works even if `K = String` |
| `Q: Hash + Eq + ?Sized` | `Q` must be hashable and comparable; `?Sized` means it's allowed to be an unsized type like `str` |

**How to read `self` variants:**
- `self` — takes ownership, consumes the value (can't use it after).
- `&self` — borrows immutably, read-only access.
- `&mut self` — borrows mutably, can modify.
- no `self` at all — an **associated function**, called as `Type::function()`
  (e.g. `Vec::new()`), not `value.function()`.

**How to read generic bounds (`where` clauses / `<T: Trait>`):**
They're a *contract*, not documentation prose — they tell you exactly what
capabilities a type needs to be used here. If you get a compile error using
a generic function, compare the error against these bounds; that's usually
the mismatch.

**Lifetimes (`'a`)** in a signature like `fn get<'a>(&'a self) -> &'a str`
mean "the returned reference is only valid as long as `self` is." You don't
need to reason about lifetimes deeply just to *use* an API — just know that a
returned reference can't outlive the thing it borrowed from.

---

## 5. Reading a `trait` page

Trait pages look different from struct pages:

1. **Signature** — `pub trait Iterator { ... }`, possibly with supertraits:
   `pub trait Ord: Eq + PartialOrd`. Supertraits mean "you must also
   implement `Eq` and `PartialOrd` before you can implement `Ord`."
2. **Associated Types / Constants** — e.g. `type Item;` on `Iterator`. This is
   a placeholder type each implementor fills in (for `Vec<i32>::IntoIter`,
   `Item = i32`).
3. **Required methods** — methods with no default body; implementors *must*
   provide these. On `Iterator`, that's just `fn next(&mut self) -> Option<Self::Item>`.
4. **Provided methods** — methods with a default implementation, built on top
   of the required ones. This is why `Iterator` gives you `.map()`, `.filter()`,
   `.collect()`, etc. "for free" once you implement just `next()`.
5. **Implementors** — at the bottom, a (sometimes huge, collapsed) list of
   every type in this crate/std that implements the trait. Useful to see
   "what can I already use this trait's methods on?"

---

## 6. Enums

Enum pages show each **variant**, including any data each variant carries:

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

Then, just like structs, an **Implementations** section below with methods
(`.unwrap()`, `.map()`, `.is_some()`, ...) and **Trait Implementations**.
Pattern-match on the variants shown here (`match`, `if let`) — the doc page
is effectively listing every case you need to handle.

---

## 7. Functions and Macros

- **Free functions** (not attached to a type) appear in a "Functions" section
  at module level, e.g. `std::mem::swap`.
- **Macros** (`vec!`, `println!`) have their own page showing valid *call
  syntaxes* (rustdoc shows multiple `macro_rules!`-style patterns you can
  use) rather than a normal function signature.

---

## 8. Search & navigation

- The search bar (press `S`) matches by name, and also by **type signature**
  if you type something like `Vec<T> -> T`. This is powerful for "I know what
  types I have and want, I don't know the function name."
  - Rustdoc syntax: `-> ReturnType` searches by return type,
    `ArgType ->` searches by argument type.
- `?` opens a keyboard shortcut / help panel.
- Every item page has a **`[src]`** link near the signature — click it to jump
  to the actual source code. When docs are ambiguous, reading the
  implementation is often faster than guessing.
- The left sidebar lists sibling items in the same module — useful for
  browsing "what else is in here" without going back to the crate root.

---

## 9. Attributes and markers you'll see in docs

| Marker | Meaning |
|---|---|
| `#[non_exhaustive]` on a struct/enum | more fields/variants may be added later; you can't exhaustively `match` without a `_` arm, and can't construct the struct with a literal outside its own crate |
| `#[must_use]` | ignoring the return value is almost certainly a bug (e.g. `Result`, iterator adapters) — compiler will warn you |
| `#[deprecated]` | shown with a note and often a suggested replacement |
| "Available on crate feature `x` only" (docs.rs) | this item only exists if you enable that Cargo feature in your `Cargo.toml` |
| "Available on `unix` only" / target-specific banner | platform-gated item |
| `#[doc(hidden)]` | intentionally excluded from docs — usually internal/implementation detail not meant for public use, even if technically `pub` |

---

## 10. Doc comments in source (for when you write your own)

If you're reading a crate's *source* rather than rendered docs:

- `///` above an item — doc comment for that item (renders as its docs).
- `//!` at the top of a file/module — doc comment for the *enclosing* module
  or crate (this is the prose you see on the crate root page).
- Inside doc comments, ` ```rust ` code blocks are **doctests** — they're
  actually compiled and run by `cargo test`. If you see example code on a
  docs.rs page, it's guaranteed (in a well-maintained crate) to actually
  compile against that version.

---

## 11. A practical reading order

When you land on an unfamiliar type/function and feel lost, read in this
order:

1. **Signature line** — what does it take, what does it return.
2. **Prose paragraph** right below the signature — usually explains intent
   and edge cases (panics, errors, when `None`/`Err` happens).
3. **"Examples" section** — almost every well-documented std/crate item has
   one; this is often faster to understand than the prose.
4. **"Panics" / "Errors" / "Safety"** sections if present — these are load-bearing:
   - **Panics** — conditions that will crash the program (call `.unwrap()`
     without checking, this is what will bite you).
   - **Errors** — what each `Err` variant means for `Result`-returning
     functions.
   - **Safety** — only on `unsafe fn`; lists the invariants *you* must
     uphold that the compiler can't check for you. Never skip this one.
5. Only then dig into generic bounds / lifetimes if something isn't compiling.

---

## 12. Quick glossary

- **Inherent impl** — methods defined directly on a type (`impl Foo { fn bar(&self) }`).
- **Trait impl** — methods a type gets *because* it implements a trait
  (`impl Display for Foo`).
- **Blanket impl** — a trait impl that applies to a whole category of types at
  once via a generic bound, not one specific type.
- **Supertrait** — a trait that another trait requires as a prerequisite.
- **Associated type** — a type chosen by the implementor, referenced as
  `Self::TypeName` in the trait definition.
- **Marker trait** — a trait with no methods, used purely as a compile-time
  tag (`Send`, `Sync`, `Copy`).
- **Turbofish (`::<>`)** — you'll see this in examples when type inference
  needs a hint: `"42".parse::<i32>()`.

---

## 13. Writing docs for your own project

Everything above is about *reading* docs. Producing the same kind of page for
your own crate just means writing specially-formatted comments — rustdoc
picks them up automatically, no extra tooling needed.

### The two comment types

```rust
//! # My Crate
//!
//! This is module/crate-level documentation. Put this at the very top of
//! `lib.rs` (or `main.rs`, or the top of any module file) — it becomes the
//! prose you see on the crate root / module index page.

/// Adds one to the given number.
///
/// This is item-level documentation. It goes directly above the item it
/// documents (a function, struct, enum, trait, field, etc.) and becomes
/// that item's page.
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

- `///` (triple slash) — docs the item **immediately below** it. Use for
  functions, structs, enums, traits, methods, fields, constants.
- `//!` (slash-bang) — docs the item that **contains** the comment (the
  enclosing module or the whole crate if placed at the top of `lib.rs`). Use
  this for module-level and crate-level overviews.
- Both support full Markdown: headings (`#`), code fences, links
  (`` [`Foo`] `` auto-links to the item named `Foo` in scope — this is called
  **intra-doc linking**), bullet lists, tables, etc.

### The sections rustdoc conventionally expects

There's no enforced schema, but the ecosystem (and clippy) expects these
headings, by name, in this order, when relevant:

```rust
/// Divides `a` by `b`.
///
/// # Examples
///
/// ```
/// assert_eq!(my_crate::divide(10, 2), 5);
/// ```
///
/// # Panics
///
/// Panics if `b` is zero.
///
/// # Errors
///
/// Returns `Err` if the division would overflow.
///
/// # Safety
///
/// (Only needed on `unsafe fn`.) Caller must ensure ...
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}
```

- **`# Examples`** — code inside ` ```rust ` fences here is a **doctest**: it
  actually gets compiled and *executed* when you run `cargo test`. If your
  example goes stale (API changes), your test suite will fail — this is how
  std keeps its examples honest. Use ` ```rust,no_run ` if the code should
  compile but not run (e.g. it starts a server), or ` ```rust,ignore ` to
  skip it entirely.
- **`# Panics`** — document any condition under which the function panics.
- **`# Errors`** — for functions returning `Result`, explain what each error
  case means.
- **`# Safety`** — required (clippy will lint you) on any `unsafe fn`; state
  the invariants the caller must uphold.

### Generating and checking your docs

| Command | What it does |
|---|---|
| `cargo doc` | builds HTML docs into `target/doc/` |
| `cargo doc --open` | builds and opens them in your browser |
| `cargo doc --no-deps` | only build docs for *your* crate, skip all dependencies (much faster, good for a quick check) |
| `cargo test` | also runs every doctest in your `///`/`//!` comments |
| `cargo doc --document-private-items` | include private (non-`pub`) items too, useful for reviewing internal docs |

### Making rustdoc nag you about missing docs

Add this near the top of `lib.rs` to turn missing documentation into a
compiler warning (or error, with `deny`) — handy for a library you intend to
publish:

```rust
#![warn(missing_docs)]
// or, stricter:
#![deny(missing_docs)]
```

### Pulling in your README

Instead of duplicating your crate overview, you can embed your `README.md`
directly as the crate-level docs:

```rust
#![doc = include_str!("../README.md")]
```

### Publishing

If your crate is published to crates.io (`cargo publish`), docs.rs
automatically builds and hosts its docs — you don't do anything extra beyond
writing the comments above and publishing. Locally, `cargo doc --open` is the
fast feedback loop while you're still writing them.
