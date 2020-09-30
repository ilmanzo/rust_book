fn main() {
    const NTH : u32 = 21;
    let fib=calculate_fib(NTH);
    println!("the {}th fibonacci number (starting from 0) is: {}",NTH,fib);
}

fn calculate_fib(n: u32) -> u32 {
    let mut n1=0;
    let mut n2=1;
    for _ in 1..n {
        let t=n2;
        n2+=n1;
        n1=t;
    }
    n1
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(calculate_fib(0), 0);
    }
    #[test]
    fn test_5() {
        assert_eq!(calculate_fib(5), 3);
    }
    #[test]
    fn test_20() {
        assert_eq!(calculate_fib(20),4181)
    }

}