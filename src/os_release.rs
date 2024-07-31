use std::{borrow::Borrow, collections::BTreeMap, fs, io};

#[derive(Debug)]
pub struct OsRelease(BTreeMap<String, String>);

macro_rules! impl_getters {
    ($($getter:ident:$name:literal)+) => {$(
        #[inline]
        pub fn $getter(&self) -> Option<&str> {
            self.get($name)
        }
    )+};
}

impl OsRelease {
    pub fn new() -> Self {
        Self(Self::new_inner().unwrap_or(BTreeMap::new()))
    }

    pub fn try_new() -> Result<Self, io::Error> {
        Self::new_inner().map(Self)
    }

    fn new_inner() -> Result<BTreeMap<String, String>, io::Error> {
        let os_release_file = fs::read_to_string("/etc/os-release")?;

        let mut entries = BTreeMap::new();

        for line in os_release_file.lines() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            if let Some((name, content)) = line.split_once('=') {
                let name = name.trim();
                let content = content.trim_matches('"');

                entries.insert(name.to_string(), content.to_string());
            }
        }

        Ok(entries)
    }

    #[inline]
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&str>
    where
        String: Borrow<Q>,
        Q: Ord,
    {
        self.0.get(key).map(String::as_str)
    }

    pub fn id_like(&self) -> Option<Vec<&str>> {
        let id_like = self.0.get("ID_LIKE")?;

        let mut list = Vec::new();

        for os in id_like.split_whitespace() {
            list.push(os)
        }

        Some(list)
    }

    impl_getters! {
        name: "NAME"
        id: "ID"
        pretty_name: "PRETTY_NAME"
        cpe_name: "CPE_NAME"
        variant: "VARIANT"
        variant_id: "VARIANT_ID"
        version: "VERSION"
    }
}
