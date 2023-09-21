#[allow(dead_code,non_camel_case_types,non_snake_case,unused_imports)]
use num_bigint::BigUint;
use num_bigint::{BigInt, ToBigInt};
use num_integer::Integer;
use num_traits::{FromPrimitive, Zero, One};
use rand::{Rng};
use std::{fmt::Error, ops::{Mul, Sub}};

pub fn check_gcd(N: &BigUint,e: &BigUint)-> BigUint{
    if *e!=BigUint::zero(){
        return check_gcd(&e,&(N%e))
    }
    else{
        return N.clone()
    }
    
}


pub fn encrypt(m: &BigUint , modulus: &BigUint , exponent: &BigUint) -> BigUint{

    m.modpow(&exponent, &modulus)

}

pub fn decrypt(modulus_oneless: &BigUint , cipher: &BigUint,exponent: &BigUint) -> BigUint{

    cipher.modpow(&exponent, &modulus_oneless)


}

fn modinv(a0: BigInt, m0: BigInt) -> BigInt {
    if m0 == BigInt::one() {return BigInt::one()}
    let (mut a, mut m, mut x0, mut inv) = (a0, m0.clone(), BigInt::zero(), BigInt::one());
    while a > BigInt::one() {
    inv -= (&a / &m) * &x0;
    a = (&a % &m);
    std::mem::swap(&mut a, &mut m);
    std::mem::swap(&mut x0, &mut inv)
    }
    if inv <BigInt::zero() { inv += m0 }
    inv
    }

fn main(){

    let p = BigUint::from_u32(23).unwrap();
    let q = BigUint::from_u32(29).unwrap();
    let e = BigUint::from_u32(11).unwrap();

    let n = p.clone() * q.clone();
    let n_minus_one = (p - BigUint::one())*(q - BigUint::one());
    let is_e_valid = check_gcd(&n_minus_one,&e);
    

    let m = BigUint::from_u32(65537).unwrap();     
    println!("{}",n_minus_one);
    let d = modinv(e.clone().to_bigint().unwrap(),n_minus_one.to_bigint().unwrap());
    println!("{} {}",e,d);


    // if(is_e_valid==BigUint::one()){
        
        
    // }
    // else{
    //     println!("choose another e pal");
    // }


}