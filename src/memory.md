## Ownership

Ownership is the cornerstone of Rust’s memory management. Every value in Rust has an owner, and the owner is responsible for deallocating the value when it is no longer needed.

- **Owner**: The variable or data structure that owns the value.
- **Scope**: The region of code where the owner is valid.
- **Move**: Transferring ownership of a value to another variable.
- **Borrow**: Using a value without taking ownership.
- **Lifetime** : The scope for which a reference is valid.

## Borrowing

Rust allows borrowing values, which means using them without taking ownership. Borrowing can be immutable (shared) or mutable (exclusive).

- **Immutable Borrow**: Multiple immutable references can exist at the same time.
- **Mutable Borrow**: Only one mutable reference can exist at a time.

## Stack vs Heap

- **Stack**: Memory allocated at compile time. Values are stored in a Last-In-First-Out (LIFO) order. Faster access but size is fixed.
- **Heap**: Memory allocated at runtime. Values are stored in a more flexible manner but with higher overhead. Must be manually managed using pointers or smart pointers.

## Smart Pointers

Smart pointers are abstractions over raw pointers that provide automatic memory management and safety features.

- **Box**: A smart pointer that owns the data it points to and deallocates it when dropped.
- **Rc** (Reference Counting): A smart pointer that allows multiple owners of the same data using reference counting.
- **Arc** (Atomic Reference Counting): A thread-safe version of Rc.

## How It Works Under the Hood

Rust’s compiler enforces memory safety through the following mechanisms:

- **Ownership System**: Ensures that each value has a single owner.
- **Borrow Checker**: Ensures that references are valid and follow the rules of borrowing.
- **Lifetime System**: Ensures that references are valid for the correct scope.

Best Practices and Common Pitfalls

- Avoid using unsafe code unless necessary.
- Use smart pointers instead of raw pointers.
- Understand lifetimes to avoid dangling pointers.
- Avoid mutable borrows to prevent data races.
