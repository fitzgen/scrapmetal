#![allow(dead_code)]
#![deny(unused_variables)]

extern crate scrapmetal;

#[macro_use]
extern crate scrapmetal_derive;

#[derive(Term)]
struct UnitStruct;

#[derive(Term)]
struct EmptyTupleStruct();

#[derive(Term)]
struct EmptyStruct {}

#[derive(Term)]
enum EmptyEnum {}
