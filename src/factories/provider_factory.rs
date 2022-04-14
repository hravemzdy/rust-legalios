use std::collections::HashMap;
use crate::props::props_health::{PropsHealth};
use crate::props::props_salary::{IPropsSalary, PropsSalary};
use crate::props::props_social::PropsSocial;
use crate::props::props_taxing::{PropsTaxing};
use crate::props::props_health_base::IPropsHealth;
use crate::props::props_social_base::IPropsSocial;
use crate::props::props_taxing_base::IPropsTaxing;
use crate::providers::period_2010::provider_health_2010::ProviderHealth2010;
use crate::providers::period_2010::provider_salary_2010::ProviderSalary2010;
use crate::providers::period_2010::provider_social_2010::ProviderSocial2010;
use crate::providers::period_2010::provider_taxing_2010::ProviderTaxing2010;
use crate::providers::period_2011::provider_health_2011::ProviderHealth2011;
use crate::providers::period_2011::provider_salary_2011::ProviderSalary2011;
use crate::providers::period_2011::provider_social_2011::ProviderSocial2011;
use crate::providers::period_2011::provider_taxing_2011::ProviderTaxing2011;
use crate::providers::period_2012::provider_health_2012::ProviderHealth2012;
use crate::providers::period_2012::provider_salary_2012::ProviderSalary2012;
use crate::providers::period_2012::provider_social_2012::ProviderSocial2012;
use crate::providers::period_2012::provider_taxing_2012::ProviderTaxing2012;
use crate::providers::period_2013::provider_health_2013::ProviderHealth2013;
use crate::providers::period_2013::provider_salary_2013::ProviderSalary2013;
use crate::providers::period_2013::provider_social_2013::ProviderSocial2013;
use crate::providers::period_2013::provider_taxing_2013::ProviderTaxing2013;
use crate::providers::period_2014::provider_health_2014::ProviderHealth2014;
use crate::providers::period_2014::provider_salary_2014::ProviderSalary2014;
use crate::providers::period_2014::provider_social_2014::ProviderSocial2014;
use crate::providers::period_2014::provider_taxing_2014::ProviderTaxing2014;
use crate::providers::period_2015::provider_health_2015::ProviderHealth2015;
use crate::providers::period_2015::provider_salary_2015::ProviderSalary2015;
use crate::providers::period_2015::provider_social_2015::ProviderSocial2015;
use crate::providers::period_2015::provider_taxing_2015::ProviderTaxing2015;
use crate::providers::period_2016::provider_health_2016::ProviderHealth2016;
use crate::providers::period_2016::provider_salary_2016::ProviderSalary2016;
use crate::providers::period_2016::provider_social_2016::ProviderSocial2016;
use crate::providers::period_2016::provider_taxing_2016::ProviderTaxing2016;
use crate::providers::period_2017::provider_health_2017::ProviderHealth2017;
use crate::providers::period_2017::provider_salary_2017::ProviderSalary2017;
use crate::providers::period_2017::provider_social_2017::ProviderSocial2017;
use crate::providers::period_2017::provider_taxing_2017::ProviderTaxing2017;
use crate::providers::period_2018::provider_health_2018::ProviderHealth2018;
use crate::providers::period_2018::provider_salary_2018::ProviderSalary2018;
use crate::providers::period_2018::provider_social_2018::ProviderSocial2018;
use crate::providers::period_2018::provider_taxing_2018::ProviderTaxing2018;
use crate::providers::period_2019::provider_health_2019::ProviderHealth2019;
use crate::providers::period_2019::provider_salary_2019::ProviderSalary2019;
use crate::providers::period_2019::provider_social_2019::ProviderSocial2019;
use crate::providers::period_2019::provider_taxing_2019::ProviderTaxing2019;
use crate::providers::period_2020::provider_health_2020::ProviderHealth2020;
use crate::providers::period_2020::provider_salary_2020::ProviderSalary2020;
use crate::providers::period_2020::provider_social_2020::ProviderSocial2020;
use crate::providers::period_2020::provider_taxing_2020::ProviderTaxing2020;
use crate::providers::period_2021::provider_health_2021::ProviderHealth2021;
use crate::providers::period_2021::provider_salary_2021::ProviderSalary2021;
use crate::providers::period_2021::provider_social_2021::ProviderSocial2021;
use crate::providers::period_2021::provider_taxing_2021::ProviderTaxing2021;
use crate::providers::period_2022::provider_health_2022::ProviderHealth2022;
use crate::providers::period_2022::provider_salary_2022::ProviderSalary2022;
use crate::providers::period_2022::provider_social_2022::ProviderSocial2022;
use crate::providers::period_2022::provider_taxing_2022::ProviderTaxing2022;
use crate::providers::props_provider::{IPropsHealthProvider, IPropsSalaryProvider, IPropsSocialProvider, IPropsTaxingProvider};
use crate::service::period::IPeriod;

