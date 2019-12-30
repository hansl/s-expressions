#![cfg(test)]

use crate::error::Error;
use crate::MagicString;

#[test]
fn basic() -> Result<(), Error> {
    //                   01234567890
    let content = "Hello World";
    let mut buffer = MagicString::new(&content);

    buffer.append_right(6, "Beautiful ")?;
    assert_eq!(&buffer.to_string()?, "Hello Beautiful World");

    buffer.append_right(6, "Great ")?;
    assert_eq!(&buffer.to_string()?, "Hello Beautiful Great World");

    buffer.append_right(0, "1 ")?;
    assert_eq!(&buffer.to_string()?, "1 Hello Beautiful Great World");

    buffer.append_right(5, "2 ")?;
    assert_eq!(&buffer.to_string()?, "1 Hello2  Beautiful Great World");

    buffer.append_right(8, "3 ")?;
    assert_eq!(&buffer.to_string()?, "1 Hello2  Beautiful Great Wo3 rld");

    buffer.append_right(0, "4 ")?;
    assert_eq!(&buffer.to_string()?, "1 4 Hello2  Beautiful Great Wo3 rld");

    buffer.append_right(8, "5 ")?;
    assert_eq!(
        &buffer.to_string()?,
        "1 4 Hello2  Beautiful Great Wo3 5 rld"
    );

    buffer.append_right(1, "a ")?;
    assert_eq!(
        &buffer.to_string()?,
        "1 4 Ha ello2  Beautiful Great Wo3 5 rld"
    );

    buffer.append_right(2, "b ")?;
    assert_eq!(
        &buffer.to_string()?,
        "1 4 Ha eb llo2  Beautiful Great Wo3 5 rld"
    );

    buffer.append_right(7, "c ")?;
    assert_eq!(
        &buffer.to_string()?,
        "1 4 Ha eb llo2  Beautiful Great Wc o3 5 rld"
    );

    buffer.append_right(10, "d 6")?;
    assert_eq!(
        &buffer.to_string()?,
        "1 4 Ha eb llo2  Beautiful Great Wc o3 5 rldd 6"
    );

    Ok(())
}

#[test]
fn magic_string_example() -> Result<(), Error> {
    let mut s = MagicString::new("problems = 99");

    s.overwrite(0, 8, "answer")?;
    s.remove(11, 13)?;
    //    s.overwrite(11, 13, "42")?;
    s.prepend("var ")?;
    s.append(";")?;
    assert_eq!(&s.to_string()?, "var answer = 42;");

    Ok(())
}
