
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, PartialEq)]
pub struct Review {
    last_review: Option<SystemTime>,
    next_review: SystemTime,
    ease: f32,
}

impl Review {
    pub fn new(ease: f32) -> Self {
        Self {
            ease,
            ..Default::default()
        }
    }

    pub fn should_be_reviewed(&self) -> bool {
        let now = SystemTime::now();
        now >= self.next_review
    }

    /// Doesn't check next review time.
    pub fn cram(&mut self) {
        let now = SystemTime::now();
        let time_since_last_review = if let Some(last_review) = self.last_review {
            match now.duration_since(last_review) {
                Ok(duration) => duration,
                Err(error) => {
                    eprintln!("Last review was in the future! {error:?}");
                    Duration::from_secs(30)
                }
            }
        } else {
            Duration::from_secs(30)
        };
        self.next_review = now + (time_since_last_review).mul_f32(self.ease);
        self.last_review = Some(now);
    }

    // 0 = again, 1 = good, >1 = easy, 0-1 = tweak review date, <0 = hard.
    pub fn review(&mut self, rating: f32) -> eyre::Result<()> {
        if !self.should_be_reviewed() {
            return Err(eyre::eyre!("Item shouldn't be reviewed yet, use cram()"));
        }
        match rating {
            x if x < -0.001 => {
                todo!();
            }
            x if x < 0.001 => {
                // AGAIN
                self.last_review = Some(SystemTime::now());
                self.next_review = SystemTime::now()
                    .checked_add(Duration::from_secs(30))
                    .ok_or(eyre::eyre!("Couldn't add 30 seconds to SystemTime::now()!"))?;
            }
            x if x < 0.999 => {
                todo!();
            }
            x if x < 1.001 => {
                // GOOD
                self.cram();
            }
            x if x >= 1.001 => {
                todo!();
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

impl Default for Review {
    fn default() -> Self {
        Self {
            last_review: None,
            next_review: SystemTime::now(),
            ease: 1.5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_reviewed() {
        let now = SystemTime::now();
        let two_minutes = Duration::from_secs(120);
        let two_minutes_ago = now.checked_sub(two_minutes).unwrap();
        let review = Review {
            last_review: Some(two_minutes_ago),
            next_review: now,
            ease: 1.5,
        };
        assert!(review.should_be_reviewed());
        let review2 = Review {
            last_review: Some(two_minutes_ago),
            next_review: now.checked_add(two_minutes).unwrap(),
            ease: 1.5,
        };
        assert!(!review2.should_be_reviewed());
    }

    #[test]
    fn review_again() {
        let test_start_time = SystemTime::now();
        let two_minutes = Duration::from_secs(120);
        let two_minutes_ago = test_start_time.checked_sub(two_minutes).unwrap();
        let mut review = Review {
            last_review: Some(two_minutes_ago),
            next_review: test_start_time,
            ease: 1.5,
        };
        review.review(0.0).unwrap();
        let Review {
            last_review,
            next_review,
            ease: _,
        } = review;
        // last review was just now.
        assert!(
            last_review
                .unwrap()
                .duration_since(test_start_time)
                .unwrap()
                <= Duration::from_secs(1)
        );
        // next review will be now + minimum period.
        dbg!(next_review.duration_since(test_start_time));
        assert!(
            next_review.duration_since(test_start_time).unwrap() - Duration::from_secs(30)
                <= Duration::from_secs(1)
        );
    }

    #[test]
    fn review_good() {
        let test_start_time = SystemTime::now();
        let two_minutes = Duration::from_secs(120);
        let two_minutes_ago = test_start_time.checked_sub(two_minutes).unwrap();
        let mut review = Review {
            last_review: Some(two_minutes_ago),
            next_review: test_start_time,
            ease: 1.5,
        };
        review.review(1.0).unwrap();

        let Review {
            last_review,
            next_review,
            ease: _,
        } = review;
        // last review was just now.
        assert!(
            last_review
                .unwrap()
                .duration_since(test_start_time)
                .unwrap()
                <= Duration::from_secs(1)
        );
        // next review will be now + time since last reviow * ease.
        assert!(
            next_review.duration_since(test_start_time).unwrap()
                - Duration::from_secs((120.0 * 1.5) as u64)
                <= Duration::from_secs(1)
        );
    }
}
