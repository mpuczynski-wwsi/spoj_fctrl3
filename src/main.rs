/**
 FCTRL3 - Dwie cyfry silni

Zadanie: Silnia
Niech n będzie nieujemną liczbą całkowitą. 
Liczbę n! (czytaj n-silnia) definiuje się następująco. 
Jeśli n ≤ 1, to n! = 1. Dla n > 1, n! jest równe iloczynowi 
wszystkich liczb od 1 do n, czyli n! = 1 * 2 * ... * n. 
Na przykład 4! = 1*2*3*4 = 24.

Zadanie
Napisz program, który:

    wczyta ze standardowego wejścia nieujemną liczbę całkowitą n,
    policzy cyfrę dziesiatek oraz cyfrę jedności w zapisie dziesiętnym liczby n!,
    wypisze wynik na standardowe wyjście.

Wejście
W pierwszej linii wejścia znajduje się jedna liczba całkowia D (1≤D≤30), 
oznaczjąca liczbę przypadków do rozważenia. 
Opis każdego przypadku składa się z jednej linii,
 w której znajduje się jedna nieujemna liczba całkowita n 
 (0 ≤ n ≤ 1 000 000 000).

Wyjście
Dla każdego przypadku z wejścia. Twój program powinien wypisać w 
osobnej linii dokładnie dwie cyfry (oddzielone pojedynczą spacją): 
cyfrę dziesiątek i cyfrę jedności liczby n! zapisanej w systemie dziesiętnym.

Przykład
Dla danych wejściowych:

2
1
4

poprawną odpowiedzią jest:

0 1
2 4
 
*/


fn factorial(n: u8) -> u32 {
    if n <= 1 {
        1u32
    } else if n > 9 {
        0u32
    } else {
        let mut sum: u32 = 1;
        for i in 1..=n {
            sum = sum * i as u32;
        } 
        sum
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("err");

    let t:u16 = buffer.trim().parse().unwrap();
    for _ in 0..t{
        buffer.clear();
        stdin.read_line(&mut buffer).expect("err");
        let n:u8 = buffer.trim().parse().unwrap();
        let fact: Vec<_> = factorial(n).to_string().chars().rev().collect();
        match fact.len() {
            n if n > 9 => println!("{} {}", 0, 0),
            n if n == 1 => println!("{} {}", 0, fact[0]),
            _ => println!("{} {}", fact[1], fact[0]),
 
        }
    }
}
