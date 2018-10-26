extern crate foo;

use foo::bar;

bar!();

#[cfg(test)]
mod tests {
    use super::BAR;
    #[test]
    fn it_works() {
        println!("{}", BAR);
    }
}
