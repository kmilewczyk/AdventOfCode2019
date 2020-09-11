/// krok indukcyjny:
/// jesteśmy na liczbie spełniającej warunek nie zmniejszenia
/// chcemy zbadać następną spełaniająca warunek liczbę
/// możemy użyć tylko liczbę o 1 większą niż cyfra w następnym rzędzie wielkości
/// 124444 -> 124445 --> 12 4449 -> 12 4455 --> 12 4499 -> 12 4555
/// operacja aby uzyskać cyfrę rzędu n: (<liczba> / 10^(n-1)) mod 10
/// Jeżeli dochodzimy do cyfry 9 to martwimy się rzędem wielkości wyżej
/// Jeżeli możemy inkremenotwać ten rząd to przechodzimy spowrotem niżej
/// Zaprzestań pętli gdy następna liczba jest spoza zakresu

use anyhow;

use std::io;
use std::io::prelude::*;
use std::fs;


pub fn count_diffrent_passwords(filepath: String) -> anyhow::Result<()> {
    let (lower_bound, upper_bound) = get_input(filepath)?;

    let answer: usize = {
        let mut count = 0;
        // Simple iteration should be enough for 6 digit numbers
        for number in lower_bound..upper_bound+1 {
            if check_for_password(number) {
                count += 1;
            }
        }

        count
    };

    println!("There are {} different passwords", answer);

    Ok(())
}

pub fn count_diffrent_passwords_part2(filepath: String) -> anyhow::Result<()> {
    let (lower_bound, upper_bound) = get_input(filepath)?;

    let answer: usize = {
        let mut count = 0;
        for number in lower_bound..upper_bound+1 {
            if check_for_password_part2(number) {
                count += 1;
                println!("{}", number);
            }
        }

        count
    };

    println!("There are {} different passwords", answer);

    Ok(())
}

fn get_input(filepath: String) -> anyhow::Result<(isize, isize)> {
    let file = io::BufReader::new(fs::File::open(filepath)?);
    let values: Vec<isize> = file.lines().nth(0).ok_or(anyhow!("File was empty"))??
        .split("-").take(2)
        .filter_map(|s| s.parse::<isize>().ok())
        .collect();

    Ok((*values.get(0).ok_or(anyhow!("Invalid input")).unwrap(),
     *values.get(1).ok_or(anyhow!("Only one number was given")).unwrap()))
}

fn check_for_password(mut number: isize) -> bool {
    let mut adjacent = false;
    let mut prv = number % 10;
    number /= 10;

    while number > 0 {
        let digit = number % 10;
        if digit == prv { // check adjacency
            adjacent = true;
        } else if prv < digit {
            return false;
        }
        else {
            prv = digit;
        }

        number /= 10;
    }

    adjacent
}

fn check_for_password_part2(mut number: isize) -> bool {
    let mut adjacent = false;
    let mut prv = number % 10;
    let mut strike = 1; // how much adjecent numbers are in a given moment
    number /= 10;

    while number > 0 {
        let digit = number % 10;
        if digit == prv { // check adjacency
            strike += 1;
        } else {
            if strike == 2 {
                adjacent = true;
            };

            strike = 1;

            if prv < digit {
                return false;
            }

            prv = digit;
        }

        number /= 10;
    }

    strike == 2 || adjacent
}
