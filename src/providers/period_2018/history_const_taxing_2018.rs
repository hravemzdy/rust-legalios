﻿use rust_decimal::Decimal;
use crate::providers::history_const_salary::HistoryConstSalary;
use crate::providers::history_const_taxing::HistoryConstTaxing;
use crate::providers::period_2017::history_const_taxing_2017::{HistoryConstTaxing2017, HistoryConstTaxing2017var07};
use crate::providers::period_2018::history_const_salary_2018::HistoryConstSalary2018;

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

pub(crate) struct HistoryConstTaxing2018 {
}

impl HistoryConstTaxing for HistoryConstTaxing2018 {
    const VERSION_CODE: i16 = 2018;

    const ALLOWANCE_PAYER: i32 = HistoryConstTaxing2017::ALLOWANCE_PAYER;
    const ALLOWANCE_DISAB_1ST: i32 = HistoryConstTaxing2017::ALLOWANCE_DISAB_1ST;
    const ALLOWANCE_DISAB_2ND: i32 = HistoryConstTaxing2017::ALLOWANCE_DISAB_2ND;
    const ALLOWANCE_DISAB_3RD: i32 = HistoryConstTaxing2017::ALLOWANCE_DISAB_3RD;
    const ALLOWANCE_STUDY: i32 = HistoryConstTaxing2017::ALLOWANCE_STUDY;
    const ALLOWANCE_CHILD_1ST: i32 = 1267;
    const ALLOWANCE_CHILD_2ND: i32 = HistoryConstTaxing2017var07::ALLOWANCE_CHILD_2ND;
    const ALLOWANCE_CHILD_3RD: i32 = HistoryConstTaxing2017var07::ALLOWANCE_CHILD_3RD;
    const SETTLEMENT_CHILD_2ND: i32 = HistoryConstTaxing2018::ALLOWANCE_CHILD_2ND;
    const SETTLEMENT_CHILD_3RD: i32 = HistoryConstTaxing2018::ALLOWANCE_CHILD_3RD;
    const FACTOR_ADVANCES: Decimal = HistoryConstTaxing2017::FACTOR_ADVANCES;
    const FACTOR_WITHHOLD: Decimal = HistoryConstTaxing2017::FACTOR_WITHHOLD;
    const FACTOR_SOLITARY: Decimal = HistoryConstTaxing2017::FACTOR_SOLITARY;
    const FACTOR_TAXRATE2: Decimal = HistoryConstTaxing2017::FACTOR_TAXRATE2;
    const MIN_AMOUNT_OF_TAXBONUS: i32 = HistoryConstTaxing2017::MIN_AMOUNT_OF_TAXBONUS;
    const MAX_AMOUNT_OF_TAXBONUS: i32 = HistoryConstTaxing2017::MAX_AMOUNT_OF_TAXBONUS;
    const MARGIN_INCOME_OF_TAXBONUS: i32 = (HistoryConstSalary2018::MIN_MONTHLY_WAGE / 2);
    const MARGIN_INCOME_OF_ROUNDING: i32 = HistoryConstTaxing2017::MARGIN_INCOME_OF_ROUNDING;
    const MARGIN_INCOME_OF_WITHHOLD: i32 = HistoryConstTaxing2017::MARGIN_INCOME_OF_WITHHOLD;
    const MARGIN_INCOME_OF_SOLITARY: i32 = (4 * 29979);
    const MARGIN_INCOME_OF_TAXRATE2: i32 = HistoryConstTaxing2017::MARGIN_INCOME_OF_TAXRATE2;
    const MARGIN_INCOME_OF_WHT_EMP: i32 = 2500;
    const MARGIN_INCOME_OF_WHT_AGR: i32 = HistoryConstTaxing2017::MARGIN_INCOME_OF_WHT_AGR;
}

