use super::common::{LatestAction, PolicyArea};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillDetailResponse {
    pub bill: BillDetail,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillDetail {
    /// 1: CBO 비용 추정
    #[serde(rename = "cboCostEstimates")]
    pub cbo_cost_estimates: Option<Vec<CboEstimate>>,

    /// 2: 위원회 보고서
    #[serde(rename = "committeeReports")]
    pub committee_reports: Option<Vec<CommitteeReport>>,
    /// 3: 의회 회기
    pub congress: i64,

    /// 4: 헌법적 권한 진술문
    #[serde(rename = "constitutionalAuthorityStatementText")]
    pub constitutional_authority_statement_text: Option<String>,

    /// 5: 발의 날짜
    #[serde(rename = "introducedDate")]
    pub introduced_date: String,

    /// 6: 최근 조치 정보
    #[serde(rename = "latestAction")]
    pub latest_action: LatestAction,

    /// 7: 법률 정보
    pub laws: Option<Vec<LawInfo>>,

    /// 8: 법안 번호
    pub number: String,

    /// 9: 발의 원
    #[serde(rename = "originChamber")]
    pub origin_chamber: String,

    /// 10: 발의 원 코드
    #[serde(rename = "originChamberCode")]
    pub origin_chamber_code: String,

    /// 11: 정책 영역
    #[serde(rename = "policyArea")]
    pub policy_area: Option<PolicyArea>,

    /// 12: 발의자 정보
    pub sponsors: Vec<Sponsor>,

    /// 13: 법안 제목
    pub title: String,

    /// 14: 법안 유형
    #[serde(rename = "type")]
    pub bill_type: String,

    /// 15: 업데이트 날짜
    #[serde(rename = "updateDate")]
    pub update_date: String,

    /// 16: 텍스트 포함 업데이트 날짜
    #[serde(rename = "updateDateIncludingText")]
    pub update_date_including_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CboEstimate {
    /// 1: 설명
    pub description: String,

    /// 2: 발행 날짜
    #[serde(rename = "pubDate")]
    pub pub_date: String,

    /// 3: 제목
    pub title: String,

    /// 4: URL
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommitteeReport {
    /// 1: 인용
    pub citation: String,

    /// 2: URL
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LawInfo {
    /// 1: 법률 번호
    pub number: String,

    /// 2: 법률 유형
    #[serde(rename = "type")]
    pub law_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sponsor {
    /// 1: 바이오가이드 ID
    #[serde(rename = "bioguideId")]
    pub bioguide_id: String,

    /// 2: 지역구
    pub district: Option<u32>,

    /// 3: 이름
    #[serde(rename = "firstName")]
    pub first_name: String,

    /// 4: 전체 이름
    #[serde(rename = "fullName")]
    pub full_name: String,

    /// 5: 요청에 의한 것인지 여부
    #[serde(rename = "isByRequest")]
    pub is_by_request: String,

    /// 6: 라스트네임
    #[serde(rename = "lastName")]
    pub last_name: String,

    /// 7: 미들네임
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,

    /// 8: 정당
    pub party: String,

    /// 9: 주
    pub state: String,

    /// 10: URL
    pub url: String,
}

impl BillDetail {
    pub fn convert_bill_type(&self) -> dto::USBillType {
        let lower_case_bill_type = self.bill_type.to_lowercase();
        match lower_case_bill_type.as_str() {
            "hr" => dto::USBillType::HouseBill,
            "s" => dto::USBillType::SenateBill,
            "hjres" => dto::USBillType::HouseJointResolution,
            "sjres" => dto::USBillType::SenateJointResolution,
            "hconres" => dto::USBillType::HouseConcurrentResolution,
            "sconres" => dto::USBillType::SenateConcurrentResolution,
            "hres" => dto::USBillType::HouseSimpleResolution,
            "sres" => dto::USBillType::SenateSimpleResolution,
            _ => dto::USBillType::Unknown,
        }
    }

    pub fn get_origin_chamber(&self) -> dto::Chamber {
        match self.origin_chamber.as_str() {
            "House" => dto::Chamber::House,
            "Senate" => dto::Chamber::Senate,
            _ => dto::Chamber::Unknown,
        }
    }

    pub fn get_policy_area(&self) -> dto::PolicyArea {
        if let Some(policy_area) = &self.policy_area {
            convert_policy_area(&policy_area.name)
        } else {
            dto::PolicyArea::Others
        }
    }
}

pub fn convert_policy_area(r#type: &str) -> dto::PolicyArea {
    match r#type {
        "Agriculture and Food" => dto::PolicyArea::AgricultureAndFood,
        "Animals" => dto::PolicyArea::Animals,
        "Armed Forces and National Security" => dto::PolicyArea::ArmedForcesAndNationalSecurity,
        "Arts, Culture, Religion" => dto::PolicyArea::ArtsCultureReligion,
        "Civil Rights and Liberties, Minority Issues" => dto::PolicyArea::CivilRightsAndLiberties,
        "Commerce" => dto::PolicyArea::Commerce,
        "Congress" => dto::PolicyArea::Congress,
        "Crime and Law Enforcement" => dto::PolicyArea::CrimeAndLawEnforcement,
        "Economics and Public Finance" => dto::PolicyArea::EconomicsAndPublicFinance,
        "Education" => dto::PolicyArea::Education,
        "Emergency Management" => dto::PolicyArea::EmergencyManagement,
        "Energy" => dto::PolicyArea::Energy,
        "Environmental Protection" => dto::PolicyArea::EnvironmentalProtection,
        "Families" => dto::PolicyArea::Families,
        "Finance and Financial Sector" => dto::PolicyArea::FinanceAndFinancialSector,
        "Foreign Trade and International Finance" => {
            dto::PolicyArea::ForeignTradeAndInternationalFinance
        }
        "Government Operations and Politics" => dto::PolicyArea::GovernmentOperationsAndPolitics,
        "Health" => dto::PolicyArea::Health,
        "Housing and Community Development" => dto::PolicyArea::HousingAndCommunityDevelopment,
        "Immigration" => dto::PolicyArea::Immigration,
        "International Affairs" => dto::PolicyArea::InternationalAffairs,
        "Labor and Employment" => dto::PolicyArea::LaborAndEmployment,
        "Law" => dto::PolicyArea::Law,
        "Native Americans" => dto::PolicyArea::NativeAmericans,
        "Public Lands and Natural Resources" => dto::PolicyArea::PublicLandsAndNaturalResources,
        "Science, Technology, Communications" => dto::PolicyArea::ScienceTechnologyCommunications,
        "Social Welfare" => dto::PolicyArea::SocialWelfare,
        "Sports and Recreation" => dto::PolicyArea::SportsAndRecreation,
        "Taxation" => dto::PolicyArea::Taxation,
        "Transportation and Public Works" => dto::PolicyArea::TransportationAndPublicWorks,
        "Water Resources Development" => dto::PolicyArea::WaterResourcesDevelopment,
        _ => dto::PolicyArea::Others,
    }
}
