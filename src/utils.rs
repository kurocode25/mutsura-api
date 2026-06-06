use serde::{Deserialize, Deserializer};

/// URLクエリパラメータの数値を Option<i64> としてデシリアライズする。
///
/// CommonPostListQueryParams を `#[serde(flatten)]` で利用した際に、
/// クエリ文字列の `"30"` を `Option<i64>` へ自動変換できず
/// `invalid type: string "30", expected i64` が発生したため使用している。
///
/// 空文字列 (`?page=`) は `None` として扱う。
pub fn deserialize_option_i64_from_str<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => s.parse::<i64>().map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
