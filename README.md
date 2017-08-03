# `scrapmetal`: Scrap Your Rust Boilerplate

[![Build Status](https://travis-ci.org/fitzgen/scrapmetal.png?branch=master)](https://travis-ci.org/fitzgen/scrapmetal) [![scrapmetal on crates.io](https://img.shields.io/crates/v/scrapmetal.svg)](https://crates.io/crates/scrapmetal) [![scrapmetal on docs.rs](https://docs.rs/scrapmetal/badge.svg)](https://docs.rs/scrapmetal/)

Generic transformations, queries, and mutations for Rust without the
boilerplate.

A port of some of the ideas and code
from
["Scrap Your Boilerplate: A Practical Design Pattern for Generic Programming" by Lämmel and Peyton Jones](https://www.microsoft.com/en-us/research/wp-content/uploads/2003/01/hmap.pdf) to
Rust.

⚠ Depends on the specialization nightly Rust feature. ⚠

--------------------------------------------------------------------------------

Say we work on some software that models companies, their departments,
sub-departments, employees, and salaries. We might have some type definitions
similar to this:

```rust
pub struct Company(pub Vec<Department>);

pub struct Department(pub Name, pub Manager, pub Vec<SubUnit>);

pub enum SubUnit {
    Person(Employee),
    Department(Box<Department>),
}

pub struct Employee(pub Person, pub Salary);

pub struct Person(pub Name, pub Address);

pub struct Salary(pub f64);

pub type Manager = Employee;
pub type Name = &'static str;
pub type Address = &'static str;
```

One of our companies has had a morale problem lately, and we want to transform
it into a new company where everyone is excited to come in every Monday through
Friday morning. But we can't really change the nature of the work, so we figure
we can just give the whole company a 10% raise and call it close enough. This
requires writing a bunch of functions with type signatures like `fn(self, k:
f64) -> Self` for every type that makes up a `Company`, and since we recognize
the pattern, we should be good Rustaceans and formalize it with a trait:

```rust
pub trait Increase: Sized {
    fn increase(self, k: f64) -> Self;
}
```

A company with increased employee salaries is made by increasing the salaries of
each of its departments' employees:

```rust
impl Increase for Company {
    fn increase(self, k: f64) -> Company {
        Company(
            self.0
                .into_iter()
                .map(|d| d.increase(k))
                .collect()
        )
    }
}
```

A department with increased employee salaries is made by increasing its
manager's salary and the salary of every employee in its sub-units:

```rust
impl Increase for Department {
    fn increase(self, k: f64) -> Department {
        Department(
            self.0,
            self.1.increase(k),
            self.2
                .into_iter()
                .map(|s| s.increase(k))
                .collect(),
        )
    }
}
```

A sub-unit is either a single employee or a sub-department, so either increase
the employee's salary, or increase the salaries of all the people in the
sub-department respectively:

```rust
impl Increase for SubUnit {
    fn increase(self, k: f64) -> SubUnit {
        match self {
            SubUnit::Person(e) => {
                SubUnit::Person(e.increase(k))
            }
            SubUnit::Department(d) => {
                SubUnit::Department(Box::new(d.increase(k)))
            }
        }
    }
}
```

An employee with an increased salary, is that same employee with the salary
increased:

```rust
impl Increase for Employee {
    fn increase(self, k: f64) -> Employee {
        Employee(self.0, self.1.increase(k))
    }
}
```

And finally, a lone salary can be increased:

```rust
impl Increase for Salary {
    fn increase(self, k: f64) -> Salary {
        Salary(self.0 * (1.0 + k))
    }
}
```

Pretty straightforward.

But at the same time, that's a *whole* lot of boilerplate. The only interesting
part that has anything to do with actually increasing salaries is the `impl
Increase for Salary`. The rest of the code is just traversal of the data
structures. If we were to write a function to rename all the employees in a
company, most of this code would remain the same. Surely there's a way to factor
all this boilerplate out so we don't have to manually write it all the time?

Enter `scrapmetal`:

```rust
// Imports
#[macro_use]
extern crate scrapmetal_derive;
extern crate scrapmetal;
use scrapmetal::{Everywhere, Transformation};

// Add derive(Term) to type definitions
#[derive(Term)]
pub struct Company(pub Vec<Department>);
// Etc...

// Define the `increase` transformation
let increase = |s: Salary| Salary(s.0 * 1.1);
let mut increase = Everywhere::new(Transformation::new(increase));

// Use the `increase` transformation
let new_company = increase.transform(old_company);
```

Nothing more required!
