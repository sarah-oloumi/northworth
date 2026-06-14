use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const MAX_ADULTS_IN_MVP_HOUSEHOLD: usize = 2;
const FULL_OWNERSHIP_BASIS_POINTS: u16 = 10_000;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PersonId(String);

impl PersonId {
    pub fn new(value: impl Into<String>) -> Result<Self, HouseholdValidationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(HouseholdValidationError::EmptyPersonId);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdProfile {
    pub id: HouseholdId,
    pub adults: Vec<AdultProfile>,
    pub relationship: HouseholdRelationship,
    pub dependents: Vec<DependentProfile>,
    pub shared_allocations: Vec<SharedAllocation>,
}

impl HouseholdProfile {
    pub fn validate(&self) -> Result<(), HouseholdValidationError> {
        if self.adults.is_empty() {
            return Err(HouseholdValidationError::MissingAdult);
        }

        if self.adults.len() > MAX_ADULTS_IN_MVP_HOUSEHOLD {
            return Err(HouseholdValidationError::TooManyAdults {
                max: MAX_ADULTS_IN_MVP_HOUSEHOLD,
                actual: self.adults.len(),
            });
        }

        let mut adult_ids = HashSet::new();

        for adult in &self.adults {
            if !adult_ids.insert(adult.person.id.clone()) {
                return Err(HouseholdValidationError::DuplicatePersonId {
                    person_id: adult.person.id.clone(),
                });
            }
        }

        self.relationship.validate_for_adults(&adult_ids)?;

        for adult in &self.adults {
            adult.validate(&adult_ids)?;
        }

        for dependent in &self.dependents {
            dependent.validate(&adult_ids)?;
        }

        for allocation in &self.shared_allocations {
            allocation.validate(&adult_ids)?;
        }

        Ok(())
    }

