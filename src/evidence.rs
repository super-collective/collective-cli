use core::fmt::Display;
use core::fmt::Formatter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportPeriod {
    #[serde(rename = "start_date")]
    pub start: String,
    #[serde(rename = "end_date")]
    pub end: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WishUntyped<Rank> {
    pub retain: Option<Rank>,
    pub promote: Option<Rank>,
}

pub enum Wish<Rank> {
    Retain(Rank),
    Promote(Rank),
}

impl<Rank: crate::traits::Rank> WishUntyped<Rank> {
    pub fn as_typed(&self) -> Wish<Rank> {
        match (&self.retain, &self.promote) {
            (Some(_), Some(_)) | (None, None) => {
                panic!("retain and promote are mutually exclusive")
            }
            (Some(rank), None) => Wish::Retain(*rank),
            (None, Some(rank)) => Wish::Promote(*rank),
        }
    }
}

impl<Rank> Wish<Rank> {
    pub fn title(&self) -> &'static str {
        match self {
            Wish::Retain(_) => "Retain",
            Wish::Promote(_) => "Promote",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tasks {
    pub title: String,
    pub links: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Evidence {
    pub title: String,
    pub tasks: Vec<Tasks>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Collective {
    #[serde(alias = "fellowship")]
    Fellowship,
}

impl Collective {
    pub fn name(&self) -> &'static str {
        match self {
            Collective::Fellowship => "Polkadot Fellowship",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct EvidenceCategoryUntyped {
    pub development: Vec<DevelopmentEvidenceCategory>,
}

impl EvidenceCategoryUntyped {
    pub fn into_typed(self) -> EvidenceCategory {
        EvidenceCategory::Development(self.development)
    }
}

pub enum EvidenceCategory {
    Development(Vec<DevelopmentEvidenceCategory>),
}

impl EvidenceCategory {
    pub fn title(&self) -> &'static str {
        match self {
            EvidenceCategory::Development(_) => "Development",
        }
    }

    // this is a bit hacky.. but makes the code in the template easier.
    pub fn sub_categories(&self) -> Option<String> {
        match self {
            EvidenceCategory::Development(categories) if categories.is_empty() => None,
            EvidenceCategory::Development(categories) => Some(
                categories
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DevelopmentEvidenceCategory {
    Sdk,
    Runtime,
    Tooling,
    Other,
}

impl Display for DevelopmentEvidenceCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DevelopmentEvidenceCategory::Sdk => write!(f, "SDK"),
            DevelopmentEvidenceCategory::Runtime => write!(f, "Runtime"),
            DevelopmentEvidenceCategory::Tooling => write!(f, "Tooling"),
            DevelopmentEvidenceCategory::Other => write!(f, "Other"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EvidenceReport<Rank: crate::traits::Rank> {
    pub name: String,
    pub address: String,
    pub github: String,
    pub wish: WishUntyped<Rank>,
    pub collective: Collective,
    #[serde(rename = "report_date")]
    pub date: String,
    #[serde(rename = "report_period")]
    pub period: ReportPeriod,
    #[serde(rename = "evidence_categories")]
    pub categories: Vec<EvidenceCategoryUntyped>,
    pub evidence: Vec<Evidence>,
}

impl<Rank: crate::traits::Rank> EvidenceReport<Rank> {
    pub fn address_link(&self) -> String {
        let shortened = if self.address.len() > 8 {
            format!("{}..", &self.address[..8])
        } else {
            self.address.clone()
        };
        format!(
            "<a target='_blank' href='https://collectives.statescan.io/#/accounts/{}'>{}</a>",
            self.address, shortened
        )
    }

    pub fn github_link(&self) -> String {
        format!(
            "<a target='_blank' href='https://github.com/{}'>{}</a>",
            self.github, self.github
        )
    }
}
