# Substrate Product Registry pallet

The Product Registry pallet provides functionality for registering and managing master data (aka class-level) about products / trade items exchanged in a supply chain between various stakeholders. This data is typically registered once by the product's manufacturer / supplier to be shared with other network participants.

When this pallet is added to a Subtrate runtime, other custom Substrate pallets can then implement additional business logic leveraging this Product Registry pallet as a reference for known products and their owning organizations.

This pallet is part of the [Substrate Enterprise sample](https://github.com/gautamdhameja/substrate-enterprise-sample).

NOTE: This pallet is intended for demonstration purposes and is not audited or ready for production use.


## Dependencies

### Traits

This pallet does not depend on any externally defined traits.

### Pallets

This pallet depends on on the [FRAME Timestamp pallet](https://docs.rs/crate/pallet-timestamp).

## Testing

Run the tests with:

    ```
    cargo test
    ```

## Installation

### Runtime `Cargo.toml`

To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.product-registry]
package = 'pallet-product-registry'
default_features = false
git = 'https://github.com/stiiifff/pallet-product-registry.git'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'product-registry/std',
]
```

### Runtime `lib.rs`

You should implement it's trait like so:

```rust
/// Used for test_module
impl product_registry::Trait for Runtime {
	type Event = Event;
}
```

and include it in your `construct_runtime!` macro:

```rust
ProductRegistry: product_registry::{Module, Call, Storage, Event<T>},
```

### Genesis Configuration

This template pallet does not have any genesis configuration.

## Reference Docs

You can view the reference docs for this pallet by running:

```
cargo doc --open
```