pub(crate) trait IProviderSalaryFactory {
    fn get_props(&self, _period: &dyn IPeriod) -> BoxSalaryProps;
}

pub(crate) trait IProviderHealthFactory {
    fn get_props(&self, _period: &dyn IPeriod) -> BoxHealthProps;
}

pub(crate) trait IProviderSocialFactory {
    fn get_props(&self, _period: &dyn IPeriod) -> BoxSocialProps;
}

pub(crate) trait IProviderTaxingFactory {
    fn get_props(&self, _period: &dyn IPeriod) -> BoxTaxingProps;
}

type VERSION = i16;

type BoxSalaryProvider = Box<dyn IPropsSalaryProvider>;
type BoxHealthProvider = Box<dyn IPropsHealthProvider>;
type BoxSocialProvider = Box<dyn IPropsSocialProvider>;
type BoxTaxingProvider = Box<dyn IPropsTaxingProvider>;

pub(crate) type BoxSalaryProps = Box<dyn IPropsSalary>;
pub(crate) type BoxHealthProps = Box<dyn IPropsHealth>;
pub(crate) type BoxSocialProps = Box<dyn IPropsSocial>;
pub(crate) type BoxTaxingProps = Box<dyn IPropsTaxing>;

pub(crate) struct ProviderSalaryFactory {
    default_provider: BoxSalaryProvider,
    empty_period_props: PropsSalary,
    versions : HashMap<VERSION, BoxSalaryProvider>
}

pub(crate) struct ProviderHealthFactory {
    default_provider: BoxHealthProvider,
    empty_period_props: PropsHealth,
    versions : HashMap<VERSION, BoxHealthProvider>
}

pub(crate) struct ProviderSocialFactory {
    default_provider: BoxSocialProvider,
    empty_period_props: PropsSocial,
    versions : HashMap<VERSION, BoxSocialProvider>
}

pub(crate) struct ProviderTaxingFactory {
    default_provider: BoxTaxingProvider,
    empty_period_props: PropsTaxing,
    versions : HashMap<VERSION, BoxTaxingProvider>
}

#[allow(dead_code)]
pub(crate) fn salary_factory() -> ProviderSalaryFactory {
    let factories: Vec<BoxSalaryProvider> = vec![
        Box::new(ProviderSalary2010::new()),
        Box::new(ProviderSalary2011::new()),
        Box::new(ProviderSalary2012::new()),
        Box::new(ProviderSalary2013::new()),
        Box::new(ProviderSalary2014::new()),
        Box::new(ProviderSalary2015::new()),
        Box::new(ProviderSalary2016::new()),
        Box::new(ProviderSalary2017::new()),
        Box::new(ProviderSalary2018::new()),
        Box::new(ProviderSalary2019::new()),
        Box::new(ProviderSalary2020::new()),
        Box::new(ProviderSalary2021::new()),
        Box::new(ProviderSalary2022::new()),
    ];
    ProviderSalaryFactory {
        default_provider: Box::new(ProviderSalary2022::new()),
        empty_period_props: PropsSalary::empty(),
        versions: factories.into_iter().map(|x| (x.get_version().value, (x))).collect(),
    }
}

