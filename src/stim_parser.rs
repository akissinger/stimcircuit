use pest::{Parser, error::LineColLocation};
use pest_derive::Parser;

use crate::{
    circuit::{Arg, Circuit, Target},
    gate::{ArgType, Gate, Pauli},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error on line {}:\n{}", self.line, self.message)
    }
}

#[derive(Parser)]
#[grammar = "stim.pest"]
pub struct StimParser;

fn get_usize(pair: pest::iterators::Pair<Rule>) -> usize {
    pair.as_str().parse().unwrap()
}

fn get_f64(pair: pest::iterators::Pair<Rule>) -> f64 {
    pair.as_str().parse().unwrap()
}

fn read_args(gate: Gate, it: pest::iterators::Pairs<Rule>) -> Result<Box<[Arg]>, ParseError> {
    let mut args = Vec::new();
    let mut line = 0;
    for pair in it {
        let arg = pair.into_inner().next().unwrap();
        line = arg.line_col().0;
        match gate.arg_type() {
            ArgType::Probability(_) | ArgType::ProbabilityOpt => {
                args.push(Arg::Probability(get_f64(arg)));
            }
            ArgType::Coords => {
                args.push(Arg::Coord(get_f64(arg)));
            }
            ArgType::Index => {
                if arg.as_rule() != Rule::num {
                    return Err(ParseError {
                        line: arg.line_col().0,
                        message: format!(
                            "Expected an unsigned integer for gate {}, found: {}",
                            gate.name(),
                            arg.as_str()
                        ),
                    });
                }
                args.push(Arg::Index(get_usize(arg)));
            }
            ArgType::Empty => {}
        }
    }

    if !gate.accepts_arg_count(args.len()) {
        return Err(ParseError {
            line,
            message: format!(
                "Incorrect number of arguments for gate {}: {}",
                gate.name(),
                args.len()
            ),
        });
    }

    Ok(args.into())
}

fn read_target(gate: Gate, target: pest::iterators::Pair<Rule>) -> Result<Target, ParseError> {
    let line = target.line_col().0;
    let mut p = target.into_inner().next().unwrap();
    match p.as_rule() {
        Rule::measurement_record if gate.accepts_measurement_record() => {
            p = p.into_inner().next().unwrap();
            Ok(Target::MeasurementRecord(get_usize(p)))
        }
        Rule::pauli_str if gate.accepts_pauli_str() => {
            let mut it1 = p.into_inner();
            p = it1.peek().unwrap();
            let negated = p.as_rule() == Rule::negate;
            if negated {
                it1.next();
            }

            Ok(Target::PauliString(
                it1.map(|p| {
                    let s = p.as_str();
                    (
                        Pauli::try_from(&s[0..1]).unwrap(),
                        (&s[1..]).parse().unwrap(),
                    )
                })
                .collect(),
                negated,
            ))
        }
        Rule::qubit if gate.accepts_qubit() => {
            let mut it1 = p.into_inner();
            p = it1.peek().unwrap();
            let negated = p.as_rule() == Rule::negate;
            if negated {
                p = it1.next().unwrap();
            }

            Ok(Target::Qubit(get_usize(p), negated))
        }
        _ => Err(ParseError {
            line,
            message: format!("Invalid target for gate {}: {}", gate.name(), p.as_str()),
        }),
    }
}

fn read_targets(gate: Gate, it: pest::iterators::Pairs<Rule>) -> Result<Box<[Target]>, ParseError> {
    let mut line = 0;
    let mut targets = Vec::new();
    for target in it {
        line = target.line_col().0;
        targets.push(read_target(gate, target)?);
    }

    if !gate.accepts_target_count(targets.len()) {
        return Err(ParseError {
            line,
            message: format!(
                "Incorrect number of targets for gate {}: {}",
                gate.name(),
                targets.len()
            ),
        });
    }
    Ok(targets.into())
}

