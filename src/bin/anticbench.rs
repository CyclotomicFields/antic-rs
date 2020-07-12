use antic::*;
use antic::safe::*;
use rand::Rng;
use std::mem::MaybeUninit;
use std::time::Instant;

fn main() {
    let n: i64 = 100;
    let num_tests: usize = 1000;

    eprintln!("n = {}", n);
    eprintln!("num_tests = {}", num_tests);

    let num_per_test: usize = 6;
    let max_num_terms: usize = 5;

    unsafe {
        let mut cyclotomic_polynomial_n = RationalPolynomial::cyclotomic(n as u64);
        let mut cyclotomic_field_n = NumberField::new(&mut cyclotomic_polynomial_n);
        let mut nums: Vec<Vec<NumberFieldElement>> = vec![];
        let mut rng = rand::thread_rng();

        eprintln!("generating test data");
        for i in 0..num_tests {
            nums.push(vec![]);
            for j in 0..num_per_test {
                let mut num = NumberFieldElement::new(&mut cyclotomic_field_n);
                let num_terms = rng.gen_range(1, max_num_terms);
                for _ in 0..num_terms {
                    let mut term = NumberFieldElement::new(&mut cyclotomic_field_n);
                    let mut pol = RationalPolynomial::new();
                    let exp: i64 = rng.gen_range(1, n);
                    let numerator = rng.gen_range(1, 11);
                    let denominator = rng.gen_range(1, 11);
                    let mut coeff = Rational::new(numerator, denominator);
                    pol.set_coeff(exp, &mut coeff);
                    term.set_to_poly(&mut pol);
                    let mut sum = NumberFieldElement::new(&mut cyclotomic_field_n);
                    sum.set_to_sum_of(&mut num, &mut term);
                    num.set(&mut sum);
                }
                nums[i].push(num);
            }
        }

        eprintln!("starting benchmark");

        let start = Instant::now();

        for i in 0..num_tests {
            let chunk = &mut nums[i];
            let mut prod1_mem: MaybeUninit<nf_elem_struct> = MaybeUninit::uninit();
            let prod1 = prod1_mem.as_mut_ptr();
            nf_elem_init(prod1, cyclotomic_field_n.raw.as_mut_ptr());
            fmpq_poly_fit_length((*prod1).elem.as_mut_ptr(), 2 * n);
            nf_elem_mul(
                prod1,
                chunk[0].raw.as_mut_ptr(),
                chunk[1].raw.as_mut_ptr(),
                cyclotomic_field_n.raw.as_mut_ptr(),
            );

            let mut prod2_mem: MaybeUninit<nf_elem_struct> = MaybeUninit::uninit();
            let prod2 = prod2_mem.as_mut_ptr();
            nf_elem_init(prod2, cyclotomic_field_n.raw.as_mut_ptr());
            fmpq_poly_fit_length((*prod2).elem.as_mut_ptr(), 2 * n);
            nf_elem_mul(
                prod2,
                chunk[2].raw.as_mut_ptr(),
                chunk[3].raw.as_mut_ptr(),
                cyclotomic_field_n.raw.as_mut_ptr(),
            );

            let mut prod3_mem: MaybeUninit<nf_elem_struct> = MaybeUninit::uninit();
            let prod3 = prod3_mem.as_mut_ptr();
            nf_elem_init(prod3, cyclotomic_field_n.raw.as_mut_ptr());
            fmpq_poly_fit_length((*prod3).elem.as_mut_ptr(), 2 * n);
            nf_elem_mul(
                prod3,
                chunk[4].raw.as_mut_ptr(),
                chunk[5].raw.as_mut_ptr(),
                cyclotomic_field_n.raw.as_mut_ptr(),
            );

            let mut sum1_mem: MaybeUninit<nf_elem_struct> = MaybeUninit::uninit();
            let sum1 = sum1_mem.as_mut_ptr();
            nf_elem_init(sum1, cyclotomic_field_n.raw.as_mut_ptr());
            fmpq_poly_fit_length((*sum1).elem.as_mut_ptr(), 2 * n);
            nf_elem_add(sum1, prod1, prod2, cyclotomic_field_n.raw.as_mut_ptr());

            let mut sum2_mem: MaybeUninit<nf_elem_struct> = MaybeUninit::uninit();
            let sum2 = sum2_mem.as_mut_ptr();
            nf_elem_init(sum2, cyclotomic_field_n.raw.as_mut_ptr());
            fmpq_poly_fit_length((*sum2).elem.as_mut_ptr(), 2 * n);
            nf_elem_add(sum2, sum1, prod3, cyclotomic_field_n.raw.as_mut_ptr());
        }

        eprintln!("time elapsed (ms):");
        println!("{}", start.elapsed().as_millis());
    }
}
