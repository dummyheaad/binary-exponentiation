# Binary Exponentiation


## Deskripsi
Binary Exponentiation merupakan sebuah algoritma yang digunakan untuk menghitung hasil perpangkatan suatu bilangan dengan bilangan tertentu.  
Secara umum, bentuk perpangkatan dapat dituliskan dalam notasi:  

$$\huge y\ =\ a^{x}$$

- y = hasil operasi eksponen
- a = basis (bilangan yang akan dipangkatkan)
- x = eksponen (bilangan pemangkat)

Hasil perpangkatan dihitung secara <em>naive</em> dengan cara mengalikan basis (a) dengan diri nya sendiri sebanyak nilai eksponen(x).

$$\huge y\ =\ a\ *\ a\ *\ a\ *\ \...\ *\ a\ (x\ kali)$$

Berdasarkan metode di atas terlihat bahwa terdapat x kali proses komputasi untuk menghitung nilai perpangkatan.  
Dengan menggunakan binary exponentiation, proses kompuasi di atas dapat direduksi sehingga mampu meningkatkan utilisasi <em>resource</em> komputer serta mampu melakukan komputasi eksponen yang lebih besar dengan <em>runtime</em> yang lebih kecil.

## Cara Kerja
Algoritma ini bekerja dengan cara mendekomposisi eksponen (x) menjadi bilangan dalam perpangkatan 2 (biner). Kemudian proses perhitungan dilakukan dengan mengalikan masing-masing nilai basis yang sudah dipangkatkan dengan nilai dalam basis 2 tersebut. Proses ini dapat dilakukan secara rekursif maupun iteratif.  
Secara umum, notasi $a^{b}$ dapat dituliskan dalam perpangkatan dengan eksponen dalam pangkat 2 sebagai berikut:

$$\huge a^{b}\ =\ b_{0}\*a^{(2^0)}\\ *\\ b_{1}\*a^{(2^1)}\\ *\\ b_{2}\*a^{(2^2)}\\ *\\ ...\\ *\\ b_{n-1}\*a^{(2^{n-1})}$$

- b<sub>k</sub> = nilai bit pada posisi ke-(k+1) dari kanan dari representasi biner eksponen x [k = 0, 1, 2, 3, .. n-1]  
  nilai ini bisa 0 atau 1.  
  0 berarti nilai $a^{2^k}$ tidak dimasukkan ke perkalian  
  1 berarti nilai $a^{2^k}$ dimasukkan ke perkalian  
  Cara untuk menentukan apakah $a^{2^k}$ masuk/tidak masuk ke perkalian:
  - Bagi nilai eksponen b dengan 2 kemudian cek hasilnya
    - Jika hasilnya adalah 0 maka bit pada posisi k+1 dari kanan bernilai 0 sehingga $a^{2^k}$ tidak dimasukkan ke perkalian
    - Jika hasilnya adalah 1 maka bit pada posisi k+1 dari kanan bernilai 1 sehinggia $a^{2^k}$ dimasukkan ke perkalian
- a = basis
- b = eksponen
  nilai b dinyatakan dalam notasi biner, yaitu b = b<sub>n-1</sub>b<sub>n-2</sub>b<sub>n-3</sub> ... b<sub>0</sub>

Contoh: Berapakah hasil dari $5^{5}$ ?  

Implementasi Rekursif:
- Recursive Call
$$\huge 5^{5}\ =\ 5^{1}\ *\ 5^{2}\ *\ 5^{2}$$
$$\huge 5^{2}\ =\ 5^{1}\ *\ 5^{1}$$
$$\huge 5^{1}\ =\ 5$$

- Backtracking
$$\huge 5^{1}\ =\ 5$$
$$\huge 5^{2}\ =\ 5^{1}\ *\ 5^{1}\ =\ 25$$
$$\huge 5^{5}\ =\ 5^{1}\ *\ 5^{2}\ *\ 5^{2} =\ 5\ *\ 25 *\ 25\ =\ 3125$$

Implementasi Iteratif:  
x = 5 -> 101 (dalam notasi biner)  
Terlihat bahwa $5^5$ dapat dihitung dengan mengalikan nilai $5^1$ dan $5^4$  
Atau dalam notasi eksponen dalam perpangkatan 2 dapat dituliskan menjadi:

$$\huge 5^5\ =\ 5^{(2^0)}\ *\ 5^{(2^2)}$$

Proses iterasi:
  - Mulai dari bit paling kanan, lakukan pengecekan nilai bit dengan melakukan operasi modulo antara nilai eksponen dengan 2.
    - Jika hasil bernilai 1 maka masukkan $a^{(2^0)}$ ke dalam perkalian hasil akhir
    - Jika hasil bernilai 0 maka jangan masukkan $a^{(2^0)}$ ke dalam perkalian hasil akhir
  - Pindah ke bit selanjutnya dengan cara membagi nilai eksponen dengan 2 atau melakukan operasi right-shift sebanyak 1 kali
  - Lakukan lagi pengecekan bit dengan melakukan operasi modulo antara nilai eksponen (yang sudah dimodifikasi) dengan 2.
  - Ulangi proses yang sama sampai diperoleh nilai eksponen = 0
