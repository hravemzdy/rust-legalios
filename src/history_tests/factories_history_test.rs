#[cfg(test)]

//#[cfg(test_report)]
mod factories_history_tests {
    use std::error::Error;
    use crate::factories::provider_factory::{health_factory, salary_factory, social_factory, taxing_factory, ProviderHealthFactory, ProviderSalaryFactory, ProviderSocialFactory, ProviderTaxingFactory, IProviderSalaryFactory, IProviderHealthFactory, IProviderSocialFactory, IProviderTaxingFactory};
    use crate::service::period;
    use crate::service::period::{IPeriod};
    use crate::history_tests::service_report_history_tests::factories_history_tests::create_history_file;
    use crate::history_tests::service_report_history_tests::factories_history_tests::export_history_start;
    use crate::history_tests::service_report_history_tests::factories_history_tests::export_history_term;
    use crate::history_tests::service_report_history_tests::factories_history_tests::close_history_file;
    use crate::history_tests::service_report_history_tests::factories_history_tests::props_int_value_to_string;
    use crate::history_tests::service_report_history_tests::factories_history_tests::props_dec_value_to_string;

    use crate::history_tests::service_report_history_tests::factories_history_tests::HEALTH_MIN_MONTHLY_BASIS;
    use crate::history_tests::service_report_history_tests::factories_history_tests::HEALTH_MAX_ANNUALS_BASIS;
    use crate::history_tests::service_report_history_tests::factories_history_tests::HEALTH_LIM_MONTHLY_STATE;
    use crate::history_tests::service_report_history_tests::factories_history_tests::HEALTH_LIM_MONTHLY_DIS50;
    use crate::history_tests::service_report_history_tests::factories_history_tests::HEALTH_FACTOR_COMPOUND  ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::HEALTH_FACTOR_EMPLOYEE  ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::HEALTH_MARGIN_INCOME_EMP;
    use crate::history_tests::service_report_history_tests::factories_history_tests::HEALTH_MARGIN_INCOME_AGR;

    use crate::history_tests::service_report_history_tests::factories_history_tests::SALARY_WORKING_SHIFT_WEEK       ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::SALARY_WORKING_SHIFT_TIME       ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::SALARY_MIN_MONTHLY_WAGE         ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::SALARY_MIN_HOURLY_WAGE          ;

    use crate::history_tests::service_report_history_tests::factories_history_tests::SOCIAL_MAX_ANNUALS_BASIS        ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::SOCIAL_FACTOR_EMPLOYER          ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::SOCIAL_FACTOR_EMPLOYER_HIGHER   ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::SOCIAL_FACTOR_EMPLOYEE          ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::SOCIAL_FACTOR_EMPLOYEE_GARANT   ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::SOCIAL_FACTOR_EMPLOYEE_REDUCE   ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::SOCIAL_MARGIN_INCOME_EMP        ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::SOCIAL_MARGIN_INCOME_AGR        ;

    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_ALLOWANCE_PAYER          ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_ALLOWANCE_DISAB_1ST      ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_ALLOWANCE_DISAB_2ND      ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_ALLOWANCE_DISAB_3RD      ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_ALLOWANCE_STUDY          ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_ALLOWANCE_CHILD_1ST      ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_ALLOWANCE_CHILD_2ND      ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_ALLOWANCE_CHILD_3RD      ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_FACTOR_ADVANCES          ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_FACTOR_WITHHOLD          ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_FACTOR_SOLIDARY          ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_FACTOR_TAXRATE2          ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_MIN_AMOUNT_OF_TAXBONUS   ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_MAX_AMOUNT_OF_TAXBONUS   ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_MARGIN_INCOME_OF_TAXBONUS;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_MARGIN_INCOME_OF_ROUNDING;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_MARGIN_INCOME_OF_WITHHOLD;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_MARGIN_INCOME_OF_SOLIDARY;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_MARGIN_INCOME_OF_TAXRATE2;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_MARGIN_INCOME_OF_WHT_EMP ;
    use crate::history_tests::service_report_history_tests::factories_history_tests::TAXING_MARGIN_INCOME_OF_WHT_AGR ;

