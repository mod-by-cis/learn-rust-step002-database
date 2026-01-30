// src/domain/antroponim.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Antroponomastykon")]
pub struct AntroponimXml {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@author")]
    pub author: Option<String>,
    #[serde(rename = "Concept", default)]
    pub concepts: Vec<NameConcept>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NameConcept {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@category")]
    pub category: NameCategory,
    #[serde(rename = "@origin")]
    pub origin: Option<String>,
    #[serde(rename = "@gender")]
    pub gender: Gender,
    #[serde(rename = "Lexeme", default)]
    pub lexemes: Vec<Lexeme>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Lexeme {
    #[serde(rename = "@glottolog")]
    pub glottolog_id: String,
    #[serde(rename = "@lemma")]
    pub lemma: String,
    #[serde(rename = "@function")]
    pub function: Option<String>,
    #[serde(rename = "@gender")]
    pub gender: Option<Gender>,
    #[serde(rename = "@type")]
    pub lex_type: Option<String>,
    #[serde(rename = "@origin")]
    pub origin: Option<String>,
    #[serde(rename = "@note")]
    pub note: Option<String>,
    #[serde(rename = "Case", default)]
    pub cases: Vec<CaseEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CaseEntry {
    #[serde(rename = "$value")]
    pub text_value: String,
    #[serde(rename = "@ipa")]
    pub ipa: Option<String>,
    #[serde(rename = "@case")]
    pub gram_case: Option<GramCase>,
    #[serde(rename = "@number")]
    pub number: Option<GramNumber>,
    #[serde(rename = "@note")]
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum NameCategory {
    #[serde(rename = "givenname")]  GivenName,
    #[serde(rename = "surname")]    Surname,
    #[serde(rename = "patronymic")] Patronymic,
    #[serde(rename = "matronymic")] Matronymic,
    #[serde(rename = "nickname")]   Nickname,
    #[serde(rename = "alias")]      Alias,
    #[serde(rename = "cognomen")]   Cognomen,
    #[serde(rename = "other")]      Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Gender { Male, Female, Neutral, Unknown }
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GramCase { Nom, Gen, Dat, Acc, Ins, Loc, Voc, Abl, Pre, Unknown }
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GramNumber { Sing, Plur, Pauc, Unknown }