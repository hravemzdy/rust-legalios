#[derive(Debug, Copy, Clone)]
#[derive(PartialEq)]
pub enum TaxDeclSignOption {
    DeclTaxNoSigned = 0,
    DeclTaxDoSigned = 1,
}

#[derive(Debug, Copy, Clone)]
#[derive(PartialEq)]
pub enum TaxNoneSignOption {
    NosignTaxWithhold = 0,
    NosignTaxAdvances = 1,
}

#[derive(Debug, Copy, Clone)]
#[derive(PartialEq)]
pub enum TaxDeclBenfOption {
    DeclTaxBenef0 = 0,
    DeclTaxBenef1 = 1,
}

#[derive(Debug, Copy, Clone)]
#[derive(PartialEq)]
pub enum TaxDeclDisabOption {
    DeclTaxBenef0 = 0,
    DeclTaxDisab1 = 1,
    DeclTaxDisab2 = 2,
    DeclTaxDisab3 = 3,
}
