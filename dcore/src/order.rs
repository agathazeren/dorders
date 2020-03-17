use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt;



pub enum Order {
    Hold(UnitType, Province),
    Move(UnitType, Province, Province),
    Support(UnitType, Province, UnitType, Province, Province),
    Convoy(UnitType, Province, UnitType, Province, Province),
}

pub enum UnitType {
    Army,
    Fleet,
}

pub struct Province(u8);

pub enum ParseError {
    UnknownError,
    BadUnitType(String),
    MissingComponent,
    BadProvince(String),
}

impl std::str::FromStr for Order {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Order, Self::Err> {
        let mut parts = s.split(&[' ', '\t', '-'] as &[char]).map(str::trim);
        let unit_type = match parts.next() {
            Some("A") => UnitType::Army,
            Some("F") => UnitType::Fleet,
            Some(other) => return Err(ParseError::BadUnitType(other.into())),
            None => return Err(ParseError::MissingComponent),
        };
        let unit_origin = parts
            .next()
            .ok_or(ParseError::MissingComponent)?
            .parse::<Province>()?;
        match parts.next() {
            Some(p) if p.parse::<Province>().is_ok() => {
                Ok(Order::Move(unit_type, unit_origin, p.parse::<Province>()?))
            }
            Some("H") => Ok(Order::Hold(unit_type, unit_origin)),
            Some("S") => {
                let target = parse_targets(&mut parts)?;
                Ok(Order::Support(
                    unit_type,
                    unit_origin,
                    target.0,
                    target.1,
                    target.2,
                ))
            }
            Some("C") => {
                let target = parse_targets(&mut parts)?;
                Ok(Order::Convoy(
                    unit_type,
                    unit_origin,
                    target.0,
                    target.1,
                    target.2,
                ))
            }
            Some(_) => Err(ParseError::UnknownError),
            None => Err(ParseError::MissingComponent),
        }
    }
}

fn parse_targets<'a, I: Iterator<Item = &'a str>>(
    parts: &mut I,
) -> Result<(UnitType, Province, Province), ParseError> {
    let target_unit_type = match parts.next() {
        Some("A") => UnitType::Army,
        Some("F") => UnitType::Fleet,
        Some(x) => return Err(ParseError::BadUnitType(x.into())),
        None => return Err(ParseError::MissingComponent),
    };
    let target_unit_from = parts
        .next()
        .ok_or(ParseError::MissingComponent)?
        .parse::<Province>()?;
    let target_unit_to = parts
        .next()
        .ok_or(ParseError::MissingComponent)?
        .parse::<Province>()?;
    Ok((target_unit_type, target_unit_from, target_unit_to))
}

impl std::str::FromStr for Province {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Province, Self::Err> {
        match PROVINCE_IDS.get(&s.trim().to_lowercase()) {
            Some(p) => Ok(Province(*p)),
            None => Err(ParseError::BadProvince(s.to_string())),
        }
    }
}

impl Display for Order{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            Self::Move(ty,fm,to) => write!(f,"{} {}-{}",ty,fm,to),
            Self::Hold(ty,prov) => write!(f,"{} {} H",ty,prov),
            Self::Convoy(ty,prov,t_ty,t_fm,t_to) => write!(f,"{} {} C {} {}-{}",ty,prov,t_ty,t_fm,t_to),
            Self::Support(ty,prov,t_ty,t_fm,t_to) => write!(f,"{} {} S {} {}-{}",ty,prov,t_ty,t_fm,t_to),
        }
    }

}

impl Display for UnitType{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"{}",match self {
            Self::
            Army => "A",
            Self::Fleet => "F",
        })
    }
}

impl Display for Province{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result{
        write!(f,"{}",PROVINCE_IDS.iter().find(|(_,id)|**id==self.0).unwrap().0)
    }

}

impl Display for ParseError{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)-> fmt::Result{
        match self{
            Self::UnknownError => write!(f,"unknown error parseing orders"),
            Self::BadUnitType(s) => write!(f,"bad unit type \"{}\"",s),
            Self::MissingComponent => write!(f,"missing component of order"),
            Self::BadProvince(s) => write!(f,"bad province \"{}\"",s),
        }
    }
}

impl Order{
    pub fn to_bytes(&self)->Vec<u8>{
        match self{
            Self::Move(ty,fm,to) => vec![0 | ty.to_u1()<<2,fm.to_u7(),to.to_u7()],
            Self::Hold(ty,prov) => vec![1 | ty.to_u1()<<2,prov.to_u7()],
            Self::Support(ty,prov,t_ty,t_fm,t_to) => vec![2 | ty.to_u1()<<2 | t_ty.to_u1()<<3,prov.to_u7(),t_fm.to_u7(),t_to.to_u7()],
            Self::Convoy(ty,prov,t_ty,t_fm,t_to) => vec![3 | ty.to_u1()<<2 | t_ty.to_u1()<<3,prov.to_u7(),t_fm.to_u7(),t_to.to_u7()],
        }
    }
}

impl Province{
    fn to_u7(&self)->u8{
        self.0
    }
}

impl UnitType{
    fn to_u1(&self)->u8{
        match self{
            Self::Army => 0,
            Self::Fleet => 1,

        }
    }
}
    



lazy_static! {
    static ref PROVINCE_IDS: HashMap<String, u8> = {
        include_str!("provinces.txt")
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut parts_itr = s.split("=").map(str::trim);
                let prov = parts_itr.next().expect("Bad line in provinces.txt");
                let id = parts_itr.next().expect("Bad line in provines.txt");
                assert!(
                    parts_itr.next() == None,
                    "Bad line \"{}\" in provinces.txt",
                    s
                );
                (
                    prov,
                    id.parse::<u8>()
                        .expect("Province Ids must be integers in [0,256)"),
                )
            })
            .map(|(prov, id)| (prov.to_lowercase(), id))
            .collect::<HashMap<String, u8>>()
    };
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn order_parseing() {
        todo!()
    }
}
