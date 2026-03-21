use crate::gate::{Gate, Pauli};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    Qubit(usize, bool),
    MeasurementRecord(usize),
    PauliString(Box<[(Pauli, usize)]>, bool),
    Bool(bool),
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Qubit(index, inverted) => {
                if *inverted {
                    write!(f, "!")?;
                }
                write!(f, "{}", index)?;
            }
            Target::MeasurementRecord(index) => {
                write!(f, "rec[-{}]", index)?;
            }
            Target::PauliString(pauli_string, inverted) => {
                if *inverted {
                    write!(f, "!")?;
                }
                for (i, (pauli, index)) in pauli_string.iter().enumerate() {
                    if i > 0 {
                        write!(f, "*")?;
                    }
                    write!(f, "{}{}", pauli, index)?;
                }
            }
            Target::Bool(value) => {
                write!(f, "{}", if *value { 1 } else { 0 })?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arg {
    Probability(f64),
    Coord(f64),
    Index(usize),
}

impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arg::Probability(p) => write!(f, "{}", p),
            Arg::Coord(c) => write!(f, "{}", c),
            Arg::Index(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CircuitInstruction {
    gate: Gate,
    targets: Box<[Target]>,
    args: Box<[Arg]>,
    block: Option<Circuit>,
}

impl std::fmt::Display for CircuitInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.gate().name())?;

        if self.gate() == Gate::REPEAT {
            if let Some(repeat_count) = self.args.first() {
                write!(f, " {}", repeat_count)?;
            }
        } else {
            if !self.args.is_empty() {
                // display args as parenthesized, comma-separated list
                write!(f, "(")?;
                for (i, arg) in self.args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")?;
            }

            for target in &self.targets {
                write!(f, " {}", target)?;
            }
        }

        if let Some(block) = &self.block {
            write!(f, " {{\n")?;
            for instruction in block.instructions() {
                let inner = format!("{}", instruction);
                for line in inner.lines() {
                    write!(f, "    {}\n", line)?;
                }
            }
            write!(f, "}}")?;
        }

        write!(f, "\n")
    }
}

impl CircuitInstruction {
    pub fn new(gate: Gate, targets: Box<[Target]>, args: Box<[Arg]>) -> Self {
        CircuitInstruction {
            gate,
            targets,
            args,
            block: None,
        }
    }

    pub fn repeat(repeat_count: usize, block: Circuit) -> Self {
        CircuitInstruction {
            gate: Gate::REPEAT,
            targets: Box::new([]),
            args: Box::new([Arg::Index(repeat_count)]),
            block: Some(block),
        }
    }

    pub fn gate(&self) -> Gate {
        self.gate
    }

    pub fn targets(&self) -> &[Target] {
        &self.targets
    }

    pub fn args(&self) -> &[Arg] {
        &self.args
    }

    pub fn repeat_count(&self) -> usize {
        if self.gate() == Gate::REPEAT {
            if let Some(arg) = self.args.first() {
                if let Arg::Index(count) = arg {
                    return *count;
                }
            }
        }
        0
    }

    pub fn block(&self) -> Option<&Circuit> {
        self.block.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Circuit {
    instructions: Vec<CircuitInstruction>,
}

impl std::fmt::Display for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for instruction in &self.instructions {
            write!(f, "{}", instruction)?;
        }
        Ok(())
    }
}

impl Circuit {
    pub fn new() -> Self {
        Circuit {
            instructions: Vec::new(),
        }
    }

    pub fn append(&mut self, gate: Gate, targets: Box<[Target]>, args: Box<[Arg]>) {
        self.instructions
            .push(CircuitInstruction::new(gate, targets, args));
    }

    pub fn append_repeat(&mut self, repeat_count: usize, block: Circuit) {
        self.instructions
            .push(CircuitInstruction::repeat(repeat_count, block));
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn instructions(&self) -> &[CircuitInstruction] {
        &self.instructions
    }

    pub fn unrolled_instructions(&self) -> Vec<CircuitInstruction> {
        let mut result = Vec::new();
        for instruction in &self.instructions {
            if instruction.gate() == Gate::REPEAT {
                if let Some(block) = instruction.block() {
                    for _ in 0..instruction.repeat_count() {
                        result.extend(block.unrolled_instructions());
                    }
                }
            } else {
                result.push(instruction.clone());
            }
        }
        result
    }

    pub fn unrolled(&self) -> Circuit {
        Circuit {
            instructions: self.unrolled_instructions(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unroll() {
        let mut circuit = Circuit::new();
        let mut block = Circuit::new();
        block.append(Gate::X, Box::new([Target::Qubit(0, false)]), Box::new([]));
        block.append(Gate::Y, Box::new([Target::Qubit(1, false)]), Box::new([]));
        circuit.append_repeat(3, block);
        circuit.append(Gate::H, Box::new([Target::Qubit(0, false)]), Box::new([]));

        let unrolled = circuit.unrolled();
        assert_eq!(unrolled.len(), 7);
        assert_eq!(unrolled.instructions()[0].gate(), Gate::X);
        assert_eq!(unrolled.instructions()[1].gate(), Gate::Y);
        assert_eq!(unrolled.instructions()[2].gate(), Gate::X);
        assert_eq!(unrolled.instructions()[3].gate(), Gate::Y);
        assert_eq!(unrolled.instructions()[4].gate(), Gate::X);
        assert_eq!(unrolled.instructions()[5].gate(), Gate::Y);
        assert_eq!(unrolled.instructions()[6].gate(), Gate::H);
    }

    #[test]
    fn test_nested_unroll() {
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
}
