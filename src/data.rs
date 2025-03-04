use std::fmt;

use serde::{
    self,
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

/// Snapshot data of instrument from server
///
/// # Source example:
///
/// "a[\"{\\\"message\\\":\\\"pid-945629::{\\\\\\\"pid\\\\\\\":\\\\\\\"945629\\\\\\\",\\\\\\\"last_dir\\\\\\\":\\\\\\\"redBg\\\\\\\",\\\\\\\"last_numeric\\\\\\\":18951.2,\\\\\\\"last\\\\\\\":\\\\\\\"18,951.2\\\\\\\",\\\\\\\"bid\\\\\\\":\\\\\\\"18,954.0\\\\\\\",\\\\\\\"ask\\\\\\\":\\\\\\\"18,956.0\\\\\\\",\\\\\\\"high\\\\\\\":\\\\\\\"19,956.0\\\\\\\",\\\\\\\"low\\\\\\\":\\\\\\\"18,279.0\\\\\\\",\\\\\\\"last_close\\\\\\\":\\\\\\\"19,188.0\\\\\\\",\\\\\\\"pc\\\\\\\":\\\\\\\"-236.8\\\\\\\",\\\\\\\"pcp\\\\\\\":\\\\\\\"-1.23%\\\\\\\",\\\\\\\"pc_col\\\\\\\":\\\\\\\"redFont\\\\\\\",\\\\\\\"turnover\\\\\\\":\\\\\\\"21.50K\\\\\\\",\\\\\\\"turnover_numeric\\\\\\\":21503,\\\\\\\"time\\\\\\\":\\\\\\\"19:21:50\\\\\\\",\\\\\\\"timestamp\\\\\\\":1606850510}\\\"}\"]"
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Snapshot {
    pub pid: String,

    //#[serde(skip_deserializing)]
    pub last_dir: Option<Box<str>>,
    pub last_numeric: f32,
    pub last: Box<str>,
    pub bid: String,
    pub ask: String,
    pub high: String,
    pub low: String,

    #[serde(default)]
    pub last_close: String,

    //#[serde(skip_deserializing)]
    pub pc: String,

    //#[serde(skip_deserializing)]
    pub pcp: String,

    //#[serde(skip_deserializing)]
    pub pc_col: String,

    //#[serde(skip_deserializing)]
    #[serde(default)]
    pub turnover: String,

    #[serde(
        deserialize_with = "deserialize_u32_or_string",
        default = "default_zero"
    )]
    pub turnover_numeric: u32,

    //#[serde(skip_deserializing)]
    pub time: String,
    pub timestamp: u64,
}

fn default_zero() -> u32 {
    0
}

fn deserialize_u32_or_string<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    struct U32Visitor;

    impl<'de> Visitor<'de> for U32Visitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("u32 or string")
        }
        fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(val as Self::Value)
        }

        fn visit_str<E>(self, sval: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if sval == "" {
                return Ok::<Self::Value, E>(default_zero());
            }
            sval.parse().map_err(de::Error::custom)
        }
    }
    let visitor = U32Visitor;
    deserializer.deserialize_any(visitor)
}

