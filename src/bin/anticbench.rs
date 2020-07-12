use antic::*;
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
        let mut cyc_z_mem: MaybeUninit<fmpz_poly_struct> = MaybeUninit::uninit();
        let cyc_z = cyc_z_mem.as_mut_ptr();
        fmpz_poly_init(cyc_z);
        fmpz_poly_cyclotomic(cyc_z, n as u64);

        let mut cyc_q_mem: MaybeUninit<fmpq_poly_struct> = MaybeUninit::uninit();
        let cyclotomic_polynomial_n = cyc_q_mem.as_mut_ptr();
        fmpq_poly_init(cyclotomic_polynomial_n);
        fmpq_poly_set_fmpz_poly(cyclotomic_polynomial_n, cyc_z);

        let mut cyclotomic_field_n_mem: MaybeUninit<nf_struct> = MaybeUninit::uninit();
        let cyclotomic_field_n = cyclotomic_field_n_mem.as_mut_ptr();
        nf_init(cyclotomic_field_n, cyclotomic_polynomial_n);

        // "initialise" the uninitialised memory
        let mut nums: Vec<Vec<MaybeUninit<nf_elem_struct>>> = vec![];

        let mut rng = rand::thread_rng();

        eprintln!("generating test data");
        for i in 0..num_tests {
            nums.push(vec![]);
            for j in 0..num_per_test {
                let mut num_mem: MaybeUninit<nf_elem_struct> = MaybeUninit::uninit();
                let num: *mut nf_elem_struct = num_mem.as_mut_ptr();
                nf_elem_init(num, cyclotomic_field_n);
                let num_terms = rng.gen_range(1, max_num_terms);

                for _ in 0..num_terms {
                    let mut term_mem: MaybeUninit<nf_elem_struct> = MaybeUninit::uninit();
                    let term = term_mem.as_mut_ptr();
                    nf_elem_init(term, cyclotomic_field_n);

                    let mut pol_mem: MaybeUninit<fmpq_poly_struct> = MaybeUninit::uninit();
                    let pol = pol_mem.as_mut_ptr();
                    fmpq_poly_init(pol);

                    let exp: i64 = rng.gen_range(1, n);
                    let numerator = rng.gen_range(1, 11);
                    let denominator = rng.gen_range(1, 11);

                    let mut coeff_mem: MaybeUninit<fmpq> = MaybeUninit::uninit();
                    let coeff = coeff_mem.as_mut_ptr();
                    fmpq_init(coeff);
                    fmpq_set_si(coeff, numerator, denominator);

                    fmpq_poly_set_coeff_fmpq(pol, exp, coeff);

                    nf_elem_set_fmpq_poly(term, pol, cyclotomic_field_n);

                    let mut sum_mem: MaybeUninit<nf_elem_struct> = MaybeUninit::uninit();
                    let sum = sum_mem.as_mut_ptr();
                    nf_elem_init(sum, cyclotomic_field_n);
                    nf_elem_add(sum, num, term, cyclotomic_field_n);

                    nf_elem_set(num, sum, cyclotomic_field_n);
                }
                nums[i].push(num_mem);
            }
        }

        eprintln!("starting benchmark");

        let start = Instant::now();

        for i in 0..num_tests {
            let mut chunk = nums[i].clone();
            let mut prod1_mem: MaybeUninit<nf_elem_struct> = MaybeUninit::uninit();
            let prod1 = prod1_mem.as_mut_ptr();
            nf_elem_init(prod1, cyclotomic_field_n);
            fmpq_poly_fit_length((*prod1).elem.as_mut_ptr(), 2 * n);
            nf_elem_mul(
                prod1,
                chunk[0].as_mut_ptr(),
                chunk[1].as_mut_ptr(),
                cyclotomic_field_n,
            );

            let mut prod2_mem: MaybeUninit<nf_elem_struct> = MaybeUninit::uninit();
            let prod2 = prod2_mem.as_mut_ptr();
            nf_elem_init(prod2, cyclotomic_field_n);
            fmpq_poly_fit_length((*prod2).elem.as_mut_ptr(), 2 * n);
            nf_elem_mul(
                prod2,
                chunk[2].as_mut_ptr(),
                chunk[3].as_mut_ptr(),
                cyclotomic_field_n,
            );

            let mut prod3_mem: MaybeUninit<nf_elem_struct> = MaybeUninit::uninit();
            let prod3 = prod3_mem.as_mut_ptr();
            nf_elem_init(prod3, cyclotomic_field_n);
            fmpq_poly_fit_length((*prod3).elem.as_mut_ptr(), 2 * n);
            nf_elem_mul(
                prod3,
                chunk[4].as_mut_ptr(),
                chunk[5].as_mut_ptr(),
                cyclotomic_field_n,
            );

            let mut sum1_mem: MaybeUninit<nf_elem_struct> = MaybeUninit::uninit();
            let sum1 = sum1_mem.as_mut_ptr();
            nf_elem_init(sum1, cyclotomic_field_n);
            fmpq_poly_fit_length((*sum1).elem.as_mut_ptr(), 2 * n);
            nf_elem_add(sum1, prod1, prod2, cyclotomic_field_n);

            let mut sum2_mem: MaybeUninit<nf_elem_struct> = MaybeUninit::uninit();
            let sum2 = sum2_mem.as_mut_ptr();
            nf_elem_init(sum2, cyclotomic_field_n);
            fmpq_poly_fit_length((*sum2).elem.as_mut_ptr(), 2 * n);
            nf_elem_add(sum2, sum1, prod3, cyclotomic_field_n);
        }

        eprintln!("time elapsed (ms):");
        println!("{}", start.elapsed().as_millis());
    }
}