fn read_lines(it: pest::iterators::Pairs<Rule>, depth: usize) -> Result<Circuit, ParseError> {
    let mut circuit = Circuit::new();
    for pair in it {
        if pair.as_rule() != Rule::line {
            continue;
        }

        if let Some(pair1) = pair.into_inner().next() {
            match pair1.as_rule() {
                Rule::block => {
                    let mut it1 = pair1.into_inner();
                    let repeat_count: usize = it1.next().unwrap().as_str().parse().unwrap();
                    let circuit1 = read_lines(it1.next().unwrap().into_inner(), depth + 1)?;
                    circuit.append_repeat(repeat_count, circuit1);
                }
                Rule::instruction => {
                    let mut it1 = pair1.into_inner();
                    let mut p = it1.next().unwrap();
                    let gate = Gate::from(p.as_str());
                    if gate == Gate::NOT_A_GATE {
                        return Err(ParseError {
                            line: p.line_col().0,
                            message: format!("Unknown gate: {}", p.as_str()),
                        });
                    }

                    p = it1.next().unwrap();

                    let args;
                    if p.as_rule() == Rule::args {
                        args = read_args(gate, p.into_inner())?;
                        p = it1.next().unwrap();
                    } else {
                        args = Box::new([])
                    };

                    let targets = read_targets(gate, p.into_inner())?;
                    circuit.append(gate, targets.into(), args);
                }
                _ => {}
            }
        }
    }
    Ok(circuit)
}

pub fn parse_target(input: &str) -> Result<Target, ParseError> {
    let input = input.to_owned() + "\n";
    let pair = StimParser::parse(Rule::target, &input)
        .map_err(|e| {
            let line = match e.line_col {
                LineColLocation::Pos(p) => p.0,
                LineColLocation::Span(p, _) => p.0,
            };
            let message = e.to_string();
            ParseError { line, message }
        })?
        .next()
        .unwrap();

    read_target(Gate::NOT_A_GATE, pair)
}

pub fn parse_string(input: &str) -> Result<Circuit, ParseError> {
    let input = input.to_owned() + "\n";
    let circuit = StimParser::parse(Rule::circuit, &input)
        .map_err(|e| {
            let line = match e.line_col {
                LineColLocation::Pos(p) => p.0,
                LineColLocation::Span(p, _) => p.0,
            };
            let message = e.to_string();
            ParseError { line, message }
        })?
        .next()
        .unwrap();

    read_lines(circuit.into_inner(), 0)
}

