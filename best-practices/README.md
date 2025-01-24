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
