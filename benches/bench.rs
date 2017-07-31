#![feature(test)]

extern crate scrapmetal;
extern crate test;

use scrapmetal::*;
use scrapmetal::company::*;
use std::cmp;

#[bench]
fn bench_increase_with_boilerplate(b: &mut test::Bencher) {
    b.iter(|| { test::black_box(Company::default().increase(1.0)); });
}

#[bench]
fn bench_increase_scrapping_boilerplate(b: &mut test::Bencher) {
    let transformation = Transformation::new(|s: Salary| Salary(s.0 + 1.0));
    let mut increase = Everywhere::new(transformation);
    b.iter(|| {
        test::black_box(increase.transform(Company::default()));
    });
}

#[bench]
fn bench_highest_salary_with_boilerplate(b: &mut test::Bencher) {
    let company = Company::default();
    b.iter(|| { test::black_box(company.highest_salary()); });
}

#[bench]
fn bench_highest_salary_scrapping_boilerplate(b: &mut test::Bencher) {
    let query = Query::new(|e: &Employee| Some(e.1.clone()));
    let mut highest_salary = Everything::new(query, cmp::max);
    let company = Company::default();
    b.iter(|| { test::black_box(highest_salary.query(&company)); });
}
