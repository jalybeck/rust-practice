pub fn add<T>(left: T, right: T) -> T 
where
    T: std::ops::Add<Output = T>,
{
    left + right
}

pub fn multiply<T>(left: T, right: T) -> T 
where
    T: std::ops::Mul<Output = T>,
{
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let result = multiply(2, 3);
        assert_eq!(result, 6);

        let result = add(2.5, 3.5);
        assert_eq!(result, 6.0);

        let result = multiply(2.5, 3.5);
        assert_eq!(result, 8.75);
    }
}
