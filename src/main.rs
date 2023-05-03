
// Binary exponentiation via rekursif
fn pow_recursive(a: i32, b: i32) -> i32 {
    if b == 1 {
        return a;
    }
    else if b % 2 == 1 {
        return a * pow_recursive(a, (b - 1)/2) * pow_recursive(a, (b - 1)/2);
    }
    return pow_recursive(a, b/2) * pow_recursive(a, b/2);
}

// Binary exponentiation via iteratif
fn pow_iter(a: i32, b: i32) -> i32 {
    let mut res = 1;
    let mut base = a;
    let mut exp = b;

    while exp > 0 {
        if exp % 2 == 1 {
            res *= base;
        }
        base *= base;
        exp /= 2;
    }
    res
}

fn main() {

    let a: i32 = 3;
    let b: i32 = 4;
    let result_1: i32 = pow_recursive(a, b);
    let result_2: i32 = pow_iter(a, b);
    println!("{}^{} = {}", a, b, result_1);
    println!("{}^{} = {}", a, b, result_2);
}
