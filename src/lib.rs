mod cpuinfo;
mod os_release;
mod passwd;

pub use cpuinfo::{Cpu, Cpuinfo};
pub use os_release::OsRelease;
pub use passwd::{getpwuid, Passwd, PasswdEntries};

macro_rules! impl_getters {
    ($($getter:ident:$name:literal)+) => {
        #[inline]
        pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&str>
        where
            String: Borrow<Q>,
            Q: Ord,
        {
            self.0.get(key).map(String::as_str)
        }

        $(
        #[inline]
        pub fn $getter(&self) -> Option<&str> {
            self.get($name)
        }
        )
    +};
}

pub(crate) use impl_getters;
