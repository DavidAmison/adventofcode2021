pub mod files;

#[macro_export]
macro_rules! parse_field {
    ($input_string:expr => $($type:ty, $seperator:literal) |+) => {
        {
            let mut right: &str = $input_string;
            ( $(
                if $seperator == "" {
                    right.parse::<$type>().ok()
                } else {
                    if let Some((l, r)) = right.split_once($seperator) {
                        right = r;
                        l.parse::<$type>().ok()
                    } else {
                        None
                    }
                },
            )* )
        }
    };
}

#[macro_export]
macro_rules! parse_field_unwrap {
    ($input_string:expr => $($type:ty, $seperator:literal) |+) => {
        {
            let mut right: &str = $input_string;
            ( $(
                if $seperator == "" {
                    right.parse::<$type>().ok().unwrap()
                } else {
                    if let Some((l, r)) = right.split_once($seperator) {
                        right = r;
                        l.parse::<$type>().ok().unwrap()
                    } else {
                        None.unwrap()
                    }
                },
            )* )
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
