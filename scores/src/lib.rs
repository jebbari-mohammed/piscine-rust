pub fn score(s : &str) -> u64 {
    let mut score = 0;
    for ch in s.to_uppercase().chars() {
        if ch == 'A' || ch == 'E' || ch == 'I' || ch == 'O' || ch == 'U' || ch == 'L' || ch == 'N' || ch == 'R' || ch == 'S' || ch == 'T' {
            score += 1;
        };
        if ch == 'D' || ch == 'G' {
            score += 2;
        };
        if ch == 'B' || ch == 'C' || ch == 'M' || ch == 'P'  {
            score += 3;
        };
        if ch == 'F' || ch == 'H' || ch == 'V' || ch == 'W' || ch == 'Y'  {
            score += 4;
        };
        if ch == 'K'  {
            score += 5;
        };
        if ch == 'J' || ch == 'X'  {
            score += 8;
        };
        if ch == 'Q' || ch == 'Z'  {
            score += 10;
        };
    };
    score
}