#[allow(dead_code)]
pub(crate) fn health_factory() -> ProviderHealthFactory {
    let factories: Vec<BoxHealthProvider> = vec![
        Box::new(ProviderHealth2010::new()),
        Box::new(ProviderHealth2011::new()),
        Box::new(ProviderHealth2012::new()),
        Box::new(ProviderHealth2013::new()),
        Box::new(ProviderHealth2014::new()),
        Box::new(ProviderHealth2015::new()),
        Box::new(ProviderHealth2016::new()),
        Box::new(ProviderHealth2017::new()),
        Box::new(ProviderHealth2018::new()),
        Box::new(ProviderHealth2019::new()),
        Box::new(ProviderHealth2020::new()),
        Box::new(ProviderHealth2021::new()),
        Box::new(ProviderHealth2022::new()),
    ];
    ProviderHealthFactory {
        default_provider: Box::new(ProviderHealth2022::new()),
        empty_period_props: PropsHealth::empty(),
        versions: factories.into_iter().map(|x| (x.get_version().value, x)).collect(),
    }
}

#[allow(dead_code)]
pub(crate) fn social_factory() -> ProviderSocialFactory {
    let factories: Vec<BoxSocialProvider> = vec![
        Box::new(ProviderSocial2010::new()),
        Box::new(ProviderSocial2011::new()),
        Box::new(ProviderSocial2012::new()),
        Box::new(ProviderSocial2013::new()),
        Box::new(ProviderSocial2014::new()),
        Box::new(ProviderSocial2015::new()),
        Box::new(ProviderSocial2016::new()),
        Box::new(ProviderSocial2017::new()),
        Box::new(ProviderSocial2018::new()),
        Box::new(ProviderSocial2019::new()),
        Box::new(ProviderSocial2020::new()),
        Box::new(ProviderSocial2021::new()),
        Box::new(ProviderSocial2022::new()),
    ];
    ProviderSocialFactory {
        default_provider: Box::new(ProviderSocial2022::new()),
        empty_period_props: PropsSocial::empty(),
        versions: factories.into_iter().map(|x| (x.get_version().value, x)).collect(),
    }
}

#[allow(dead_code)]
pub(crate) fn taxing_factory() -> ProviderTaxingFactory {
    let factories: Vec<BoxTaxingProvider> = vec![
        Box::new(ProviderTaxing2010::new()),
        Box::new(ProviderTaxing2011::new()),
        Box::new(ProviderTaxing2012::new()),
        Box::new(ProviderTaxing2013::new()),
        Box::new(ProviderTaxing2014::new()),
        Box::new(ProviderTaxing2015::new()),
        Box::new(ProviderTaxing2016::new()),
        Box::new(ProviderTaxing2017::new()),
        Box::new(ProviderTaxing2018::new()),
        Box::new(ProviderTaxing2019::new()),
        Box::new(ProviderTaxing2020::new()),
        Box::new(ProviderTaxing2021::new()),
        Box::new(ProviderTaxing2022::new()),
    ];
    ProviderTaxingFactory {
        default_provider: Box::new(ProviderTaxing2022::new()),
        empty_period_props: PropsTaxing::empty(),
        versions: factories.into_iter().map(|x| (x.get_version().value, x)).collect(),
    }
}

