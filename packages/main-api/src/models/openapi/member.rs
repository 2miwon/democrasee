pub trait MemberTrait {
    fn name(&self) -> &str;
    fn party(&self) -> &str;
    fn district(&self) -> &str;
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Member {
    #[serde(rename(deserialize = "HG_NM"))]
    pub name: String, // Korean Name
    #[serde(rename(deserialize = "POLY_NM"))]
    pub party: String, // Korean Party Name
    #[serde(rename(deserialize = "ORIG_NM"))]
    pub district: String, // Korean District Name
    #[serde(rename(deserialize = "MONA_CD"))]
    pub code: String, // Unique Code
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EnMember {
    #[serde(rename(deserialize = "NAAS_EN_NM"))]
    pub name: String, // English Name
    #[serde(rename(deserialize = "PLPT_NM"))]
    pub party: String, // English Party Name
    #[serde(rename(deserialize = "ELECD_NM"))]
    pub district: Option<String>, // English District Name
}

impl MemberTrait for Member {
    fn name(&self) -> &str {
        &self.name
    }
    fn party(&self) -> &str {
        &self.party
    }
    fn district(&self) -> &str {
        &self.district
    }
}

impl MemberTrait for EnMember {
    fn name(&self) -> &str {
        &self.name
    }
    fn party(&self) -> &str {
        &self.party
    }
    fn district(&self) -> &str {
        &self.district.as_deref().unwrap_or("")
    }
}