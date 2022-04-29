#[allow(unused_variables, unused_assignments, dead_code)]
fn main() {
    {
        struct Appellation {
            name: String,
            nicknames: Vec<String>,
        }

        impl Drop for Appellation {
            fn drop(&mut self) {
                print!("dropping {}", self.name);
                if !self.nicknames.is_empty() {
                    print!(" (AKA {})", self.nicknames.join(", "));
                }
                println!("");
            }
        }

        let mut a = Appellation {
            name: "Zeus".to_string(),
            nicknames: vec![
                "cloud collector".to_string(),
                "king of the gods".to_string(),
            ],
        };

        println!("before assignment");
        a = Appellation {
            name: "Hera".to_string(),
            nicknames: vec![],
        };
        println!("at end of block");
    }

    {
        struct Selector<T> {
            /// Elements available in this `Selector`
            elements: Vec<T>,

            /// The index of the "current" element in `elements`. A `Selector`
            /// behaves like a pointer to the current element.
            current: usize,
        }

        use std::ops::{Deref, DerefMut};
        impl<T> Deref for Selector<T> {
            type Target = T;
            fn deref(&self) -> &T {
                &self.elements[self.current]
            }
        }

        impl<T> DerefMut for Selector<T> {
            fn deref_mut(&mut self) -> &mut T {
                &mut self.elements[self.current]
            }
        }

        let mut s = Selector {
            elements: vec!['x', 'y', 'z'],
            current: 2,
        };

        assert_eq!(*s, 'z');
        assert!(s.is_alphabetic());

        *s = 'w';
        assert_eq!(s.elements, ['x', 'y', 'w']);
    }

    {
        // exploring Default trait
        use std::collections::HashSet;
        let squares = [4, 9, 16, 25, 36, 49, 64];
        let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) =
            squares.iter().partition(|&n| n & (n - 1) == 0);
        assert_eq!(powers_of_two.len(), 3);
        assert_eq!(impure.len(), 4);

        let (upper, lower): (String, String) = "Great Teacher Onizuka"
            .chars()
            .partition(|&c| c.is_uppercase());
        assert_eq!(upper, "GTO");
        assert_eq!(lower, "reat eacher nizuka");
    }

    {
        use std::net::Ipv4Addr;
        fn ping<A>(address: A) -> std::io::Result<bool>
        where
            A: Into<Ipv4Addr>,
        {
            let ipv4_address = address.into();
            println!("pinging {}", ipv4_address);
            Ok(true)
        }

        println!("{:?}", ping(Ipv4Addr::new(23, 21, 68, 141)));
        println!("{:?}", ping([66, 146, 219, 98]));
        println!("{:?}", ping(0xd076eb94_u32));

        println!("{:?}", Ipv4Addr::from([66, 146, 219, 98]));
        println!("{:?}", Ipv4Addr::from(0xd076eb94_u32));
    }

    {
        type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
        type GenericResult<T> = Result<T, GenericError>;
        fn parse_i32_bytes(b: &[u8]) -> GenericResult<i32> {
            Ok(std::str::from_utf8(b)?.parse::<i32>()?)
        }

        let huge = 2_000_000_000_000i64;
        let smaller: i32 = huge.try_into().unwrap_or(i32::MAX);
        let smaller: i32 =
            huge.try_into()
                .unwrap_or_else(|_| if huge >= 0 { i32::MAX } else { i32::MIN });
    }
}