impl ProviderSalaryFactory {
    fn get_provider<'a>(&'a self, _period: &dyn IPeriod, def_provider: &'a BoxSalaryProvider) -> Option<&'a BoxSalaryProvider> {
        let period_version: VERSION = _period.year();
        let map_provider = self.versions.get(&period_version);
        let per_provider = match map_provider {
            Some(provider) => {
                Some(provider)
            },
            None => {
                if _period.year() > def_provider.get_version().value {
                    return Some(def_provider);
                }
                None
            },
        };
        match per_provider {
            Some(provider) => {
                if _period.year() != provider.get_version().value {
                    return None;
                }
                Some(provider)
            },
            None => None,
        }
    }
}

impl IProviderSalaryFactory for ProviderSalaryFactory {
    fn get_props(&self, _period: &dyn IPeriod) -> BoxSalaryProps {
        let option_provider = self.get_provider(_period, &self.default_provider);
        match option_provider {
            Some(provider) => provider.get_props(_period),
            None => Box::new(self.empty_period_props),
        }
    }
}

impl ProviderHealthFactory {
    fn get_provider<'a>(&'a self, _period: &dyn IPeriod, def_provider: &'a BoxHealthProvider) -> Option<&'a BoxHealthProvider> {
        let period_version: VERSION = _period.year();
        let map_provider = self.versions.get(&period_version);
        let per_provider = match map_provider {
            Some(provider) => {
                Some(provider)
            },
            None => {
                if _period.year() > def_provider.get_version().value {
                    return Some(def_provider);
                }
                None
            },
        };
        match per_provider {
            Some(provider) => {
                if _period.year() != provider.get_version().value {
                    return None;
                }
                Some(provider)
            },
            None => None,
        }
    }
}

impl IProviderHealthFactory for ProviderHealthFactory {
    fn get_props(&self, _period: &dyn IPeriod) -> BoxHealthProps {
        let option_provider = self.get_provider(_period, &self.default_provider);
        match option_provider {
            Some(provider) => provider.get_props(_period),
            None => Box::new(self.empty_period_props),
        }
    }
}

impl ProviderSocialFactory {
    fn get_provider<'a>(&'a self, _period: &dyn IPeriod, def_provider: &'a BoxSocialProvider) -> Option<&'a BoxSocialProvider> {
        let period_version: VERSION = _period.year();
        let map_provider = self.versions.get(&period_version);
        let per_provider = match map_provider {
            Some(provider) => {
                Some(provider)
            },
            None => {
                if _period.year() > def_provider.get_version().value {
                    return Some(def_provider);
                }
                None
            },
        };
        match per_provider {
            Some(provider) => {
                if _period.year() != provider.get_version().value {
                    return None;
                }
                Some(provider)
            },
            None => None,
        }
    }
}

impl IProviderSocialFactory for ProviderSocialFactory {
    fn get_props(&self, _period: &dyn IPeriod) -> BoxSocialProps {
        let option_provider = self.get_provider(_period, &self.default_provider);
        match option_provider {
            Some(provider) => provider.get_props(_period),
            None => Box::new(self.empty_period_props),
        }
    }
}

impl ProviderTaxingFactory {
    fn get_provider<'a>(&'a self, _period: &dyn IPeriod, def_provider: &'a BoxTaxingProvider) -> Option<&'a BoxTaxingProvider> {
        let period_version: VERSION = _period.year();
        let map_provider = self.versions.get(&period_version);
        let per_provider = match map_provider {
            Some(provider) => {
                Some(provider)
            },
            None => {
                if _period.year() > def_provider.get_version().value {
                    return Some(def_provider);
                }
                None
            },
        };
        match per_provider {
            Some(provider) => {
                if _period.year() != provider.get_version().value {
                    return None;
                }
                Some(provider)
            },
            None => None,
        }
    }
}

impl IProviderTaxingFactory for ProviderTaxingFactory {
    fn get_props(&self, _period: &dyn IPeriod) -> BoxTaxingProps {
        let option_provider = self.get_provider(_period, &self.default_provider);
        match option_provider {
            Some(provider) => provider.get_props(_period),
            None => Box::new(self.empty_period_props),
        }
    }
}

