// Some tests of dependencies, more as a demo than to serve as verification.
#[cfg(test)]
mod rand_demo {
    use rand::prelude::StdRng;
    use rand::Rng;
    use rand::SeedableRng;
    use rand::thread_rng;

    // https://rust-random.github.io/book/overview.html
    #[test]
    fn autoseed() {
        let mut rng = thread_rng();
        let uni: f64 = rng.gen();
        assert!(uni >= 0.0);
        assert!(uni < 1.0);
    }

    #[test]
    fn repeatable() {
        const MY_SEED: [u8; 32] = [
            123, 164, 185, 95, 103, 243, 38, 140,
            133, 27, 36, 178, 255, 156, 87, 155,
            130, 52, 56, 167, 183, 98, 6, 242,
            214, 42, 82, 202, 230, 246, 83, 234];
        fn random_from_seed() -> f64 {
            let mut rng = StdRng::from_seed(MY_SEED);
            rng.gen::<f64>()
        }
        assert_eq!(random_from_seed(), random_from_seed());
    }
}

#[cfg(test)]
mod lazy_static_demo {
    use lazy_static::lazy_static;

    lazy_static! {
		static ref MY_VEC: Vec<&'static str> = vec!["hello", "world"];
	}

    #[test]
    fn str_vector() {
        assert_eq!("hello", MY_VEC[0]);
    }
}

#[cfg(test)]
mod regex_demo {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
		static ref DATE_RE: Regex = Regex::new(
			r"^(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})$"
		).unwrap();
	}

    #[test]
    fn ymd_date_is_match() {
        assert!(DATE_RE.is_match("2019-04-27"));
        assert!(!DATE_RE.is_match("2019-04-27 "));
    }

    #[test]
    fn ymd_date_capture() {
        let caps = DATE_RE.captures("2019-04-27").unwrap();
        assert_eq!("2019", &caps["year"]);
        assert_eq!("04", &caps["month"]);
        assert_eq!("27", &caps["day"]);
    }
}

#[cfg(test)]
mod chrono_demo {
    use chrono::{NaiveDate, Utc};
    use chrono::Datelike;
    use chrono::TimeZone;
    use chrono_tz::Europe::Amsterdam;

    #[test]
    fn utc() {
        let now = Utc::now();
        assert!(now.year() > 2018);
        let _ = Utc.ymd(2014, 7, 8).and_hms(11, 30, 01);
    }

    #[test]
    fn local() {
        let naive = NaiveDate::from_ymd(2014, 7, 8).and_hms(11, 30, 01);
        let ams = Amsterdam.from_local_datetime(&naive).unwrap();
        let txt = ams.to_string();
        assert_eq!("2014-07-08 11:30:01 CEST", txt)
    }
}

#[cfg(test)]
mod itertools_demo {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    use itertools::Itertools;

    #[test]
    fn interleave() {
        let mut intl = (0..2).interleave(10..20);
        assert_eq!(0, intl.next().unwrap());
        assert_eq!(10, intl.next().unwrap());
        assert_eq!(1, intl.next().unwrap());
        assert_eq!(11, intl.next().unwrap());
        assert_eq!(12, intl.next().unwrap());
    }

    #[test]
    fn zip() {
        let mut zp = (0..2).zip(10..12);
        assert_eq!((0, 10), zp.next().unwrap());
        assert_eq!((1, 11), zp.next().unwrap());
        assert_eq!(None, zp.next());
    }

    #[test]
    fn batch() {
        // Convert any number of elements into any other number of elements.
        // (In this case, its like `.tuples()`).
        let mut btch = (0..8).batching(|it|
            match it.next() {
                None => None,
                Some(v1) => match it.next() {
                    None => None,
                    Some(v2) => match it.next() {
                        None => None,
                        Some(v3) => Some((v1, v2, v3)),
                    }
                }
            });
        assert_eq!((0, 1, 2), btch.next().unwrap());
        assert_eq!((3, 4, 5), btch.next().unwrap());
        assert_eq!(None, btch.next());
    }

    #[test]
    fn chunks() {
        let dat = vec![0u8, 1, 2, 3, 4, 5, 6, 7];
        let mut cnk = dat.chunks(3);
        assert_eq!([0, 1, 2], cnk.next().unwrap());
        assert_eq!([3, 4, 5], cnk.next().unwrap());
        assert_eq!([6, 7], cnk.next().unwrap());
    }

    #[test]
    fn tee() {
        let mut it = 0..2;
        it.next();
        let (mut it1, mut it2) = it.tee();
        assert_eq!(1, it1.next().unwrap());
        assert_eq!(1, it2.next().unwrap());
        assert_eq!(None, it1.next());
        assert_eq!(None, it2.next());
    }

    #[test]
    fn merge() {
        let a = (0..16).step_by(3);
        let b = (0..16).step_by(5);
        let mut it = a.merge(b);
        assert_eq!(0, it.next().unwrap());
        assert_eq!(0, it.next().unwrap());
        assert_eq!(3, it.next().unwrap());
        assert_eq!(5, it.next().unwrap());
        assert_eq!(6, it.next().unwrap());
        assert_eq!(9, it.next().unwrap());
        assert_eq!(10, it.next().unwrap());
        assert_eq!(12, it.next().unwrap());
        assert_eq!(15, it.next().unwrap());
        assert_eq!(15, it.next().unwrap());
        assert_eq!(None, it.next());
    }

    #[test]
    fn cartesian_product() {
        let suit = vec!["♤", "♥", "♢", "♣"].into_iter();
        let rank = 2..15;
        let deck = HashSet::<(&str, u8)>::from_iter(suit.cartesian_product(rank).into_iter());
        assert_eq!(52, deck.len());
        assert!(deck.contains(&("♥", 2)));
    }

    #[test]
    fn filtering() {
        // Get positions of items divisible by 3 after the 8th
        let mut it = (0..15).dropping(8).positions(|i| i % 3 == 0);
        assert_eq!(1, it.next().unwrap());
        assert_eq!(4, it.next().unwrap());
        assert_eq!(None, it.next());
    }
}

#[cfg(test)]
mod generic_array_demo {
    use std::mem::size_of;
    use generic_array::{arr, GenericArray};
    use generic_array::typenum::U4;

    #[test]
    fn macro_create() {
        let array = arr![u32; 1, 2, 3];
        assert_eq!(3, array.len());
        assert_eq!(array[2], 3);
    }

    #[test]
    fn no_overhead() {
        assert!(size_of::<GenericArray<u32, U4>>() == 16)
    }
}

#[cfg(test)]
mod array_tool_demo {
    use array_tool::string::WordWrap;
    use array_tool::vec::{Intersect, Union, Uniq};

    #[test]
    fn vector_set() {
        let a1 = vec![1, 1, 2];
        let a2 = vec![2, 3, 3];
        let inters = a1.intersect(a2.clone());
        assert_eq!(vec![2], inters);
        let unio = a1.union(a2);
        assert_eq!(vec![1, 2, 3], unio);
        assert_eq!(vec![3, 4, 6], vec![1, 2, 3, 4, 5, 6].uniq(vec![1, 2, 5, 7, 9]));
    }

    #[test]
    fn strings() {
        assert_eq!("asd asdf\nasd", "asd asdf asd".word_wrap(8));
    }
}
