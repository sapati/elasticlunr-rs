use super::{
    common::{RustStemmer, StopWordFilter, RegexTrimmer},
    Language,
};
use crate::pipeline::Pipeline;
use rust_stemmers::Algorithm;

pub struct Romanian {}

impl Romanian {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for Romanian {
    fn name(&self) -> String {
        "Romanian".into()
    }
    fn code(&self) -> String {
        "ro".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        super::tokenize_whitespace(text)
    }

    fn make_pipeline(&self) -> Pipeline {
        Pipeline {
            queue: vec![
                Box::new(RegexTrimmer::new("trimmer-ro", r"\p{Latin}")),
                Box::new(StopWordFilter::new("stopWordFilter-ro", STOP_WORDS)),
                Box::new(RustStemmer::new("stemmer-ro", Algorithm::Romanian)),
            ],
        }
    }
}

const STOP_WORDS: &'static [&'static str] = &[
    "",
    "acea",
    "aceasta",
    "această",
    "aceea",
    "acei",
    "aceia",
    "acel",
    "acela",
    "acele",
    "acelea",
    "acest",
    "acesta",
    "aceste",
    "acestea",
    "aceşti",
    "aceştia",
    "acolo",
    "acord",
    "acum",
    "ai",
    "aia",
    "aibă",
    "aici",
    "al",
    "ale",
    "alea",
    "altceva",
    "altcineva",
    "am",
    "ar",
    "are",
    "asemenea",
    "asta",
    "astea",
    "astăzi",
    "asupra",
    "au",
    "avea",
    "avem",
    "aveţi",
    "azi",
    "aş",
    "aşadar",
    "aţi",
    "bine",
    "bucur",
    "bună",
    "ca",
    "care",
    "caut",
    "ce",
    "cel",
    "ceva",
    "chiar",
    "cinci",
    "cine",
    "cineva",
    "contra",
    "cu",
    "cum",
    "cumva",
    "curând",
    "curînd",
    "când",
    "cât",
    "câte",
    "câtva",
    "câţi",
    "cînd",
    "cît",
    "cîte",
    "cîtva",
    "cîţi",
    "că",
    "căci",
    "cărei",
    "căror",
    "cărui",
    "către",
    "da",
    "dacă",
    "dar",
    "datorită",
    "dată",
    "dau",
    "de",
    "deci",
    "deja",
    "deoarece",
    "departe",
    "deşi",
    "din",
    "dinaintea",
    "dintr-",
    "dintre",
    "doi",
    "doilea",
    "două",
    "drept",
    "după",
    "dă",
    "ea",
    "ei",
    "el",
    "ele",
    "eram",
    "este",
    "eu",
    "eşti",
    "face",
    "fata",
    "fi",
    "fie",
    "fiecare",
    "fii",
    "fim",
    "fiu",
    "fiţi",
    "frumos",
    "fără",
    "graţie",
    "halbă",
    "iar",
    "ieri",
    "la",
    "le",
    "li",
    "lor",
    "lui",
    "lângă",
    "lîngă",
    "mai",
    "mea",
    "mei",
    "mele",
    "mereu",
    "meu",
    "mi",
    "mie",
    "mine",
    "mult",
    "multă",
    "mulţi",
    "mulţumesc",
    "mâine",
    "mîine",
    "mă",
    "ne",
    "nevoie",
    "nici",
    "nicăieri",
    "nimeni",
    "nimeri",
    "nimic",
    "nişte",
    "noastre",
    "noastră",
    "noi",
    "noroc",
    "nostru",
    "nouă",
    "noştri",
    "nu",
    "opt",
    "ori",
    "oricare",
    "orice",
    "oricine",
    "oricum",
    "oricând",
    "oricât",
    "oricînd",
    "oricît",
    "oriunde",
    "patra",
    "patru",
    "patrulea",
    "pe",
    "pentru",
    "peste",
    "pic",
    "poate",
    "pot",
    "prea",
    "prima",
    "primul",
    "prin",
    "puţin",
    "puţina",
    "puţină",
    "până",
    "pînă",
    "rog",
    "sa",
    "sale",
    "sau",
    "se",
    "spate",
    "spre",
    "sub",
    "sunt",
    "suntem",
    "sunteţi",
    "sută",
    "sînt",
    "sîntem",
    "sînteţi",
    "să",
    "săi",
    "său",
    "ta",
    "tale",
    "te",
    "timp",
    "tine",
    "toate",
    "toată",
    "tot",
    "totuşi",
    "toţi",
    "trei",
    "treia",
    "treilea",
    "tu",
    "tăi",
    "tău",
    "un",
    "una",
    "unde",
    "undeva",
    "unei",
    "uneia",
    "unele",
    "uneori",
    "unii",
    "unor",
    "unora",
    "unu",
    "unui",
    "unuia",
    "unul",
    "vi",
    "voastre",
    "voastră",
    "voi",
    "vostru",
    "vouă",
    "voştri",
    "vreme",
    "vreo",
    "vreun",
    "vă",
    "zece",
    "zero",
    "zi",
    "zice",
    "îi",
    "îl",
    "îmi",
    "împotriva",
    "în",
    "înainte",
    "înaintea",
    "încotro",
    "încât",
    "încît",
    "între",
    "întrucât",
    "întrucît",
    "îţi",
    "ăla",
    "ălea",
    "ăsta",
    "ăstea",
    "ăştia",
    "şapte",
    "şase",
    "şi",
    "ştiu",
    "ţi",
    "ţie",
];
