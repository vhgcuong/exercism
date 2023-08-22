// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    function: Box<dyn Fn(T) -> bool>,
    word: String
}

impl<T> Matcher<T> {
    pub fn new<F: Fn(T) -> bool, S: ToString>(_matcher: F, _subs: S) -> Matcher<T>
        where
            F: Fn(T) -> bool + 'static,
            S: ToString
    {
        Self {
            function: Box::new(_matcher),
            word: _subs.to_string()
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>
}

impl<T> Fizzy<T> where T: ToString + Copy {
    pub fn new() -> Self {
        Self { matchers: vec![] }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.matchers.push(_matcher);

        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, _iter: I) -> impl Iterator<Item = String>
        where
            I: Iterator<Item = T>
    {
        _iter.map(move |elem| {
            let mut matched = false;

            let mut result = String::new();

            for matcher in self.matchers.iter() {
                if (matcher.function)(elem) {
                    result.push_str(&matcher.word);
                    matched = true;
                }
            }

            if matched {
                result
            } else {
                elem.to_string()
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: ToString + Copy>() -> Fizzy<T>
    where
        T: From<u8> + std::ops::Rem<Output = T> + PartialEq
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
