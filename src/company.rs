//! Some toy types to test with, taken from the paper.

#[derive(Clone, Debug, PartialEq)]
struct Company(Vec<Department>);

#[derive(Clone, Debug, PartialEq)]
struct Department(Name, Manager, Vec<SubUnit>);

#[derive(Clone, Debug, PartialEq)]
enum SubUnit {
    Person(Employee),
    Department(Box<Department>),
}

#[derive(Clone, Debug, PartialEq)]
struct Employee(Person, Salary);

#[derive(Clone, Debug, PartialEq)]
struct Person(Name, Address);

#[derive(Clone, Debug, Default, PartialEq)]
struct Salary(f64);

type Manager = Employee;
type Name = &'static str;
type Address = &'static str;

impl Default for Company {
    fn defualt() -> Company {
        let ralf = Employee(Person("Ralf", "Amsterdam"), Salary(8000.0));
        let joost = Employee(Person("Joost", "Amsterdam"), Salary(1000.0));
        let marlow = Employee(Person("Marlow", "Cambridge"), Salary(2000.0));
        let blair = Employee(Person("Blair", "London"), Salary(100000.0));
        let jim = Employee(Person("Jim", "Portland"), Salary(3.0));
        Company(vec![
            Department(
                "Research",
                ralf,
                vec![
                    SubUnit::Person(joost),
                    SubUnit::Person(marlow),
                    SubUnit::Department(Box::new(Department("Funsies", jim, vec![]))),
                ],
            ),
            Department("Strategy", blair, vec![]),
        ])
    }
}

// Term impls //////////////////////////////////////////////////////////////////

// TODO: derive these.

impl Term for Company {
    fn map_one_transform<F>(self, f: &mut F) -> Self
        where
        F: TransformAll,
    {
        Company(f.transform(self.0))
    }

    fn map_one_query<Q, R>(&self, q: &mut Q) -> Vec<R>
        where
        Q: QueryAll<R>,
    {
        vec![q.query(&self.0)]
    }
}

impl Term for Department {
    fn map_one_transform<F>(self, f: &mut F) -> Self
        where
        F: TransformAll,
    {
        let name = f.transform(self.0);
        let mgr = f.transform(self.1);
        let units = f.transform(self.2);
        Department(name, mgr, units)
    }

    fn map_one_query<Q, R>(&self, q: &mut Q) -> Vec<R>
        where
        Q: QueryAll<R>,
    {
        vec![q.query(&self.0), q.query(&self.1), q.query(&self.2)]
    }
}

impl Term for SubUnit {
    fn map_one_transform<F>(self, f: &mut F) -> Self
        where
        F: TransformAll,
    {
        match self {
            SubUnit::Person(e) => SubUnit::Person(f.transform(e)),
            SubUnit::Department(d) => SubUnit::Department(f.transform(d)),
        }
    }

    fn map_one_query<Q, R>(&self, q: &mut Q) -> Vec<R>
        where
        Q: QueryAll<R>,
    {
        match *self {
            SubUnit::Person(ref e) => vec![q.query(e)],
            SubUnit::Department(ref d) => vec![q.query(d)],
        }
    }
}

impl Term for Employee {
    fn map_one_transform<F>(self, f: &mut F) -> Self
        where
        F: TransformAll,
    {
        Employee(f.transform(self.0), f.transform(self.1))
    }

    fn map_one_query<Q, R>(&self, q: &mut Q) -> Vec<R>
        where
        Q: QueryAll<R>,
    {
        vec![q.query(&self.0), q.query(&self.1)]
    }
}

impl Term for Person {
    fn map_one_transform<F>(self, f: &mut F) -> Self
        where
        F: TransformAll,
    {
        Person(f.transform(self.0), f.transform(self.1))
    }

    fn map_one_query<Q, R>(&self, q: &mut Q) -> Vec<R>
        where
        Q: QueryAll<R>,
    {
        vec![q.query(&self.0), q.query(&self.1)]
    }
}

impl Term for Salary {
    fn map_one_transform<F>(self, f: &mut F) -> Self
        where
        F: TransformAll,
    {
        Salary(f.transform(self.0))
    }

    fn map_one_query<Q, R>(&self, q: &mut Q) -> Vec<R>
        where
        Q: QueryAll<R>,
    {
        vec![q.query(&self.0)]
    }
}

// Boilerplate version of `increase`! //////////////////////////////////////////

trait Increase: Sized {
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

// Tests ///////////////////////////////////////////////////////////////////////

#[test]
fn increase_with_boilerplate() {
    let company = Company::default();
    let company = company.increase(1.0);
    assert_eq!(
        company,
        Company(vec![
            Department(
                "Research",
                Employee(Person("Ralf", "Amsterdam"), Salary(8001.0)),
                vec![
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

    let company = Company::default();
    let company = increase.transform(company);
    assert_eq!(
        company,
        Company(vec![
            Department(
                "Research",
                Employee(Person("Ralf", "Amsterdam"), Salary(8001.0)),
                vec![
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
fn query_highest_salary() {
    let query = Query::new(|s: &Salary| Some(s.clone()));
    let mut highest_salary = Everything::new(query, |a, b| match (a, b) {
        (Some(a), Some(b)) => if a.0 > b.0 {
            Some(a)
        } else {
            Some(b)
        },
        (Some(a), None) => Some(a),
        (_, b) => b,
    });

    let company = Company::default();
    assert_eq!(highest_salary.query(&company), Some(Salary(100000.0)));
}
