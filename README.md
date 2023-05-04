# Binary Exponentiation


## Deskripsi
Binary Exponentiation merupakan sebuah algoritma yang digunakan untuk menghitung hasil perpangkatan suatu bilangan dengan bilangan tertentu.  
Secara umum, bentuk perpangkatan dapat dituliskan dalam notasi:  

$$\huge y\ =\ a^x$$

- y = hasil operasi eksponen
- a = basis (bilangan yang akan dipangkatkan)
- x = eksponen (bilangan pemangkat)

Hasil perpangkatan dihitung secara <em>naive</em> dengan cara mengalikan basis (a) dengan diri nya sendiri sebanyak nilai eksponen(x).

$$\huge y\ =\ a\ *\ a\ *\ a\ *\ \...\ *\ a\ (x\ kali)$$

Berdasarkan metode di atas terlihat bahwa terdapat x kali proses komputasi untuk menghitung nilai perpangkatan.  
Dengan menggunakan binary exponentiation, proses kompuasi di atas dapat direduksi sehingga mampu meningkatkan utilisasi <em>resource</em> komputer serta mampu melakukan komputasi eksponen yang lebih besar dengan <em>runtime</em> yang lebih kecil.

## Cara Kerja
Algoritma ini bekerja dengan cara mendekomposisi eksponen (x) menjadi bilangan perpangkatan 2 (biner). Kemudian proses perhitungan dilakukan dengan mengalikan masing-masing nilai basis yang sudah dipangkatkan dengan nilai dalam basis 2 tersebut. Proses ini dapat dilakukan secara rekursif maupun iteratif.  
Contoh Implementasi Rekursif:
- Recursive Call
$$\huge 5^5\ =\ 5^1\ *\ 5^2\ *\ 5^2$$
$$\huge 5^2\ =\ 5^1\ *\ 5^1$$
$$\huge 5^1\ =\ 5$$

- Backtracking
$$\huge 5^1\ =\ 5$$
$$\huge 5^2\ =\ 5^1\ *\ 5^1\ =\ 25$$
$$\huge 5^5\ =\ 5^1\ *\ 5^2\ *\ 5^2 =\ 5\ *\ 25 *\ 25\ =\ 3125$$

## Visualisasi
