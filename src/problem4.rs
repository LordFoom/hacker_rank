/*
*  Given two strings s1 and s2, return 1 if s2 is a rotation of s1 but not identical to s1, otherwise return 0.
* Complete the 'isNonTrivialRotation' function below.
*
* The function is expected to return a BOOLEAN.
* The function accepts following parameters:
*  1. STRING s1
*  2. STRING s2
*  1 <= |s1| <= 1000
*  1 <= |s2| <= 1000
*  |s1| = |s2|
*/

fn isNonTrivialRotation(s1: &str, s2: &str) -> bool {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    s1_chars.sort();
    s2_chars.sort();
}
