use std::fmt::Display;

#[derive(Debug)]
pub enum Expr {
    Integer(i64),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(n) => write!(f, "{}", n),
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expr::Integer;
    use std::fmt::Write;

    #[test]
    fn display_integer() {
        let value = Integer(10);
        let mut buf = String::new();
        write!(buf, "{}", value).unwrap();
        assert_eq!(buf, "10");
    }

    #[test]
    fn eq_integer() {
        assert_eq!(Integer(32), Integer(32));
    }

    #[test]
    fn not_eq_intger() {
        assert_ne!(Integer(1), Integer(2));
    }
}
