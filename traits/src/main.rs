use std::fs::File;
use std::io::{self, Write};

#[allow(unused)]
fn main() {
    // rust can achieve polymorphism using either trait objects or
    // generics
    {
        let mut local_file = File::create("hello.txt").unwrap();
        say_hello(&mut local_file).unwrap();

        let mut bytes = vec![];
        say_hello(&mut bytes).unwrap();
        assert_eq!(bytes, b"hello world\n");

        // out is called a trait object
        fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
            out.write_all(b"hello world\n")?;
            out.flush()
        }
    }

    {
        let mut local_file = File::create("hello.txt").unwrap();
        say_hello(&mut local_file).unwrap();

        let mut bytes = vec![];
        say_hello(&mut bytes).unwrap();

        // W is called a "type parameter"
        fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
            out.write_all(b"hello world\\n")?;
            out.flush()
        }
    }

    {
        // given two values, pick whichever one is less
        fn min<T: Ord>(value1: T, value2: T) -> T {
            if value1 < value2 {
                return value1;
            }
            value2
        }

        assert_eq!(2, min(2, 3));
    }

    {
        let mut buf: Vec<u8> = vec![];
        buf.write_all(b"hello").unwrap();
    }

    {
        let v1 = (0..10).collect::<Vec<i32>>();
        println!("{:?}", v1);
    }

    {
        use std::fmt::Debug;
        use std::hash::Hash;

        fn top_ten<T: Debug + Hash + Eq>(_values: &Vec<T>) {}

        struct DataSet {}
        struct Results {}
        trait Mapper {}
        trait Reducer {}
        trait Serialize {}

        fn run_query<M, R>(_data: &DataSet, _map: M, _reduce: R) -> Results
        where
            M: Mapper + Serialize,
            R: Reducer + Serialize,
        {
            Results {}
        }
    }

    // lifetime and type parameters
    {
        trait MeasureDistance {
            fn dist(&self, m: &dyn MeasureDistance) -> f64;
        }
        struct Point2d {}
        impl MeasureDistance for Point2d {
            fn dist(&self, _m: &dyn MeasureDistance) -> f64 {
                0.0
            }
        }

        fn nearest<'t, 'c, P>(target: &'t P, candidates: &'c [P]) -> &'c P
        where
            P: MeasureDistance,
        {
            let mut nearest = &candidates[0];
            for elem in candidates {
                nearest = if elem.dist(target) < nearest.dist(target) {
                    elem
                } else {
                    nearest
                };
            }
            nearest
        }

        {
            // trait objects are the right choice whenever we need a collecton
            // of values of mixed types, all together.
            // trait objects are good for reducing total amount of compiled code.
            trait Vegetable {}
            struct Salad {
                veggies: Vec<Box<dyn Vegetable>>,
            }

            // generic functions are generally faster because there is no dynamic
            // dispatch involved i.e. no `dyn` keyword. Compare to the behaviour
            // with trait objects, Rust never knows what type of value a trait
            // object points to until run time.
        }

        {
            struct Canvas {}
            impl Canvas {
                fn write_at(&self, _x: i32, _y: i32, _symbol: char) {}
            }
            trait Visible {
                fn draw(&self, canvas: &mut Canvas);
                fn hit_test(&self, x: i32, y: i32) -> bool;
            }

            struct Broom {
                x: i32,
                y: i32,
                height: i32,
            }
            impl Broom {
                fn broomstick_range(&self) -> std::ops::Range<i32> {
                    self.y - self.height - 1..self.y
                }
            }
            impl Visible for Broom {
                fn draw(&self, canvas: &mut Canvas) {
                    for y in self.broomstick_range() {
                        canvas.write_at(self.x, y, '|');
                    }
                }

                fn hit_test(&self, x: i32, y: i32) -> bool {
                    self.x == x && self.y - self.height - 1 <= y && y <= self.y
                }
            }

            // subtrait
            struct Direction {}
            trait Creature: Visible {
                fn position(&self) -> (i32, i32);
                fn facing(&self) -> Direction;
            }
        }

        {
            use std::io::Result;

            // we don't need to implement write_all for Sink because the
            // standard library's definition of the `Write` trait contains a
            // default implementation for write_all.
            pub struct Sink;
            impl Write for Sink {
                fn write(&mut self, buf: &[u8]) -> Result<usize> {
                    Ok(buf.len())
                }

                fn flush(&mut self) -> Result<()> {
                    Ok(())
                }
            }
        }

        {
            // extension trait can be used to add new methods to an existing type.
            // a generic `impl` block can add an extension trait to a whole
            // family of types at once.
            struct HtmlDocument {}
            trait WriteHtml {
                fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
            }
            impl<W: Write> WriteHtml for W {
                fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
                    Ok(())
                }
            }
        }

        {
            // trait can include type-associated (static) functions
            trait StringSet {
                fn new() -> Self;
                fn from_slice(strings: &[&str]) -> Self;
                fn contains(&self, string: &str) -> bool;
                fn add(&mut self, string: &str);
            }

            fn unknown_words<S: StringSet>(document: &[String], wordlist: &S) -> S {
                let mut unknowns = S::new();
                for word in document {
                    if !wordlist.contains(word) {
                        unknowns.add(word);
                    }
                }
                unknowns
            }
        }

        {
            // a custom iterator extended from the std iterator
            trait ObjectIterator: Iterator {
                type Object;
            }

            struct Args {}
            impl ObjectIterator for Args {
                type Object = String;
            }

            impl Iterator for Args {
                type Item = String;
                fn next(&mut self) -> Option<String> {
                    None
                }
            }

            fn collect_info_vector<I>(iter: I) -> Vec<I::Item>
            where
                I: ObjectIterator,
            {
                let mut results = Vec::new();
                for value in iter {
                    results.push(value);
                }
                results
            }

            fn dump<I>(iter: I)
            where
                I: Iterator<Item = String>,
            {
                for (index, value) in iter.enumerate() {
                    println!("{}: {:?}", index, value);
                }
            }
        }
    }

    {
        fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> Box<dyn Iterator<Item = u8>> {
            Box::new(v.into_iter().chain(u.into_iter()).cycle())
        }
    }

    {
        // using `impl Trait` means that we cna change the actual type being
        // returned in the future as long as it still implements
        // Iterator<Item=u8>, any code calling the function will continue to
        // compile without an issue.
        fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item = u8> {
            v.into_iter().chain(u.into_iter()).cycle()
        }
    }

    {
        trait Greet {
            const GREETING: &'static str = "Hello";
            fn greet(&self) -> String;
        }

        // trait consts can be declared without any values.
        // implementators of the trait can define these values.
        trait Float {
            const ZERO: Self;
            const ONE: Self;
        }

        impl Float for f32 {
            const ZERO: f32 = 0.0;
            const ONE: f32 = 1.0;
        }
        impl Float for f64 {
            const ZERO: f64 = 0.0;
            const ONE: f64 = 1.0;
        }

        use std::ops::Add;
        fn add_one<T: Float + Add<Output = T>>(value: T) -> T {
            value + T::ONE
        }

        fn fib<T: Float + Add<Output = T>>(n: usize) -> T {
            match n {
                0 => T::ZERO,
                1 => T::ONE,
                n => fib::<T>(n - 1) + fib::<T>(n - 2),
            }
        }
    }

    {
        use std::ops::{Add, Mul};
        fn dot<N>(v1: &[N], v2: &[N]) -> N
        where
            N: Add<Output = N> + Mul<Output = N> + Default + Copy,
        {
            let mut total = N::default();
            for i in 0..v1.len() {
                total = total + v1[i] * v2[i];
            }
            total
        }

        fn test_dot() {
            assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
            assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
        }
    }
}
