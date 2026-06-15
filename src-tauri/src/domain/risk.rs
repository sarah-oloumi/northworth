use serde::{Deserialize, Serialize};

const MAX_RISK_SCORE: u8 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskInterviewAnswers {
    pub time_horizon: TimeHorizon,
    pub drawdown_response: DrawdownResponse,
    pub income_stability: IncomeStability,
    pub emergency_fund: EmergencyFundCoverage,
    pub debt_pressure: DebtPressure,
    pub investing_experience: InvestingExperience,
    pub liquidity_need: LiquidityNeed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeHorizon {
    UnderThreeYears,
    ThreeToFiveYears,
    SixToTenYears,
    ElevenToTwentyYears,
    OverTwentyYears,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrawdownResponse {
    SellMost,
    ReduceRisk,
    HoldPlan,
    BuyMore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncomeStability {
    Unstable,
    Variable,
    Stable,
    VeryStable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmergencyFundCoverage {
    UnderOneMonth,
    OneToThreeMonths,
    ThreeToSixMonths,
    OverSixMonths,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DebtPressure {
    High,
    Moderate,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvestingExperience {
    New,
    Some,
    Experienced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiquidityNeed {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskInterviewResult {
    pub score: u8,
    pub max_score: u8,
    pub profile: RiskProfile,
    pub confidence: RiskConfidence,
    pub equity_range: EquityRange,
    pub planning_input: PortfolioPlanningInput,
    pub factors: Vec<RiskFactorScore>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskProfile {
    CapitalPreservation,
    Conservative,
    Balanced,
    Growth,
    AggressiveGrowth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskConfidence {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EquityRange {
    pub minimum_percent: u8,
    pub target_percent: u8,
    pub maximum_percent: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortfolioPlanningInput {
    pub risk_profile: RiskProfile,
    pub equity_target_percent: u8,
    pub planning_note: String,
    pub not_a_trade_instruction: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskFactorScore {
    pub factor: RiskFactor,
    pub label: String,
    pub points: u8,
    pub max_points: u8,
    pub explanation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskFactor {
    TimeHorizon,
    DrawdownResponse,
    IncomeStability,
    EmergencyFund,
    DebtPressure,
    InvestingExperience,
    LiquidityNeed,
}

pub fn score_risk_interview(answers: RiskInterviewAnswers) -> RiskInterviewResult {
    let factors = vec![
        factor(
            RiskFactor::TimeHorizon,
            "Time horizon",
            time_horizon_points(answers.time_horizon),
            4,
            time_horizon_explanation(answers.time_horizon),
        ),
        factor(
            RiskFactor::DrawdownResponse,
            "Drawdown response",
            drawdown_points(answers.drawdown_response),
            4,
            drawdown_explanation(answers.drawdown_response),
        ),
        factor(
            RiskFactor::IncomeStability,
            "Income stability",
            income_points(answers.income_stability),
            3,
            income_explanation(answers.income_stability),
        ),
        factor(
            RiskFactor::EmergencyFund,
            "Emergency fund",
            emergency_fund_points(answers.emergency_fund),
            3,
            emergency_fund_explanation(answers.emergency_fund),
        ),
        factor(
            RiskFactor::DebtPressure,
            "Debt pressure",
            debt_pressure_points(answers.debt_pressure),
            2,
            debt_pressure_explanation(answers.debt_pressure),
        ),
        factor(
            RiskFactor::InvestingExperience,
            "Investing experience",
            investing_experience_points(answers.investing_experience),
            2,
            investing_experience_explanation(answers.investing_experience),
        ),
        factor(
            RiskFactor::LiquidityNeed,
            "Liquidity need",
            liquidity_need_points(answers.liquidity_need),
            2,
            liquidity_need_explanation(answers.liquidity_need),
        ),
    ];
    let score = factors.iter().map(|factor| factor.points).sum::<u8>();
    let profile = risk_profile(score);
    let equity_range = equity_range(profile);
    let confidence = confidence(answers);
    let limitations = limitations(answers, confidence);

    RiskInterviewResult {
        score,
        max_score: MAX_RISK_SCORE,
        profile,
        confidence,
        equity_range,
        planning_input: PortfolioPlanningInput {
            risk_profile: profile,
            equity_target_percent: equity_range.target_percent,
            planning_note: "Use this as an input to portfolio planning, then compare against goals, taxes, account type, costs, currency exposure, and cash needs.".to_string(),
            not_a_trade_instruction: true,
        },
        factors,
        limitations,
    }
}

fn factor(
    factor: RiskFactor,
    label: &str,
    points: u8,
    max_points: u8,
    explanation: &str,
) -> RiskFactorScore {
    RiskFactorScore {
        factor,
        label: label.to_string(),
        points,
        max_points,
        explanation: explanation.to_string(),
    }
}

fn time_horizon_points(value: TimeHorizon) -> u8 {
    match value {
        TimeHorizon::UnderThreeYears => 0,
        TimeHorizon::ThreeToFiveYears => 1,
        TimeHorizon::SixToTenYears => 2,
        TimeHorizon::ElevenToTwentyYears => 3,
        TimeHorizon::OverTwentyYears => 4,
    }
}

fn drawdown_points(value: DrawdownResponse) -> u8 {
    match value {
        DrawdownResponse::SellMost => 0,
        DrawdownResponse::ReduceRisk => 1,
        DrawdownResponse::HoldPlan => 3,
        DrawdownResponse::BuyMore => 4,
    }
}

fn income_points(value: IncomeStability) -> u8 {
    match value {
        IncomeStability::Unstable => 0,
        IncomeStability::Variable => 1,
        IncomeStability::Stable => 2,
        IncomeStability::VeryStable => 3,
    }
}

fn emergency_fund_points(value: EmergencyFundCoverage) -> u8 {
    match value {
        EmergencyFundCoverage::UnderOneMonth => 0,
        EmergencyFundCoverage::OneToThreeMonths => 1,
        EmergencyFundCoverage::ThreeToSixMonths => 2,
        EmergencyFundCoverage::OverSixMonths => 3,
    }
}

fn debt_pressure_points(value: DebtPressure) -> u8 {
    match value {
        DebtPressure::High => 0,
        DebtPressure::Moderate => 1,
        DebtPressure::Low => 2,
    }
}

fn investing_experience_points(value: InvestingExperience) -> u8 {
    match value {
        InvestingExperience::New => 0,
        InvestingExperience::Some => 1,
        InvestingExperience::Experienced => 2,
    }
}

fn liquidity_need_points(value: LiquidityNeed) -> u8 {
    match value {
        LiquidityNeed::High => 0,
        LiquidityNeed::Medium => 1,
        LiquidityNeed::Low => 2,
    }
}

fn risk_profile(score: u8) -> RiskProfile {
    match score {
        0..=4 => RiskProfile::CapitalPreservation,
        5..=8 => RiskProfile::Conservative,
        9..=12 => RiskProfile::Balanced,
        13..=16 => RiskProfile::Growth,
        _ => RiskProfile::AggressiveGrowth,
    }
}

fn equity_range(profile: RiskProfile) -> EquityRange {
    match profile {
        RiskProfile::CapitalPreservation => EquityRange {
            minimum_percent: 0,
            target_percent: 20,
            maximum_percent: 35,
        },
        RiskProfile::Conservative => EquityRange {
            minimum_percent: 20,
            target_percent: 40,
            maximum_percent: 55,
        },
        RiskProfile::Balanced => EquityRange {
            minimum_percent: 45,
            target_percent: 60,
            maximum_percent: 75,
        },
        RiskProfile::Growth => EquityRange {
            minimum_percent: 65,
            target_percent: 80,
            maximum_percent: 90,
        },
        RiskProfile::AggressiveGrowth => EquityRange {
            minimum_percent: 80,
            target_percent: 90,
            maximum_percent: 100,
        },
    }
}

fn confidence(answers: RiskInterviewAnswers) -> RiskConfidence {
    let guardrails = [
        answers.time_horizon == TimeHorizon::UnderThreeYears,
        answers.drawdown_response == DrawdownResponse::SellMost,
        answers.emergency_fund == EmergencyFundCoverage::UnderOneMonth,
        answers.debt_pressure == DebtPressure::High,
        answers.liquidity_need == LiquidityNeed::High,
    ]
    .iter()
    .filter(|guardrail| **guardrail)
    .count();

    if guardrails >= 2 {
        RiskConfidence::Low
    } else if guardrails == 1 || answers.investing_experience == InvestingExperience::New {
        RiskConfidence::Medium
    } else {
        RiskConfidence::High
    }
}

fn limitations(answers: RiskInterviewAnswers, confidence: RiskConfidence) -> Vec<String> {
    let mut limitations = vec![
        "This score is deterministic and local; it is not a suitability review or a trade instruction.".to_string(),
        "Portfolio planning still needs account type, tax treatment, currency exposure, costs, and goal timing.".to_string(),
    ];

    if answers.time_horizon == TimeHorizon::UnderThreeYears {
        limitations.push("Short time horizons can make volatile investments unsuitable even when other answers score higher.".to_string());
    }

    if answers.emergency_fund == EmergencyFundCoverage::UnderOneMonth {
        limitations.push("A thin emergency fund should usually be addressed before taking more portfolio risk.".to_string());
    }

    if answers.debt_pressure == DebtPressure::High {
        limitations.push("High-interest or high-pressure debt can reduce practical risk capacity.".to_string());
    }

    if confidence == RiskConfidence::Low {
        limitations.push("Conflicting guardrail answers lower confidence; review the inputs before using this in a plan.".to_string());
    }

    limitations
}

fn time_horizon_explanation(value: TimeHorizon) -> &'static str {
    match value {
        TimeHorizon::UnderThreeYears => "Near-term money has little room to recover from market losses.",
        TimeHorizon::ThreeToFiveYears => "A short-to-medium horizon allows limited volatility.",
        TimeHorizon::SixToTenYears => "A medium horizon can usually absorb some market cycles.",
        TimeHorizon::ElevenToTwentyYears => "A long horizon supports a higher growth allocation.",
        TimeHorizon::OverTwentyYears => "Very long horizons can support meaningful equity exposure.",
    }
}

fn drawdown_explanation(value: DrawdownResponse) -> &'static str {
    match value {
        DrawdownResponse::SellMost => "Selling after losses signals low tolerance for volatility.",
        DrawdownResponse::ReduceRisk => "Reducing risk after losses suggests a cautious limit.",
        DrawdownResponse::HoldPlan => "Holding through losses supports a higher long-term risk score.",
        DrawdownResponse::BuyMore => "Adding during losses signals high tolerance, subject to capacity.",
    }
}

fn income_explanation(value: IncomeStability) -> &'static str {
    match value {
        IncomeStability::Unstable => "Unstable income reduces capacity to absorb portfolio losses.",
        IncomeStability::Variable => "Variable income calls for more cushion before taking risk.",
        IncomeStability::Stable => "Stable income supports moderate risk capacity.",
        IncomeStability::VeryStable => "Very stable income can support higher risk capacity.",
    }
}

fn emergency_fund_explanation(value: EmergencyFundCoverage) -> &'static str {
    match value {
        EmergencyFundCoverage::UnderOneMonth => "Less than one month of cash is a major guardrail.",
        EmergencyFundCoverage::OneToThreeMonths => "Some cash cushion exists, but risk capacity is still limited.",
        EmergencyFundCoverage::ThreeToSixMonths => "A practical emergency fund supports portfolio discipline.",
        EmergencyFundCoverage::OverSixMonths => "A strong emergency fund supports higher risk capacity.",
    }
}

fn debt_pressure_explanation(value: DebtPressure) -> &'static str {
    match value {
        DebtPressure::High => "High debt pressure reduces flexibility.",
        DebtPressure::Moderate => "Moderate debt pressure leaves some planning flexibility.",
        DebtPressure::Low => "Low debt pressure improves capacity to stay invested.",
    }
}

fn investing_experience_explanation(value: InvestingExperience) -> &'static str {
    match value {
        InvestingExperience::New => "New investors may need simpler plans and more conservative guardrails.",
        InvestingExperience::Some => "Some experience supports moderate confidence in the answers.",
        InvestingExperience::Experienced => "Experience with market cycles improves confidence in the answers.",
    }
}

fn liquidity_need_explanation(value: LiquidityNeed) -> &'static str {
    match value {
        LiquidityNeed::High => "Near-term cash needs reduce capacity for volatile assets.",
        LiquidityNeed::Medium => "Some cash needs call for a balanced allocation.",
        LiquidityNeed::Low => "Low cash needs support a longer-term allocation.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scores_same_answers_deterministically() {
        let answers = sample_growth_answers();

        assert_eq!(score_risk_interview(answers), score_risk_interview(answers));
    }

    #[test]
    fn produces_growth_profile_for_high_capacity_answers() {
        let result = score_risk_interview(sample_growth_answers());

        assert_eq!(result.score, 18);
        assert_eq!(result.profile, RiskProfile::AggressiveGrowth);
        assert_eq!(result.equity_range.target_percent, 90);
        assert!(result.planning_input.not_a_trade_instruction);
    }

    #[test]
    fn lowers_confidence_when_guardrails_conflict() {
        let result = score_risk_interview(RiskInterviewAnswers {
            time_horizon: TimeHorizon::UnderThreeYears,
            drawdown_response: DrawdownResponse::SellMost,
            income_stability: IncomeStability::Stable,
            emergency_fund: EmergencyFundCoverage::UnderOneMonth,
            debt_pressure: DebtPressure::High,
            investing_experience: InvestingExperience::New,
            liquidity_need: LiquidityNeed::High,
        });

        assert_eq!(result.confidence, RiskConfidence::Low);
        assert_eq!(result.profile, RiskProfile::CapitalPreservation);
        assert!(result
            .limitations
            .iter()
            .any(|limitation| limitation.contains("not a suitability review")));
    }

    fn sample_growth_answers() -> RiskInterviewAnswers {
        RiskInterviewAnswers {
            time_horizon: TimeHorizon::OverTwentyYears,
            drawdown_response: DrawdownResponse::HoldPlan,
            income_stability: IncomeStability::VeryStable,
            emergency_fund: EmergencyFundCoverage::ThreeToSixMonths,
            debt_pressure: DebtPressure::Low,
            investing_experience: InvestingExperience::Experienced,
            liquidity_need: LiquidityNeed::Low,
        }
    }
}
