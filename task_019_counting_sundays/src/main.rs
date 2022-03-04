extern crate termion;
use termion::{clear};
//use termion::{clear, cursor};
//use std::collections::HashMap;

//extern crate unicode_segmentation;
//use unicode_segmentation::UnicodeSegmentation;

use std::collections::HashMap;


mod year;

fn main() {
    println!("{}", clear::All);

    let numbers = find_counting_sundays();

    println!("Counting sundays: {}", numbers);
}

fn find_counting_sundays() -> u32 {
    let mut sundays: u32 = 0;
    let mut day_number: u8 = 1;

    for year in 1900..(2000 + 1) {
        let months_with_days = get_months_with_days(year);
        for month in 1..13 {
            let days = months_with_days.get(&month).unwrap();
            for day in 1..(days + 1) {
                if day == 1 && is_sunday(day_number) && year > 1900 {
                    sundays += 1;
                }
                day_number += 1;
                if day_number > 7 {
                    day_number = 1;
                }
            }
        }
    }

    return sundays;
}

fn is_sunday(day_number: u8) -> bool {
    day_number == 7
}

fn get_months_with_days(year: u32) -> HashMap<u32, u32> {
    let mut map: HashMap<u32, u32> = HashMap::with_capacity(12);
    map.insert(1, 31);
    map.insert(2, get_days_of_february(year));
    map.insert(3, 31);
    map.insert(4, 30);
    map.insert(5, 31);
    map.insert(6, 30);
    map.insert(7, 31);
    map.insert(8, 31);
    map.insert(9, 30);
    map.insert(10, 31);
    map.insert(11, 30);
    map.insert(12, 31);

    map
}

fn get_days_of_february(year: u32) -> u32 {
    if year::is_leap_year(year) {
        29
    } else {
        28
    }
}

#[test]
fn test_get_days_of_february() {
    assert!(get_days_of_february(1900) == 28);
    assert!(get_days_of_february(1901) == 28);
    assert!(get_days_of_february(1902) == 28);
    assert!(get_days_of_february(1903) == 28);
    assert!(get_days_of_february(1904) == 29);
    assert!(get_days_of_february(1905) == 28);
    assert!(get_days_of_february(1906) == 28);
    assert!(get_days_of_february(1907) == 28);
    assert!(get_days_of_february(1908) == 29);
    assert!(get_days_of_february(1909) == 28);
    assert!(get_days_of_february(1910) == 28);
    assert!(get_days_of_february(1911) == 28);
    assert!(get_days_of_february(1912) == 29);
    assert!(get_days_of_february(1913) == 28);
    assert!(get_days_of_february(1914) == 28);
    assert!(get_days_of_february(1915) == 28);
    assert!(get_days_of_february(1916) == 29);
    assert!(get_days_of_february(1917) == 28);
    assert!(get_days_of_february(1918) == 28);
    assert!(get_days_of_february(1919) == 28);
    assert!(get_days_of_february(1920) == 29);
    assert!(get_days_of_february(1921) == 28);
    assert!(get_days_of_february(1922) == 28);
    assert!(get_days_of_february(1923) == 28);
    assert!(get_days_of_february(1924) == 29);
    assert!(get_days_of_february(1925) == 28);
    assert!(get_days_of_february(1926) == 28);
    assert!(get_days_of_february(1927) == 28);
    assert!(get_days_of_february(1928) == 29);
    assert!(get_days_of_february(1929) == 28);
    assert!(get_days_of_february(1930) == 28);
    assert!(get_days_of_february(1931) == 28);
    assert!(get_days_of_february(1932) == 29);
    assert!(get_days_of_february(1933) == 28);
    assert!(get_days_of_february(1934) == 28);
    assert!(get_days_of_february(1935) == 28);
    assert!(get_days_of_february(1936) == 29);
    assert!(get_days_of_february(1937) == 28);
    assert!(get_days_of_february(1938) == 28);
    assert!(get_days_of_february(1939) == 28);
    assert!(get_days_of_february(1940) == 29);
    assert!(get_days_of_february(1941) == 28);
    assert!(get_days_of_february(1942) == 28);
    assert!(get_days_of_february(1943) == 28);
    assert!(get_days_of_february(1944) == 29);
    assert!(get_days_of_february(1945) == 28);
    assert!(get_days_of_february(1946) == 28);
    assert!(get_days_of_february(1947) == 28);
    assert!(get_days_of_february(1948) == 29);
    assert!(get_days_of_february(1949) == 28);
    assert!(get_days_of_february(1950) == 28);
    assert!(get_days_of_february(1951) == 28);
    assert!(get_days_of_february(1952) == 29);
    assert!(get_days_of_february(1953) == 28);
    assert!(get_days_of_february(1954) == 28);
    assert!(get_days_of_february(1955) == 28);
    assert!(get_days_of_february(1956) == 29);
    assert!(get_days_of_february(1957) == 28);
    assert!(get_days_of_february(1958) == 28);
    assert!(get_days_of_february(1959) == 28);
    assert!(get_days_of_february(1960) == 29);
    assert!(get_days_of_february(1961) == 28);
    assert!(get_days_of_february(1962) == 28);
    assert!(get_days_of_february(1963) == 28);
    assert!(get_days_of_february(1964) == 29);
    assert!(get_days_of_february(1965) == 28);
    assert!(get_days_of_february(1966) == 28);
    assert!(get_days_of_february(1967) == 28);
    assert!(get_days_of_february(1968) == 29);
    assert!(get_days_of_february(1969) == 28);
    assert!(get_days_of_february(1970) == 28);
    assert!(get_days_of_february(1971) == 28);
    assert!(get_days_of_february(1972) == 29);
    assert!(get_days_of_february(1973) == 28);
    assert!(get_days_of_february(1974) == 28);
    assert!(get_days_of_february(1975) == 28);
    assert!(get_days_of_february(1976) == 29);
    assert!(get_days_of_february(1977) == 28);
    assert!(get_days_of_february(1978) == 28);
    assert!(get_days_of_february(1979) == 28);
    assert!(get_days_of_february(1980) == 29);
    assert!(get_days_of_february(1981) == 28);
    assert!(get_days_of_february(1982) == 28);
    assert!(get_days_of_february(1983) == 28);
    assert!(get_days_of_february(1984) == 29);
    assert!(get_days_of_february(1985) == 28);
    assert!(get_days_of_february(1986) == 28);
    assert!(get_days_of_february(1987) == 28);
    assert!(get_days_of_february(1988) == 29);
    assert!(get_days_of_february(1989) == 28);
    assert!(get_days_of_february(1990) == 28);
    assert!(get_days_of_february(1991) == 28);
    assert!(get_days_of_february(1992) == 29);
    assert!(get_days_of_february(1993) == 28);
    assert!(get_days_of_february(1994) == 28);
    assert!(get_days_of_february(1995) == 28);
    assert!(get_days_of_february(1996) == 29);
    assert!(get_days_of_february(1997) == 28);
    assert!(get_days_of_february(1998) == 28);
    assert!(get_days_of_february(1999) == 28);
    assert!(get_days_of_february(2000) == 29);
    assert!(get_days_of_february(2001) == 28);
}

#[test]
fn test_find_counting_sundays() {
    assert!(find_counting_sundays() == 171);
}