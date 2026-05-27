Good work overall — it runs, it's structured, and your questions show you're thinking about the right things. Let me go through each question and then add a few observations.

---

**"Too many `pub`"**

Yes. In a binary crate (not a library), nothing is exported outside the binary itself, so `pub` on structs and fields within modules is just noise. You only need `pub` to make something visible *across modules* — `cli::Args` needs to be `pub` so `main.rs` can use it, but the fields don't need to be `pub` because you access them inside `service.rs` via the struct directly. Clean it up when you get a chance.

---

**"What data structure to hold params"**

Passing `Args` directly is fine for a project this size. The only reason to create a separate params struct is if your CLI args and your business logic params start to diverge — e.g. CLI takes a string `--region europe` but your logic wants a `Region` enum. Not worth it here.

---

**"`Some(ref region)` vs `as_deref()`"**

Both work, but your current approach can be simplified. The `ref` is needed because you can't move out of `args.region` inside the closure. `as_deref()` sidesteps this cleanly:

```rust
.filter(|x| {
    args.region.as_deref()
        .map_or(true, |r| x.region == r)
})
```

`as_deref()` converts `Option<String>` to `Option<&str>`, then `map_or(true, ...)` returns `true` if it's `None`, otherwise runs the comparison. More idiomatic.

---

**"Should I unwrap"**

`.expect()` in `main` is fine for a CLI — it crashes with a message, which is acceptable. You're already using `anyhow::Result` in `get_results` and propagating with `?`, which is the right pattern. The only thing I'd change is the `serde_json::to_string_pretty(...).unwrap()` in main — that could also use `?` if main returns `anyhow::Result<()>`:

```rust
fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let xs = service::get_results(args)?;
    println!("{}", serde_json::to_string_pretty(&xs)?);
    Ok(())
}
```

---

**"Why do I need Eq, Ord, PartialEq, PartialOrd on Name"**

You derived them because you do `a.name.cmp(&b.name)` in the sort, which requires `Ord` on `Name`. Since `Name` only contains `String` fields, all four are safe to derive. But you could avoid needing any of them by just comparing the field directly:

```rust
cli::SortBy::Name => xs.sort_by(|a, b| a.name.common.cmp(&b.name.common)),
```

`String` already implements `Ord`, so this works without touching `Name` at all. Worth thinking about whether `Name` really needs ordering as a concept, or whether you just need to sort by one of its fields.

---

**"Is my return type OK"**

Yes, `anyhow::Result<Vec<Item>>` is exactly right.

---

**"`iter()` vs `into_iter()`"**

- `iter()` — borrows the collection, gives you `&T`, collection still usable after
- `into_iter()` — consumes the collection, gives you `T`, collection is gone after

You correctly use `into_iter()` here since you want to consume `xs` and transform the items, not just read them.

---

**"Is this really more readable"**

The iterator chain is good. The filter closure is the one rough spot — the `if let` inside it is a bit heavy. With `as_deref()` it gets cleaner (see above). The rest is solid.

---

**Two things you didn't ask about but should fix:**

1. `use serde_json::json;` in `main.rs` is unused — remove it.

2. The requirements asked you to implement `Display` on `Item` and use `println!("{}", item)` — right now you're using `serde_json::to_string_pretty` which is a shortcut. Implementing `Display` is worth doing since it's one of the main traits exercises here.

---

Overall this is clean for a first pass. The main next step is implementing `Display` on `Item` and then tackling the stretch goals — the `--language` filter will force you to use `filter_map` or `flat_map` on the nested languages map, which is where iterators get interesting.
