#[cfg(test)]
pub mod service_examples_tests {
    #[macro_export]
    macro_rules! report_test_examples_dec_salary {
        ($suite:ident, $name:ident, $name_str:expr, $test: expr, $year_from: expr, $year_ends: expr) => {
            mod $suite {
                use std::error::Error;

                use rust_decimal::Decimal;
                use crate::service::period;
                use crate::service::service_legalios as legalios;
                use crate::service::service_legalios::IServiceLegalios;
                use crate::service::bundle_props::{IBundleProps};
                use crate::factories::provider_factory::{BoxSalaryProps};

                use crate::service_tests::service_report_examples_tests::service_examples_tests::{close_report_file, create_report_file, write_report_head, write_report_year_ends, write_report_year_head, write_report_year_dec_value};
                #[test]
                fn $name() -> Result<(), Box<dyn Error>> {
                    let mut file = create_report_file(&($name_str.to_owned() + ".txt"));
                    write_report_head(&mut file);

                    for exp_year in $year_from..=$year_ends {
                        write_report_year_head(&mut file, exp_year);
                        for exp_month in 1i16..=12i16 {
                            let test_period = period::Period::get_with_year_month(exp_year, exp_month);
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
                            let result_value: Decimal = match result_bundle {
                                Some(bundle) => {
                                    let bundle_into: &dyn IBundleProps = bundle;
                                    $test(&bundle_into.salary_props())
                                },
                                None => Decimal::ZERO,
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
                            assert_eq!(exp_year, result_year);
                            assert_eq!(exp_month, result_month);
                            write_report_year_dec_value(&mut file, result_value);
                        }
                        write_report_year_ends(&mut file);
                    }
                    close_report_file(&mut file);
                    Ok(())
                }
            }
        }
    }
    #[macro_export]
    macro_rules! report_test_examples_dec_health {
        ($suite:ident, $name:ident, $name_str:expr, $test: expr, $year_from: expr, $year_ends: expr) => {
            mod $suite {
                use std::error::Error;

                use rust_decimal::Decimal;
                use crate::service::period;
                use crate::service::service_legalios as legalios;
                use crate::service::service_legalios::IServiceLegalios;
                use crate::service::bundle_props::{IBundleProps};
                use crate::factories::provider_factory::{BoxHealthProps};

                use crate::service_tests::service_report_examples_tests::service_examples_tests::{close_report_file, create_report_file, write_report_head, write_report_year_ends, write_report_year_head, write_report_year_dec_value};
                #[test]
                fn $name() -> Result<(), Box<dyn Error>> {
                    let mut file = create_report_file(&($name_str.to_owned() + ".txt"));
                    write_report_head(&mut file);

                    for exp_year in $year_from..=$year_ends {
                        write_report_year_head(&mut file, exp_year);
                        for exp_month in 1i16..=12i16 {
                            let test_period = period::Period::get_with_year_month(exp_year, exp_month);
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
                            let result_value: Decimal = match result_bundle {
                                Some(bundle) => {
                                    let bundle_into: &dyn IBundleProps = bundle;
                                    $test(&bundle_into.health_props())
                                },
                                None => Decimal::ZERO,
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
                            assert_eq!(exp_year, result_year);
                            assert_eq!(exp_month, result_month);
                            write_report_year_dec_value(&mut file, result_value);
                        }
                        write_report_year_ends(&mut file);
                    }
                    close_report_file(&mut file);
                    Ok(())
                }
            }
        }
    }
    #[macro_export]
    macro_rules! report_test_examples_dec_social {
        ($suite:ident, $name:ident, $name_str:expr, $test: expr, $year_from: expr, $year_ends: expr) => {
            mod $suite {
                use std::error::Error;

                use rust_decimal::Decimal;
                use crate::service::period;
                use crate::service::service_legalios as legalios;
                use crate::service::service_legalios::IServiceLegalios;
                use crate::service::bundle_props::{IBundleProps};
                use crate::factories::provider_factory::{BoxSocialProps};

                use crate::service_tests::service_report_examples_tests::service_examples_tests::{close_report_file, create_report_file, write_report_head, write_report_year_ends, write_report_year_head, write_report_year_dec_value};
                #[test]
                fn $name() -> Result<(), Box<dyn Error>> {
                    let mut file = create_report_file(&($name_str.to_owned() + ".txt"));
                    write_report_head(&mut file);

                    for exp_year in $year_from..=$year_ends {
                        write_report_year_head(&mut file, exp_year);
                        for exp_month in 1i16..=12i16 {
                            let test_period = period::Period::get_with_year_month(exp_year, exp_month);
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
                            let result_value: Decimal = match result_bundle {
                                Some(bundle) => {
                                    let bundle_into: &dyn IBundleProps = bundle;
                                    $test(&bundle_into.social_props())
                                },
                                None => Decimal::ZERO,
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
                            assert_eq!(exp_year, result_year);
                            assert_eq!(exp_month, result_month);
                            write_report_year_dec_value(&mut file, result_value);
                        }
                        write_report_year_ends(&mut file);
                    }
                    close_report_file(&mut file);
                    Ok(())
                }
            }
        }
    }
    #[macro_export]
    macro_rules! report_test_examples_dec_taxing {
        ($suite:ident, $name:ident, $name_str:expr, $test: expr, $year_from: expr, $year_ends: expr) => {
            mod $suite {
                use std::error::Error;

                use rust_decimal::Decimal;
                use crate::service::period;
                use crate::service::service_legalios as legalios;
                use crate::service::service_legalios::IServiceLegalios;
                use crate::service::bundle_props::{IBundleProps};
                use crate::factories::provider_factory::{BoxTaxingProps};

                use crate::service_tests::service_report_examples_tests::service_examples_tests::{close_report_file, create_report_file, write_report_head, write_report_year_ends, write_report_year_head, write_report_year_dec_value};
                #[test]
                fn $name() -> Result<(), Box<dyn Error>> {
                    let mut file = create_report_file(&($name_str.to_owned() + ".txt"));
                    write_report_head(&mut file);

                    for exp_year in $year_from..=$year_ends {
                        write_report_year_head(&mut file, exp_year);
                        for exp_month in 1i16..=12i16 {
                            let test_period = period::Period::get_with_year_month(exp_year, exp_month);
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
                            let result_value: Decimal = match result_bundle {
                                Some(bundle) => {
                                    let bundle_into: &dyn IBundleProps = bundle;
                                    $test(&bundle_into.taxing_props())
                                },
                                None => Decimal::ZERO,
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
                            assert_eq!(exp_year, result_year);
                            assert_eq!(exp_month, result_month);
                            write_report_year_dec_value(&mut file, result_value);
                        }
                        write_report_year_ends(&mut file);
                    }
                    close_report_file(&mut file);
                    Ok(())
                }
            }
        }
    }
}