use rust_decimal_macros::dec;
use rust_decimal::Decimal;
use crate::providers::history_const_salary::HistoryConstSalary;
use crate::providers::history_const_taxing::HistoryConstTaxing;
use crate::providers::period_2010::history_const_salary_2010::HistoryConstSalary2010;

// ALLOWANCE_PAYER                  Částka slevy na poplatníka
//
// ALLOWANCE_DISAB_1ST              Částka slevy na invaliditu 1.stupně poplatníka
//
// ALLOWANCE_DISAB_2ND              Částka slevy na invaliditu 2.stupně poplatníka
//
// ALLOWANCE_DISAB_3RD              Částka slevy na invaliditu 3.stupně poplatníka
//
// ALLOWANCE_STUDY                  Částka slevy na poplatníka studenta
//
// ALLOWANCE_CHILD_1ST              Částka slevy na dítě 1.pořadí
//
// ALLOWANCE_CHILD_2ND              Částka slevy na dítě 2.pořadí
//
// ALLOWANCE_CHILD_3RD              Částka slevy na dítě 3.pořadí
//
// FACTOR_ADVANCES                  Sazba daně na zálohový příjem
//
// FACTOR_WITHHOLD                  Sazba daně na srážkový příjem
//
// FACTOR_SOLIDARY                  Sazba daně na solidární zvýšení
//
// FACTOR_TAXRATE2                  Sazba daně pro druhé pásmo daně
//
// MIN_AMOUNT_OF_TAXBONUS           Minimální částka pro daňový bonus
//
// MAX_AMOUNT_OF_TAXBONUS           Maximální částka pro daňový bonus
//
// MARGIN_INCOME_OF_TAXBONUS        Minimální výše příjmu pro nároku na daňový bonus
//
// MARGIN_INCOME_OF_ROUNDING        Maximální výše příjmu pro zaokrouhlování
//
// MARGIN_INCOME_OF_WITHHOLD        Maximální výše příjmu pro srážkový příjem
//
// MARGIN_INCOME_OF_SOLIDARY        Minimální výše příjmu pro solidární zvýšení daně
//
// MARGIN_INCOME_OF_TAXRATE2        Minimální výše příjmu pro druhé pásmo daně
//
// MARGIN_INCOME_OF_WHT_AGR         hranice příjmu pro srážkovou daň pro zaměstnace v pracovním poměru (nepodepsal prohlášení)
//
// MARGIN_INCOME_OF_WHT_EMP         hranice příjmu pro srážkovou daň pro zaměstnace na dohodu (nepodepsal prohlášení)

pub(crate) struct HistoryConstTaxing2010 {
}

impl HistoryConstTaxing for HistoryConstTaxing2010 {
    const VERSION_CODE: i16 = 2010;

    const ALLOWANCE_PAYER: i32 = 2070;
    const ALLOWANCE_DISAB_1ST: i32 = 210;
    const ALLOWANCE_DISAB_2ND: i32 = 420;
    const ALLOWANCE_DISAB_3RD: i32 = 1345;
    const ALLOWANCE_STUDY: i32 = 335;
    const ALLOWANCE_CHILD_1ST: i32 = 967;
    const ALLOWANCE_CHILD_2ND: i32 = 967;
    const ALLOWANCE_CHILD_3RD: i32 = 967;
    const SETTLEMENT_CHILD_2ND: i32 = HistoryConstTaxing2010::ALLOWANCE_CHILD_2ND;
    const SETTLEMENT_CHILD_3RD: i32 = HistoryConstTaxing2010::ALLOWANCE_CHILD_3RD;
    const FACTOR_ADVANCES: Decimal = dec!(15);
    const FACTOR_WITHHOLD: Decimal = dec!(15);
    const FACTOR_SOLITARY: Decimal = Decimal::ZERO;
    const FACTOR_TAXRATE2: Decimal = Decimal::ZERO;
    const MIN_AMOUNT_OF_TAXBONUS: i32 = 50;
    const MAX_AMOUNT_OF_TAXBONUS: i32 = 4350;
    const MARGIN_INCOME_OF_TAXBONUS: i32 = (HistoryConstSalary2010::MIN_MONTHLY_WAGE / 2);
    const MARGIN_INCOME_OF_ROUNDING: i32 = 100;
    const MARGIN_INCOME_OF_WITHHOLD: i32 = 5000;
    const MARGIN_INCOME_OF_SOLITARY: i32 = 0;
    const MARGIN_INCOME_OF_TAXRATE2: i32 = 0;
    const MARGIN_INCOME_OF_WHT_EMP: i32 = 0;
    const MARGIN_INCOME_OF_WHT_AGR: i32 = 0;
}