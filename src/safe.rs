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

pub struct NumberField<'a> {
    raw: MaybeUninit<nf_struct>,
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

#[derive(Clone)]
pub struct NumberFieldElement {
    raw: MaybeUninit<nf_elem_struct>,
}

impl NumberFieldElement {
    pub fn new(field: &mut NumberField) -> Self {
        let mut raw = MaybeUninit::uninit();
        unsafe {
            nf_elem_init(raw.as_mut_ptr(), field.raw.as_mut_ptr());
        }
        NumberFieldElement { raw: raw }
    }
    pub fn set_to_poly(&mut self, poly: &mut RationalPolynomial, field: &mut NumberField) {
        unsafe {
            nf_elem_set_fmpq_poly(
                self.raw.as_mut_ptr(),
                poly.raw.as_mut_ptr(),
                field.raw.as_mut_ptr(),
            );
        }
    }
    pub fn set(&mut self, other: &mut NumberFieldElement, field: &mut NumberField) {
        unsafe {
            nf_elem_set(
                self.raw.as_mut_ptr(),
                other.raw.as_mut_ptr(),
                field.raw.as_mut_ptr(),
            );
        }
    }
    pub fn set_to_sum_of(
        &mut self,
        a: &mut NumberFieldElement,
        b: &mut NumberFieldElement,
        field: &mut NumberField,
    ) {
        unsafe {
            nf_elem_add(
                self.raw.as_mut_ptr(),
                a.raw.as_mut_ptr(),
                b.raw.as_mut_ptr(),
                field.raw.as_mut_ptr(),
            );
        }
    }
    pub fn set_to_mul_of(
        &mut self,
        a: &mut NumberFieldElement,
        b: &mut NumberFieldElement,
        field: &mut NumberField,
    ) {
        unsafe {
            fmpq_poly_fit_length(
                (*self.raw.as_mut_ptr()).elem.as_mut_ptr(),
                fmpq_poly_degree((*a.raw.as_mut_ptr()).elem.as_mut_ptr())
                    + fmpq_poly_degree((*b.raw.as_mut_ptr()).elem.as_mut_ptr()) + 1,
            );
            nf_elem_mul(
                self.raw.as_mut_ptr(),
                a.raw.as_mut_ptr(),
                b.raw.as_mut_ptr(),
                field.raw.as_mut_ptr(),
            );
        }
    }
}

impl Drop for NumberFieldElement {
    fn drop(&mut self) {
        unsafe {
            // This is what we want, but it's not possible without aliasing
            // field, I think. TODO: work out how to do this
            //nf_elem_clear(self.raw.as_mut_ptr(), self.field.raw.as_mut_ptr());
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
