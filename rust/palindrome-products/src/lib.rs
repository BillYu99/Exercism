/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let comp = value.to_string();
        if comp == comp.chars().rev().collect::<String>() {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_pal = None;
    let mut max_pal = None;

    for i in min..=max {
        for j in min..=max {
            let pal = Palindrome::new(i * j);
            if let Some(p) = pal {
                if min_pal.map_or(true, |m: Palindrome| p.into_inner() < m.into_inner()) {
                    min_pal = Some(p);
                }
                if max_pal.map_or(true, |m: Palindrome| p.into_inner() > m.into_inner()) {
                    max_pal = Some(p);
                }
            }
        }
    }

    // Return a tuple of the minimum and maximum palindromes as an Option
    min_pal.and_then(|min| max_pal.map(|max| (min, max)))
}
