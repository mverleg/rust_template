
// Some tests of dependencies, more as a demo than to serve as verification.
#[cfg(test)]
mod rand_tests {
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
mod lazy_static_tests {
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
mod regex_tests {
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
mod chrono_tests {
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

