use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, PartialEq, PartialOrd)]
struct Complex<T> {
    real: T,
    imag: T,
}

impl<T> Complex<T>
where
    T: Default + Copy + From<T> + Add<T> + Sub<T> + Mul<T>,
{
    fn new<U, V>(real_insert: U, imag_insert: V) -> Self
    where
        U: Into<T> + Default + Copy,
        V: Into<T> + Default + Copy,
    {
        Complex {
            real: real_insert.into(),
            imag: imag_insert.into(),
        }
    }

    fn conjugate(&self) -> Complex<T>
    where
        T: Neg<Output = T>,
    {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }

    fn display_string(&self) -> String
    where
        T: fmt::Display + PartialEq + Default + PartialOrd,
    {
        if self.imag == T::default() {
            format!("{}", self.real)
        } else if self.imag < T::default() {
            format!("{}{}i", self.real, self.imag)
        } else {
            format!("{}+{}i", self.real, self.imag)
        }
    }
}
impl From<i32> for Complex<f64> {
    fn from(real: i32) -> Self {
        Complex::new(real as f64, 0.0)
    }
}

impl From<f64> for Complex<f64> {
    fn from(real: f64) -> Self {
        Complex::new(real, 0.0)
    }
}

impl<T> Add<Complex<T>> for Complex<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl<T> Sub<Complex<T>> for Complex<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl<T> Mul<Complex<T>> for Complex<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl<T> fmt::Display for Complex<T>
where
    T: fmt::Display + PartialEq + PartialOrd + Default + From<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag == T::default() {
            write!(f, "{}", self.real)
        } else if self.imag < T::default() {
            write!(f, "{}{}i", self.real, self.imag)
        } else {
            write!(f, "{}+{}i", self.real, self.imag)
        }
    }
}

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}

macro_rules! assert_eq_rel {
    ($x:expr, $y:expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    // let h = Complex::new(-4, -5);
    // let i = h - (h + 5) * 2.0;
    // assert_eq_rel!(i.real, -6);

    // let j = -i + i;
    // assert_eq_rel!(j.real, 0);
    // assert_eq_rel!(j.imag, 0);

    println!("ok!");
}
