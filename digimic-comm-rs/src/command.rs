use chrono::{DateTime, Utc, NaiveDate};
use logos::Logos;
use std::{num::ParseIntError, str::FromStr, string::ParseError};




#[derive(PartialEq, PartialOrd, Debug)]
pub enum Micrometer {
    QUERY(f64),
    LIN(i64),  //LIN ? //TCOR?
    SN(i64),   // SN? -> FACCFGSN(XXXXXX)
    LCAL(NaiveDate),
    BTDN(String),
    VERX(String),
    ERR(i64), // ERR2/ERR1
    UnknownCommand(String),
    REF,
}



//todo; lexer can take command in args of token/regex so can be reformatted to use singular enum
#[derive(Logos)]
enum CommandKind {
    #[token("REF!")]
    Ref,
    #[token("SN")]
    Sn,
    #[token("LIN")]
    Lin,
    #[regex(r"[0-9]{2}\.[0-9]{2}\.[0-9]{4}")]
    Lcal,
    #[regex(r"[-+]?([0-9]*\.[0-9]+|[0-9]+)")]
    Reading,
    #[regex(r"\d{1}.\d{2}.\d{4}\w")]
    Verx,
    #[regex(r"HCT-MM\d{3}-\d{7}")]
    Btdn,
    #[token("ERR")]
    MicErr,
}

impl FromStr for Micrometer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lexer = CommandKind::lexer(s);
        let kind = lexer.next().unwrap(); //next here returns an option of a result
        let remainder = lexer.remainder();
        Ok(match kind {
            Ok(CommandKind::Ref) => Micrometer::REF,
            Ok(CommandKind::Sn) => Micrometer::SN(remainder.parse::<i64>()?),
            Ok(CommandKind::Lin) => Micrometer::LIN(
                remainder
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<i64>()?,
            ),
            Ok(CommandKind::Lcal) => {
                Micrometer::LCAL(NaiveDate::parse_from_str(s, "%d.%m.%Y").unwrap())
            },
            Ok(CommandKind::Reading) => {
                Micrometer::QUERY(s.get(0..s.len()).unwrap_or("0.0").parse::<f64>()?)
            }
            Ok(CommandKind::Verx) => Micrometer::VERX(s.to_string()),
            Ok(CommandKind::Btdn) => Micrometer::BTDN(s.to_string()),
            Ok(CommandKind::MicErr) => Micrometer::ERR(remainder.parse::<i64>()?),

            _ => Micrometer::UnknownCommand(s.to_string()),
        })
    }
}


#[cfg(test)]
mod tests {
    use std::{str::FromStr};
    use chrono::NaiveDate;
    use crate::{command::{self, Micrometer}};

    #[test]
    fn test_parse_sn() {
        let command = command::Micrometer::from_str("SN123456");
        assert_eq!(command.unwrap(), Micrometer::SN(123456));
    }
    #[test]
    fn test_parse_lin() {
        let command = command::Micrometer::from_str("LIN +033");
        assert_eq!(command.unwrap(), Micrometer::LIN(33));
    }
    #[test]
    fn test_parse_lcal() {
        let from_ymd_opt = NaiveDate::from_ymd_opt;
        let command = command::Micrometer::from_str("08.09.2023");
        assert_eq!(command.unwrap(), Micrometer::LCAL(from_ymd_opt(2023, 9, 8).unwrap()));
    }

    #[test]
    fn test_parse_mic_reading() {
        let command = command::Micrometer::from_str("+000.001");
        assert_eq!(command.unwrap(), Micrometer::QUERY(0.001));

        let command = command::Micrometer::from_str("-000.001");
        assert_eq!(command.unwrap(), Micrometer::QUERY(-0.001));
    }

    #[test]
    fn test_bad_reading() {
        let command = command::Micrometer::from_str("WG23TG23GE");
        assert_eq!(command.unwrap(), Micrometer::UnknownCommand("WG23TG23GE".to_string()));
    }

    
    #[test]
    fn test_parse_verx() {
        let command = command::Micrometer::from_str("1.03.1046H");
        assert_eq!(command.unwrap(), Micrometer::VERX("1.03.1046H".to_string()));
    }

    #[test]
    fn test_parse_btdn() {
        let command = command::Micrometer::from_str("HCT-MM025-1234567");
        assert_eq!(command.unwrap(), Micrometer::BTDN("HCT-MM025-1234567".to_string()));
    }

    #[test]
    fn test_parse_ref() {
        let command = command::Micrometer::from_str("REF!");
        assert_eq!(command.unwrap(), Micrometer::REF);
    }

    #[test]
    fn test_parse_err() {
        let command = command::Micrometer::from_str("ERR2");
        assert_eq!(command.unwrap(), Micrometer::ERR(2));
    }


}
