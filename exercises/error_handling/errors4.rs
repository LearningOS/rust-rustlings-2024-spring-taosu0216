// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.


#[derive(PartialEq, Debug)]
struct Pos(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl Pos {
    fn new(value: i64) -> Result<Pos, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        if value >0 {
            Ok(Pos(value as u64))
        }else if value == 0 {
            Err(CreationError::Zero)
        }else {
            Err(CreationError::Negative)
        }
    }
}

#[test]
fn test_creation() {
    assert!(Pos::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        Pos::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), Pos::new(0));
}
