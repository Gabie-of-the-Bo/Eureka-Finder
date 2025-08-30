use std::{fmt::Display, ops::{Add, Div, Mul, Neg, Sub}};

use num::{Complex, Float};

pub trait Number
where
    Self: Copy + Clone + PartialEq,
    Self: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Neg<Output = Self>,
    Self: Send + Sync
{
    fn sqrt(&self) -> Self;
    fn pow(&self, other: &Self) -> Self;
    fn distance(&self, other: &Self) -> f64;
    fn from_f32(n: f32) -> Self;
    fn to_latex(&self) -> String;
}

impl Number for f32 {
    fn sqrt(&self) -> Self {
        f32::sqrt(*self)
    }

    fn pow(&self, other: &Self) -> Self {
        f32::powf(*self, *other)
    }
    
    fn distance(&self, other: &Self) -> f64 {
        (self - other).abs() as f64
    }
    
    fn from_f32(n: f32) -> Self {
        n
    }
    
    fn to_latex(&self) -> String {
        ToString::to_string(&self)
    }
}

impl Number for f64 {
    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }

    fn pow(&self, other: &Self) -> Self {
        f64::powf(*self, *other)
    }
    
    fn distance(&self, other: &Self) -> f64 {
        (self - other).abs()
    }
    
    fn from_f32(n: f32) -> Self {
        n as f64
    }
    
    fn to_latex(&self) -> String {
        ToString::to_string(&self)
    }
}

impl<T: Float + Display + Send + Sync + Into<f64> + From<f32>> Number for Complex<T> {
    fn sqrt(&self) -> Self {
        Complex::<T>::sqrt(*self)
    }

    fn pow(&self, other: &Self) -> Self {
        Complex::<T>::powc(*self, *other)
    }
    
    fn distance(&self, other: &Self) -> f64 {
        (self - other).norm().into()
    }
    
    fn from_f32(n: f32) -> Self {
        Complex { re: n.into(), im: T::zero() }
    }
    
    fn to_latex(&self) -> String {
        if self.im.is_zero() {
            self.re.to_string()

        } else if self.re.is_zero() {
            format!("({}i)", self.im)

        } else {
            format!("({})", self)
        }
    }
}