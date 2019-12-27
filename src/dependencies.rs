//TODO @mark: split into files?

use ::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Location(f64, f64, f64);
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Thing {
    name: String,
    age: u16,
    location: Location
}

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
    #[allow(clippy::float_cmp)]
    fn repeatable() {
        const MY_SEED: [u8; 32] = [
            123, 164, 185, 95, 103, 243, 38, 140, 133, 27, 36, 178, 255, 156, 87, 155, 130, 52, 56,
            167, 183, 98, 6, 242, 214, 42, 82, 202, 230, 246, 83, 234,
        ];
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
    use regex::Regex;

    use lazy_static::lazy_static;

    lazy_static! {
        static ref DATE_RE: Regex =
            Regex::new(r"^(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})$").unwrap();
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
        let _ = Utc.ymd(2014, 7, 8).and_hms(11, 30, 1);
    }

    #[test]
    fn local() {
        let naive = NaiveDate::from_ymd(2014, 7, 8).and_hms(11, 30, 1);
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
        let mut btch = (0..8).batching(|it| match it.next() {
            None => None,
            Some(v1) => match it.next() {
                None => None,
                Some(v2) => match it.next() {
                    None => None,
                    Some(v3) => Some((v1, v2, v3)),
                },
            },
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
        let deck = HashSet::<(&str, u8)>::from_iter(suit.cartesian_product(rank));
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
        assert_eq!(
            vec![3, 4, 6],
            vec![1, 2, 3, 4, 5, 6].uniq(vec![1, 2, 5, 7, 9])
        );
    }

    #[test]
    fn strings() {
        assert_eq!("asd asdf\nasd", "asd asdf asd".word_wrap(8));
    }
}

#[cfg(test)]
mod array_compression_demo {
    use std::io::{Read, Write};

    use ::brotli::{CompressorWriter, Decompressor, enc};
    use ::lipsum::lipsum;
    use ::mockstream::SharedMockStream;

    //TODO @mark: re-enable
    // #[test]
    fn _smaller() {
        let mut stream = SharedMockStream::new();

        let text = lipsum(100);

        {
            let params = enc::BrotliEncoderParams::default();
            let mut writer = CompressorWriter::with_params(&mut stream, 4096, &params);
            writer.write_all(&text.as_bytes()).unwrap();
            writer.flush().unwrap();
        }

        {
            let mut reader = Decompressor::new(&mut stream, 4096);
            //            let mut res = String::new();
            let mut res = [0u8; 4096];
            //TODO @mark: should this really be '_exact'? (clippy)
            reader.read_exact(&mut res).unwrap();
            println!("{:?}", res[0]);
            println!("{:?}", res[1]);
            println!("{:?}", res[2]);
            println!("{:?}", res[3]);
            println!("{:?}", res[4]);
            println!("{:?}", res[5]);
            println!("{:?}", res[6]);
            println!("{:?}", res[7]);
            println!("{:?}", res[8]);
            println!("{:?}", res[9]);
            println!("{:?}", res[10]);
            println!("{:?}", res[11]);
            println!("{:?}", res[12]);
            println!("{:?}", res[13]);
            println!("{:?}", res[14]);
            println!("{:?}", res[15]);
            println!("{:?}", res[16]);
            println!("{:?}", res[17]);
            println!("{:?}", res[18]);
            println!("{:?}", res[19]);
            println!("{:?}", res[20]);
            println!("{:?}", res[21]);
            println!("{:?}", res[22]);
            //            assert_eq!(text, res);
        }

        //TODO @mark: test that it was actually shorter while compressed?
    }
}

#[cfg(test)]
mod bincode_serde {
    use std::mem::size_of;

    use ::bincode;

    use super::{Location, Thing};

    #[test]
    //noinspection RsApproxConstant
    fn encode_decode_smaller() {
        let original = vec![
            Some(Thing { name: "Alpha".to_owned(), age: 37, location: Location(3.1416, 2.7182, -999_999_999.999_999_999) }),
            Some(Thing { name: "Beta".to_owned(), age: 111, location: Location(-1.0, 1.0, 0.0) }),
            None,
        ];
        let data = bincode::serialize(&original).unwrap();
        assert!(data.len() <= 2 * size_of::<Thing>());
        let back: Vec<Option<Thing>> = bincode::deserialize(&data).unwrap();
        assert_eq!(original, back);
    }
}

#[cfg(test)]
mod smallvec {
    use std::mem::size_of;

    use ::smallvec::{smallvec, SmallVec};

    use super::Location;

    #[test]
    //noinspection RsApproxConstant
    fn growing_small_vec() {
        assert_eq!(3 * size_of::<Location>() + 2 * 8, size_of::<SmallVec<[Location; 3]>>());
        let mut data: SmallVec<[Location; 3]> = smallvec![Location(-1.0, 1.0, 0.0), Location(-1.0, 1.0, 1.0)];
        data.push(Location(-1.0, 1.0, 0.0));
        assert!(!data.spilled());
        data.push(Location(-1.0, 1.0, 0.0));
        assert!(data.spilled());
        data.pop();
        assert!(data.spilled());
        data.shrink_to_fit();
        assert!(!data.spilled());
    }
}

mod ndarray {
    use ::approx::assert_abs_diff_eq;
    use ::ndarray::Array;
    use ::ndarray::ArrayBase;
    use ::ndarray::Dim;
    use ::ndarray::OwnedRepr;
    use ::ndarray_linalg::eigh::Eigh;
    use ::ndarray_linalg::UPLO;
    use ::ndarray_rand::RandomExt;
    use ::rand::distributions::Uniform;
    use ::rand::SeedableRng;
    use ::rand_xorshift::XorShiftRng;

    #[test]
    fn mul_2d() {
        let mut rng = XorShiftRng::seed_from_u64(42);
        let a = Array::random_using((10, 7),
            Uniform::new(-10., 10.), &mut rng);
        let b = Array::random_using((7, 10),
            Uniform::new(0., 1.), &mut rng);
        let c = a.dot(&b);
        assert_eq!(&[10, 10], c.shape());
        assert_abs_diff_eq!(-15.666947811241746, c[[2, 4]], epsilon = 1.0e-10);
    }

    #[test]
    fn eigh_2d() {
        let mut rng = XorShiftRng::seed_from_u64(42);
        let mut a = Array::random_using((9, 9),
            Uniform::new(-10., 10.), &mut rng);
        // Make the matrix symmetric (thus Hermitian for reals)
        for i in 0 .. a.shape()[0] {
            for j in i .. a.shape()[0] {
                a[[i, j]] = a[[j, i]];
            }
        }
        type Vector = ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>>;
        let (e, _): (Vector, _) = a.eigh(UPLO::Upper).expect("eigenvalue decomposition failed");
        assert_abs_diff_eq!(32.824746293304266, e[8], epsilon = 1.0e-10);
    }
}

mod num {
    use ::num::BigInt;
    use ::num::FromPrimitive;
    use ::num::rational::Ratio;

    #[test]
    fn complex() {
        panic!();
    }

    #[test]
    fn bigint() {
        panic!();
    }

    #[test]
    fn ratio() {
        // Newton's method, from the manual: https://rust-num.github.io/num/num/index.html
        let two = Ratio::from_integer(FromPrimitive::from_u64(2).unwrap());
        let start: Ratio<BigInt> = Ratio::from_integer(FromPrimitive::from_u64(5).unwrap());
        let mut approx = start.clone();
        for i in 0 .. 5 {
            println!("i = {}", i);
            approx = (&approx + (&start / &approx)) / two.clone();
        }
        println!("{:?}", approx);
        let epsilon = Ratio::<BigInt>::new(BigInt::from_u64(1).unwrap(), BigInt::from_u64(1_000_000_000).unwrap());
        let known = Ratio::<BigInt>::new(BigInt::from_u64(22_360_679_775).unwrap(), BigInt::from_u64(10_000_000_000).unwrap());
        assert!(approx.clone() - known.clone() < epsilon.clone() && known.clone() - approx.clone() > -epsilon.clone());
        // assert_abs_diff_eq!(32.824746293304266, approx, epsilon = 1.0e-10);
        // https://rust-num.github.io/num/num/index.html
    }
}