#[cfg(test)]
//#[cfg(test_report)]

pub(crate) mod factories_history_tests {
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use rust_decimal::Decimal;
    use rust_decimal::prelude::ToPrimitive;
    use rust_decimal::prelude::FromPrimitive;

    pub(crate) const HEALTH_MIN_MONTHLY_BASIS        :i16 = 101;
    pub(crate) const HEALTH_MAX_ANNUALS_BASIS        :i16 = 102;
    pub(crate) const HEALTH_LIM_MONTHLY_STATE        :i16 = 103;
    pub(crate) const HEALTH_LIM_MONTHLY_DIS50        :i16 = 104;
    pub(crate) const HEALTH_FACTOR_COMPOUND          :i16 = 105;
    pub(crate) const HEALTH_FACTOR_EMPLOYEE          :i16 = 106;
    pub(crate) const HEALTH_MARGIN_INCOME_EMP        :i16 = 107;
    pub(crate) const HEALTH_MARGIN_INCOME_AGR        :i16 = 108;

    pub(crate) const SALARY_WORKING_SHIFT_WEEK       :i16 = 201;
    pub(crate) const SALARY_WORKING_SHIFT_TIME       :i16 = 202;
    pub(crate) const SALARY_MIN_MONTHLY_WAGE         :i16 = 203;
    pub(crate) const SALARY_MIN_HOURLY_WAGE          :i16 = 204;

    pub(crate) const SOCIAL_MAX_ANNUALS_BASIS        :i16 = 301;
    pub(crate) const SOCIAL_FACTOR_EMPLOYER          :i16 = 302;
    pub(crate) const SOCIAL_FACTOR_EMPLOYER_HIGHER   :i16 = 303;
    pub(crate) const SOCIAL_FACTOR_EMPLOYEE          :i16 = 304;
    pub(crate) const SOCIAL_FACTOR_EMPLOYEE_GARANT   :i16 = 305;
    pub(crate) const SOCIAL_FACTOR_EMPLOYEE_REDUCE   :i16 = 306;
    pub(crate) const SOCIAL_MARGIN_INCOME_EMP        :i16 = 307;
    pub(crate) const SOCIAL_MARGIN_INCOME_AGR        :i16 = 308;

    pub(crate) const TAXING_ALLOWANCE_PAYER          :i16 = 401;
    pub(crate) const TAXING_ALLOWANCE_DISAB_1ST      :i16 = 402;
    pub(crate) const TAXING_ALLOWANCE_DISAB_2ND      :i16 = 403;
    pub(crate) const TAXING_ALLOWANCE_DISAB_3RD      :i16 = 404;
    pub(crate) const TAXING_ALLOWANCE_STUDY          :i16 = 405;
    pub(crate) const TAXING_ALLOWANCE_CHILD_1ST      :i16 = 406;
    pub(crate) const TAXING_ALLOWANCE_CHILD_2ND      :i16 = 407;
    pub(crate) const TAXING_ALLOWANCE_CHILD_3RD      :i16 = 408;
    pub(crate) const TAXING_FACTOR_ADVANCES          :i16 = 409;
    pub(crate) const TAXING_FACTOR_WITHHOLD          :i16 = 410;
    pub(crate) const TAXING_FACTOR_SOLIDARY          :i16 = 411;
    pub(crate) const TAXING_FACTOR_TAXRATE2          :i16 = 412;
    pub(crate) const TAXING_MIN_AMOUNT_OF_TAXBONUS   :i16 = 413;
    pub(crate) const TAXING_MAX_AMOUNT_OF_TAXBONUS   :i16 = 414;
    pub(crate) const TAXING_MARGIN_INCOME_OF_TAXBONUS:i16 = 415;
    pub(crate) const TAXING_MARGIN_INCOME_OF_ROUNDING:i16 = 416;
    pub(crate) const TAXING_MARGIN_INCOME_OF_WITHHOLD:i16 = 417;
    pub(crate) const TAXING_MARGIN_INCOME_OF_SOLIDARY:i16 = 418;
    pub(crate) const TAXING_MARGIN_INCOME_OF_TAXRATE2:i16 = 419;
    pub(crate) const TAXING_MARGIN_INCOME_OF_WHT_EMP :i16 = 420;
    pub(crate) const TAXING_MARGIN_INCOME_OF_WHT_AGR :i16 = 421;

