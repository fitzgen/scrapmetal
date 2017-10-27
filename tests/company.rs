extern crate scrapmetal;

#[macro_use]
extern crate scrapmetal_derive;

use scrapmetal::*;
use std::cmp;
use std::collections::LinkedList;

// Some toy types to test and bench with, taken from the paper.

#[derive(Clone, Debug, PartialEq, Term)]
pub struct Company(pub Vec<Department>);

#[derive(Clone, Debug, PartialEq, Term)]
pub struct Department(pub Name, pub Manager, pub Vec<SubUnit>);

#[derive(Clone, Debug, PartialEq, Term)]
pub enum SubUnit {
    Person(Employee),
    Department(Box<Department>),
    Group(LinkedList<Employee>),
}

#[derive(Clone, Debug, PartialEq, Term)]
pub struct Employee(pub Person, pub Salary);

#[derive(Clone, Debug, PartialEq, Term)]
pub struct Person(pub Name, pub Address);

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Term)]
pub struct Salary(pub f64);

pub type Manager = Employee;
pub type Name = &'static str;
pub type Address = &'static str;

impl Default for Company {
    fn default() -> Company {
        let ralf = Employee(Person("Ralf", "Amsterdam"), Salary(8000.0));
        let joost = Employee(Person("Joost", "Amsterdam"), Salary(1000.0));
        let marlow = Employee(Person("Marlow", "Cambridge"), Salary(2000.0));
        let blair = Employee(Person("Blair", "London"), Salary(100000.0));
        let jim = Employee(Person("Jim", "Portland"), Salary(3.0));
        let mut group = LinkedList::new();
        group.push_front(Employee(Person("Joe", "Vancouver"), Salary(22.2)));
        group.push_front(Employee(Person("Mike", "Toronto"), Salary(33.3)));
        Company(vec![
            Department(
                "Research",
                ralf,
                vec![
                    SubUnit::Group(group),
                    SubUnit::Person(joost),
                    SubUnit::Person(marlow),
                    SubUnit::Department(Box::new(Department("Funsies", jim, vec![]))),
                ],
            ),
            Department("Strategy", blair, vec![]),
        ])
    }
}

impl cmp::Eq for Salary {}

impl cmp::Ord for Salary {
    fn cmp(&self, rhs: &Salary) -> cmp::Ordering {
        assert!(!self.0.is_nan());
        assert!(!rhs.0.is_nan());

        if self.0 < rhs.0 {
            cmp::Ordering::Less
        } else if self.0 > rhs.0 {
            cmp::Ordering::Greater
        } else {
            cmp::Ordering::Equal
        }
    }
}

// Boilerplate version of `increase` ///////////////////////////////////////////

pub trait Increase: Sized {
    fn increase(self, k: f64) -> Self;
}

impl Increase for Company {
    fn increase(self, k: f64) -> Company {
        Company(self.0.into_iter().map(|d| d.increase(k)).collect())
    }
}

impl Increase for Department {
    fn increase(self, k: f64) -> Department {
        Department(
            self.0,
            self.1.increase(k),
            self.2.into_iter().map(|s| s.increase(k)).collect(),
        )
    }
}

impl Increase for SubUnit {
    fn increase(self, k: f64) -> SubUnit {
        match self {
            SubUnit::Group(g) => SubUnit::Group(g.into_iter().map(|e| e.increase(k)).collect()),
            SubUnit::Person(e) => SubUnit::Person(e.increase(k)),
            SubUnit::Department(d) => SubUnit::Department(Box::new(d.increase(k))),
        }
    }
}

impl Increase for Employee {
    fn increase(self, k: f64) -> Employee {
        Employee(self.0, self.1.increase(k))
    }
}

impl Increase for Salary {
    fn increase(self, k: f64) -> Salary {
        Salary(self.0 + k)
    }
}

// Boilerplate version of `increase_in_place` //////////////////////////////////

pub trait IncreaseInPlace {
    fn increase_in_place(&mut self, k: f64);
}

impl IncreaseInPlace for Company {
    fn increase_in_place(&mut self, k: f64) {
        self.0.iter_mut().map(|d| d.increase_in_place(k)).count();
    }
}

impl IncreaseInPlace for Department {
    fn increase_in_place(&mut self, k: f64) {
        self.1.increase_in_place(k);
        self.2.iter_mut().map(|s| s.increase_in_place(k)).count();
    }
}

impl IncreaseInPlace for SubUnit {
    fn increase_in_place(&mut self, k: f64) {
        match *self {
            SubUnit::Person(ref mut e) => e.increase_in_place(k),
            SubUnit::Group(ref mut g) => g.into_iter().for_each(|e| e.increase_in_place(k)),
            SubUnit::Department(ref mut d) => d.increase_in_place(k),
        }
    }
}

impl IncreaseInPlace for Employee {
    fn increase_in_place(&mut self, k: f64) {
        self.1.increase_in_place(k);
    }
}

impl IncreaseInPlace for Salary {
    fn increase_in_place(&mut self, k: f64) {
        self.0 += k;
    }
}

// Boilerplate version of `highest_salary` /////////////////////////////////////

pub trait HighestSalary {
    fn highest_salary(&self) -> Option<Salary>;
}

impl HighestSalary for Company {
    fn highest_salary(&self) -> Option<Salary> {
        self.0
            .iter()
            .map(|d| d.highest_salary())
            .fold(None, cmp::max)
    }
}

impl HighestSalary for Department {
    fn highest_salary(&self) -> Option<Salary> {
        let mgr_salary = self.1.highest_salary();
        let units_highest = self.2
            .iter()
            .map(|u| u.highest_salary())
            .fold(None, cmp::max);
        cmp::max(mgr_salary, units_highest)
    }
}

