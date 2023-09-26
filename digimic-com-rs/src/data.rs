use chrono::NaiveDate;
use logos::{Logos, Lexer};
use std::str::FromStr;



fn query(lex: &mut Lexer<CommandKind>) -> Option<f64> {
    Some(lex.source().get(0..lex.source().len()).unwrap_or("0.0").parse::<f64>().ok()?)
}
fn lcal(lex: &mut Lexer<CommandKind>) -> Option<NaiveDate> {
    Some(NaiveDate::parse_from_str(lex.source(), "%d.%m.%Y").ok()?)
}
fn sn(lex: &mut Lexer<CommandKind>) -> Option<i64> {
    Some(lex.remainder().parse::<i64>().ok()?)
}
fn lin(lex: &mut Lexer<CommandKind>) -> Option<i64> {
    Some(lex.remainder().chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i64>().ok()?)
}
fn err(lex: &mut Lexer<CommandKind>) -> Option<i64> {
    Some(lex.remainder().parse::<i64>().ok()?)
}


//todo; lexer can take command in args of token/regex so can be reformatted to use singular enum
#[derive(Logos, PartialEq, PartialOrd, Debug)]
enum CommandKind {
    #[token("REF!")]
    REF, // REF!
    #[token("SN", sn)]
    SN(i64), // SN? -> FACCFGSN(XXXXXX)
    #[token("LIN", lin)]
    LIN(i64),   //LIN ? 
    #[regex(r"[0-9]{2}\.[0-9]{2}\.[0-9]{4}", lcal)]
    LCAL(NaiveDate), // dd.mm.yyyy
    #[regex(r"[-+]?([0-9]*\.[0-9]+|[0-9]+)", query)]
    QUERY(f64), // +000.0001
    #[regex(r"\d{1}.\d{2}.\d{4}\w", |lex| lex.source().to_string())]
    VERX(String), 
    #[regex(r"HCT-MM\d{3}-\d{7}", |lex| lex.source().to_string())]
    BTDN(String),
    #[token("ERR",err)]
    ERR(i64),
    UnknownCommand(String),
}



impl FromStr for CommandKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lexer = CommandKind::lexer(s);
        let kind = lexer.next().unwrap().unwrap_or(CommandKind::UnknownCommand(format!("{s}"))); //next here returns an option of a result
        Ok(kind)
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::data::CommandKind;
    use std::str::FromStr;

    #[test]
    fn test_parse_sn() {
        let command = CommandKind::from_str("SN123456");
        assert_eq!(command.unwrap(), CommandKind::SN(123456));
    }
    #[test]
    fn test_parse_lin() {
        let command = CommandKind::from_str("LIN +033");
        assert_eq!(command.unwrap(), CommandKind::LIN(33));
    }
    #[test]
    fn test_parse_lcal() {
        let command = CommandKind::from_str("08.09.2023");
        assert_eq!(command.unwrap(), CommandKind::LCAL(from_ymd_opt(2023, 9, 8).unwrap()));
    }

    #[test]
    fn test_parse_mic_reading() {
        let command = CommandKind::from_str("+000.001");
        assert_eq!(command.unwrap(), CommandKind::QUERY(0.001));

        let command = CommandKind::from_str("-000.001");
        assert_eq!(command.unwrap(), CommandKind::QUERY(-0.001));
    }

    #[test]
    fn test_bad_reading() {
        let command = CommandKind::from_str("WG23TG23GE");
        assert_eq!(command.unwrap(), CommandKind::UnknownCommand("WG23TG23GE".to_string()));
    }

    
    #[test]
    fn test_parse_verx() {
        let command = CommandKind::from_str("1.03.1046H");
        assert_eq!(command.unwrap(), CommandKind::VERX("1.03.1046H".to_string()));
    }

    #[test]
    fn test_parse_btdn() {
        let command = CommandKind::from_str("HCT-MM025-1234567");
        assert_eq!(command.unwrap(), CommandKind::BTDN("HCT-MM025-1234567".to_string()));
    }

    #[test]
    fn test_parse_ref() {
        let command = CommandKind::from_str("REF!");
        assert_eq!(command.unwrap(), CommandKind::REF);
    }

    #[test]
    fn test_parse_err() {
        let command = CommandKind::from_str("ERR2");
        assert_eq!(command.unwrap(), CommandKind::ERR(2));
    }


}