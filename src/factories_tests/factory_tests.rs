#[cfg(test)]
mod factories_tests {
    #[macro_export]
    macro_rules! test_factory_salary {
        ($suite:ident, $($name:ident: $year: expr, $month: expr, $expected: expr,)*) => {
            mod $suite {
                use std::error::Error;

                use crate::factories::provider_factory::{IProviderSalaryFactory, salary_factory};
                use crate::service::period;

                $(
                    #[test]
                    fn $name() -> Result<(), Box<dyn Error>> {
                        let test_period = period::Period::get_with_year_month($year, $month);
                        let test_factory = salary_factory();
                        let props = test_factory.get_props(&test_period);

                        assert_eq!($expected, props.get_version().value);
                        Ok(())
                    }
                )*
            }
        }
    }
    #[macro_export]
    macro_rules! test_factory_health {
        ($suite:ident, $($name:ident: $year: expr, $month: expr, $expected: expr,)*) => {
            mod $suite {
                use std::error::Error;

                use crate::factories::provider_factory::{IProviderHealthFactory, health_factory};
                use crate::service::period;

                $(
                    #[test]
                    fn $name() -> Result<(), Box<dyn Error>> {
                        let test_period = period::Period::get_with_year_month($year, $month);
                        let test_factory = health_factory();
                        let props = test_factory.get_props(&test_period);

                        assert_eq!($expected, props.get_version().value);
                        Ok(())
                    }
                )*
            }
        }
    }
    #[macro_export]
    macro_rules! test_factory_social {
        ($suite:ident, $($name:ident: $year: expr, $month: expr, $expected: expr,)*) => {
            mod $suite {
                use std::error::Error;

                use crate::factories::provider_factory::{IProviderSocialFactory, social_factory};
                use crate::service::period;

                $(
                    #[test]
                    fn $name() -> Result<(), Box<dyn Error>> {
                        let test_period = period::Period::get_with_year_month($year, $month);
                        let test_factory = social_factory();
                        let props = test_factory.get_props(&test_period);

                        assert_eq!($expected, props.get_version().value);
                        Ok(())
                    }
                )*
            }
        }
    }
    #[macro_export]
    macro_rules! test_factory_taxing {
        ($suite:ident, $($name:ident: $year: expr, $month: expr, $expected: expr,)*) => {
            mod $suite {
                use std::error::Error;

                use crate::factories::provider_factory::{IProviderTaxingFactory, taxing_factory};
                use crate::service::period;

                $(
                    #[test]
                    fn $name() -> Result<(), Box<dyn Error>> {
                        let test_period = period::Period::get_with_year_month($year, $month);
                        let test_factory = taxing_factory();
                        let props = test_factory.get_props(&test_period);

                        assert_eq!($expected, props.get_version().value);
                        Ok(())
                    }
                )*
            }
        }
    }
}