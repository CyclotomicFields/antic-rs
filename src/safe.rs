use crate::*;
use std::mem::MaybeUninit;

pub struct IntegerPolynomial {
    raw: MaybeUninit<fmpz_poly_struct>,
}

impl IntegerPolynomial {
    pub fn new() -> Self {
        let mut raw = unsafe { MaybeUninit::uninit() };

        unsafe {
            fmpz_poly_init(raw.as_mut_ptr());
        }

        IntegerPolynomial { raw: raw }
    }
}

impl Drop for IntegerPolynomial {
    fn drop(&mut self) {
        unsafe {
            fmpz_poly_clear(self.raw.as_mut_ptr());
        }
    }
}

pub struct RationalPolynomial {
    raw: MaybeUninit<fmpq_poly_struct>,
}

impl RationalPolynomial {
    pub fn new() -> Self {
        let mut raw = MaybeUninit::uninit();

        unsafe {
            fmpq_poly_init(raw.as_mut_ptr());
        }

        RationalPolynomial { raw: raw }
    }

    pub fn cyclotomic(n: u64) -> Self {
        let mut zpoly = IntegerPolynomial::new();
        let mut qpoly = Self::new();

        unsafe {
            fmpz_poly_cyclotomic(zpoly.raw.as_mut_ptr(), n);
            fmpq_poly_set_fmpz_poly(qpoly.raw.as_mut_ptr(), zpoly.raw.as_mut_ptr());
        }

        qpoly
    }

    /// sets the coefficient of x^exponent to be coeff
    pub fn set_coeff(&mut self, exponent: i64, coeff: &mut Rational) {
        unsafe {
            fmpq_poly_set_coeff_fmpq(self.raw.as_mut_ptr(), exponent, coeff.raw.as_mut_ptr());
        }
    }
}

impl Drop for RationalPolynomial {
    fn drop(&mut self) {
        unsafe {
            fmpq_poly_clear(self.raw.as_mut_ptr());
        }
    }
}

// TODO: make raw private
pub struct NumberField<'a> {
    pub raw: MaybeUninit<nf_struct>,
    polynomial: &'a mut RationalPolynomial,
}

impl<'a> NumberField<'a> {
    /// Constructs a number field F, such that F = Q[x]/(f(x))
    pub fn new(f: &'a mut RationalPolynomial) -> Self {
        let mut raw = MaybeUninit::uninit();
        unsafe {
            nf_init(raw.as_mut_ptr(), f.raw.as_mut_ptr());
        }
        NumberField {
            raw: raw,
            polynomial: f,
        }
    }
}

impl Drop for NumberField<'_> {
    fn drop(&mut self) {
        unsafe {
            nf_clear(self.raw.as_mut_ptr());
        }
    }
}

// TODO: make raw private (fully convert code that uses this)
pub struct NumberFieldElement<'a, 'b> {
    pub raw: MaybeUninit<nf_elem_struct>,
    field: &'a mut NumberField<'b>,
}

impl<'a, 'b> NumberFieldElement<'a, 'b> {
    pub fn new(field: &'a mut NumberField<'b>) -> Self {
        let mut raw = MaybeUninit::uninit();
        unsafe {
            nf_elem_init(raw.as_mut_ptr(), field.raw.as_mut_ptr());
        }
        NumberFieldElement {
            raw: raw,
            field: field,
        }
    }
    pub fn set_to_poly(&mut self, poly: &mut RationalPolynomial) {
        unsafe {
            nf_elem_set_fmpq_poly(
                self.raw.as_mut_ptr(),
                poly.raw.as_mut_ptr(),
                self.field.raw.as_mut_ptr(),
            );
        }
    }
    pub fn set(&mut self, other: &mut NumberFieldElement) {
        unsafe {
            nf_elem_set(
                self.raw.as_mut_ptr(),
                other.raw.as_mut_ptr(),
                self.field.raw.as_mut_ptr(),
            );
        }
    }
    pub fn set_to_sum_of(&mut self, a: &mut NumberFieldElement, b: &mut NumberFieldElement) {
        unsafe {
            nf_elem_add(
                self.raw.as_mut_ptr(),
                a.raw.as_mut_ptr(),
                b.raw.as_mut_ptr(),
                self.field.raw.as_mut_ptr(),
            );
        }
    }
}

impl Drop for NumberFieldElement<'_, '_> {
    fn drop(&mut self) {
        unsafe {
            nf_elem_clear(self.raw.as_mut_ptr(), self.field.raw.as_mut_ptr());
        }
    }
}

pub struct Rational {
    raw: MaybeUninit<fmpq>,
}

impl Rational {
    pub fn new(numerator: i64, denominator: u64) -> Self {
        let mut raw = MaybeUninit::uninit();
        unsafe {
            fmpq_init(raw.as_mut_ptr());
            fmpq_set_si(raw.as_mut_ptr(), numerator, denominator);
        }
        Rational { raw: raw }
    }
}

impl Drop for Rational {
    fn drop(&mut self) {
        unsafe {
            fmpq_clear(self.raw.as_mut_ptr());
        }
    }
}