    #[test]
    pub(crate) fn test_factories_history_test() -> Result<(), Box<dyn Error>> {
        let min_year: i16 = 2010;
        let max_year: i16 = 2022;

        let _sut_salary : ProviderSalaryFactory = salary_factory();
        let _sut_health : ProviderHealthFactory = health_factory();
        let _sut_social : ProviderSocialFactory = social_factory();
        let _sut_taxing : ProviderTaxingFactory = taxing_factory();


        let mut file = create_history_file(format!("history_{}_{}.xls", min_year, max_year));

        let mut header_data:Vec<(i16, bool)> = vec![];
        for test_year in min_year..=max_year {
            let mut year_with_changes = false;

            let test_period = period::Period::get_with_year_month(test_year, 1);

            let mut test_salary_prop = _sut_salary.get_props(&test_period);
            let mut test_health_prop = _sut_health.get_props(&test_period);
            let mut test_social_prop = _sut_social.get_props(&test_period);
            let mut test_taxing_prop = _sut_taxing.get_props(&test_period);

            for test_month in 2i16..=12i16 {
                let next_period = period::Period::get_with_year_month(test_year, test_month);

                let test_salary_next = _sut_salary.get_props(&next_period);
                let test_health_next = _sut_health.get_props(&next_period);
                let test_social_next = _sut_social.get_props(&next_period);
                let test_taxing_next = _sut_taxing.get_props(&next_period);

                if test_salary_next.value_equals(&test_salary_prop) == false {
                    year_with_changes = true;
                }
                if test_health_next.value_equals(&test_health_prop) == false {
                    year_with_changes = true;
                }
                if test_social_next.value_equals(&test_social_prop) == false {
                    year_with_changes = true;
                }
                if test_taxing_next.value_equals(&test_taxing_prop) == false {
                    year_with_changes = true;
                }
                test_salary_prop = test_salary_next;
                test_health_prop = test_health_next;
                test_social_prop = test_social_next;
                test_taxing_prop = test_taxing_next;
            }
            header_data.push((test_year, year_with_changes));
        }
        export_history_start(&mut file, &header_data);

        let mut vect_health_min_monthly_basis: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_health_max_annuals_basis: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_health_lim_monthly_state: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_health_lim_monthly_dis50: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_health_factor_compound: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_health_factor_employee: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_health_margin_income_emp: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_health_margin_income_agr: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_salary_working_shift_week: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_salary_working_shift_time: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_salary_min_monthly_wage: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_salary_min_hourly_wage: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_max_annuals_basis: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_factor_employer: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_factor_employer_higher: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_factor_employee: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_factor_employee_garant: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_factor_employee_reduce: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_margin_income_emp: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_social_margin_income_agr: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_payer: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_disab_1st: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_disab_2nd: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_disab_3rd: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_study: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_child_1st: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_child_2nd: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_allowance_child_3rd: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_factor_advances: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_factor_withhold: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_factor_solidary: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_factor_taxrate2: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_min_amount_of_taxbonus: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_max_amount_of_taxbonus: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_margin_income_of_taxbonus: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_margin_income_of_rounding: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_margin_income_of_withhold: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_margin_income_of_solidary: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_margin_income_of_taxrate2: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_margin_income_of_wht_emp: Vec<(i16, i16, String, String)> = vec![];
        let mut vect_taxing_margin_income_of_wht_agr: Vec<(i16, i16, String, String)> = vec![];

        for test_year in min_year..=max_year {
            let mut mes_health_min_monthly_basis = 0;
            let mut mes_health_max_annuals_basis = 0;
            let mut mes_health_lim_monthly_state = 0;
            let mut mes_health_lim_monthly_dis50 = 0;
            let mut mes_health_factor_compound = 0;
            let mut mes_health_factor_employee = 0;
            let mut mes_health_margin_income_emp = 0;
            let mut mes_health_margin_income_agr = 0;
            let mut mes_salary_working_shift_week = 0;
            let mut mes_salary_working_shift_time = 0;
            let mut mes_salary_min_monthly_wage = 0;
            let mut mes_salary_min_hourly_wage = 0;
            let mut mes_social_max_annuals_basis = 0;
            let mut mes_social_factor_employer = 0;
            let mut mes_social_factor_employer_higher = 0;
            let mut mes_social_factor_employee = 0;
            let mut mes_social_factor_employee_garant = 0;
            let mut mes_social_factor_employee_reduce = 0;
            let mut mes_social_margin_income_emp = 0;
            let mut mes_social_margin_income_agr = 0;
            let mut mes_taxing_allowance_payer = 0;
            let mut mes_taxing_allowance_disab_1st = 0;
            let mut mes_taxing_allowance_disab_2nd = 0;
            let mut mes_taxing_allowance_disab_3rd = 0;
            let mut mes_taxing_allowance_study = 0;
            let mut mes_taxing_allowance_child_1st = 0;
            let mut mes_taxing_allowance_child_2nd = 0;
            let mut mes_taxing_allowance_child_3rd = 0;
            let mut mes_taxing_factor_advances = 0;
            let mut mes_taxing_factor_withhold = 0;
            let mut mes_taxing_factor_solidary = 0;
            let mut mes_taxing_factor_taxrate2 = 0;
            let mut mes_taxing_min_amount_of_taxbonus = 0;
            let mut mes_taxing_max_amount_of_taxbonus = 0;
            let mut mes_taxing_margin_income_of_taxbonus = 0;
            let mut mes_taxing_margin_income_of_rounding = 0;
            let mut mes_taxing_margin_income_of_withhold = 0;
            let mut mes_taxing_margin_income_of_solidary = 0;
            let mut mes_taxing_margin_income_of_taxrate2 = 0;
            let mut mes_taxing_margin_income_of_wht_emp = 0;
            let mut mes_taxing_margin_income_of_wht_agr = 0;

            let test_period: &dyn IPeriod = &period::Period::get_with_year_month(test_year, 1);

            let mut test_salary_prop = _sut_salary.get_props(test_period);
            let mut test_health_prop = _sut_health.get_props(test_period);
            let mut test_social_prop = _sut_social.get_props(test_period);
            let mut test_taxing_prop = _sut_taxing.get_props(test_period);

            let jan_health_min_monthly_basis = props_int_value_to_string(test_health_prop.min_monthly_basis());
            let jan_health_max_annuals_basis = props_int_value_to_string(test_health_prop.max_annuals_basis());
            let jan_health_lim_monthly_state = props_int_value_to_string(test_health_prop.lim_monthly_state());
            let jan_health_lim_monthly_dis50 = props_int_value_to_string(test_health_prop.lim_monthly_dis50());
            let jan_health_factor_compound = props_dec_value_to_string(test_health_prop.factor_compound());
            let jan_health_factor_employee = props_dec_value_to_string(test_health_prop.factor_employee());
            let jan_health_margin_income_emp = props_int_value_to_string(test_health_prop.margin_income_emp());
            let jan_health_margin_income_agr = props_int_value_to_string(test_health_prop.margin_income_agr());
            let jan_salary_working_shift_week = props_int_value_to_string(test_salary_prop.working_shift_week());
            let jan_salary_working_shift_time = props_int_value_to_string(test_salary_prop.working_shift_time());
            let jan_salary_min_monthly_wage = props_int_value_to_string(test_salary_prop.min_monthly_wage());
            let jan_salary_min_hourly_wage = props_int_value_to_string(test_salary_prop.min_hourly_wage());
            let jan_social_max_annuals_basis = props_int_value_to_string(test_social_prop.max_annuals_basis());
            let jan_social_factor_employer = props_dec_value_to_string(test_social_prop.factor_employer());
            let jan_social_factor_employer_higher = props_dec_value_to_string(test_social_prop.factor_employer_higher());
            let jan_social_factor_employee = props_dec_value_to_string(test_social_prop.factor_employee());
            let jan_social_factor_employee_garant = props_dec_value_to_string(test_social_prop.factor_employee_garant());
            let jan_social_factor_employee_reduce = props_dec_value_to_string(test_social_prop.factor_employee_reduce());
            let jan_social_margin_income_emp = props_int_value_to_string(test_social_prop.margin_income_emp());
            let jan_social_margin_income_agr = props_int_value_to_string(test_social_prop.margin_income_agr());
            let jan_taxing_allowance_payer = props_int_value_to_string(test_taxing_prop.allowance_payer());
            let jan_taxing_allowance_disab_1st = props_int_value_to_string(test_taxing_prop.allowance_disab1st());
            let jan_taxing_allowance_disab_2nd = props_int_value_to_string(test_taxing_prop.allowance_disab2nd());
            let jan_taxing_allowance_disab_3rd = props_int_value_to_string(test_taxing_prop.allowance_disab3rd());
            let jan_taxing_allowance_study = props_int_value_to_string(test_taxing_prop.allowance_study());
            let jan_taxing_allowance_child_1st = props_int_value_to_string(test_taxing_prop.allowance_child1st());
            let jan_taxing_allowance_child_2nd = props_int_value_to_string(test_taxing_prop.allowance_child2nd());
            let jan_taxing_allowance_child_3rd = props_int_value_to_string(test_taxing_prop.allowance_child3rd());
            let jan_taxing_factor_advances = props_dec_value_to_string(test_taxing_prop.factor_advances());
            let jan_taxing_factor_withhold = props_dec_value_to_string(test_taxing_prop.factor_withhold());
            let jan_taxing_factor_solidary = props_dec_value_to_string(test_taxing_prop.factor_solidary());
            let jan_taxing_factor_taxrate2 = props_dec_value_to_string(test_taxing_prop.factor_taxrate2());
            let jan_taxing_min_amount_of_taxbonus = props_int_value_to_string(test_taxing_prop.min_amount_of_tax_bonus());
            let jan_taxing_max_amount_of_taxbonus = props_int_value_to_string(test_taxing_prop.max_amount_of_tax_bonus());
            let jan_taxing_margin_income_of_taxbonus = props_int_value_to_string(test_taxing_prop.margin_income_of_tax_bonus());
            let jan_taxing_margin_income_of_rounding = props_int_value_to_string(test_taxing_prop.margin_income_of_rounding());
            let jan_taxing_margin_income_of_withhold = props_int_value_to_string(test_taxing_prop.margin_income_of_withhold());
            let jan_taxing_margin_income_of_solidary = props_int_value_to_string(test_taxing_prop.margin_income_of_solidary());
            let jan_taxing_margin_income_of_taxrate2 = props_int_value_to_string(test_taxing_prop.margin_income_of_taxrate2());
            let jan_taxing_margin_income_of_wht_emp = props_int_value_to_string(test_taxing_prop.margin_income_of_wth_emp());
            let jan_taxing_margin_income_of_wht_agr = props_int_value_to_string(test_taxing_prop.margin_income_of_wth_agr());

            for test_month in 2i16..=12i16 {
                let next_period: &dyn IPeriod  = &period::Period::get_with_year_month(test_year, test_month);
                
                let test_salary_next = _sut_salary.get_props(next_period);
                let test_health_next = _sut_health.get_props(next_period);
                let test_social_next = _sut_social.get_props(next_period);
                let test_taxing_next = _sut_taxing.get_props(next_period);

                if test_health_next.min_monthly_basis().eq(&test_health_prop.min_monthly_basis())==false { mes_health_min_monthly_basis = test_month; }
                if test_health_next.max_annuals_basis().eq(&test_health_prop.max_annuals_basis())==false { mes_health_max_annuals_basis = test_month; }
                if test_health_next.lim_monthly_state().eq(&test_health_prop.lim_monthly_state())==false { mes_health_lim_monthly_state = test_month; }
                if test_health_next.lim_monthly_dis50().eq(&test_health_prop.lim_monthly_dis50())==false { mes_health_lim_monthly_dis50 = test_month; }
                if test_health_next.factor_compound().eq(&test_health_prop.factor_compound())==false { mes_health_factor_compound = test_month;}
                if test_health_next.factor_employee().eq(&test_health_prop.factor_employee())==false { mes_health_factor_employee = test_month; }
                if test_health_next.margin_income_emp().eq(&test_health_prop.margin_income_emp())==false { mes_health_margin_income_emp = test_month; }
                if test_health_next.margin_income_agr().eq(&test_health_prop.margin_income_agr())==false { mes_health_margin_income_agr = test_month; }
                if test_salary_next.working_shift_week().eq(&test_salary_prop.working_shift_week())==false { mes_salary_working_shift_week = test_month; }
                if test_salary_next.working_shift_time().eq(&test_salary_prop.working_shift_time())==false { mes_salary_working_shift_time = test_month; }
                if test_salary_next.min_monthly_wage().eq(&test_salary_prop.min_monthly_wage())==false { mes_salary_min_monthly_wage = test_month; }
                if test_salary_next.min_hourly_wage().eq(&test_salary_prop.min_hourly_wage())==false { mes_salary_min_hourly_wage = test_month; }
                if test_social_next.max_annuals_basis().eq(&test_social_prop.max_annuals_basis())==false { mes_social_max_annuals_basis = test_month; }
                if test_social_next.factor_employer().eq(&test_social_prop.factor_employer())==false { mes_social_factor_employer = test_month; }
                if test_social_next.factor_employer_higher().eq(&test_social_prop.factor_employer_higher())==false { mes_social_factor_employer_higher = test_month; }
                if test_social_next.factor_employee().eq(&test_social_prop.factor_employee())==false { mes_social_factor_employee = test_month; }
                if test_social_next.factor_employee_garant().eq(&test_social_prop.factor_employee_garant())==false { mes_social_factor_employee_reduce = test_month; }
                if test_social_next.factor_employee_reduce().eq(&test_social_prop.factor_employee_reduce())==false { mes_social_factor_employee_garant = test_month; }
                if test_social_next.margin_income_emp().eq(&test_social_prop.margin_income_emp())==false { mes_social_margin_income_emp = test_month; }
                if test_social_next.margin_income_agr().eq(&test_social_prop.margin_income_agr())==false { mes_social_margin_income_agr = test_month; }
                if test_taxing_next.allowance_payer().eq(&test_taxing_prop.allowance_payer())==false { mes_taxing_allowance_payer = test_month; }
                if test_taxing_next.allowance_disab1st().eq(&test_taxing_prop.allowance_disab1st())==false { mes_taxing_allowance_disab_1st = test_month; }
                if test_taxing_next.allowance_disab2nd().eq(&test_taxing_prop.allowance_disab2nd())==false { mes_taxing_allowance_disab_2nd = test_month; }
                if test_taxing_next.allowance_disab3rd().eq(&test_taxing_prop.allowance_disab3rd())==false { mes_taxing_allowance_disab_3rd = test_month; }
                if test_taxing_next.allowance_study().eq(&test_taxing_prop.allowance_study())==false { mes_taxing_allowance_study = test_month; }
                if test_taxing_next.allowance_child1st().eq(&test_taxing_prop.allowance_child1st())==false { mes_taxing_allowance_child_1st = test_month; }
                if test_taxing_next.allowance_child2nd().eq(&test_taxing_prop.allowance_child2nd())==false { mes_taxing_allowance_child_2nd = test_month; }
                if test_taxing_next.allowance_child3rd().eq(&test_taxing_prop.allowance_child3rd())==false { mes_taxing_allowance_child_3rd = test_month; }
                if test_taxing_next.factor_advances().eq(&test_taxing_prop.factor_advances())==false { mes_taxing_factor_advances = test_month; }
                if test_taxing_next.factor_withhold().eq(&test_taxing_prop.factor_withhold())==false { mes_taxing_factor_withhold = test_month; }
                if test_taxing_next.factor_solidary().eq(&test_taxing_prop.factor_solidary())==false { mes_taxing_factor_solidary = test_month; }
                if test_taxing_next.factor_taxrate2().eq(&test_taxing_prop.factor_taxrate2())==false { mes_taxing_factor_taxrate2 = test_month; }
                if test_taxing_next.min_amount_of_tax_bonus().eq(&test_taxing_prop.min_amount_of_tax_bonus())==false { mes_taxing_min_amount_of_taxbonus = test_month; }
                if test_taxing_next.max_amount_of_tax_bonus().eq(&test_taxing_prop.max_amount_of_tax_bonus())==false { mes_taxing_max_amount_of_taxbonus = test_month; }
                if test_taxing_next.margin_income_of_tax_bonus().eq(&test_taxing_prop.margin_income_of_tax_bonus())==false { mes_taxing_margin_income_of_taxbonus = test_month; }
                if test_taxing_next.margin_income_of_rounding().eq(&test_taxing_prop.margin_income_of_rounding())==false { mes_taxing_margin_income_of_rounding = test_month; }
                if test_taxing_next.margin_income_of_withhold().eq(&test_taxing_prop.margin_income_of_withhold())==false { mes_taxing_margin_income_of_withhold = test_month; }
                if test_taxing_next.margin_income_of_solidary().eq(&test_taxing_prop.margin_income_of_solidary())==false { mes_taxing_margin_income_of_solidary = test_month; }
                if test_taxing_next.margin_income_of_taxrate2().eq(&test_taxing_prop.margin_income_of_taxrate2())==false { mes_taxing_margin_income_of_taxrate2 = test_month; }
                if test_taxing_next.margin_income_of_wth_emp().eq(&test_taxing_prop.margin_income_of_wth_emp())==false { mes_taxing_margin_income_of_wht_emp = test_month; }
                if test_taxing_next.margin_income_of_wth_agr().eq(&test_taxing_prop.margin_income_of_wth_agr())==false { mes_taxing_margin_income_of_wht_agr = test_month; }

                test_salary_prop = test_salary_next;
                test_health_prop = test_health_next;
                test_social_prop = test_social_next;
                test_taxing_prop = test_taxing_next;
            }
            vect_health_min_monthly_basis.push((test_year, mes_health_min_monthly_basis, jan_health_min_monthly_basis, props_int_value_to_string(test_health_prop.min_monthly_basis())));
            vect_health_max_annuals_basis.push((test_year, mes_health_max_annuals_basis, jan_health_max_annuals_basis, props_int_value_to_string(test_health_prop.max_annuals_basis())));
            vect_health_lim_monthly_state.push((test_year, mes_health_lim_monthly_state, jan_health_lim_monthly_state, props_int_value_to_string(test_health_prop.lim_monthly_state())));
            vect_health_lim_monthly_dis50.push((test_year, mes_health_lim_monthly_dis50, jan_health_lim_monthly_dis50, props_int_value_to_string(test_health_prop.lim_monthly_dis50())));
            vect_health_factor_compound.push((test_year, mes_health_factor_compound, jan_health_factor_compound, props_dec_value_to_string(test_health_prop.factor_compound())));
            vect_health_factor_employee.push((test_year, mes_health_factor_employee, jan_health_factor_employee, props_dec_value_to_string(test_health_prop.factor_employee())));
            vect_health_margin_income_emp.push((test_year, mes_health_margin_income_emp, jan_health_margin_income_emp, props_int_value_to_string(test_health_prop.margin_income_emp())));
            vect_health_margin_income_agr.push((test_year, mes_health_margin_income_agr, jan_health_margin_income_agr, props_int_value_to_string(test_health_prop.margin_income_agr())));
            vect_salary_working_shift_week.push((test_year, mes_salary_working_shift_week, jan_salary_working_shift_week, props_int_value_to_string(test_salary_prop.working_shift_week())));
            vect_salary_working_shift_time.push((test_year, mes_salary_working_shift_time, jan_salary_working_shift_time, props_int_value_to_string(test_salary_prop.working_shift_time())));
            vect_salary_min_monthly_wage.push((test_year, mes_salary_min_monthly_wage, jan_salary_min_monthly_wage, props_int_value_to_string(test_salary_prop.min_monthly_wage())));
            vect_salary_min_hourly_wage.push((test_year, mes_salary_min_hourly_wage, jan_salary_min_hourly_wage, props_int_value_to_string(test_salary_prop.min_hourly_wage())));
            vect_social_max_annuals_basis.push((test_year, mes_social_max_annuals_basis, jan_social_max_annuals_basis, props_int_value_to_string(test_social_prop.max_annuals_basis())));
            vect_social_factor_employer.push((test_year, mes_social_factor_employer, jan_social_factor_employer, props_dec_value_to_string(test_social_prop.factor_employer())));
            vect_social_factor_employer_higher.push((test_year, mes_social_factor_employer_higher, jan_social_factor_employer_higher, props_dec_value_to_string(test_social_prop.factor_employer_higher())));
            vect_social_factor_employee.push((test_year, mes_social_factor_employee, jan_social_factor_employee, props_dec_value_to_string(test_social_prop.factor_employee())));
            vect_social_factor_employee_garant.push((test_year, mes_social_factor_employee_garant, jan_social_factor_employee_garant, props_dec_value_to_string(test_social_prop.factor_employee_garant())));
            vect_social_factor_employee_reduce.push((test_year, mes_social_factor_employee_reduce, jan_social_factor_employee_reduce, props_dec_value_to_string(test_social_prop.factor_employee_reduce())));
            vect_social_margin_income_emp.push((test_year, mes_social_margin_income_emp, jan_social_margin_income_emp, props_int_value_to_string(test_social_prop.margin_income_emp())));
            vect_social_margin_income_agr.push((test_year, mes_social_margin_income_agr, jan_social_margin_income_agr, props_int_value_to_string(test_social_prop.margin_income_agr())));
            vect_taxing_allowance_payer.push((test_year, mes_taxing_allowance_payer, jan_taxing_allowance_payer, props_int_value_to_string(test_taxing_prop.allowance_payer())));
            vect_taxing_allowance_disab_1st.push((test_year, mes_taxing_allowance_disab_1st, jan_taxing_allowance_disab_1st, props_int_value_to_string(test_taxing_prop.allowance_disab1st())));
            vect_taxing_allowance_disab_2nd.push((test_year, mes_taxing_allowance_disab_2nd, jan_taxing_allowance_disab_2nd, props_int_value_to_string(test_taxing_prop.allowance_disab2nd())));
            vect_taxing_allowance_disab_3rd.push((test_year, mes_taxing_allowance_disab_3rd, jan_taxing_allowance_disab_3rd, props_int_value_to_string(test_taxing_prop.allowance_disab3rd())));
            vect_taxing_allowance_study.push((test_year, mes_taxing_allowance_study, jan_taxing_allowance_study, props_int_value_to_string(test_taxing_prop.allowance_study())));
            vect_taxing_allowance_child_1st.push((test_year, mes_taxing_allowance_child_1st, jan_taxing_allowance_child_1st, props_int_value_to_string(test_taxing_prop.allowance_child1st())));
            vect_taxing_allowance_child_2nd.push((test_year, mes_taxing_allowance_child_2nd, jan_taxing_allowance_child_2nd, props_int_value_to_string(test_taxing_prop.allowance_child2nd())));
            vect_taxing_allowance_child_3rd.push((test_year, mes_taxing_allowance_child_3rd, jan_taxing_allowance_child_3rd, props_int_value_to_string(test_taxing_prop.allowance_child3rd())));
            vect_taxing_factor_advances.push((test_year, mes_taxing_factor_advances, jan_taxing_factor_advances, props_dec_value_to_string(test_taxing_prop.factor_advances())));
            vect_taxing_factor_withhold.push((test_year, mes_taxing_factor_withhold, jan_taxing_factor_withhold, props_dec_value_to_string(test_taxing_prop.factor_withhold())));
            vect_taxing_factor_solidary.push((test_year, mes_taxing_factor_solidary, jan_taxing_factor_solidary, props_dec_value_to_string(test_taxing_prop.factor_solidary())));
            vect_taxing_factor_taxrate2.push((test_year, mes_taxing_factor_taxrate2, jan_taxing_factor_taxrate2, props_dec_value_to_string(test_taxing_prop.factor_taxrate2())));
            vect_taxing_min_amount_of_taxbonus.push((test_year, mes_taxing_min_amount_of_taxbonus, jan_taxing_min_amount_of_taxbonus, props_int_value_to_string(test_taxing_prop.min_amount_of_tax_bonus())));
            vect_taxing_max_amount_of_taxbonus.push((test_year, mes_taxing_max_amount_of_taxbonus, jan_taxing_max_amount_of_taxbonus, props_int_value_to_string(test_taxing_prop.max_amount_of_tax_bonus())));
            vect_taxing_margin_income_of_taxbonus.push((test_year, mes_taxing_margin_income_of_taxbonus, jan_taxing_margin_income_of_taxbonus, props_int_value_to_string(test_taxing_prop.margin_income_of_tax_bonus())));
            vect_taxing_margin_income_of_rounding.push((test_year, mes_taxing_margin_income_of_rounding, jan_taxing_margin_income_of_rounding, props_int_value_to_string(test_taxing_prop.margin_income_of_rounding())));
            vect_taxing_margin_income_of_withhold.push((test_year, mes_taxing_margin_income_of_withhold, jan_taxing_margin_income_of_withhold, props_int_value_to_string(test_taxing_prop.margin_income_of_withhold())));
            vect_taxing_margin_income_of_solidary.push((test_year, mes_taxing_margin_income_of_solidary, jan_taxing_margin_income_of_solidary, props_int_value_to_string(test_taxing_prop.margin_income_of_solidary())));
            vect_taxing_margin_income_of_taxrate2.push((test_year, mes_taxing_margin_income_of_taxrate2, jan_taxing_margin_income_of_taxrate2, props_int_value_to_string(test_taxing_prop.margin_income_of_taxrate2())));
            vect_taxing_margin_income_of_wht_emp.push((test_year, mes_taxing_margin_income_of_wht_emp, jan_taxing_margin_income_of_wht_emp, props_int_value_to_string(test_taxing_prop.margin_income_of_wth_emp())));
            vect_taxing_margin_income_of_wht_agr.push((test_year, mes_taxing_margin_income_of_wht_agr, jan_taxing_margin_income_of_wht_agr, props_int_value_to_string(test_taxing_prop.margin_income_of_wth_agr())));
        }
        let table_data: Vec<(i16, Vec<(i16, i16, String, String)>)> = vec![
            (HEALTH_MIN_MONTHLY_BASIS, vect_health_min_monthly_basis),
            (HEALTH_MAX_ANNUALS_BASIS, vect_health_max_annuals_basis),
            (HEALTH_LIM_MONTHLY_STATE, vect_health_lim_monthly_state),
            (HEALTH_LIM_MONTHLY_DIS50, vect_health_lim_monthly_dis50),
            (HEALTH_FACTOR_COMPOUND, vect_health_factor_compound),
            (HEALTH_FACTOR_EMPLOYEE, vect_health_factor_employee),
            (HEALTH_MARGIN_INCOME_EMP, vect_health_margin_income_emp),
            (HEALTH_MARGIN_INCOME_AGR, vect_health_margin_income_agr),
            (SALARY_WORKING_SHIFT_WEEK, vect_salary_working_shift_week),
            (SALARY_WORKING_SHIFT_TIME, vect_salary_working_shift_time),
            (SALARY_MIN_MONTHLY_WAGE, vect_salary_min_monthly_wage),
            (SALARY_MIN_HOURLY_WAGE, vect_salary_min_hourly_wage),
            (SOCIAL_MAX_ANNUALS_BASIS, vect_social_max_annuals_basis),
            (SOCIAL_FACTOR_EMPLOYER, vect_social_factor_employer),
            (SOCIAL_FACTOR_EMPLOYER_HIGHER, vect_social_factor_employer_higher),
            (SOCIAL_FACTOR_EMPLOYEE, vect_social_factor_employee),
            (SOCIAL_FACTOR_EMPLOYEE_GARANT, vect_social_factor_employee_garant),
            (SOCIAL_FACTOR_EMPLOYEE_REDUCE, vect_social_factor_employee_reduce),
            (SOCIAL_MARGIN_INCOME_EMP, vect_social_margin_income_emp),
            (SOCIAL_MARGIN_INCOME_AGR, vect_social_margin_income_agr),
            (TAXING_ALLOWANCE_PAYER, vect_taxing_allowance_payer),
            (TAXING_ALLOWANCE_DISAB_1ST, vect_taxing_allowance_disab_1st),
            (TAXING_ALLOWANCE_DISAB_2ND, vect_taxing_allowance_disab_2nd),
            (TAXING_ALLOWANCE_DISAB_3RD, vect_taxing_allowance_disab_3rd),
            (TAXING_ALLOWANCE_STUDY, vect_taxing_allowance_study),
            (TAXING_ALLOWANCE_CHILD_1ST, vect_taxing_allowance_child_1st),
            (TAXING_ALLOWANCE_CHILD_2ND, vect_taxing_allowance_child_2nd),
            (TAXING_ALLOWANCE_CHILD_3RD, vect_taxing_allowance_child_3rd),
            (TAXING_FACTOR_ADVANCES, vect_taxing_factor_advances),
            (TAXING_FACTOR_WITHHOLD, vect_taxing_factor_withhold),
            (TAXING_FACTOR_SOLIDARY, vect_taxing_factor_solidary),
            (TAXING_FACTOR_TAXRATE2, vect_taxing_factor_taxrate2),
            (TAXING_MIN_AMOUNT_OF_TAXBONUS, vect_taxing_min_amount_of_taxbonus),
            (TAXING_MAX_AMOUNT_OF_TAXBONUS, vect_taxing_max_amount_of_taxbonus),
            (TAXING_MARGIN_INCOME_OF_TAXBONUS, vect_taxing_margin_income_of_taxbonus),
            (TAXING_MARGIN_INCOME_OF_ROUNDING, vect_taxing_margin_income_of_rounding),
            (TAXING_MARGIN_INCOME_OF_WITHHOLD, vect_taxing_margin_income_of_withhold),
            (TAXING_MARGIN_INCOME_OF_SOLIDARY, vect_taxing_margin_income_of_solidary),
            (TAXING_MARGIN_INCOME_OF_TAXRATE2, vect_taxing_margin_income_of_taxrate2),
            (TAXING_MARGIN_INCOME_OF_WHT_EMP, vect_taxing_margin_income_of_wht_emp),
            (TAXING_MARGIN_INCOME_OF_WHT_AGR, vect_taxing_margin_income_of_wht_agr),
        ];
        for data in table_data {
            export_history_term(&mut file, &header_data, &data);
        }
        close_history_file(&mut file);
        Ok(())
    }
}
