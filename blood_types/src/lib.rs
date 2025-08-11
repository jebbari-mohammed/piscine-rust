use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(format!("Invalid antigen: {}", s)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(format!("Invalid Rh factor: {}", s)),
        }
    }
}

impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err("Input too short".to_string());
        }
        let rh_char = &s[s.len() - 1..];
        let antigen_str = &s[..s.len() - 1];

        let antigen = antigen_str.parse::<Antigen>()?;
        let rh_factor = rh_char.parse::<RhFactor>()?;

        Ok(BloodType { antigen, rh_factor })
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            other_order => other_order,
        }
    }
}

impl PartialOrd for BloodType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::AB => "AB",
            Antigen::B => "B",
            Antigen::O => "O",
        };

        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{}{}", antigen_str, rh_str)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let antigen_ok = match other.antigen {
            Antigen::O => true,
            Antigen::A => self.antigen == Antigen::A || self.antigen == Antigen::AB,
            Antigen::B => self.antigen == Antigen::B || self.antigen == Antigen::AB,
            Antigen::AB => self.antigen == Antigen::AB,
        };

        let rh_ok = match other.rh_factor {
            RhFactor::Negative => true,
            RhFactor::Positive => self.rh_factor == RhFactor::Positive,
        };

        antigen_ok && rh_ok
    }

    pub fn donors(&self) -> Vec<Self> {
        let antigens = [Antigen::O, Antigen::A, Antigen::B, Antigen::AB];
        let rh_factors = [RhFactor::Positive, RhFactor::Negative];

        let mut result = vec![];
        for antigen in &antigens {
            for rh in &rh_factors {
                let candidate = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh.clone(),
                };
                if self.can_receive_from(&candidate) {
                    result.push(candidate);
                }
            }
        }
        result
    }

    pub fn recipients(&self) -> Vec<Self> {
        let antigens = [Antigen::O, Antigen::A, Antigen::B, Antigen::AB];
        let rh_factors = [RhFactor::Positive, RhFactor::Negative];

        let mut result = vec![];
        for antigen in &antigens {
            for rh in &rh_factors {
                let candidate = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh.clone(),
                };
                if candidate.can_receive_from(self) {
                    result.push(candidate);
                }
            }
        }
        result
    }
}
