use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

/// Read in lines of a file to a vector
///
/// # Arguments
///
/// * `filename` - String containing the filename
pub fn read_in_lines(filename: &str) -> Vec<String> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .flatten()
        .collect()
}

/// Read in lines of a file to a vector
///
/// # Arguments
///
/// * `filename` - String containing the filename
pub fn read_in_lines_as<T>(filename: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .flatten()
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}

/// Read in lines and chars of a file to a matrix
///
/// # Arguments
///
/// * `filename` - String containing the filename
pub fn read_in_matrix(filename: &str) -> Vec<Vec<char>>
{
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .flatten()
        .map(|x| x.chars().collect())
        .collect()
}

/// Read in lines and chars of a file to a matrix
///
/// # Arguments
///
/// * `filename` - String containing the filename
pub fn read_in_matrix_as<T>(filename: &str) -> Vec<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .flatten()
        .map(
            |x| x.chars()
                .map(|x| String::from(x).parse::<T>().unwrap())
                .collect()
        )
        .collect()
}

/// Read in lines and chars of a file to a matrix
///
/// # Arguments
///
/// * `filename` - String containing the filename
pub fn read_in_csv_matrix(filename: &str) -> Vec<Vec<String>>
{
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .flatten()
        .map(
            |x| x.split(",")
                .map(|s| String::from(s))
                .collect()
        )
        .collect()
}

/// Read in lines and chars of a file to a matrix
///
/// # Arguments
///
/// * `filename` - String containing the filename
pub fn read_in_csv_matrix_as<T>(filename: &str) -> Vec<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .flatten()
        .map(
            |x| x.split(",")
                .map(|s| s.parse::<T>().unwrap())
                .collect()
        )
        .collect()
}

/// Lists of values separated by empty lines
pub fn read_in_chunks(filename: &str) -> Vec<Vec<String>> {
    let mut chunks = Vec::new();
    let mut chunk = Vec::new();
    for line in read_in_lines(filename) {
        if line.trim().is_empty() {
            chunks.push(chunk.clone());
            chunk = Vec::new();
        } else {
            chunk.push(line);
        }
    }
    chunks
}

pub fn read_in_chunks_to_map(filename: &str, item_separator: &str, map_separator: &str) -> Vec<HashMap<String, String>> {
    let mut maps = Vec::new();
    let mut map = HashMap::new();
    for line in read_in_lines(filename) {
        if line.trim().is_empty() {
            maps.push(map.clone());
            map = HashMap::new();
        } else {
            for pair in line.split(item_separator) {
                let (k, v) = pair.split_once(map_separator).unwrap();
                map.insert(String::from(k), String::from(v));
            }
        }
    }
    maps
}
