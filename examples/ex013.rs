//! 小数点以下2桁の精度で円周率を求めるサンプル

use rust_callenge::math::pi_gauss_legendre;

pub fn main() {
    println!("{}", pi_gauss_legendre(2));
}