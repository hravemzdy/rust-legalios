#[cfg(test)]
pub mod service_examples_tests {
    #[macro_export]
    macro_rules! test_examples_log_int_salary {
        ($suite:ident, $name_str:expr, $test:expr, $($test_name:ident: $test_year:expr, [$($name:ident: $year: expr, $month: expr, $exp_year: expr, $exp_month: expr, $expected: expr,)*],)*) => {
            mod $suite {
                use std::error::Error;

                use crate::service_tests::service_report_examples_tests::service_examples_tests::{close_report_file, create_expected_file, write_report_head, write_report_year_ends, write_report_year_head, write_report_year_int_value};
                #[test]
                fn log_expected() -> Result<(), Box<dyn Error>> {
                    let mut file = create_expected_file(&($name_str.to_owned() + ".txt"));
                    write_report_head(&mut file);
                    $(
                        write_report_year_head(&mut file, $test_year);
                        $(
                            write_report_year_int_value(&mut file, $expected);
                        )*
                        write_report_year_ends(&mut file);
                    )*
                    close_report_file(&mut file);
                    Ok(())
                }
                $(
                    mod $test_name {
                        use std::error::Error;

                        use crate::service::period;
                        use crate::service::service_legalios as legalios;
                        use crate::service::service_legalios::IServiceLegalios;
                        use crate::service::bundle_props::{IBundleProps};
                        use crate::factories::provider_factory::{BoxSalaryProps};
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
                                let result_value: i32 = match result_bundle {
                                    Some(bundle) => {
                                        let bundle_into: &dyn IBundleProps = bundle;
                                        $test(&bundle_into.salary_props())
                                    },
                                    None => 0,
                                };
                                let result_year: i16 = match result_bundle {
                                    Some(bundle) => bundle.get_period_year(),
                                    None => 0,
                                };
                                let result_month: i16 = match result_bundle {
                                    Some(bundle) => bundle.get_period_month(),
                                    None => 0,
                                };
                                assert_eq!(result_error.is_none(), true);
                                assert_eq!(result_bundle.is_some(), true);
                                assert_eq!($exp_year, result_year);
                                assert_eq!($exp_month, result_month);
                                assert_eq!($expected, result_value);
                                Ok(())
                            }
                        )*
                    }
                )*
            }
        }
    }
    #[macro_export]
    macro_rules! test_examples_log_int_health {
        ($suite:ident, $name_str:expr, $test:expr, $($test_name:ident: $test_year:expr, [$($name:ident: $year: expr, $month: expr, $exp_year: expr, $exp_month: expr, $expected: expr,)*],)*) => {
            mod $suite {
                use std::error::Error;

                use crate::service_tests::service_report_examples_tests::service_examples_tests::{close_report_file, create_expected_file, write_report_head, write_report_year_ends, write_report_year_head, write_report_year_int_value};
                #[test]
                fn log_expected() -> Result<(), Box<dyn Error>> {
                    let mut file = create_expected_file(&($name_str.to_owned() + ".txt"));
                    write_report_head(&mut file);
                    $(
                        write_report_year_head(&mut file, $test_year);
                        $(
                            write_report_year_int_value(&mut file, $expected);
                        )*
                        write_report_year_ends(&mut file);
                    )*
                    close_report_file(&mut file);
                    Ok(())
                }
                $(
                    mod $test_name {
                        use std::error::Error;

                        use crate::service::period;
                        use crate::service::service_legalios as legalios;
                        use crate::service::service_legalios::IServiceLegalios;
                        use crate::service::bundle_props::{IBundleProps};
                        use crate::factories::provider_factory::{BoxHealthProps};
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
                                let result_value: i32 = match result_bundle {
                                    Some(bundle) => {
                                        let bundle_into: &dyn IBundleProps = bundle;
                                        $test(&bundle_into.health_props())
                                    },
                                    None => 0,
                                };
                                let result_year: i16 = match result_bundle {
                                    Some(bundle) => bundle.get_period_year(),
                                    None => 0,
                                };
                                let result_month: i16 = match result_bundle {
                                    Some(bundle) => bundle.get_period_month(),
                                    None => 0,
                                };
                                assert_eq!(result_error.is_none(), true);
                                assert_eq!(result_bundle.is_some(), true);
                                assert_eq!($exp_year, result_year);
                                assert_eq!($exp_month, result_month);
                                assert_eq!($expected, result_value);
                                Ok(())
                            }
                        )*
                    }
                )*
            }
        }
    }
    #[macro_export]
    macro_rules! test_examples_log_int_social {
        ($suite:ident, $name_str:expr, $test:expr, $($test_name:ident: $test_year:expr, [$($name:ident: $year: expr, $month: expr, $exp_year: expr, $exp_month: expr, $expected: expr,)*],)*) => {
            mod $suite {
                use std::error::Error;

                use crate::service_tests::service_report_examples_tests::service_examples_tests::{close_report_file, create_expected_file, write_report_head, write_report_year_ends, write_report_year_head, write_report_year_int_value};
                #[test]
                fn log_expected() -> Result<(), Box<dyn Error>> {
                    let mut file = create_expected_file(&($name_str.to_owned() + ".txt"));
                    write_report_head(&mut file);
                    $(
                        write_report_year_head(&mut file, $test_year);
                        $(
                            write_report_year_int_value(&mut file, $expected);
                        )*
                        write_report_year_ends(&mut file);
                    )*
                    close_report_file(&mut file);
                    Ok(())
                }
                $(
                   mod $test_name {
                        use std::error::Error;

                        use crate::service::period;
                        use crate::service::service_legalios as legalios;
                        use crate::service::service_legalios::IServiceLegalios;
                        use crate::service::bundle_props::{IBundleProps};
                        use crate::factories::provider_factory::{BoxSocialProps};
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
                                let result_value: i32 = match result_bundle {
                                    Some(bundle) => {
                                        let bundle_into: &dyn IBundleProps = bundle;
                                        $test(&bundle_into.social_props())
                                    },
                                    None => 0,
                                };
                                let result_year: i16 = match result_bundle {
                                    Some(bundle) => bundle.get_period_year(),
                                    None => 0,
                                };
                                let result_month: i16 = match result_bundle {
                                    Some(bundle) => bundle.get_period_month(),
                                    None => 0,
                                };
                                assert_eq!(result_error.is_none(), true);
                                assert_eq!(result_bundle.is_some(), true);
                                assert_eq!($exp_year, result_year);
                                assert_eq!($exp_month, result_month);
                                assert_eq!($expected, result_value);
                                Ok(())
                            }
                        )*
                    }
                )*
            }
        }
    }
    #[macro_export]
    macro_rules! test_examples_log_int_taxing {
        ($suite:ident, $name_str:expr, $test:expr, $($test_name:ident: $test_year:expr, [$($name:ident: $year: expr, $month: expr, $exp_year: expr, $exp_month: expr, $expected: expr,)*],)*) => {
            mod $suite {
                use std::error::Error;

                use crate::service_tests::service_report_examples_tests::service_examples_tests::{close_report_file, create_expected_file, write_report_head, write_report_year_ends, write_report_year_head, write_report_year_int_value};
                #[test]
                fn log_expected() -> Result<(), Box<dyn Error>> {
                    let mut file = create_expected_file(&($name_str.to_owned() + ".txt"));
                    write_report_head(&mut file);
                    $(
                        write_report_year_head(&mut file, $test_year);
                        $(
                            write_report_year_int_value(&mut file, $expected);
                        )*
                        write_report_year_ends(&mut file);
                    )*
                    close_report_file(&mut file);
                    Ok(())
                }
                $(
                    mod $test_name {
                        use std::error::Error;

                        use crate::service::period;
                        use crate::service::service_legalios as legalios;
                        use crate::service::service_legalios::IServiceLegalios;
                        use crate::service::bundle_props::{IBundleProps};
                        use crate::factories::provider_factory::{BoxTaxingProps};
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
                                let result_value: i32 = match result_bundle {
                                    Some(bundle) => {
                                        let bundle_into: &dyn IBundleProps = bundle;
                                        $test(&bundle_into.taxing_props())
                                    },
                                    None => 0,
                                };
                                let result_year: i16 = match result_bundle {
                                    Some(bundle) => bundle.get_period_year(),
                                    None => 0,
                                };
                                let result_month: i16 = match result_bundle {
                                    Some(bundle) => bundle.get_period_month(),
                                    None => 0,
                                };
                                assert_eq!(result_error.is_none(), true);
                                assert_eq!(result_bundle.is_some(), true);
                                assert_eq!($exp_year, result_year);
                                assert_eq!($exp_month, result_month);
                                assert_eq!($expected, result_value);
                                Ok(())
                            }
                        )*
                    }
                )*
            }
        }
    }
}