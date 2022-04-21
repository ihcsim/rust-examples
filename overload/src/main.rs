fn main() {
    {
        #[derive(Clone, Copy, Debug)]
        struct Complex<T> {
            re: T,
            im: T,
        }

        use std::ops::Add;
        impl<T> Add for Complex<T>
        where
            T: Add<Output = T>,
        {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Complex {
                    re: self.re + rhs.re,
                    im: self.im + rhs.im,
                }
            }
        }

        use std::ops::Neg;
        impl<T> Neg for Complex<T>
        where
            T: Neg<Output = T>,
        {
            type Output = Complex<T>;

            fn neg(self) -> Complex<T> {
                Complex {
                    re: -self.re,
                    im: -self.im,
                }
            }
        }

        use std::ops::AddAssign;
        impl<T> AddAssign for Complex<T>
        where
            T: AddAssign<T>,
        {
            fn add_assign(&mut self, rhs: Complex<T>) {
                self.re += rhs.re;
                self.im += rhs.im;
            }
        }

        // since the ne method has a default definition, we only need to define
        // the eq method to implement the PartialEq trait.
        // instead of defining eq, we can derive the PartialEq attribute for the
        // Complex struct
        impl<T: PartialEq> PartialEq for Complex<T> {
            fn eq(&self, other: &Complex<T>) -> bool {
                self.re == other.re && self.im == other.im
            }
        }
    }

    {
        #[derive(Debug, PartialEq)]
        struct Interval<T> {
            lower: T, // inclusive
            upper: T, // exclusive
        }

        use std::cmp::Ordering;

        impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
            fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
                if self == other {
                    Some(Ordering::Equal)
                } else if self.lower >= other.upper {
                    Some(Ordering::Greater)
                } else if self.upper <= other.lower {
                    Some(Ordering::Less)
                } else {
                    None
                }
            }
        }

        assert!(
            Interval {
                lower: 10,
                upper: 20
            } < Interval {
                lower: 20,
                upper: 40
            }
        );
        assert!(Interval { lower: 7, upper: 8 } >= Interval { lower: 0, upper: 1 });
        assert!(Interval { lower: 7, upper: 8 } <= Interval { lower: 7, upper: 8 });

        let left = Interval {
            lower: 10,
            upper: 30,
        };
        let right = Interval {
            lower: 20,
            upper: 40,
        };
        assert!(!(left < right));
        assert!(!(left >= right));
    }

    {
        let mut desserts = vec!["Howalon".to_string(), "Soan papdi".to_string()];
        desserts[0].push_str(" (fictional)");
        desserts[1].push_str(" (real)");

        use std::ops::IndexMut;
        (*desserts.index_mut(0)).push_str(" (fictional)");
        (*desserts.index_mut(0)).push_str(" (real)");
        println!("{:?}", desserts);
    }

    {
        struct Image<P> {
            width: usize,
            pixels: Vec<P>,
        }

        impl<P: Default + Copy> Image<P> {
            /// Create a new image of the given size.
            fn new(width: usize, height: usize) -> Image<P> {
                Image {
                    width,
                    pixels: vec![P::default(); width * height],
                }
            }
        }

        impl<P> std::ops::Index<usize> for Image<P> {
            type Output = [P];
            fn index(&self, row: usize) -> &[P] {
                let start = row * self.width;
                &self.pixels[start..start + self.width]
            }
        }

        impl<P> std::ops::IndexMut<usize> for Image<P> {
            fn index_mut(&mut self, row: usize) -> &mut [P] {
                let start = row * self.width;
                &mut self.pixels[start..start + self.width]
            }
        }
    }
}
