use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Expr {
    Atom(String),
    Integer(i64),
    Float(f64),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(str) => write!(f, "{}", str),
            Self::Integer(n) => write!(f, "{}", n),
            Self::Float(float) => write!(f, "{}.{}", float.trunc(), float.fract()),
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Atom(left), Self::Atom(right)) => left == right,
            (Self::Integer(left), Self::Integer(right)) => left == right,
            (Self::Float(left), Self::Float(right)) => left == right,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expr::{Atom, Integer};
    use std::fmt::Write;

    #[test]
    fn display_integer() {
        let value = Integer(10);
        let mut buf = String::new();
        write!(buf, "{}", value).unwrap();
        assert_eq!(buf, "10");
    }

    #[test]
    fn display_atom() {
        let value = Atom("example".to_string());
        let mut buf = String::new();
        write!(buf, "{}", value).unwrap();
        assert_eq!(buf, "example");
    }

    #[test]
    fn eq_integer() {
        assert_eq!(Integer(32), Integer(32));
    }

    #[test]
    fn not_eq_intger() {
        assert_ne!(Integer(1), Integer(2));
    }

    #[test]
    fn eq_atom() {
        assert_eq!(Atom("test".to_string()), Atom("test".to_string()));
    }

    #[test]
    fn no_eq_atom() {
        assert_ne!(Atom("test".to_string()), Atom("example".to_string()));
    }

    #[test]
    fn not_eq_atom_and_integer() {
        assert_ne!(Atom("test".to_string()), Integer(10));
    }
}