impl HighestSalary for SubUnit {
    fn highest_salary(&self) -> Option<Salary> {
        match *self {
            SubUnit::Person(ref e) => e.highest_salary(),
            SubUnit::Group(ref g) => g.into_iter().map(|e| e.highest_salary()).max().unwrap(),
            SubUnit::Department(ref d) => d.highest_salary(),
        }
    }
}

impl HighestSalary for Employee {
    fn highest_salary(&self) -> Option<Salary> {
        Some(self.1.clone())
    }
}

// Tests ///////////////////////////////////////////////////////////////////////

#[test]
fn increase_with_boilerplate() {
    let company = Company::default();
    let company = company.increase(1.0);
    let mut group = LinkedList::new();
    group.push_front(Employee(Person("Joe", "Vancouver"), Salary(23.2)));
    group.push_front(Employee(Person("Mike", "Toronto"), Salary(34.3)));
    assert_eq!(
        company,
        Company(vec![
            Department(
                "Research",
                Employee(Person("Ralf", "Amsterdam"), Salary(8001.0)),
                vec![
                    SubUnit::Group(group),
                    SubUnit::Person(Employee(Person("Joost", "Amsterdam"), Salary(1001.0))),
                    SubUnit::Person(Employee(Person("Marlow", "Cambridge"), Salary(2001.0))),
                    SubUnit::Department(Box::new(Department(
                        "Funsies",
                        Employee(Person("Jim", "Portland"), Salary(4.0)),
                        vec![],
                    ))),
                ],
            ),
            Department(
                "Strategy",
                Employee(Person("Blair", "London"), Salary(100001.0)),
                vec![],
            ),
        ])
    );
}

#[test]
fn increase_scrapping_boilerplate() {
    let transformation = Transformation::new(|s: Salary| Salary(s.0 + 1.0));
    let mut increase = Everywhere::new(transformation);
    let mut group = LinkedList::new();
    group.push_front(Employee(Person("Joe", "Vancouver"), Salary(23.2)));
    group.push_front(Employee(Person("Mike", "Toronto"), Salary(34.3)));
    let company = Company::default();
    let company = increase.transform(company);
    assert_eq!(
        company,
        Company(vec![
            Department(
                "Research",
                Employee(Person("Ralf", "Amsterdam"), Salary(8001.0)),
                vec![
                    SubUnit::Group(group),
                    SubUnit::Person(Employee(Person("Joost", "Amsterdam"), Salary(1001.0))),
                    SubUnit::Person(Employee(Person("Marlow", "Cambridge"), Salary(2001.0))),
                    SubUnit::Department(Box::new(Department(
                        "Funsies",
                        Employee(Person("Jim", "Portland"), Salary(4.0)),
                        vec![],
                    ))),
                ],
            ),
            Department(
                "Strategy",
                Employee(Person("Blair", "London"), Salary(100001.0)),
                vec![],
            ),
        ])
    );
}

#[test]
fn increase_in_place_with_boilerplate() {
    let mut company = Company::default();
    let mut group = LinkedList::new();
    group.push_front(Employee(Person("Joe", "Vancouver"), Salary(23.2)));
    group.push_front(Employee(Person("Mike", "Toronto"), Salary(34.3)));
    company.increase_in_place(1.0);
    assert_eq!(
        company,
        Company(vec![
            Department(
                "Research",
                Employee(Person("Ralf", "Amsterdam"), Salary(8001.0)),
                vec![
                    SubUnit::Group(group),
                    SubUnit::Person(Employee(Person("Joost", "Amsterdam"), Salary(1001.0))),
                    SubUnit::Person(Employee(Person("Marlow", "Cambridge"), Salary(2001.0))),
                    SubUnit::Department(Box::new(Department(
                        "Funsies",
                        Employee(Person("Jim", "Portland"), Salary(4.0)),
                        vec![],
                    ))),
                ],
            ),
            Department(
                "Strategy",
                Employee(Person("Blair", "London"), Salary(100001.0)),
                vec![],
            ),
        ])
    );
}

#[test]
fn increase_in_place_scrapping_boilerplate() {
    let mutation = Mutation::new(|s: &mut Salary| s.0 += 1.0);
    let mut increase_in_place = MutateEverything::new(mutation);

    let mut company = Company::default();
    let mut group = LinkedList::new();
    group.push_front(Employee(Person("Joe", "Vancouver"), Salary(23.2)));
    group.push_front(Employee(Person("Mike", "Toronto"), Salary(34.3)));
    increase_in_place.mutate(&mut company);
    assert_eq!(
        company,
        Company(vec![
            Department(
                "Research",
                Employee(Person("Ralf", "Amsterdam"), Salary(8001.0)),
                vec![
                    SubUnit::Group(group),
                    SubUnit::Person(Employee(Person("Joost", "Amsterdam"), Salary(1001.0))),
                    SubUnit::Person(Employee(Person("Marlow", "Cambridge"), Salary(2001.0))),
                    SubUnit::Department(Box::new(Department(
                        "Funsies",
                        Employee(Person("Jim", "Portland"), Salary(4.0)),
                        vec![],
                    ))),
                ],
            ),
            Department(
                "Strategy",
                Employee(Person("Blair", "London"), Salary(100001.0)),
                vec![],
            ),
        ])
    );
}

#[test]
fn query_highest_salary_with_boilerplate() {
    let company = Company::default();
    assert_eq!(company.highest_salary(), Some(Salary(100000.0)));
}

#[test]
fn query_highest_salary_scrapping_boilerplate() {
    let query = Query::new(|e: &Employee| Some(e.1.clone()));
    let mut highest_salary = Everything::new(query, cmp::max);

    let company = Company::default();
    assert_eq!(highest_salary.query(&company), Some(Salary(100000.0)));
}
