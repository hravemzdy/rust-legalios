use std::ops::{Add, Div, Mul, Sub};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

#[allow(dead_code)]
pub(crate) fn multiply(op1: Decimal, op2: Decimal) -> Decimal {
    return op1.mul(op2);
}

#[allow(dead_code)]
pub(crate) fn divide(op1: Decimal, div: Decimal) -> Decimal {
    if div == Decimal::ZERO {
        return Decimal::ZERO;
    }
    return op1.div(div);
}

#[allow(dead_code)]
pub(crate) fn multiply_and_divide(op1: Decimal, op2: Decimal, div: Decimal) -> Decimal {
    if div == Decimal::ZERO {
        return Decimal::ZERO;
    }
    let multi_ret = op1.mul(op2);
    let divid_ret = multi_ret.div(div);

    return divid_ret;
}

#[allow(dead_code)]
pub(crate) fn decimal_cast(number: i32) -> Decimal {
    return Decimal::from_i32(number).unwrap();
}

#[allow(dead_code)]
pub(crate) fn min_inc_max_dec_value(value_to_min_max: Decimal, acc_value_to_max: Decimal, min_limit_to: Decimal, max_limit_to: Decimal) -> Decimal {
    let min_base = min_inc_value(value_to_min_max, min_limit_to);

    let max_base = max_dec_accum_value(min_base, acc_value_to_max, max_limit_to);

    return max_base;
}

#[allow(dead_code)]
pub(crate) fn max_dec_accum_value(value_to_max: Decimal, accum_to_max: Decimal, max_limit_to: Decimal) -> Decimal {
    if max_limit_to > Decimal::ZERO {
        let value_to_reduce = value_to_max.add(accum_to_max).max(max_limit_to);

        return Decimal::ZERO.max(value_to_reduce.sub(accum_to_max));
    }
    return value_to_max;
}

#[allow(dead_code)]
pub(crate) fn max_dec_accum_above(value_to_max: Decimal, accum_to_max: Decimal, max_limit_to: Decimal) -> Decimal {
    if max_limit_to > Decimal::ZERO {
        let under_to_limits = max_dec_accum_value(value_to_max, accum_to_max, max_limit_to);

        return Decimal::ZERO.max(value_to_max.sub(under_to_limits));
    }
    return Decimal::ZERO;
}

#[allow(dead_code)]
pub(crate) fn min_inc_value(value_to_min: Decimal, min_limit_to: Decimal) -> Decimal {
    if min_limit_to > Decimal::ZERO {
        if min_limit_to > value_to_min {
            return min_limit_to;
        }
    }
    return value_to_min;
}

#[allow(dead_code)]
pub(crate) fn max_dec_value(value_to_max: Decimal, max_limit_to: Decimal) -> Decimal {
    if max_limit_to > Decimal::ZERO {
        return value_to_max.min(max_limit_to);
    }
    return value_to_max;
}

#[allow(dead_code)]
pub(crate) fn suppress_negative(suppress: bool, value_dec: Decimal) -> Decimal {
    if suppress && value_dec < Decimal::ZERO {
        return Decimal::ZERO;
    }
    return value_dec;
}
