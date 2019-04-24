# Assignment 2 Operating System
Assalamualaikum warahmatullahi wabarakatuh, alhamdulillah saya dapat menyelesaikan tugas ini tepat pada waktunya.
Tugas ini disusun untuk mata kuliah Sistem Operasi yang dibimbing oleh pak Eka, dalam essay ini saya akan menjelaskan
bagaimana cara menyelesaikan masalah level medium, "Perfect Number", yang terdapat di https://exercism.io .


## Perfect Number Solution
Problem ini meminta kita untuk menentukan apakah sebuah bilangan itu merupakan sempurna (perfect), berlimpah(abundant), atau kurang(deficient) berdasarkan skema klasifikasi Nicomachus (60M -120M) untuk bilangan asli.

Pengidentifikasian suatu bilangan asli itu perfect, abundant, atau deficient didapatkan dari penjumlahan faktor-faktor dari suatu bilangan tidak termasuk bilangan itu sendiri atau yang disebut dengan alikuot.


## Ketentuan
1. Sempurna (Perfect): Jumlah alikuot = bilangan
  > 6 adalah bilangan sempurna karena (1 + 2 + 3) = 6
  
2. Berlimpah (Abundant): Jumlah alikuot > bilangan
  > 12 adalah abundant karena (1 + 2 + 3 + 4 + 6) = 16
  
3. Kekurangan (Deficient): jumlah alikuot < bilangan
  > 8 adalah bilangan yang deficient karena (1 + 2 + 4) = 7

## Solusi saya

1. Untuk menyelesaikan test pertama, dimana kalau angkanya 0  hasilnya None, caranya begini
 if num == 0{
        return None;
    } 



 
