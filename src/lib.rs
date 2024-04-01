#[macro_export]
macro_rules! if_not_matches {
    (($expression:expr, $pattern:pat) $else_expression:block ) => {
        let $pattern = $expression else { $else_expression };
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn works() {

        let a: Option<i32> = None;

        if_not_matches!{ 
            (a, Some(n)) { 
                println!("Did not match");
                return;
            }
        }

        println!("{}", n);
    }
}