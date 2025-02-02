use serde::Serialize;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionAssemblyMemberRequest {
    /// Fetches assembly members by Assembly Open API.
    /// And update the information of the assembly members.
    FetchMembers,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionAssemblyMemberByIdRequest {
    /// Manually, update crypto stance.
    /// It will be utilized to update crypto stance by contact.
    UpdateCryptoStance(CryptoStance),
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CryptoStance {
    #[serde(rename = "supportive")]
    Supportive,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "against")]
    Against,
    #[default]
    NoStance,
}

impl std::fmt::Display for CryptoStance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoStance::Supportive => write!(f, "supportive"),
            CryptoStance::Against => write!(f, "against"),
            CryptoStance::Neutral => write!(f, "neutral"),
            CryptoStance::NoStance => write!(f, "no_stance"),
        }
    }
}

impl std::str::FromStr for CryptoStance {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "supportive" => Ok(CryptoStance::Supportive),
            "against" => Ok(CryptoStance::Against),
            "neutral" => Ok(CryptoStance::Neutral),
            "no_stance" => Ok(CryptoStance::NoStance),
            _ => Err(format!("Unknown crypto stance: {}", s)),
        }
    }
}
