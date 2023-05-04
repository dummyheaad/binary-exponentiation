
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

    // lakukan iterasi selama exp masih bernilai > 0
    while exp > 0 {
        // periksa bit paling kanan pada iterasi sekarang
        // jika bit bernilai satu maka
        if exp % 2 == 1 {

            // masukkan a^(2^k) ke dalam perkalian hasil akhir
            res *= base;
        }

        // jika bit tidak bernilai 1 maka jangan masukkan a^(2^k) ke dalam perkalian hasil akhir

        // lakukan update base.
        // Proses ini akan mengubah nilai base dari a^(2^0), a^(2^1), a^(2^2), a^(2^3), dan seterusnya
        base *= base;

        // Berpindah ke bit selanjutnya (di sebelah kiri dari bit terkanan)
        // Operasi ini juga bisa dilakukan dengan right shift (>>)
        exp /= 2;
    }

    // kembalikan hasil akhir
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
