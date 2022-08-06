#[macro_export]
macro_rules! expression {
    ($x: ident) => {
        ::oat_symbol::Symbol::from(stringify!($x))
    };

    ($e: expr) => {
        $e.into()
    };

    ($left: expr, +, $right: expr) => {
        $crate::Expression::Binary {
            op: $crate::BinaryOp::Add,
            left: Box::new(expression!($left)),
            right: Box::new(expression!($right)),
        }
    };

    ($left: expr; -; $right: expr) => {
        $crate::Expression::Binary {
            op: $crate::BinaryOp::Sub,
            left: Box::new(expression!($left)),
            right: Box::new(expression!($right)),
        }
    };

    ($left: expr; *; $right: expr) => {
        $crate::Expression::Binary {
            op: $crate::BinaryOp::Mul,
            left: Box::new(expression!($left)),
            right: Box::new(expression!($right)),
        }
    };

    ($left: expr; lt; $right: expr) => {
        $crate::Expression::Binary {
            op: $crate::BinaryOp::Lt,
            left: Box::new(expression!($left)),
            right: Box::new(expression!($right)),
        }
    };
}

#[macro_export]
macro_rules! assign {
    ($target: expr, $value: expr) => {
        $crate::Statement::Assignment(expression!($target), expression!($value))
    };
}
