use stimcircuit::{Circuit, Gate, Target};

fn main() {
    let mut circuit = Circuit::new();
    let mut block = Circuit::new();
    block.append(Gate::X, Box::new([Target::Qubit(0, false)]), Box::new([]));
    block.append(Gate::Y, Box::new([Target::Qubit(1, false)]), Box::new([]));
    let mut nested_block = Circuit::new();
    nested_block.append(Gate::Z, Box::new([Target::Qubit(2, false)]), Box::new([]));
    block.append_repeat(2, nested_block);
    circuit.append_repeat(3, block);
    circuit.append(Gate::H, Box::new([Target::Qubit(0, false)]), Box::new([]));

    let unrolled = circuit.unrolled();
    assert_eq!(unrolled.len(), 13);
    assert_eq!(unrolled.instructions()[0].gate(), Gate::X);
    assert_eq!(unrolled.instructions()[1].gate(), Gate::Y);
    assert_eq!(unrolled.instructions()[2].gate(), Gate::Z);
    assert_eq!(unrolled.instructions()[3].gate(), Gate::Z);
    assert_eq!(unrolled.instructions()[4].gate(), Gate::X);
    assert_eq!(unrolled.instructions()[5].gate(), Gate::Y);
    assert_eq!(unrolled.instructions()[6].gate(), Gate::Z);
    assert_eq!(unrolled.instructions()[7].gate(), Gate::Z);
    assert_eq!(unrolled.instructions()[8].gate(), Gate::X);
    assert_eq!(unrolled.instructions()[9].gate(), Gate::Y);
    assert_eq!(unrolled.instructions()[10].gate(), Gate::Z);
    assert_eq!(unrolled.instructions()[11].gate(), Gate::Z);
    assert_eq!(unrolled.instructions()[12].gate(), Gate::H);
}
