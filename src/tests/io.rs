use crate::core::io::{read::{Section,Reader,Terminator},errors::ReadError};
#[test]
fn test_readline_validation() {
    let mut reader = Reader::init();

    // 1. Valid section transitions
    assert_eq!(reader.readline("data:"), Ok(()));
    assert_eq!(reader.readline("end"), Ok(()));
    assert_eq!(reader.readline("code:"), Ok(()));

    // 2. Valid data declaration
    reader.updatesec(Section::Data);
    assert_eq!(reader.readline("0 10 ;"), Ok(()));

    // 3. Valid SUBLEQ instruction
    reader.updatesec(Section::Code);
    assert_eq!(reader.readline("1 11 20;;"), Ok(()));

    // 4. Invalid terminator
    assert!(matches!(
        reader.readline("1 11 20;"),
        Err(ReadError::InvalidTerminator(Terminator::Data))
    ));

    // 5. Content outside a section
    reader.updatesec(Section::None);
    assert!(matches!(
        reader.readline("1 11 20;;"),
        Err(ReadError::InvalidContext(Section::None))
    ));

    // 6. Missing terminator
    reader.updatesec(Section::Code);
    assert!(matches!(
        reader.readline("1 11 20"),
        Err(ReadError::MissingTerminator)
    ));
}

#[test]
    fn section_transitions() {
        let mut reader = Reader::init();

        reader.readsec("data:");
        assert!(matches!(reader.section, Section::Data));

        reader.readsec("end");
        assert!(matches!(reader.section, Section::None));

        reader.readsec("code:");
        assert!(matches!(reader.section, Section::Code));

        reader.readsec("end");
        assert!(matches!(reader.section, Section::None));
    }