    pub fn create_history_file(file_name: String) -> Option<File> {
        const PARENT_HISTORY_FOLDER_NAME: &str = "legalios";
        const HISTORY_FOLDER_NAME: &str = "test_history";
        let res_curr_path = std::env::current_dir();
        if res_curr_path.is_err(){
            return None;
        }
        let mut curr_path = res_curr_path.unwrap();
        while !curr_path.ends_with(PARENT_HISTORY_FOLDER_NAME) && curr_path.ancestors().count() != 1 {
            curr_path.pop();
        }
        let base_path = curr_path.join(HISTORY_FOLDER_NAME);
        let res_norm_path = fs::canonicalize(&base_path);
        if res_norm_path.is_err(){
            return None;
        }
        let norm_path = res_norm_path.unwrap();

        let file_path = norm_path.join(file_name);

        let res_file_handle = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path);
        if res_file_handle.is_err(){
            return None;
        }
        let file_handle = res_file_handle.unwrap();

        Some(file_handle)
    }

    pub fn export_history_start(opt_file_handle: &mut Option<File>, data: &Vec<(i16, bool)>) {
        if opt_file_handle.is_none(){
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        write!(file_handle, "History Term").unwrap();
        for col in data {
            if col.1 {
                write!(file_handle, "\t{} Begin Value", col.0).unwrap();
                write!(file_handle, "\t{} Change Month", col.0).unwrap();
                write!(file_handle, "\t{} End Value", col.0).unwrap();
            } else {
                write!(file_handle, "\t{} Value", col.0).unwrap();
            }
        }
        write!(file_handle, "\n").unwrap();
    }

    pub fn export_history_term(opt_file_handle: &mut Option<File>, head: &Vec<(i16, bool)>, data: &(i16, Vec<(i16, i16, String, String)>)) {
        if opt_file_handle.is_none() {
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        write!(file_handle, "{}", history_term_name(data.0)).unwrap();
        for col in &data.1 {
            let header = head.iter().find(|x| x.0 == col.0);
            let year_of_change: bool = match header {
                Some(val) => val.1,
                _ => false,
            };
            write!(file_handle, "\t{}", col.2).unwrap();
            if year_of_change {
                if col.1 == 0 {
                    write!(file_handle, "\t").unwrap();
                } else {
                    write!(file_handle, "\t{}", col.1).unwrap();
                }
                write!(file_handle, "\t{}", col.3).unwrap();
            }
        }
        write!(file_handle, "\n").unwrap();
    }
    #[allow(dead_code)]
    fn history_term_name(term_id: i16) -> &'static str {
        return match term_id {
            HEALTH_MIN_MONTHLY_BASIS => "01_Health_01_MinMonthlyBasis",
            HEALTH_MAX_ANNUALS_BASIS => "01_Health_02_MaxAnnualsBasis",
            HEALTH_LIM_MONTHLY_STATE => "01_Health_03_LimMonthlyState",
            HEALTH_LIM_MONTHLY_DIS50 => "01_Health_04_LimMonthlyDis50",
            HEALTH_FACTOR_COMPOUND => "01_Health_05_FactorCompound",
            HEALTH_FACTOR_EMPLOYEE => "01_Health_06_FactorEmployee",
            HEALTH_MARGIN_INCOME_EMP => "01_Health_07_MarginIncomeEmp",
            HEALTH_MARGIN_INCOME_AGR => "01_Health_08_MarginIncomeAgr",
            SALARY_WORKING_SHIFT_WEEK => "02_Salary_01_WorkingShiftWeek",
            SALARY_WORKING_SHIFT_TIME => "02_Salary_02_WorkingShiftTime",
            SALARY_MIN_MONTHLY_WAGE => "02_Salary_03_MinMonthlyWage",
            SALARY_MIN_HOURLY_WAGE => "02_Salary_04_MinHourlyWage",
            SOCIAL_MAX_ANNUALS_BASIS => "03_Social_01_MaxAnnualsBasis",
            SOCIAL_FACTOR_EMPLOYER => "03_Social_02_FactorEmployer",
            SOCIAL_FACTOR_EMPLOYER_HIGHER => "03_Social_03_FactorEmployerHigher",
            SOCIAL_FACTOR_EMPLOYEE => "03_Social_04_FactorEmployee",
            SOCIAL_FACTOR_EMPLOYEE_GARANT => "03_Social_05_FactorEmployeeGarant",
            SOCIAL_FACTOR_EMPLOYEE_REDUCE => "03_Social_06_FactorEmployeeReduce",
            SOCIAL_MARGIN_INCOME_EMP => "03_Social_07_MarginIncomeEmp",
            SOCIAL_MARGIN_INCOME_AGR => "03_Social_08_MarginIncomeAgr",
            TAXING_ALLOWANCE_PAYER => "04_Taxing_01_AllowancePayer",
            TAXING_ALLOWANCE_DISAB_1ST => "04_Taxing_02_AllowanceDisab1st",
            TAXING_ALLOWANCE_DISAB_2ND => "04_Taxing_03_AllowanceDisab2nd",
            TAXING_ALLOWANCE_DISAB_3RD => "04_Taxing_04_AllowanceDisab3rd",
            TAXING_ALLOWANCE_STUDY => "04_Taxing_05_AllowanceStudy",
            TAXING_ALLOWANCE_CHILD_1ST => "04_Taxing_06_AllowanceChild1st",
            TAXING_ALLOWANCE_CHILD_2ND => "04_Taxing_07_AllowanceChild2nd",
            TAXING_ALLOWANCE_CHILD_3RD => "04_Taxing_08_AllowanceChild3rd",
            TAXING_FACTOR_ADVANCES => "04_Taxing_09_FactorAdvances",
            TAXING_FACTOR_WITHHOLD => "04_Taxing_10_FactorWithhold",
            TAXING_FACTOR_SOLIDARY => "04_Taxing_11_FactorSolidary",
            TAXING_FACTOR_TAXRATE2 => "04_Taxing_12_FactorTaxRate2",
            TAXING_MIN_AMOUNT_OF_TAXBONUS => "04_Taxing_13_MinAmountOfTaxBonus",
            TAXING_MAX_AMOUNT_OF_TAXBONUS => "04_Taxing_14_MaxAmountOfTaxBonus",
            TAXING_MARGIN_INCOME_OF_TAXBONUS => "04_Taxing_15_MarginIncomeOfTaxBonus",
            TAXING_MARGIN_INCOME_OF_ROUNDING => "04_Taxing_16_MarginIncomeOfRounding",
            TAXING_MARGIN_INCOME_OF_WITHHOLD => "04_Taxing_17_MarginIncomeOfWithhold",
            TAXING_MARGIN_INCOME_OF_SOLIDARY => "04_Taxing_18_MarginIncomeOfSolidary",
            TAXING_MARGIN_INCOME_OF_TAXRATE2 => "04_Taxing_18_MarginIncomeOfTaxRate2",
            TAXING_MARGIN_INCOME_OF_WHT_EMP => "04_Taxing_20_MarginIncomeOfWthEmp",
            TAXING_MARGIN_INCOME_OF_WHT_AGR => "04_Taxing_21_MarginIncomeOfWthAgr",
            _ => "Unknown Term",
        };
    }

    #[allow(dead_code)]
    fn history_term_name_cz(term_id: i16) -> &'static str {
        return match term_id {
            HEALTH_MIN_MONTHLY_BASIS => "01_Health_01 Minimální základ zdravotního pojištění na jednoho pracovníka",
            HEALTH_MAX_ANNUALS_BASIS => "01_Health_02 Maximální roční vyměřovací základ na jednoho pracovníka (tzv.strop)",
            HEALTH_LIM_MONTHLY_STATE => "01_Health_03 Vyměřovací základ ze kterého platí pojistné stát za státní pojištěnce (mateřská, studenti, důchodci)",
            HEALTH_LIM_MONTHLY_DIS50 => "01_Health_04 Vyměřovací základ ze kterého platí pojistné stát za státní pojištěnce (mateřská, studenti, důchodci) u zaměstnavatele zaměstnávajícího více než 50% osob OZP",
            HEALTH_FACTOR_COMPOUND => "01_Health_05 složená sazba zdravotního pojištění (zaměstnace + zaměstnavatele)",
            HEALTH_FACTOR_EMPLOYEE => "01_Health_06 podíl sazby zdravotního pojištění připadajícího na zaměstnace (1/FACTOR_EMPLOYEE)",
            HEALTH_MARGIN_INCOME_EMP => "01_Health_07 hranice příjmu pro vznik účasti na pojištění pro zaměstnace v pracovním poměru",
            HEALTH_MARGIN_INCOME_AGR => "01_Health_08 hranice příjmu pro vznik účasti na pojištění pro zaměstnace na dohodu",
            SALARY_WORKING_SHIFT_WEEK => "02_Salary_01 Počet pracovních dnů v týdnu",
            SALARY_WORKING_SHIFT_TIME => "02_Salary_02 Počet pracovních hodin denně",
            SALARY_MIN_MONTHLY_WAGE => "02_Salary_03 Minimální mzda měsíční",
            SALARY_MIN_HOURLY_WAGE => "02_Salary_04 Minimální mzda hodinová (100*Kč)",
            SOCIAL_MAX_ANNUALS_BASIS => "03_Social_01 Maximální roční vyměřovací základ na jednoho pracovníka (tzv.strop)",
            SOCIAL_FACTOR_EMPLOYER => "03_Social_02 Sazba - standardní sociálního pojištění - zaměstnavatele",
            SOCIAL_FACTOR_EMPLOYER_HIGHER => "03_Social_03 Sazba - vyší sociálního pojištění - zaměstnavatele",
            SOCIAL_FACTOR_EMPLOYEE => "03_Social_04 Sazba sociálního pojištění - zaměstnance",
            SOCIAL_FACTOR_EMPLOYEE_GARANT => "03_Social_05 Sazba důchodového spoření - zaměstnance - s důchodovým spořením",
            SOCIAL_FACTOR_EMPLOYEE_REDUCE => "03_Social_06 Snížení sazby sociálního pojištění - zaměstnance - s důchodovým spořením",
            SOCIAL_MARGIN_INCOME_EMP => "03_Social_07 hranice příjmu pro vznik účasti na pojištění pro zaměstnace v pracovním poměru",
            SOCIAL_MARGIN_INCOME_AGR => "03_Social_08 hranice příjmu pro vznik účasti na pojištění pro zaměstnace na dohodu",
            TAXING_ALLOWANCE_PAYER => "04_Taxing_01 Částka slevy na poplatníka",
            TAXING_ALLOWANCE_DISAB_1ST => "04_Taxing_02 Částka slevy na invaliditu 1.stupně poplatníka",
            TAXING_ALLOWANCE_DISAB_2ND => "04_Taxing_03 Částka slevy na invaliditu 2.stupně poplatníka",
            TAXING_ALLOWANCE_DISAB_3RD => "04_Taxing_04 Částka slevy na invaliditu 3.stupně poplatníka",
            TAXING_ALLOWANCE_STUDY => "04_Taxing_05 Částka slevy na poplatníka studenta",
            TAXING_ALLOWANCE_CHILD_1ST => "04_Taxing_06 Částka slevy na dítě 1.pořadí",
            TAXING_ALLOWANCE_CHILD_2ND => "04_Taxing_07 Částka slevy na dítě 2.pořadí",
            TAXING_ALLOWANCE_CHILD_3RD => "04_Taxing_08 Částka slevy na dítě 3.pořadí",
            TAXING_FACTOR_ADVANCES => "04_Taxing_09 Sazba daně na zálohový příjem",
            TAXING_FACTOR_WITHHOLD => "04_Taxing_10 Sazba daně na srážkový příjem",
            TAXING_FACTOR_SOLIDARY => "04_Taxing_11 Sazba daně na solidární zvýšení",
            TAXING_FACTOR_TAXRATE2 => "04_Taxing_12 Sazba daně pro druhé pásmo daně",
            TAXING_MIN_AMOUNT_OF_TAXBONUS => "04_Taxing_13 Minimální částka pro daňový bonus",
            TAXING_MAX_AMOUNT_OF_TAXBONUS => "04_Taxing_14 Maximální částka pro daňový bonus",
            TAXING_MARGIN_INCOME_OF_TAXBONUS => "04_Taxing_15 Minimální výše příjmu pro nároku na daňový bonus",
            TAXING_MARGIN_INCOME_OF_ROUNDING => "04_Taxing_16 Maximální výše příjmu pro zaokrouhlování",
            TAXING_MARGIN_INCOME_OF_WITHHOLD => "04_Taxing_17 Maximální výše příjmu pro srážkový příjem",
            TAXING_MARGIN_INCOME_OF_SOLIDARY => "04_Taxing_18 Minimální výše příjmu pro solidární zvýšení daně",
            TAXING_MARGIN_INCOME_OF_TAXRATE2 => "04_Taxing_18 Minimální výše příjmu pro druhé pásmo daně",
            TAXING_MARGIN_INCOME_OF_WHT_EMP => "04_Taxing_20 hranice příjmu pro srážkovou daň pro zaměstnace v pracovním poměru (nepodepsal prohlášení)",
            TAXING_MARGIN_INCOME_OF_WHT_AGR => "04_Taxing_21 hranice příjmu pro srážkovou daň pro zaměstnace na dohodu (nepodepsal prohlášení)",
            _ => "Neznámý Termín",
        };
    }

    pub fn props_int_value_to_string(value: i32) -> String {
        return format!("{}", value);
    }
    pub fn props_dec_value_to_string(value: Decimal) -> String {
        let dec_option = value*Decimal::from_i32(100).unwrap();
        let int_option = dec_option.to_i32();
        let int_value: i32 = match int_option {
            Some(value) => value,
            None=> 0i32,
        };
        return format!("{}", int_value);
    }
    pub fn close_history_file(opt_file_handle: &mut Option<File>) {
        if opt_file_handle.is_none(){
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        file_handle.flush().unwrap();
    }
}