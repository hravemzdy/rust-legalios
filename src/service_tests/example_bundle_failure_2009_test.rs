#[cfg(test)]

pub mod service_tests {
    #[macro_export]
    macro_rules! test_examples_bundle {
        ($suite:ident, $($name:ident: $year: expr, $month: expr,)*) => {
            mod $suite {
                use std::error::Error;

                use crate::service::period;
                use crate::service::service_legalios as legalios;
                use crate::service::service_legalios::IServiceLegalios;

                $(
                    #[test]
                    fn $name() -> Result<(), Box<dyn Error>> {
                        let test_period = period::Period::get_with_year_month($year, $month);
                        let test_service = legalios::ServiceLegalios::new();
                        let test_result = test_service.get_bundle(&test_period);

                        let result_error = match &test_result {
                            Result::Err(_err) => Some(_err),
                            _ => None
                        };
                        let result_bundle = match &test_result {
                            Result::Ok(_val) => Some(_val),
                            _ => None
                        };
                        assert_eq!(result_error.is_some(), true);
                        assert_eq!(result_bundle.is_none(), true);
                        Ok(())
                    }
                )*
            }
        }
    }

    crate::test_examples_bundle!(test_examples_bundle_failure_2009,
            bundle_failure_test_period_2009_01: 2009, 1,
            bundle_failure_test_period_2009_02: 2009, 2,
            bundle_failure_test_period_2009_03: 2009, 3,
            bundle_failure_test_period_2009_04: 2009, 4,
            bundle_failure_test_period_2009_05: 2009, 5,
            bundle_failure_test_period_2009_06: 2009, 6,
            bundle_failure_test_period_2009_07: 2009, 7,
            bundle_failure_test_period_2009_08: 2009, 8,
            bundle_failure_test_period_2009_09: 2009, 9,
            bundle_failure_test_period_2009_10: 2009, 10,
            bundle_failure_test_period_2009_11: 2009, 11,
            bundle_failure_test_period_2009_12: 2009, 12,
    );
}
