use crate::domain::risk::{
    score_risk_interview as score_risk_interview_domain, RiskInterviewAnswers, RiskInterviewResult,
};

#[tauri::command]
pub fn score_risk_interview(answers: RiskInterviewAnswers) -> RiskInterviewResult {
    score_risk_interview_domain(answers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::risk::{
        DebtPressure, DrawdownResponse, EmergencyFundCoverage, IncomeStability,
        InvestingExperience, LiquidityNeed, RiskProfile, TimeHorizon,
    };

    #[test]
    fn scores_risk_interview_through_command() {
        let result = score_risk_interview(RiskInterviewAnswers {
            time_horizon: TimeHorizon::ElevenToTwentyYears,
            drawdown_response: DrawdownResponse::HoldPlan,
            income_stability: IncomeStability::Stable,
            emergency_fund: EmergencyFundCoverage::ThreeToSixMonths,
            debt_pressure: DebtPressure::Moderate,
            investing_experience: InvestingExperience::Some,
            liquidity_need: LiquidityNeed::Medium,
        });

        assert_eq!(result.profile, RiskProfile::Growth);
        assert!(result.planning_input.not_a_trade_instruction);
    }
}
