use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        if n == 0 { RomanDigit::Nulla }
        else if n == 1 { RomanDigit::I }
        else if n == 5 { RomanDigit::V }
        else if n == 10 { RomanDigit::X }
        else if n == 50 { RomanDigit::L }
        else if n == 100 { RomanDigit::C }
        else if n == 500 { RomanDigit::D }
        else if n == 1000 { RomanDigit::M }
        else { panic!("not a roman digit") }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut n: u32) -> Self {
        let mut result = Vec::new();
        
        if n == 0 {
            result.push(Nulla);
            return RomanNumber(result);
        }
        
        while n >= 1000 { result.push(M); n -= 1000; }
        while n >= 900 { result.push(C); result.push(M); n -= 900; }
        while n >= 500 { result.push(D); n -= 500; }
        while n >= 400 { result.push(C); result.push(D); n -= 400; }
        while n >= 100 { result.push(C); n -= 100; }
        while n >= 90 { result.push(X); result.push(C); n -= 90; }
        while n >= 50 { result.push(L); n -= 50; }
        while n >= 40 { result.push(X); result.push(L); n -= 40; }
        while n >= 10 { result.push(X); n -= 10; }
        while n >= 9 { result.push(I); result.push(X); n -= 9; }
        while n >= 5 { result.push(V); n -= 5; }
        while n >= 4 { result.push(I); result.push(V); n -= 4; }
        while n >= 1 { result.push(I); n -= 1; }
        
        RomanNumber(result)
    }
}