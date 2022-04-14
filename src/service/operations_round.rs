use std::ops::{Add, Neg};
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, FromStr, ToPrimitive};
use crate::service::operations_dec;


#[allow(dead_code)]
pub(crate) fn round_to_int(value_dec: Decimal) -> i32 {
    let int_rounding_const: Decimal = Decimal::from_str("0.5").unwrap();
    let round_ret = (value_dec.abs().add(int_rounding_const)).floor();

    if value_dec < Decimal::ZERO {
        return round_ret.neg().to_i32().unwrap_or(0i32);
    }
    return round_ret.to_i32().unwrap_or(0i32);
}

#[allow(dead_code)]
pub(crate) fn round_up(value_dec: Decimal) -> i32 {
    let int_rounding_const: Decimal = Decimal::from_str("0.5").unwrap();
    let round_ret = (value_dec.abs().add(int_rounding_const)).ceil();

    if value_dec < Decimal::ZERO {
        return round_ret.neg().to_i32().unwrap_or(0i32);
    }
    return round_ret.to_i32().unwrap_or(0i32);
}

#[allow(dead_code)]
pub(crate) fn round_down(value_dec: Decimal) -> i32 {
    let round_ret = value_dec.abs().floor();

    if value_dec < Decimal::ZERO {
        return round_ret.neg().to_i32().unwrap_or(0i32);
    }
    return round_ret.to_i32().unwrap_or(0i32);
}

#[allow(dead_code)]
pub(crate) fn round_norm(value_dec: Decimal) -> i32 {
    let int_rounding_const: Decimal = Decimal::from_str("0.5").unwrap();
    let round_ret = (value_dec.abs().add(int_rounding_const)).floor();

    if value_dec < Decimal::ZERO {
        return round_ret.neg().to_i32().unwrap_or(0i32);
    }
    return round_ret.to_i32().unwrap_or(0i32);
}

#[allow(dead_code)]
pub(crate) fn near_round_up(value_dec: Decimal, nearest: i32) -> i32 {
    let nearest_big = Decimal::from_i32(nearest).unwrap();
    let divid_ret = operations_dec::divide(value_dec, nearest_big);

    let multi_ret = operations_dec::multiply(dec_round_up(divid_ret), nearest_big);

    return round_to_int(multi_ret);
}

#[allow(dead_code)]
pub(crate) fn near_round_up100(value_dec: Decimal) -> i32 {
    let nearest: i32 = 100i32;
    return near_round_up(value_dec, nearest);
}

#[allow(dead_code)]
pub(crate) fn near_round_down(value_dec: Decimal, nearest: i32) -> i32 {
    let nearest_big = Decimal::from_i32(nearest).unwrap();
    let divid_ret = operations_dec::divide(value_dec, nearest_big);

    let multi_ret = operations_dec::multiply(dec_round_down(divid_ret), nearest_big);

    return round_to_int(multi_ret);
}

#[allow(dead_code)]
pub(crate) fn near_round_down100(value_dec: Decimal) -> i32 {
    let nearest: i32 = 100i32;
    return near_round_down(value_dec, nearest);
}

#[allow(dead_code)]
pub(crate) fn round_up50(value_dec: Decimal) -> i32 {
    let divider = Decimal::from_i32(2).unwrap();
    let divid_ret = operations_dec::divide(dec_round_up(operations_dec::multiply(value_dec, divider)), divider);
    return round_to_int(divid_ret);
}

#[allow(dead_code)]
pub(crate) fn round_up25(value_dec: Decimal) -> i32 {
    let divider = Decimal::from_i32(4).unwrap();
    let divid_ret = operations_dec::divide(dec_round_up(operations_dec::multiply(value_dec, divider)), divider);
    return round_to_int(divid_ret)
}

#[allow(dead_code)]
pub(crate) fn dec_round_up(value_dec: Decimal) -> Decimal {
    let round_ret = value_dec.abs().ceil();

    if value_dec < Decimal::ZERO {
        return round_ret.neg();
    }
    return round_ret;
}

#[allow(dead_code)]
pub(crate) fn dec_round_down(value_dec: Decimal) -> Decimal {
    let round_ret = value_dec.abs().floor();

    if value_dec < Decimal::ZERO {
        return round_ret.neg();
    }
    return round_ret;
}

