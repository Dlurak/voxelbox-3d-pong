#[macro_export]
macro_rules! plus_minus {
    ($($expr:expr),*$(,)?) => {
        [
            $(
                $expr - 1,
                $expr + 1,
            )*
        ]
    }
}

#[macro_export]
macro_rules! dynamic_vec {
    ($($cond:expr => $value:expr),* $(,)?) => {
        {
            let mut v = Vec::new();

            $(
                if $cond {
                    v.push($value)
                }
            )*

            v
        }
    };
    ($init:expr, $($cond:expr => $value:expr),* $(,)?) => {
        {
            let mut v = $init;

            $(
                if $cond {
                    v.push($value)
                }
            )*

            v
        }
    }
}
