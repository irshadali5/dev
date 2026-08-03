---
title: "When not to use anyhow"
description: ""
---

libs-vs-bins" class="section">

<div class="breadcrumb">

Advanced › Libs vs binaries



# When not to use anyhow

anyhow belongs in *application* code, not *library* code.

## The problem with anyhow in libraries

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
// If your library returns anyhow::Error, callers CAN'T do this:
match your_lib::do_thing() {
    Ok(_) => {},
    Err(e) if e.is::<NetworkError>() => { /* retry */ },
    Err(e) => return Err(e),
}
// Callers have to use downcast_ref, which is fragile and unexpected.
```

## The right split

| Code type | Error approach | Why |
|----|----|----|
| Library crate (lib.rs) | Custom enum + thiserror | Callers need to match on variants |
| Application crate (main.rs, handlers) | anyhow::Result | Convenience, no callers above you |
| Internal module (not public API) | Either, prefer anyhow | Convenience is fine internally |



> [!TIP] > **Rule** > If the function is
`pub` in a library crate — use a typed error. If
it's in a binary or an internal function — use anyhow. When in doubt:
"does the caller need to match on the error variant?" If yes → typed. If
no → anyhow.



Next: Real-world patterns →