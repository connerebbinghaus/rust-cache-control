//! Rust crate to parse the HTTP Cache-Control header.
//! # Example
//! ```
//! use cache_control::{Cachability, CacheControl};
//! use std::time::Duration;
//!
//! let cache_control = CacheControl::from_header("Cache-Control: public, max-age=60").unwrap();
//! assert_eq!(cache_control.cachability, Some(Cachability::Public));
//! assert_eq!(cache_control.max_age, Some(Duration::from_secs(60)));
//! ```

use core::time::Duration;

/// How the data may be cached.
#[derive(Eq, PartialEq, Debug)]
pub enum Cachability {
    /// Any cache can cache this data.
    Public,

    /// Data cannot be cached in shared caches.
    Private,

    /// No one can cache this data.
    NoCache,

    /// Cache the data the first time, and use the cache from then on.
    OnlyIfCached,
}

/// Represents a Cache-Control header
#[derive(Eq, PartialEq, Debug, Default)]
pub struct CacheControl {
    pub cachability: Option<Cachability>,
    /// The maximum amount of time a resource is considered fresh.
    /// Unlike `Expires`, this directive is relative to the time of the request.
    pub max_age: Option<Duration>,
    /// Overrides max-age or the `Expires` header, but only for shared caches (e.g., proxies).
    /// Ignored by private caches.
    pub s_max_age: Option<Duration>,
    /// Indicates the client will accept a stale response. An optional value in seconds
    /// indicates the upper limit of staleness the client will accept.
    pub max_stale: Option<Duration>,
    /// Indicates the client wants a response that will still be fresh for at least
    /// the specified number of seconds.
    pub min_fresh: Option<Duration>,
    /// Indicates that once a resource becomes stale, caches do not use their stale
    /// copy without successful validation on the origin server.
    pub must_revalidate: bool,
    /// Like `must-revalidate`, but only for shared caches (e.g., proxies).
    /// Ignored by private caches.
    pub proxy_revalidate: bool,
    /// Indicates that the response body **will not change** over time.
    pub immutable: bool,
    /// The response may not be stored in _any_ cache.
    pub no_store: bool,
    /// An intermediate cache or proxy cannot edit the response body, 
    /// `Content-Encoding`, `Content-Range`, or `Content-Type`.
    pub no_transform: bool,
}

impl CacheControl {
    /// Parses the value of the Cache-Control header (i.e. everything after "Cache-Control:").
    /// ```
    /// use cache_control::{Cachability, CacheControl};
    /// use std::time::Duration;
    ///
    /// let cache_control = CacheControl::from_value("public, max-age=60").unwrap();
    /// assert_eq!(cache_control.cachability, Some(Cachability::Public));
    /// assert_eq!(cache_control.max_age, Some(Duration::from_secs(60)));
    /// ```
    pub fn from_value(value: &str) -> Option<Self> {
        let mut ret = Self::default();
        for token in value.split(',') {
            let (key, val) = {
                let mut split = token.split('=').map(|s| s.trim());
                (split.next().unwrap(), split.next())
            };

            match key {
                "public" => ret.cachability = Some(Cachability::Public),
                "private" => ret.cachability = Some(Cachability::Private),
                "no-cache" => ret.cachability = Some(Cachability::NoCache),
                "only-if-cached" => ret.cachability = Some(Cachability::OnlyIfCached),
                "max-age" => match val.and_then(|v| v.parse().ok()) {
                    Some(secs) => ret.max_age = Some(Duration::from_secs(secs)),
                    None => return None,
                },
                "max-stale" => match val.and_then(|v| v.parse().ok()) {
                    Some(secs) => ret.max_stale = Some(Duration::from_secs(secs)),
                    None => return None,
                },
                "min-fresh" => match val.and_then(|v| v.parse().ok()) {
                    Some(secs) => ret.min_fresh = Some(Duration::from_secs(secs)),
                    None => return None,
                },
                "must-revalidate" => ret.must_revalidate = true,
                "proxy-revalidate" => ret.proxy_revalidate = true,
                "immutable" => ret.immutable = true,
                "no-store" => ret.no_store = true,
                "no-transform" => ret.no_transform = true,
                _ => (),
            };
        }
        Some(ret)
    }

    /// Parses a Cache-Control header.
    pub fn from_header(value: &str) -> Option<Self> {
        let (name, value) = value.split_once(':')?;
        if !name.trim().eq_ignore_ascii_case("Cache-Control") {
            return None;
        }
        Self::from_value(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_value() {
        assert_eq!(
            CacheControl::from_value("").unwrap(),
            CacheControl::default()
        );
        assert_eq!(
            CacheControl::from_value("private")
                .unwrap()
                .cachability
                .unwrap(),
            Cachability::Private
        );
        assert_eq!(
            CacheControl::from_value("max-age=60")
                .unwrap()
                .max_age
                .unwrap(),
            Duration::from_secs(60)
        );
    }

    #[test]
    fn test_from_value_multi() {
        let test1 = &CacheControl::from_value("no-cache, no-store, must-revalidate").unwrap();
        assert_eq!(test1.cachability, Some(Cachability::NoCache));
        assert!(test1.no_store);
        assert!(test1.must_revalidate);
        assert_eq!(
            *test1,
            CacheControl {
                cachability: Some(Cachability::NoCache),
                max_age: None,
                s_max_age: None,
                max_stale: None,
                min_fresh: None,
                must_revalidate: true,
                proxy_revalidate: false,
                immutable: false,
                no_store: true,
                no_transform: false,
            }
        );
    }

    #[test]
    fn test_from_header() {
        assert_eq!(
            CacheControl::from_header("Cache-Control: ").unwrap(),
            CacheControl::default()
        );
        assert_eq!(
            CacheControl::from_header("Cache-Control: private")
                .unwrap()
                .cachability
                .unwrap(),
            Cachability::Private
        );
        assert_eq!(
            CacheControl::from_header("Cache-Control: max-age=60")
                .unwrap()
                .max_age
                .unwrap(),
            Duration::from_secs(60)
        );
        assert_eq!(CacheControl::from_header("foo"), None);
        assert_eq!(CacheControl::from_header("bar: max-age=60"), None);
    }

    #[test]
    fn test_from_header_multi() {
        let test1 = &CacheControl::from_header("Cache-Control: public, max-age=600").unwrap();
        assert_eq!(test1.cachability, Some(Cachability::Public));
        assert_eq!(test1.max_age, Some(Duration::from_secs(600)));
        assert_eq!(
            *test1,
            CacheControl {
                cachability: Some(Cachability::Public),
                max_age: Some(Duration::from_secs(600)),
                s_max_age: None,
                max_stale: None,
                min_fresh: None,
                must_revalidate: false,
                proxy_revalidate: false,
                immutable: false,
                no_store: false,
                no_transform: false,
            }
        );
    }
}
