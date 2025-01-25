## Favor Iterators

Iterators Key benefits:

- Lazy evaluation: Operations are performed only when needed
- Chain operations efficiently: `filter().map().collect()`
- Compiler optimizes to machine code as efficient as hand-written loops
- More expressive and composable code
- Thread safety through immutable access

## Minimizing cloning

Here's how to minimize cloning in Rust:
```rust
// 1. Use references when possible
let data = vec![1, 2, 3];
// Bad: data.clone()
// Good: &data

// 2. Use Cow for conditional cloning
use std::borrow::Cow;
fn process(input: &str) -> Cow<str> {
    if input.is_empty() {
        Cow::Borrowed(input)    // No clone needed
    } else {
        Cow::Owned(input.to_uppercase())  // Clone only when needed
    }
}

// 3. Use into_iter() for ownership transfer
let strings = vec!["hello".to_string()];
// Bad: strings.iter().cloned()
// Good: strings.into_iter()

// 4. Use String::from instead of to_string()
// Bad: "hello".to_string()
// Good: String::from("hello")

// 5. Implement Clone judiciously
#[derive(Clone)]  // Only when necessary
struct Data {
    field: String,
}
```

## Do not emulate OOP

```rust
struct Employee;
impl Name for Employee {}
impl Address for Employee {}
impl Salary for Employee {}
```

Instead, favor composition, at Employee level can gain mutable access toi each of the properties.

```rust
struct Name;
struct Address;
struct Salary;

struct Employee {
    name: Option<Name>,
    address: Option<Address>,
    salary: Option<Salary>,
}
```

## Think in terms of Ownership

On top of that, it's beneficial to think in terms of ownership from Rust's perspective. If you have a big list of
employees, and want to transfer a red swingline stapler from one person to another - you need to find both people, find
out if they have the stapler, and then move it. In a textbook OOP setup, you'd have methods at each level - and probably
pass one person to another for the transfer. Rust will be much happier if you implement the operation at the top level,
take the time to check each precondition (and return an appropriate error).

## Platform Specific

```rust
#[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
mod native;
pub use native::*;

#[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
mod wasm;
#[cfg(all(feature = "opengl", target_arch = "wasm32"))]
#[cfg(all(feature = "opengl", target_arch = "wasm32"))]
pub use wasm::*;
```

Now when you compile, it only includes the appropriate module and shares the common type defined in each of the modules.
That's a great way to share functionality between platform-specific implementations (which can be managed by different
teams, even) without resorting to dynamic dispatch.

## TANSTAAFL (there ain't no such as a free lunch)

Means every technical choice has tradeoffs. "Free" solutions often hide costs in maintenance, performance, or technical
debt. For example, using third-party libraries saves development time but adds dependencies and security risks.
Similarly, quick fixes may solve immediate problems but complicate future changes. Understanding these hidden costs is
crucial for making informed architectural decisions.

## YAGNI (you ain't gonna need it)

YAGNI is a software development principle advocating against adding functionality until it's actually needed. It warns
against over-engineering based on speculation about future requirements. This principle helps maintain simpler, more
maintainable code by focusing only on current, proven needs rather than anticipated ones.

## Domain boundaries

Domain boundaries in software architecture represent explicit borders between different business domains or bounded
contexts within a system. They help manage complexity and maintain separation of concerns.

Key aspects:

- Define clear interfaces and contracts between domains
- Prevent unwanted coupling between different parts of the system
- Each domain has its own data models, business rules, and vocabulary
- Communication between domains happens through well-defined interfaces

Benefits:

- Better maintainability through isolation of changes
- Easier to understand and modify individual domains
- Enables parallel development by different teams
- Facilitates migration and evolution of different parts of the system

Implementation involves:

- Identifying domain events and commands that cross boundaries
- Using anti-corruption layers to translate between domain models
- Defining clear contracts for inter-domain communication
- Maintaining separate data stores where appropriate

For example, in an e-commerce system, Order Management and Inventory Management would be separate domains, communicating
through specific interfaces while maintaining their own internal models and rules.
