use stimcircuit::{parse_string, Arg, Circuit, Gate, ParseError, Target};

fn main() {
    let input = r#"
    REPEAT 10 {
        X 0 # comment
        Y 1
    }
    "#;
    let result: Result<Circuit, ParseError> = parse_string(input);
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

    println!("Stim circuit parsed successfully.");
}