    pub fn calculation_subjects(&self) -> Vec<CalculationSubject> {
        let mut subjects = self
            .adults
            .iter()
            .map(|adult| CalculationSubject::Person(adult.person.id.clone()))
            .collect::<Vec<_>>();

        subjects.push(CalculationSubject::Household(self.id.clone()));
        subjects
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HouseholdId(String);

impl HouseholdId {
    pub fn new(value: impl Into<String>) -> Result<Self, HouseholdValidationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(HouseholdValidationError::EmptyHouseholdId);
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdultProfile {
    pub person: PersonProfile,
    pub residence_by_year: Vec<ResidenceForTaxYear>,
    pub income_sources: Vec<IncomeSource>,
    pub registered_accounts: Vec<RegisteredAccountSnapshot>,
    pub non_registered_accounts: Vec<NonRegisteredAccountSnapshot>,
}

impl AdultProfile {
    fn validate(&self, adult_ids: &HashSet<PersonId>) -> Result<(), HouseholdValidationError> {
        for income_source in &self.income_sources {
            if !adult_ids.contains(&income_source.owner) {
                return Err(HouseholdValidationError::UnknownPersonReference {
                    person_id: income_source.owner.clone(),
                });
            }
        }

        for account in &self.registered_accounts {
            if !adult_ids.contains(&account.owner) {
                return Err(HouseholdValidationError::UnknownPersonReference {
                    person_id: account.owner.clone(),
                });
            }
        }

        for account in &self.non_registered_accounts {
            account.owner.validate(adult_ids)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersonProfile {
    pub id: PersonId,
    pub display_name: Option<String>,
    pub birth_year: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdRelationship {
    Single,
    SpousesOrCommonLaw {
        person_a: PersonId,
        person_b: PersonId,
    },
    RoommatesOrUnrelated,
    Unknown,
}

impl HouseholdRelationship {
    fn validate_for_adults(
        &self,
        adult_ids: &HashSet<PersonId>,
    ) -> Result<(), HouseholdValidationError> {
        match self {
            HouseholdRelationship::Single => {
                if adult_ids.len() != 1 {
                    return Err(HouseholdValidationError::SingleRelationshipRequiresOneAdult);
                }
            }
            HouseholdRelationship::SpousesOrCommonLaw { person_a, person_b } => {
                if adult_ids.len() != 2 {
                    return Err(HouseholdValidationError::PartnerRelationshipRequiresTwoAdults);
                }

                if person_a == person_b {
                    return Err(HouseholdValidationError::PartnerRelationshipUsesSamePerson);
                }

                if !adult_ids.contains(person_a) {
                    return Err(HouseholdValidationError::UnknownPersonReference {
                        person_id: person_a.clone(),
                    });
                }

                if !adult_ids.contains(person_b) {
                    return Err(HouseholdValidationError::UnknownPersonReference {
                        person_id: person_b.clone(),
                    });
                }
            }
            HouseholdRelationship::RoommatesOrUnrelated | HouseholdRelationship::Unknown => {}
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidenceForTaxYear {
    pub tax_year: u16,
    pub jurisdiction: CanadianJurisdiction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanadianJurisdiction {
    Alberta,
    BritishColumbia,
    Manitoba,
    NewBrunswick,
    NewfoundlandAndLabrador,
    NorthwestTerritories,
    NovaScotia,
    Nunavut,
    Ontario,
    PrinceEdwardIsland,
    Quebec,
    Saskatchewan,
    Yukon,
    OutsideCanada,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncomeSource {
    pub kind: IncomeSourceKind,
    pub owner: PersonId,
    pub annual_amount: MoneyAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncomeSourceKind {
    Employment,
    Bonus,
    RsuOrEquityCompensation,
    SelfEmployment,
    RentalOrProperty,
    Interest,
    EligibleDividends,
    NonEligibleDividends,
    CapitalGains,
    ForeignIncome,
    Pension,
    GovernmentBenefits,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisteredAccountSnapshot {
    pub owner: PersonId,
    pub account_type: RegisteredAccountType,
    pub balance: MoneyAmount,
    pub contribution_room: Option<MoneyAmount>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegisteredAccountType {
    Rrsp,
    SpousalRrsp,
    Tfsa,
    Fhsa,
    Resp,
    Rdsp,
    Rrif,
    Dpsp,
    Rpp,
    GroupRrsp,
    Lira,
    Lif,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonRegisteredAccountSnapshot {
    pub owner: Ownership,
    pub account_type: NonRegisteredAccountType,
    pub balance: MoneyAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonRegisteredAccountType {
    Cash,
    TaxableBrokerage,
    Hisa,
    Gic,
    Margin,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependentProfile {
    pub id: DependentId,
    pub display_name: Option<String>,
    pub birth_year: Option<u16>,
    pub relationship: DependentRelationship,
    pub claimed_by_or_supported_by: Vec<PersonId>,
}

impl DependentProfile {
    fn validate(&self, adult_ids: &HashSet<PersonId>) -> Result<(), HouseholdValidationError> {
        if self.claimed_by_or_supported_by.is_empty() {
            return Err(HouseholdValidationError::DependentMissingSupportPerson {
                dependent_id: self.id.clone(),
            });
        }

        for person_id in &self.claimed_by_or_supported_by {
            if !adult_ids.contains(person_id) {
                return Err(HouseholdValidationError::UnknownPersonReference {
                    person_id: person_id.clone(),
                });
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DependentId(String);

impl DependentId {
    pub fn new(value: impl Into<String>) -> Result<Self, HouseholdValidationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(HouseholdValidationError::EmptyDependentId);
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependentRelationship {
    Child,
    Parent,
    Relative,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedAllocation {
    pub item_id: SharedItemId,
    pub item_type: SharedItemType,
    pub ownership: Ownership,
}

impl SharedAllocation {
    fn validate(&self, adult_ids: &HashSet<PersonId>) -> Result<(), HouseholdValidationError> {
        self.ownership.validate(adult_ids)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SharedItemId(String);

impl SharedItemId {
    pub fn new(value: impl Into<String>) -> Result<Self, HouseholdValidationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(HouseholdValidationError::EmptySharedItemId);
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharedItemType {
    Home,
    SecondaryProperty,
    Mortgage,
    Vehicle,
    Debt,
    Insurance,
    Subscription,
    OtherAsset,
    OtherExpense,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ownership {
    Sole(PersonId),
    Joint(Vec<OwnershipShare>),
    Unknown,
}

impl Ownership {
    fn validate(&self, adult_ids: &HashSet<PersonId>) -> Result<(), HouseholdValidationError> {
        match self {
            Ownership::Sole(person_id) => {
                if !adult_ids.contains(person_id) {
                    return Err(HouseholdValidationError::UnknownPersonReference {
                        person_id: person_id.clone(),
                    });
                }
            }
            Ownership::Joint(shares) => {
                if shares.is_empty() {
                    return Err(HouseholdValidationError::EmptyJointOwnership);
                }

                let mut seen = HashSet::new();
                let mut total_basis_points: u16 = 0;

                for share in shares {
                    if !adult_ids.contains(&share.owner) {
                        return Err(HouseholdValidationError::UnknownPersonReference {
                            person_id: share.owner.clone(),
                        });
                    }

                    if !seen.insert(share.owner.clone()) {
                        return Err(HouseholdValidationError::DuplicateOwnershipShare {
                            person_id: share.owner.clone(),
                        });
                    }

                    total_basis_points = total_basis_points.saturating_add(share.basis_points);
                }

                if total_basis_points != FULL_OWNERSHIP_BASIS_POINTS {
                    return Err(HouseholdValidationError::InvalidOwnershipTotal {
                        expected: FULL_OWNERSHIP_BASIS_POINTS,
                        actual: total_basis_points,
                    });
                }
            }
            Ownership::Unknown => {}
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipShare {
    pub owner: PersonId,
    pub basis_points: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoneyAmount {
    pub cents: i64,
    pub currency: Currency,
}

impl MoneyAmount {
    pub fn cad_dollars(dollars: i64) -> Self {
        Self {
            cents: dollars * 100,
            currency: Currency::Cad,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    Cad,
    Usd,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CalculationSubject {
    Person(PersonId),
    Household(HouseholdId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HouseholdValidationError {
    EmptyHouseholdId,
    EmptyPersonId,
    EmptyDependentId,
    EmptySharedItemId,
    MissingAdult,
    TooManyAdults { max: usize, actual: usize },
    DuplicatePersonId { person_id: PersonId },
    UnknownPersonReference { person_id: PersonId },
    SingleRelationshipRequiresOneAdult,
    PartnerRelationshipRequiresTwoAdults,
    PartnerRelationshipUsesSamePerson,
    DependentMissingSupportPerson { dependent_id: DependentId },
    EmptyJointOwnership,
    DuplicateOwnershipShare { person_id: PersonId },
    InvalidOwnershipTotal { expected: u16, actual: u16 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_single_adult_household() {
        let household = HouseholdProfile {
            id: household_id("household-1"),
            adults: vec![adult("person-a")],
            relationship: HouseholdRelationship::Single,
            dependents: vec![],
            shared_allocations: vec![],
        };

        assert_eq!(household.validate(), Ok(()));
        assert_eq!(
            household.calculation_subjects(),
            vec![
                CalculationSubject::Person(person_id("person-a")),
                CalculationSubject::Household(household_id("household-1"))
            ]
        );
    }

    #[test]
    fn validates_two_adult_partner_household_without_merging_people() {
        let person_a = person_id("person-a");
        let person_b = person_id("person-b");
        let household = HouseholdProfile {
            id: household_id("household-1"),
            adults: vec![adult(person_a.as_str()), adult(person_b.as_str())],
            relationship: HouseholdRelationship::SpousesOrCommonLaw {
                person_a: person_a.clone(),
                person_b: person_b.clone(),
            },
            dependents: vec![DependentProfile {
                id: dependent_id("dependent-1"),
                display_name: Some("Demo dependent".to_string()),
                birth_year: Some(2020),
                relationship: DependentRelationship::Child,
                claimed_by_or_supported_by: vec![person_a.clone(), person_b.clone()],
            }],
            shared_allocations: vec![SharedAllocation {
                item_id: shared_item_id("primary-home"),
                item_type: SharedItemType::Home,
                ownership: Ownership::Joint(vec![
                    OwnershipShare {
                        owner: person_a.clone(),
                        basis_points: 5_000,
                    },
                    OwnershipShare {
                        owner: person_b.clone(),
                        basis_points: 5_000,
                    },
                ]),
            }],
        };

        assert_eq!(household.validate(), Ok(()));
        assert_eq!(
            household.calculation_subjects(),
            vec![
                CalculationSubject::Person(person_a),
                CalculationSubject::Person(person_b),
                CalculationSubject::Household(household_id("household-1"))
            ]
        );
    }

    #[test]
    fn rejects_households_without_adults() {
        let household = HouseholdProfile {
            id: household_id("household-1"),
            adults: vec![],
            relationship: HouseholdRelationship::Unknown,
            dependents: vec![],
            shared_allocations: vec![],
        };

        assert_eq!(
            household.validate(),
            Err(HouseholdValidationError::MissingAdult)
        );
    }

    #[test]
    fn rejects_more_than_two_adults_for_mvp_household() {
        let household = HouseholdProfile {
            id: household_id("household-1"),
            adults: vec![adult("person-a"), adult("person-b"), adult("person-c")],
            relationship: HouseholdRelationship::Unknown,
            dependents: vec![],
            shared_allocations: vec![],
        };

        assert_eq!(
            household.validate(),
            Err(HouseholdValidationError::TooManyAdults { max: 2, actual: 3 })
        );
    }

    #[test]
    fn rejects_dependent_references_to_unknown_people() {
        let household = HouseholdProfile {
            id: household_id("household-1"),
            adults: vec![adult("person-a")],
            relationship: HouseholdRelationship::Single,
            dependents: vec![DependentProfile {
                id: dependent_id("dependent-1"),
                display_name: None,
                birth_year: None,
                relationship: DependentRelationship::Child,
                claimed_by_or_supported_by: vec![person_id("person-b")],
            }],
            shared_allocations: vec![],
        };

        assert_eq!(
            household.validate(),
            Err(HouseholdValidationError::UnknownPersonReference {
                person_id: person_id("person-b")
            })
        );
    }

    #[test]
    fn rejects_income_sources_that_reference_unknown_people() {
        let mut profile = adult("person-a");
        profile.income_sources = vec![IncomeSource {
            kind: IncomeSourceKind::Bonus,
            owner: person_id("person-b"),
            annual_amount: MoneyAmount::cad_dollars(10_000),
        }];

        let household = HouseholdProfile {
            id: household_id("household-1"),
            adults: vec![profile],
            relationship: HouseholdRelationship::Single,
            dependents: vec![],
            shared_allocations: vec![],
        };

        assert_eq!(
            household.validate(),
            Err(HouseholdValidationError::UnknownPersonReference {
                person_id: person_id("person-b")
            })
        );
    }

    #[test]
    fn rejects_account_ownership_that_references_unknown_people() {
        let mut profile = adult("person-a");
        profile.non_registered_accounts = vec![NonRegisteredAccountSnapshot {
            owner: Ownership::Sole(person_id("person-b")),
            account_type: NonRegisteredAccountType::TaxableBrokerage,
            balance: MoneyAmount::cad_dollars(5_000),
        }];

        let household = HouseholdProfile {
            id: household_id("household-1"),
            adults: vec![profile],
            relationship: HouseholdRelationship::Single,
            dependents: vec![],
            shared_allocations: vec![],
        };

        assert_eq!(
            household.validate(),
            Err(HouseholdValidationError::UnknownPersonReference {
                person_id: person_id("person-b")
            })
        );
    }

    #[test]
    fn rejects_joint_ownership_that_does_not_total_one_hundred_percent() {
        let household = HouseholdProfile {
            id: household_id("household-1"),
            adults: vec![adult("person-a"), adult("person-b")],
            relationship: HouseholdRelationship::RoommatesOrUnrelated,
            dependents: vec![],
            shared_allocations: vec![SharedAllocation {
                item_id: shared_item_id("secondary-property"),
                item_type: SharedItemType::SecondaryProperty,
                ownership: Ownership::Joint(vec![
                    OwnershipShare {
                        owner: person_id("person-a"),
                        basis_points: 7_000,
                    },
                    OwnershipShare {
                        owner: person_id("person-b"),
                        basis_points: 2_000,
                    },
                ]),
            }],
        };

        assert_eq!(
            household.validate(),
            Err(HouseholdValidationError::InvalidOwnershipTotal {
                expected: 10_000,
                actual: 9_000
            })
        );
    }

    fn adult(id: &str) -> AdultProfile {
        let owner = person_id(id);
        AdultProfile {
            person: PersonProfile {
                id: owner.clone(),
                display_name: None,
                birth_year: Some(1990),
            },
            residence_by_year: vec![ResidenceForTaxYear {
                tax_year: 2026,
                jurisdiction: CanadianJurisdiction::Ontario,
            }],
            income_sources: vec![IncomeSource {
                kind: IncomeSourceKind::Employment,
                owner: owner.clone(),
                annual_amount: MoneyAmount::cad_dollars(100_000),
            }],
            registered_accounts: vec![RegisteredAccountSnapshot {
                owner: owner.clone(),
                account_type: RegisteredAccountType::Tfsa,
                balance: MoneyAmount::cad_dollars(25_000),
                contribution_room: Some(MoneyAmount::cad_dollars(10_000)),
            }],
            non_registered_accounts: vec![NonRegisteredAccountSnapshot {
                owner: Ownership::Sole(owner),
                account_type: NonRegisteredAccountType::TaxableBrokerage,
                balance: MoneyAmount::cad_dollars(5_000),
            }],
        }
    }

    fn household_id(value: &str) -> HouseholdId {
        HouseholdId::new(value).expect("valid household id")
    }

    fn person_id(value: &str) -> PersonId {
        PersonId::new(value).expect("valid person id")
    }

    fn dependent_id(value: &str) -> DependentId {
        DependentId::new(value).expect("valid dependent id")
    }

    fn shared_item_id(value: &str) -> SharedItemId {
        SharedItemId::new(value).expect("valid shared item id")
    }
}