impl Snapshot {
    /// Given original data from forexpros wss server, returns the Snapshot with extracted data.
    pub fn from_str<'a>(src: &'a str) -> Self {
        let idx_start = src.find("::{").expect("Expect the opening brace");
        let idx_end = src.find("}").expect("Expect the closing brace");

        let src = &src[idx_start + 2..idx_end + 1].replace("\\\\\\", "");
        serde_json::from_str(src).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_from_str() {
        //let src = "a[\"{\\\"message\\\":\\\"pid-945629::{\\\\\\\"pid\\\\\\\":\\\\\\\"945629\\\\\\\",\\\\\\\"last_dir\\\\\\\":\\\\\\\"redBg\\\\\\\",\\\\\\\"last_numeric\\\\\\\":18951.2,\\\\\\\"last\\\\\\\":\\\\\\\"18,951.2\\\\\\\",\\\\\\\"bid\\\\\\\":\\\\\\\"18,954.0\\\\\\\",\\\\\\\"ask\\\\\\\":\\\\\\\"18,956.0\\\\\\\",\\\\\\\"high\\\\\\\":\\\\\\\"19,956.0\\\\\\\",\\\\\\\"low\\\\\\\":\\\\\\\"18,279.0\\\\\\\",\\\\\\\"last_close\\\\\\\":\\\\\\\"19,188.0\\\\\\\",\\\\\\\"pc\\\\\\\":\\\\\\\"-236.8\\\\\\\",\\\\\\\"pcp\\\\\\\":\\\\\\\"-1.23%\\\\\\\",\\\\\\\"pc_col\\\\\\\":\\\\\\\"redFont\\\\\\\",\\\\\\\"turnover\\\\\\\":\\\\\\\"21.50K\\\\\\\",\\\\\\\"turnover_numeric\\\\\\\":21503,\\\\\\\"time\\\\\\\":\\\\\\\"19:21:50\\\\\\\",\\\\\\\"timestamp\\\\\\\":1606850510}\\\"}\"]";
        let pid = "945629";
        let last_dir = "redDir";
        let last_numeric = 12312.4;
        let last = "3,234.5";
        let bid = "3,535.5";
        let ask = "3,567.4";
        let high = "3,678.1";
        let low = "3,452.1";
        let last_close = "3,513.3";
        let pc = "-3.3";
        let pcp = "-0.3%";
        let pc_col = "redFont";
        let turnover = "3.51K";
        let turnover_numeric = 3513;
        let time = "19:21:50";
        let timestamp = 1606850510;
        let src = format ! ( "a[\"{{\\\"message\\\":\\\"pid-{pid}::{{\\\\\\\"pid\\\\\\\":\\\\\\\"{pid}\\\\\\\",\\\\\\\"last_dir\\\\\\\":\\\\\\\"{last_dir}\\\\\\\",\\\\\\\"last_numeric\\\\\\\":{last_numeric},\\\\\\\"last\\\\\\\":\\\\\\\"{last}\\\\\\\",\\\\\\\"bid\\\\\\\":\\\\\\\"{bid}\\\\\\\",\\\\\\\"ask\\\\\\\":\\\\\\\"{ask}\\\\\\\",\\\\\\\"high\\\\\\\":\\\\\\\"{high}\\\\\\\",\\\\\\\"low\\\\\\\":\\\\\\\"{low}\\\\\\\",\\\\\\\"last_close\\\\\\\":\\\\\\\"{last_close}\\\\\\\",\\\\\\\"pc\\\\\\\":\\\\\\\"{pc}\\\\\\\",\\\\\\\"pcp\\\\\\\":\\\\\\\"{pcp}\\\\\\\",\\\\\\\"pc_col\\\\\\\":\\\\\\\"{pc_col}\\\\\\\",\\\\\\\"turnover\\\\\\\":\\\\\\\"{turnover}\\\\\\\",\\\\\\\"turnover_numeric\\\\\\\":{turnover_numeric},\\\\\\\"time\\\\\\\":\\\\\\\"{time}\\\\\\\",\\\\\\\"timestamp\\\\\\\":{timestamp}}}\\\"}}\"]",
			pid=pid,
			last_dir=last_dir,
			last_numeric=last_numeric,
			last=last,
			bid=bid,
			ask=ask,
			high=high,
			low=low,
			last_close=last_close,
			pc=pc,
			pcp=pcp,
			pc_col=pc_col,
			turnover=turnover,
			turnover_numeric=turnover_numeric,
			time=time,
			timestamp=timestamp,
		);
        let src = src.as_str();

        let snapshot = Snapshot::from_str(src);

        // assertions
        assert_eq!(snapshot.pid, pid);
        assert_eq!(
            snapshot.last_dir,
            Some(String::into_boxed_str(last_dir.to_string()))
        );
        assert_eq!(snapshot.last_numeric, last_numeric);
        assert_eq!(snapshot.bid, bid);
        assert_eq!(snapshot.ask, ask);
        assert_eq!(snapshot.high, high);
        assert_eq!(snapshot.low, low);
        assert_eq!(snapshot.last_close, last_close);
        assert_eq!(snapshot.pc, pc);
        assert_eq!(snapshot.pcp, pcp);
        assert_eq!(snapshot.pc_col, pc_col);
        assert_eq!(snapshot.turnover, turnover);
        assert_eq!(snapshot.turnover_numeric, turnover_numeric);
        assert_eq!(snapshot.time, time);
        assert_eq!(snapshot.timestamp, timestamp);
    }

    #[test]
    pub fn test_turnover_as_int() {
        let src = r#"{"pid":"945629","last_dir":"redBg","last_numeric":18951.2,"last":"18,951.2","bid":"18,954.0","ask":"18,956.0","high":"19,956.0","low":"18,279.0",
		"last_close":"19,188.0","pc":"-236.8","pcp":"-1.23%","pc_col":"redFont","turnover":"21.50K",
		"turnover_numeric":21503,"time":"19:21:50","timestamp":1606850510}"#;

        let snapshot: Snapshot = serde_json::from_str(src).unwrap();

        // assertions
        assert_eq!(snapshot.turnover_numeric, 21503);
    }

    #[test]
    pub fn test_turnover_as_str() {
        let src = r#"{"pid":"945629","last_dir":"redBg","last_numeric":18951.2,"last":"18,951.2","bid":"18,954.0","ask":"18,956.0","high":"19,956.0","low":"18,279.0",
		"last_close":"19,188.0","pc":"-236.8","pcp":"-1.23%","pc_col":"redFont","turnover":"21.50K",
		"turnover_numeric":"21503","time":"19:21:50","timestamp":1606850510}"#;

        let snapshot: Snapshot = serde_json::from_str(src).unwrap();

        // assertions
        assert_eq!(snapshot.turnover_numeric, 21503);
    }

    #[test]
    pub fn test_turnover_empty_str() {
        let src = r#"{"pid":"945629","last_dir":"redBg","last_numeric":18951.2,"last":"18,951.2","bid":"18,954.0","ask":"18,956.0","high":"19,956.0","low":"18,279.0",
		"last_close":"19,188.0","pc":"-236.8","pcp":"-1.23%","pc_col":"redFont","turnover":"21.50K",
		"turnover_numeric":"","time":"19:21:50","timestamp":1606850510}"#;

        let snapshot: Snapshot = serde_json::from_str(src).unwrap();

        // assertions
        assert_eq!(snapshot.turnover_numeric, 0);
    }

    #[test]
    pub fn test_no_turnover() {
        let src = r#"{"pid":"945629","last_dir":"redBg","last_numeric":18951.2,"last":"18,951.2","bid":"18,954.0","ask":"18,956.0","high":"19,956.0","low":"18,279.0",
		"last_close":"19,188.0","pc":"-236.8","pcp":"-1.23%","pc_col":"redFont","turnover":"21.50K" ,"time":"19:21:50","timestamp":1606850510}"#;

        let snapshot: Snapshot = serde_json::from_str(src).unwrap();

        // assertions
        assert_eq!(snapshot.turnover_numeric, 0);
    }

	#[test]
	#[should_panic(expected = "invalid digit found in string")]
    pub fn test_no_turnover_err() {
		let src = r#"{"pid":"945629","last_dir":"redBg","last_numeric":18951.2,"last":"18,951.2","bid":"18,954.0","ask":"18,956.0","high":"19,956.0","low":"18,279.0",
		"last_close":"19,188.0","pc":"-236.8","pcp":"-1.23%","pc_col":"redFont","turnover":"21.50K",
		"turnover_numeric":"olia","time":"19:21:50","timestamp":1606850510}"#;
        let snapshot: Snapshot = serde_json::from_str(src).unwrap();

        // assertions
        assert_eq!(snapshot.turnover_numeric, 0);
    }
}
