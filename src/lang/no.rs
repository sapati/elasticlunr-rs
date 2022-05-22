use super::{
    common::{RustStemmer, StopWordFilter, RegexTrimmer},
    Language,
};
use crate::pipeline::Pipeline;
use rust_stemmers::Algorithm;

pub struct Norwegian {}

impl Norwegian {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for Norwegian {
    fn name(&self) -> String {
        "Norwegian".into()
    }
    fn code(&self) -> String {
        "no".into()
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        super::tokenize_whitespace(text)
    }

    fn make_pipeline(&self) -> Pipeline {
        Pipeline {
            queue: vec![
                Box::new(RegexTrimmer::new("trimmer-no", r"\p{Latin}")),
                Box::new(StopWordFilter::new("stopWordFilter-no", STOP_WORDS)),
                Box::new(RustStemmer::new("stemmer-no", Algorithm::Norwegian)),
            ],
        }
    }
}

const STOP_WORDS: &'static [&'static str] = &[
    "",
    "alle",
    "at",
    "av",
    "bare",
    "begge",
    "ble",
    "blei",
    "bli",
    "blir",
    "blitt",
    "både",
    "båe",
    "da",
    "de",
    "deg",
    "dei",
    "deim",
    "deira",
    "deires",
    "dem",
    "den",
    "denne",
    "der",
    "dere",
    "deres",
    "det",
    "dette",
    "di",
    "din",
    "disse",
    "ditt",
    "du",
    "dykk",
    "dykkar",
    "då",
    "eg",
    "ein",
    "eit",
    "eitt",
    "eller",
    "elles",
    "en",
    "enn",
    "er",
    "et",
    "ett",
    "etter",
    "for",
    "fordi",
    "fra",
    "før",
    "ha",
    "hadde",
    "han",
    "hans",
    "har",
    "hennar",
    "henne",
    "hennes",
    "her",
    "hjå",
    "ho",
    "hoe",
    "honom",
    "hoss",
    "hossen",
    "hun",
    "hva",
    "hvem",
    "hver",
    "hvilke",
    "hvilken",
    "hvis",
    "hvor",
    "hvordan",
    "hvorfor",
    "i",
    "ikke",
    "ikkje",
    "ikkje",
    "ingen",
    "ingi",
    "inkje",
    "inn",
    "inni",
    "ja",
    "jeg",
    "kan",
    "kom",
    "korleis",
    "korso",
    "kun",
    "kunne",
    "kva",
    "kvar",
    "kvarhelst",
    "kven",
    "kvi",
    "kvifor",
    "man",
    "mange",
    "me",
    "med",
    "medan",
    "meg",
    "meget",
    "mellom",
    "men",
    "mi",
    "min",
    "mine",
    "mitt",
    "mot",
    "mykje",
    "ned",
    "no",
    "noe",
    "noen",
    "noka",
    "noko",
    "nokon",
    "nokor",
    "nokre",
    "nå",
    "når",
    "og",
    "også",
    "om",
    "opp",
    "oss",
    "over",
    "på",
    "samme",
    "seg",
    "selv",
    "si",
    "si",
    "sia",
    "sidan",
    "siden",
    "sin",
    "sine",
    "sitt",
    "sjøl",
    "skal",
    "skulle",
    "slik",
    "so",
    "som",
    "som",
    "somme",
    "somt",
    "så",
    "sånn",
    "til",
    "um",
    "upp",
    "ut",
    "uten",
    "var",
    "vart",
    "varte",
    "ved",
    "vere",
    "verte",
    "vi",
    "vil",
    "ville",
    "vore",
    "vors",
    "vort",
    "vår",
    "være",
    "være",
    "vært",
    "å",
];
