use rand;
use std::cell::Cell;

use crate::lib::ket;
use ket::Ket;

#[derive(Debug)]
pub struct QuantumRegister {
    width: usize,
    collapsed: Cell<bool>,
    ket: Ket,
}


impl QuantumRegister {}