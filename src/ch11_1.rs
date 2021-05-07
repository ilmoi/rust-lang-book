fn main() {
}

// -----------------------------------------------------------------------------

#[derive(Debug,PartialEq)]
struct ZeStruct {
    num: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {

        let s1 = ZeStruct {num: 32};
        let s2 = ZeStruct {num: 31};

        assert_ne!(s1, s2);
    }
}

// -----------------------------------------------------------------------------

fn might_fail() -> std::io::Result<i32> {
    let mut f = std::fs::File::open("asdf")?;
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() -> Result<(), i32> {
        if 3 < 5 {
            Ok(())
        } else {
            Err(2)
        }
    }

    #[test]
    fn works2() -> Result<(), std::io::Error> {
        might_fail()?;
        Ok(())
    }
}