use super::service::proto;

#[derive(Clone, Debug)]
pub struct StockPoint {
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
}

#[derive(Clone, Debug)]
pub struct Actionss {
    pub x: Option<Vec<String>>,
    pub y: Option<Vec<StockPoint>>,
    pub owner_id: Option<String>,
    pub group_id: Option<String>,
}

impl TryFrom<proto::ChartRequest> for Actionss {
    type Error = ();
    fn try_from(v: proto::ChartRequest) -> Result<Self, Self::Error> {
        let l = v.clone();
        let x = l.x.unwrap().x;
        let y: Vec<StockPoint> =
            l.y.into_iter()
                .map(|y| StockPoint {
                    open: y.open,
                    close: y.close,
                    low: y.low,
                    high: y.high,
                })
                .collect();

        Ok(Actionss {
            x: Some(x),
            y: Some(y),
            owner_id: Some(l.user_id),
            group_id: Some(l.group_id),
        })
    }
}
