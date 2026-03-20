#[derive(Debug, PartialEq, Eq)]
pub struct GateError(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pauli {
    I,
    X,
    Y,
    Z,
}

// all gate types supported by .stim files
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Gate {
    NOT_A_GATE = 0,
    // Annotations
    DETECTOR,
    OBSERVABLE_INCLUDE,
    TICK,
    QUBIT_COORDS,
    SHIFT_COORDS,
    // Control flow
    REPEAT,
    // Collapsing gates
    MPAD,
    MX,
    MY,
    M, // alias when parsing: MZ
    MRX,
    MRY,
    MR, // alias when parsing: MRZ
    RX,
    RY,
    R, // alias when parsing: RZ
    // Controlled gates
    XCX,
    XCY,
    XCZ,
    YCX,
    YCY,
    YCZ,
    CX, // alias when parsing: CNOT, ZCX
    CY, // alias when parsing: ZCY
    CZ, // alias when parsing: ZCZ
    // Hadamard-like gates
    H, // alias when parsing: H_XZ
    H_XY,
    H_YZ,
    // Noise channels
    DEPOLARIZE1,
    DEPOLARIZE2,
    X_ERROR,
    Y_ERROR,
    Z_ERROR,
    PAULI_CHANNEL_1,
    PAULI_CHANNEL_2,
    E, // alias when parsing: CORRELATED_ERROR
    ELSE_CORRELATED_ERROR,
    // Heralded noise channels
    HERALDED_ERASE,
    HERALDED_PAULI_CHANNEL_1,
    // Pauli gates
    I,
    X,
    Y,
    Z,
    // Period 3 gates
    C_XYZ,
    C_ZYX,
    // Period 4 gates
    SQRT_X,
    SQRT_X_DAG,
    SQRT_Y,
    SQRT_Y_DAG,
    S,     // alias when parsing: SQRT_Z
    S_DAG, // alias when parsing: SQRT_Z_DAG
    // Pair measurement gates
    SQRT_XX,
    SQRT_XX_DAG,
    SQRT_YY,
    SQRT_YY_DAG,
    SQRT_ZZ,
    SQRT_ZZ_DAG,
    // Pauli product gates
    MPP,
    SPP,
    SPP_DAG,
    // Swap gates
    SWAP,
    ISWAP,
    CXSWAP,
    SWAPCX,
    CZSWAP,
    ISWAP_DAG,
    // Pair measurement gates
    MXX,
    MYY,
    MZZ,
}

impl Default for Gate {
    fn default() -> Self {
        Gate::NOT_A_GATE
    }
}

const NUM_GATES: usize = 70;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetType {
    Empty,
    Single,
    SingleCQ,
    Pair,
    PairCQ,
    Pauli,
    MeasurementRecord,
    PauliOrMeasurementRecord,
    Bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgType {
    Empty,
    ProbabilityOpt,
    Probability(usize),
    Coords,
    Index,
}

pub struct GateData {
    name: &'static str,
    arg_type: ArgType,
    block: bool,
    flow_data: &'static [&'static str],
    fusable: bool,
    inverse: Gate,
    noisy: bool,
    noop_on_qubits: bool,
    produces_results: bool,
    reset: bool,
    target_type: TargetType,
    unitary: bool,
}

const DEFAULT_DATA: GateData = GateData {
    name: "",
    arg_type: ArgType::Empty,
    block: false,
    flow_data: &[],
    fusable: true,
    inverse: NOT_A_GATE,
    noisy: false,
    noop_on_qubits: false,
    produces_results: false,
    reset: false,
    target_type: TargetType::Empty,
    unitary: false,
};

impl Default for GateData {
    fn default() -> Self {
        DEFAULT_DATA
    }
}

use Gate::*;

impl From<usize> for Gate {
    fn from(value: usize) -> Self {
        if value < NUM_GATES {
            // we can use transmute safely here because a `Gate` is represented in memory as a `usize`
            // ranging from 0 to NUM_GATES.
            unsafe { std::mem::transmute(value) }
        } else {
            NOT_A_GATE
        }
    }
}

impl From<&str> for Gate {
    fn from(s: &str) -> Self {
        match s {
            "DETECTOR" => DETECTOR,
            "OBSERVABLE_INCLUDE" => OBSERVABLE_INCLUDE,
            "TICK" => TICK,
            "QUBIT_COORDS" => QUBIT_COORDS,
            "SHIFT_COORDS" => SHIFT_COORDS,
            "REPEAT" => REPEAT,
            "MPAD" => MPAD,
            "MX" => MX,
            "MY" => MY,
            "M" | "MZ" => M,
            "MRX" => MRX,
            "MRY" => MRY,
            "MR" | "MRZ" => MR,
            "RX" => RX,
            "RY" => RY,
            "R" | "RZ" => R,
            "XCX" => XCX,
            "XCY" => XCY,
            "XCZ" => XCZ,
            "YCX" => YCX,
            "YCY" => YCY,
            "YCZ" => YCZ,
            "CX" | "CNOT" | "ZCX" => CX,
            "CY" | "ZCY" => CY,
            "CZ" | "ZCZ" => CZ,
            "H" | "H_XZ" => H,
            "H_XY" => H_XY,
            "H_YZ" => H_YZ,
            "DEPOLARIZE1" => DEPOLARIZE1,
            "DEPOLARIZE2" => DEPOLARIZE2,
            "X_ERROR" => X_ERROR,
            "Y_ERROR" => Y_ERROR,
            "Z_ERROR" => Z_ERROR,
            "PAULI_CHANNEL_1" => PAULI_CHANNEL_1,
            "PAULI_CHANNEL_2" => PAULI_CHANNEL_2,
            "E" | "CORRELATED_ERROR" => E,
            "ELSE_CORRELATED_ERROR" => ELSE_CORRELATED_ERROR,
            "HERALDED_ERASE" => HERALDED_ERASE,
            "HERALDED_PAULI_CHANNEL_1" => HERALDED_PAULI_CHANNEL_1,
            "I" => I,
            "X" => X,
            "Y" => Y,
            "Z" => Z,
            "C_XYZ" => C_XYZ,
            "C_ZYX" => C_ZYX,
            "SQRT_X" => SQRT_X,
            "SQRT_X_DAG" => SQRT_X_DAG,
            "SQRT_Y" => SQRT_Y,
            "SQRT_Y_DAG" => SQRT_Y_DAG,
            "S" | "SQRT_Z" => S,
            "S_DAG" | "SQRT_Z_DAG" => S_DAG,
            "SQRT_XX" => SQRT_XX,
            "SQRT_XX_DAG" => SQRT_XX_DAG,
            "SQRT_YY" => SQRT_YY,
            "SQRT_YY_DAG" => SQRT_YY_DAG,
            "SQRT_ZZ" => SQRT_ZZ,
            "SQRT_ZZ_DAG" => SQRT_ZZ_DAG,
            "MPP" => MPP,
            "SPP" => SPP,
            "SPP_DAG" => SPP_DAG,
            "SWAP" => SWAP,
            "ISWAP" => ISWAP,
            "CXSWAP" => CXSWAP,
            "SWAPCX" => SWAPCX,
            "CZSWAP" => CZSWAP,
            "ISWAP_DAG" => ISWAP_DAG,
            "MXX" => MXX,
            "MYY" => MYY,
            "MZZ" => MZZ,
            _ => NOT_A_GATE,
        }
    }
}

const GATE_DATA: [GateData; NUM_GATES] = [
    GateData {
        name: "NOT_A_GATE",
        noop_on_qubits: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "DETECTOR",
        arg_type: ArgType::Coords,
        fusable: false,
        inverse: DETECTOR,
        noop_on_qubits: true,
        target_type: TargetType::MeasurementRecord,
        ..DEFAULT_DATA
    },
    GateData {
        name: "OBSERVABLE_INCLUDE",
        noop_on_qubits: true,
        inverse: OBSERVABLE_INCLUDE,
        arg_type: ArgType::Index,
        target_type: TargetType::PauliOrMeasurementRecord,
        ..DEFAULT_DATA
    },
    GateData {
        name: "TICK",
        fusable: false,
        inverse: TICK,
        noop_on_qubits: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "QUBIT_COORDS",
        arg_type: ArgType::Coords,
        fusable: false,
        inverse: QUBIT_COORDS,
        noop_on_qubits: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SHIFT_COORDS",
        arg_type: ArgType::Coords,
        fusable: false,
        inverse: SHIFT_COORDS,
        noop_on_qubits: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "REPEAT",
        noop_on_qubits: true,
        block: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "MPAD",
        arg_type: ArgType::ProbabilityOpt,
        inverse: MPAD,
        produces_results: true,
        target_type: TargetType::Bool,
        ..DEFAULT_DATA
    },
    GateData {
        name: "MX",
        arg_type: ArgType::ProbabilityOpt,
        inverse: MX,
        noisy: true,
        produces_results: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "MY",
        arg_type: ArgType::ProbabilityOpt,
        inverse: MY,
        noisy: true,
        produces_results: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "M",
        arg_type: ArgType::ProbabilityOpt,
        inverse: M,
        noisy: true,
        produces_results: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "MRX",
        arg_type: ArgType::ProbabilityOpt,
        inverse: MRX,
        noisy: true,
        produces_results: true,
        reset: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "MRY",
        arg_type: ArgType::ProbabilityOpt,
        inverse: MRY,
        noisy: true,
        produces_results: true,
        reset: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "MR",
        arg_type: ArgType::ProbabilityOpt,
        inverse: MR,
        noisy: true,
        produces_results: true,
        reset: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "RX",
        inverse: MX,
        reset: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "RY",
        inverse: MY,
        reset: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "R",
        inverse: M,
        reset: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "XCX",
        inverse: XCX,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "XCY",
        inverse: XCY,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "XCZ",
        inverse: XCZ,
        target_type: TargetType::PairCQ,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "YCX",
        inverse: YCX,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "YCY",
        inverse: YCY,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "YCZ",
        inverse: YCZ,
        target_type: TargetType::PairCQ,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "CX",
        inverse: CX,
        target_type: TargetType::PairCQ,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "CY",
        inverse: CY,
        target_type: TargetType::PairCQ,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "CZ",
        inverse: CZ,
        target_type: TargetType::PairCQ,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "H",
        inverse: H,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "H_XY",
        inverse: H_XY,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "H_YZ",
        inverse: H_YZ,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "DEPOLARIZE1",
        arg_type: ArgType::Probability(1),
        inverse: DEPOLARIZE1,
        noisy: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "DEPOLARIZE2",
        arg_type: ArgType::Probability(1),
        inverse: DEPOLARIZE2,
        noisy: true,
        target_type: TargetType::Pair,
        ..DEFAULT_DATA
    },
    GateData {
        name: "X_ERROR",
        arg_type: ArgType::Probability(1),
        inverse: X_ERROR,
        noisy: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "Y_ERROR",
        arg_type: ArgType::Probability(1),
        inverse: Y_ERROR,
        noisy: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "Z_ERROR",
        arg_type: ArgType::Probability(1),
        inverse: Z_ERROR,
        noisy: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "PAULI_CHANNEL_1",
        arg_type: ArgType::Probability(3),
        inverse: PAULI_CHANNEL_1,
        noisy: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "PAULI_CHANNEL_2",
        arg_type: ArgType::Probability(15),
        inverse: PAULI_CHANNEL_2,
        noisy: true,
        target_type: TargetType::Pair,
        ..DEFAULT_DATA
    },
    GateData {
        name: "E", // alias: CORRELATED_ERROR
        arg_type: ArgType::Probability(1),
        fusable: false,
        inverse: E,
        noisy: true,
        target_type: TargetType::Pauli,
        ..DEFAULT_DATA
    },
    GateData {
        name: "ELSE_CORRELATED_ERROR",
        arg_type: ArgType::Probability(1),
        fusable: false,
        inverse: ELSE_CORRELATED_ERROR,
        noisy: true,
        target_type: TargetType::Pauli,
        ..DEFAULT_DATA
    },
    GateData {
        name: "HERALDED_ERASE",
        arg_type: ArgType::Probability(1),
        inverse: HERALDED_ERASE,
        noisy: true,
        produces_results: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "HERALDED_PAULI_CHANNEL_1",
        arg_type: ArgType::Probability(4),
        inverse: HERALDED_PAULI_CHANNEL_1,
        noisy: true,
        produces_results: true,
        target_type: TargetType::Single,
        ..DEFAULT_DATA
    },
    GateData {
        name: "I",
        inverse: I,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "X",
        inverse: X,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "Y",
        inverse: Y,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "Z",
        inverse: Z,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "C_XYZ",
        inverse: C_ZYX,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "C_ZYX",
        inverse: C_XYZ,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SQRT_X",
        inverse: SQRT_X_DAG,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SQRT_X_DAG",
        inverse: SQRT_X,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SQRT_Y",
        inverse: SQRT_Y_DAG,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SQRT_Y_DAG",
        inverse: SQRT_Y,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "S",
        inverse: S_DAG,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "S_DAG",
        inverse: S,
        target_type: TargetType::Single,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SQRT_XX",
        inverse: SQRT_XX_DAG,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SQRT_XX_DAG",
        inverse: SQRT_XX,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SQRT_YY",
        inverse: SQRT_YY_DAG,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SQRT_YY_DAG",
        inverse: SQRT_YY,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SQRT_ZZ",
        inverse: SQRT_ZZ_DAG,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SQRT_ZZ_DAG",
        inverse: SQRT_ZZ,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "MPP",
        inverse: MPP,
        target_type: TargetType::Pauli,
        produces_results: true,
        noisy: true,
        arg_type: ArgType::ProbabilityOpt,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SPP",
        inverse: SPP_DAG,
        target_type: TargetType::Pauli,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SPP_DAG",
        inverse: SPP,
        target_type: TargetType::Pauli,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SWAP",
        inverse: SWAP,
        unitary: true,
        target_type: TargetType::Pair,
        ..DEFAULT_DATA
    },
    GateData {
        name: "ISWAP",
        inverse: ISWAP_DAG,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "CXSWAP",
        inverse: SWAPCX,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "SWAPCX",
        inverse: CXSWAP,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "CZSWAP",
        inverse: CZSWAP,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "ISWAP_DAG",
        inverse: ISWAP,
        target_type: TargetType::Pair,
        unitary: true,
        ..DEFAULT_DATA
    },
    GateData {
        name: "MXX",
        arg_type: ArgType::ProbabilityOpt,
        inverse: MXX,
        noisy: true,
        produces_results: true,
        target_type: TargetType::Pair,
        ..DEFAULT_DATA
    },
    GateData {
        name: "MYY",
        arg_type: ArgType::ProbabilityOpt,
        inverse: MYY,
        noisy: true,
        produces_results: true,
        target_type: TargetType::Pair,
        ..DEFAULT_DATA
    },
    GateData {
        name: "MZZ",
        arg_type: ArgType::ProbabilityOpt,
        inverse: MZZ,
        noisy: true,
        produces_results: true,
        target_type: TargetType::Pair,
        ..DEFAULT_DATA
    },
];

impl Gate {
    pub fn name(&self) -> &str {
        GATE_DATA[(*self) as usize].name
    }
    pub fn unitary(&self) -> bool {
        GATE_DATA[(*self) as usize].unitary
    }
    pub fn noisy(&self) -> bool {
        GATE_DATA[(*self) as usize].noisy
    }
    pub fn inverse(&self) -> Gate {
        GATE_DATA[(*self) as usize].inverse
    }
    pub fn produces_results(&self) -> bool {
        GATE_DATA[(*self) as usize].produces_results
    }
    pub fn flow_data(&self) -> &'static [&'static str] {
        GATE_DATA[(*self) as usize].flow_data
    }
    pub fn fusable(&self) -> bool {
        GATE_DATA[(*self) as usize].fusable
    }
    pub fn block(&self) -> bool {
        GATE_DATA[(*self) as usize].block
    }
    pub fn reset(&self) -> bool {
        GATE_DATA[(*self) as usize].reset
    }
    pub fn noop_on_qubits(&self) -> bool {
        GATE_DATA[(*self) as usize].noop_on_qubits
    }
    pub fn target_type(&self) -> TargetType {
        GATE_DATA[(*self) as usize].target_type
    }
    pub fn arg_type(&self) -> ArgType {
        GATE_DATA[(*self) as usize].arg_type
    }
    pub fn accepts_arg_count(&self, count: usize) -> bool {
        match self.arg_type() {
            ArgType::Empty => count == 0,
            ArgType::ProbabilityOpt => count <= 1,
            ArgType::Probability(n) => count == n,
            ArgType::Coords => count >= 1 && count <= 16,
            ArgType::Index => count == 1,
        }
    }
    pub fn accepts_target_count(&self, count: usize) -> bool {
        match self.target_type() {
            TargetType::Empty => count == 0,
            TargetType::Pair | TargetType::PairCQ => count > 0 && count % 2 == 0,
            TargetType::Bool => count == 1,
            _ => count > 0,
        }
    }
    pub fn accepts_measurement_record(&self) -> bool {
        matches!(
            self.target_type(),
            TargetType::MeasurementRecord
                | TargetType::PauliOrMeasurementRecord
                | TargetType::SingleCQ
                | TargetType::PairCQ
        )
    }
    pub fn accepts_qubit(&self) -> bool {
        matches!(
            self.target_type(),
            TargetType::Single | TargetType::SingleCQ | TargetType::Pair | TargetType::PairCQ
        )
    }
    pub fn accepts_pauli_str(&self) -> bool {
        matches!(
            self.target_type(),
            TargetType::Pauli | TargetType::PauliOrMeasurementRecord
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gate_data_aligned() {
        // check the `GateData` at index i := (G as usize) indeed corresponds to the gate G
        for i in 0..NUM_GATES {
            let g = Gate::from(i);
            let g1 = Gate::from(g.name());
            assert_eq!(g, g1);
        }
    }

    #[test]
    fn gate_properties() {
        assert_eq!(S.name(), "S");
        assert_eq!(S.arg_type(), ArgType::Empty);
        assert_eq!(S.block(), false);
        assert_eq!(S.fusable(), true);
        assert_eq!(S.inverse(), S_DAG);
        assert_eq!(S.noisy(), false);
        assert_eq!(S.noop_on_qubits(), false);
        assert_eq!(S.produces_results(), false);
        assert_eq!(S.reset(), false);
        assert_eq!(S.target_type(), TargetType::Single);
        assert_eq!(S.unitary(), true);
    }
}
