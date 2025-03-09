use crate::entity::Resbody;
use csv::Reader;
use log::error;

pub async fn id_to_name(mut res: Resbody) -> Resbody {
    let mut rdr = Reader::from_path("data/chara.csv").unwrap();
    let mut rec: Vec<(u32, String)> = Vec::new();

    for data in rdr.records() {
        let record = data.unwrap();
        let id: u32 = record[0].parse().unwrap();
        let name: String = record[1].to_string();
        rec.push((id, name));
    }

    if rec.len() == 0 {
        error!("CSVの読み込みに失敗しました");
    }

    (0..res.avatar_info_list.len()).for_each(|i| {
        for reco in rec.iter() {
            if res.avatar_info_list[i].avatar_id == reco.0 {
                res.avatar_info_list[i].chara_name = reco.1.to_string();
            }
        }
    });

    return res;
}
