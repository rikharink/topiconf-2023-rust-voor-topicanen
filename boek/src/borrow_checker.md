# Voorbeeld 1: de borrow checker

Onze eerste poging voor de telhulp voor Graaf Tel is het volgende:

```rust,compile_fail
{{#include ./examples/count_1.rs}}
```

Na het compileren zien we dat we een fout hebben gemaakt.
In Rust zijn dingen standaard niet mutable, je kunt ze dus niet aanpassen. Om dit op te lossen maken we de referentie mutable zodat we wel kunnen optellen! Precies wat de compile fout ons ook verteld: `help: consider changing this to be a mutable reference`

Poging 2:

```rust, compile_fail
{{#include ./examples/count_2.rs}}
```

Poging 3:

```rust
{{#include ./examples/count_3.rs}}
```