#[allow(dead_code)]
pub(crate) fn dec_round_norm(value_dec: Decimal) -> Decimal {
    let int_rounding_const: Decimal = Decimal::from_str("0.5").unwrap();
    let round_ret = (value_dec.abs().add(int_rounding_const)).floor();

    if value_dec < Decimal::ZERO {
        return round_ret.neg();
    }
    return round_ret;
}

#[allow(dead_code)]
pub(crate) fn dec_near_round_up(value_dec: Decimal, nearest: i32) -> Decimal {
    let nearest_big = Decimal::from_i32(nearest).unwrap();
    let divid_ret = operations_dec::divide(value_dec, nearest_big);

    let multi_ret = operations_dec::multiply(dec_round_up(divid_ret), nearest_big);

    return multi_ret;
}

#[allow(dead_code)]
pub(crate) fn dec_near_round_up100(value_dec: Decimal) -> Decimal {
    let nearest: i32 = 100i32;
    return dec_near_round_up(value_dec, nearest);
}

#[allow(dead_code)]
pub(crate) fn dec_near_round_down(value_dec: Decimal, nearest: i32) -> Decimal {
    let nearest_big = Decimal::from_i32(nearest).unwrap();
    let divid_ret = operations_dec::divide(value_dec, nearest_big);

    let multi_ret = operations_dec::multiply(dec_round_down(divid_ret), nearest_big);

    return multi_ret;
}

#[allow(dead_code)]
pub(crate) fn dec_near_round_down100(value_dec: Decimal) -> Decimal {
    let nearest: i32 = 100i32;
    return dec_near_round_down(value_dec, nearest);
}

#[allow(dead_code)]
pub(crate) fn dec_round_up50(value_dec: Decimal) -> Decimal {
    let divider = Decimal::from_i32(2).unwrap();
    return operations_dec::divide(dec_round_up(operations_dec::multiply(value_dec, divider)), divider);
}

#[allow(dead_code)]
pub(crate) fn dec_round_up25(value_dec: Decimal) -> Decimal {
    let divider = Decimal::from_i32(4).unwrap();
    return operations_dec::divide(dec_round_up(operations_dec::multiply(value_dec, divider)), divider);
}

#[allow(dead_code)]
pub(crate) fn dec_round_up01(value_dec: Decimal) -> Decimal {
    let divider = Decimal::from_i32(100).unwrap();
    return operations_dec::divide(dec_round_up(operations_dec::multiply(value_dec, divider)), divider);
}

#[allow(dead_code)]
pub(crate) fn dec_round_down50(value_dec: Decimal) -> Decimal {
    let divider = Decimal::from_i32(2).unwrap();
    return operations_dec::divide(dec_round_down(operations_dec::multiply(value_dec, divider)), divider);
}

#[allow(dead_code)]
pub(crate) fn dec_round_down25(value_dec: Decimal) -> Decimal {
    let divider = Decimal::from_i32(4).unwrap();
    return operations_dec::divide(dec_round_down(operations_dec::multiply(value_dec, divider)), divider);
}

#[allow(dead_code)]
pub(crate) fn dec_round_down01(value_dec: Decimal) -> Decimal {
    let divider = Decimal::from_i32(100).unwrap();
    return operations_dec::divide(dec_round_down(operations_dec::multiply(value_dec, divider)), divider);
}

#[allow(dead_code)]
pub(crate) fn dec_round_norm50(value_dec: Decimal) -> Decimal {
    let divider = Decimal::from_i32(2).unwrap();
    return operations_dec::divide(dec_round_norm(operations_dec::multiply(value_dec, divider)), divider);
}

#[allow(dead_code)]
pub(crate) fn dec_round_norm25(value_dec: Decimal) -> Decimal {
    let divider = Decimal::from_i32(4).unwrap();
    return operations_dec::divide(dec_round_norm(operations_dec::multiply(value_dec, divider)), divider);
}

#[allow(dead_code)]
pub(crate) fn dec_round_norm01(value_dec: Decimal) -> Decimal {
    let divider = Decimal::from_i32(100).unwrap();
    return operations_dec::divide(dec_round_norm(operations_dec::multiply(value_dec, divider)), divider);
}