pub fn parse_file(path: &str) -> Result<Circuit, ParseError> {
    let input = std::fs::read_to_string(path).map_err(|e| ParseError {
        line: 0,
        message: e.to_string(),
    })?;
    parse_string(&input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_stim_file() {
        let input = r#"
        REPEAT 10 {
          X 0 # comment

          Y 1
        }
        "#;
        let result = parse_string(input);
        assert!(
            result.is_ok(),
            "Failed to parse simple stim file:\n{:}",
            result.unwrap_err().message
        );

        let circuit = result.unwrap();
        assert_eq!(circuit.instructions().len(), 1);
        assert_eq!(circuit.instructions()[0].gate(), Gate::REPEAT);
        assert_eq!(circuit.instructions()[0].args()[0], Arg::Index(10));

        let circuit1 = circuit.instructions()[0].block().unwrap();
        assert_eq!(circuit1.instructions().len(), 2);
        assert_eq!(circuit1.instructions()[0].gate(), Gate::X);
        assert_eq!(
            circuit1.instructions()[0].targets()[0],
            Target::Qubit(0, false)
        );
        assert_eq!(circuit1.instructions()[1].gate(), Gate::Y);
        assert_eq!(
            circuit1.instructions()[1].targets()[0],
            Target::Qubit(1, false)
        );
    }

    #[test]
    fn big_stim_file() {
        // Test comes from the stim documentation: https://github.com/quantumlib/Stim/blob/9898718/doc/file_format_stim_circuit.md
        let input = r#"
# Generated surface_code circuit.
# task: rotated_memory_x
# rounds: 1000
# distance: 3
# before_round_data_depolarization: 0
# before_measure_flip_probability: 0
# after_reset_flip_probability: 0
# after_clifford_depolarization: 0.001
# layout:
#                 X25
#     L15     d17     d19
# Z14     X16     Z18
#     L8      d10     d12
#         Z9      X11     Z13
#     L1      d3      d5 
#         X2 
# Legend:
#     d# = data qubit
#     L# = data qubit with logical observable crossing
#     X# = measurement qubit (X stabilizer)
#     Z# = measurement qubit (Z stabilizer)
QUBIT_COORDS(1, 1) 1
QUBIT_COORDS(2, 0) 2
QUBIT_COORDS(3, 1) 3
QUBIT_COORDS(5, 1) 5
QUBIT_COORDS(1, 3) 8
QUBIT_COORDS(2, 2) 9
QUBIT_COORDS(3, 3) 10
QUBIT_COORDS(4, 2) 11
QUBIT_COORDS(5, 3) 12
QUBIT_COORDS(6, 2) 13
QUBIT_COORDS(0, 4) 14
QUBIT_COORDS(1, 5) 15
QUBIT_COORDS(2, 4) 16
QUBIT_COORDS(3, 5) 17
QUBIT_COORDS(4, 4) 18
QUBIT_COORDS(5, 5) 19
QUBIT_COORDS(4, 6) 25
RX 1 3 5 8 10 12 15 17 19
R 2 9 11 13 14 16 18 25
TICK
H 2 11 16 25
DEPOLARIZE1(0.001) 2 11 16 25
TICK
CX 2 3 16 17 11 12 15 14 10 9 19 18
DEPOLARIZE2(0.001) 2 3 16 17 11 12 15 14 10 9 19 18
TICK
CX 2 1 16 15 11 10 8 14 3 9 12 18
DEPOLARIZE2(0.001) 2 1 16 15 11 10 8 14 3 9 12 18
TICK
CX 16 10 11 5 25 19 8 9 17 18 12 13
DEPOLARIZE2(0.001) 16 10 11 5 25 19 8 9 17 18 12 13
TICK
CX 16 8 11 3 25 17 1 9 10 18 5 13
DEPOLARIZE2(0.001) 16 8 11 3 25 17 1 9 10 18 5 13
TICK
H 2 11 16 25
DEPOLARIZE1(0.001) 2 11 16 25
TICK
MR 2 9 11 13 14 16 18 25
DETECTOR(2, 0, 0) rec[-8]
DETECTOR(2, 4, 0) rec[-3]
DETECTOR(4, 2, 0) rec[-6]
DETECTOR(4, 6, 0) rec[-1]
REPEAT 999 {
    TICK
    H 2 11 16 25
    DEPOLARIZE1(0.001) 2 11 16 25
    TICK
    CX 2 3 16 17 11 12 15 14 10 9 19 18
    DEPOLARIZE2(0.001) 2 3 16 17 11 12 15 14 10 9 19 18
    TICK
    CX 2 1 16 15 11 10 8 14 3 9 12 18
    DEPOLARIZE2(0.001) 2 1 16 15 11 10 8 14 3 9 12 18
    TICK
    CX 16 10 11 5 25 19 8 9 17 18 12 13
    DEPOLARIZE2(0.001) 16 10 11 5 25 19 8 9 17 18 12 13
    TICK
    CX 16 8 11 3 25 17 1 9 10 18 5 13
    DEPOLARIZE2(0.001) 16 8 11 3 25 17 1 9 10 18 5 13
    TICK
    H 2 11 16 25
    DEPOLARIZE1(0.001) 2 11 16 25
    TICK
    MR 2 9 11 13 14 16 18 25
    SHIFT_COORDS(0, 0, 1)
    DETECTOR(2, 0, 0) rec[-8] rec[-16]
    DETECTOR(2, 2, 0) rec[-7] rec[-15]
    DETECTOR(4, 2, 0) rec[-6] rec[-14]
    DETECTOR(6, 2, 0) rec[-5] rec[-13]
    DETECTOR(0, 4, 0) rec[-4] rec[-12]
    DETECTOR(2, 4, 0) rec[-3] rec[-11]
    DETECTOR(4, 4, 0) rec[-2] rec[-10]
    DETECTOR(4, 6, 0) rec[-1] rec[-9]
}
MX 1 3 5 8 10 12 15 17 19
DETECTOR(2, 0, 1) rec[-8] rec[-9] rec[-17]
DETECTOR(2, 4, 1) rec[-2] rec[-3] rec[-5] rec[-6] rec[-12]
DETECTOR(4, 2, 1) rec[-4] rec[-5] rec[-7] rec[-8] rec[-15]
DETECTOR(4, 6, 1) rec[-1] rec[-2] rec[-10]
OBSERVABLE_INCLUDE(0) rec[-3] rec[-6] rec[-9]
"#;
        let result = parse_string(input);

        assert!(
            result.is_ok(),
            "Failed to parse big stim file:\n{:}",
            result.unwrap_err().message
        );

        println!("Parsed circuit:\n{}", result.unwrap());
    }

    #[test]
    fn nested_repeat() {
        let input = r#"
        H 0
        REPEAT 2 {
            X 0
            REPEAT 3 {
                Y 0
                REPEAT 4 {
                    Z 0
                }
                Y 1
            }
            X 1
        }
        H 1
        "#;
        let result = parse_string(input);
        assert!(
            result.is_ok(),
            "Failed to parse nested repeat:\n{:}",
            result.unwrap_err().message
        );
        let circuit = result.unwrap();
        println!("Parsed circuit:\n{}", circuit);
    }
}
