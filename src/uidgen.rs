const TS6SIDLENGTH: usize = 3;
const TS6IDLENGTH: usize = 6;

pub struct TS6UIDGenerator {
    state: [u8; TS6SIDLENGTH+TS6IDLENGTH],
}

impl TS6UIDGenerator {
    pub fn new(sid: &str) -> TS6UIDGenerator {
        let mut sidbytes = sid.bytes();
        assert_eq!(sidbytes.len(), 3);
        TS6UIDGenerator {
            state: [
                sidbytes.next().unwrap(), sidbytes.next().unwrap(), sidbytes.next().unwrap(),
                b'A', b'A', b'A', b'A', b'A', b'@'],
        }
    }

    fn increment(&mut self, index: usize) {
        if self.state[index] == b'Z' {
            if index == TS6SIDLENGTH {
                self.state[index] = b'A';
            } else {
                self.state[index] = b'0';
            }
        } else if self.state[index] == b'9' {
            self.state[index] = b'A';
            self.increment(index-1);
        } else {
            self.state[index] += 1;
        }
    }
}

impl Iterator for TS6UIDGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.increment(TS6SIDLENGTH+TS6IDLENGTH-1);

        Some(String::from_utf8_lossy(&self.state).into_owned())
    }
}

#[cfg(test)]
mod test {
    pub use super::*;
    pub use hamcrest::{assert_that, is, not, none, equal_to};

    describe! ts6_uid_generator {
        before_each {
            let mut gen = TS6UIDGenerator::new("42X");
        }

        it "uses SID as the first three bytes" {
            assert_that(gen.next().unwrap().as_str(), is(equal_to("42XAAAAAA")));

            let mut gen = TS6UIDGenerator::new("002");
            assert_that(gen.next().unwrap().as_str(), is(equal_to("002AAAAAA")));
        }

        it "has initial id of AAAAAA" {
            assert_that(gen.next().unwrap().as_str(), is(equal_to("42XAAAAAA")));
        }

        it "increments over letters" {
            gen.next();

            assert_that(gen.next().unwrap().as_str(), is(equal_to("42XAAAAAB")));
            assert_that(gen.next().unwrap().as_str(), is(equal_to("42XAAAAAC")));
        }

        it "increments over numbers when letters are exhausted" {
            gen.state = b"42XAAAAAZ".clone();

            assert_that(gen.next().unwrap().as_str(), is(equal_to("42XAAAAA0")));
            assert_that(gen.next().unwrap().as_str(), is(equal_to("42XAAAAA1")));
        }

        it "increments the second position after first is exhausted" {
            gen.state = b"42XAAAAA9".clone();

            assert_that(gen.next().unwrap().as_str(), is(equal_to("42XAAAABA")));
        }

        it "increments over letters for the last position" {
            gen.state = b"42XA99999".clone();

            assert_that(gen.next().unwrap().as_str(), is(equal_to("42XBAAAAA")));
        }

        it "wraps over when all UIDs are exhausted" {
            gen.state = b"42XZ99999".clone();

            assert_that(gen.next().unwrap().as_str(), is(equal_to("42XAAAAAA")));
        }
    }
}
