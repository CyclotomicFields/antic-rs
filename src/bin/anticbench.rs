use antic::safe::*;
use rand::Rng;
use std::time::Instant;

fn main() {
    let n: i64 = 100;
    let num_tests: usize = 1000;

    eprintln!("n = {}", n);
    eprintln!("num_tests = {}", num_tests);

    let num_per_test: usize = 6;
    let max_num_terms: usize = 5;

    let mut cyclotomic_polynomial_n = RationalPolynomial::cyclotomic(n as u64);
    let mut cyclotomic_field_n = NumberField::new(&mut cyclotomic_polynomial_n);
    let mut nums: Vec<Vec<NumberFieldElement>> = vec![];
    let mut rng = rand::thread_rng();

    eprintln!("generating test data");
    for i in 0..num_tests {
        nums.push(vec![]);
        for _ in 0..num_per_test {
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
                term.set_to_poly(&mut pol, &mut cyclotomic_field_n);
                let mut sum = NumberFieldElement::new(&mut cyclotomic_field_n);
                sum.set_to_sum_of(&mut num, &mut term, &mut cyclotomic_field_n);
                num.set(&mut sum, &mut cyclotomic_field_n);
            }
            nums[i].push(num);
        }
    }

    eprintln!("starting benchmark");

    let start = Instant::now();

    for i in 0..num_tests {
        let chunk = &mut nums[i];
        let mut prod1 = NumberFieldElement::new(&mut cyclotomic_field_n);
        prod1.set_to_mul_of(
            &mut chunk[0].clone(),
            &mut chunk[1].clone(),
            &mut cyclotomic_field_n,
        );

        let mut prod2 = NumberFieldElement::new(&mut cyclotomic_field_n);
        prod2.set_to_mul_of(
            &mut chunk[2].clone(),
            &mut chunk[3].clone(),
            &mut cyclotomic_field_n,
        );

        let mut prod3 = NumberFieldElement::new(&mut cyclotomic_field_n);
        prod3.set_to_mul_of(
            &mut chunk[4].clone(),
            &mut chunk[5].clone(),
            &mut cyclotomic_field_n,
        );

        let mut sum1 = NumberFieldElement::new(&mut cyclotomic_field_n);
        sum1.set_to_sum_of(&mut prod1, &mut prod2, &mut cyclotomic_field_n);

        let mut sum2 = NumberFieldElement::new(&mut cyclotomic_field_n);
        sum2.set_to_sum_of(&mut sum1, &mut prod3, &mut cyclotomic_field_n);
    }

    eprintln!("time elapsed (ms):");
    println!("{}", start.elapsed().as_millis());
}
