use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;

use crate::TryGetableFromJson;

impl<K, V> TryGetableFromJson for BTreeMap<K, V> where for<'de> Self: Deserialize<'de> {}
impl<K, V> TryGetableFromJson for HashMap<K, V> where for<'de> Self: Deserialize<'de> {}
