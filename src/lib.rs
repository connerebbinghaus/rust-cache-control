extern crate time;

use time::Duration;

#[derive(Eq, PartialEq, Debug)]
pub enum Cachability {
    Public,
    Private,
    NoCache,
    OnlyIfCached,
}


#[derive(Eq, PartialEq, Debug)]
pub struct CacheControl {
    pub cachability: Option<Cachability>,
    pub max_age: Option<Duration>,
    pub s_max_age: Option<Duration>,
    pub max_stale: Option<Duration>,
    pub min_fresh: Option<Duration>,
    pub must_revalidate: bool,
    pub proxy_revalidate: bool,
    pub immutable: bool,
    pub no_store: bool,
    pub no_transform: bool,
}

impl CacheControl {
    fn new() -> CacheControl {
        CacheControl::default()
    }

    pub fn from_value(value: &str) -> Option<CacheControl> {
        let mut ret = CacheControl::new();
        let tokens: Vec<&str> = value.split(",").collect();
        for token in tokens {
            let key_value: Vec<&str> = token.split("=").map(|s| s.trim()).collect();
            let key = key_value.first().unwrap();
            let val = key_value.get(1);

            match *key {
                "public" => ret.cachability = Some(Cachability::Public),
                "private" => ret.cachability = Some(Cachability::Private),
                "no-cache" => ret.cachability = Some(Cachability::NoCache),
                "only-if-cached" => ret.cachability = Some(Cachability::OnlyIfCached),
                "max-age" => {
                    if let None = val {
                        return None;
                    }
                    let val_d = *(val.unwrap());
                    let p_val = val_d.parse();
                    if let Err(_) = p_val {
                        return None;
                    }
                    ret.max_age = Some(Duration::seconds(p_val.unwrap()));
                },
                "max-stale" => {
                    if let None = val {
                        return None;
                    }
                    let val_d = *(val.unwrap());
                    let p_val = val_d.parse();
                    if let Err(_) = p_val {
                        return None;
                    }
                    ret.max_stale = Some(Duration::seconds(p_val.unwrap()));
                },
                "min-fresh" => {
                    if let None = val {
                        return None;
                    }
                    let val_d = *(val.unwrap());
                    let p_val = val_d.parse();
                    if let Err(_) = p_val {
                        return None;
                    }
                    ret.min_fresh = Some(Duration::seconds(p_val.unwrap()));
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

    pub fn from_header(value: &str) -> Option<CacheControl> {
        let header_value: Vec<&str> = value.split(":").map(|s| s.trim()).collect();
        if header_value.len() != 2 || header_value.first().unwrap() != &"Cache-Control" {
            return None;
        }
        let val = header_value.get(1).unwrap();
        CacheControl::from_value(val)
    }
}

impl Default for CacheControl {
    fn default() -> Self {
        CacheControl {
            cachability: None,
            max_age: None,
            s_max_age: None,
            max_stale: None,
            min_fresh: None,
            must_revalidate: false,
            proxy_revalidate: false,
            immutable: false,
            no_store: false,
            no_transform: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{CacheControl, Cachability};
    use time::Duration;

    #[test]
    fn test_from_value() {
        assert_eq!(CacheControl::from_value("").unwrap(), CacheControl::default());
        assert_eq!(CacheControl::from_value("private").unwrap().cachability.unwrap(), Cachability::Private);
        assert_eq!(CacheControl::from_value("max-age=60").unwrap().max_age.unwrap(), Duration::seconds(60));
    }

    #[test]
    fn test_from_value_multi() {
        let test1 = &CacheControl::from_value("no-cache, no-store, must-revalidate").unwrap();
        assert_eq!(test1.cachability, Some(Cachability::NoCache));
        assert_eq!(test1.no_store, true);
        assert_eq!(test1.must_revalidate, true);
        assert_eq!(*test1, CacheControl {
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
        });
    }

    #[test]
    fn test_from_header() {
        assert_eq!(CacheControl::from_header("Cache-Control: ").unwrap(), CacheControl::default());
        assert_eq!(CacheControl::from_header("Cache-Control: private").unwrap().cachability.unwrap(), Cachability::Private);
        assert_eq!(CacheControl::from_header("Cache-Control: max-age=60").unwrap().max_age.unwrap(), Duration::seconds(60));
        assert_eq!(CacheControl::from_header("foo"), None);
        assert_eq!(CacheControl::from_header("bar: max-age=60"), None);
    }

    #[test]
    fn test_from_header_multi() {
        let test1 = &CacheControl::from_header("Cache-Control: public, max-age=600").unwrap();
        assert_eq!(test1.cachability, Some(Cachability::Public));
        assert_eq!(test1.max_age, Some(Duration::seconds(600)));
        assert_eq!(*test1, CacheControl {
            cachability: Some(Cachability::Public),
            max_age: Some(Duration::seconds(600)),
            s_max_age: None,
            max_stale: None,
            min_fresh: None,
            must_revalidate: false,
            proxy_revalidate: false,
            immutable: false,
            no_store: false,
            no_transform: false,
        });
    }
}