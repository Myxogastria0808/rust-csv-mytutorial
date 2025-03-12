use serde_derive::{Deserialize, Serialize};

// error
#[derive(Debug, thiserror::Error)]
enum MyError {
    #[error(transparent)]
    CsvError(#[from] csv::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

// 構造
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct MyRecord {
    name: String,
    age: i32,
    sex: String,
}

fn main() -> Result<(), MyError> {
    //* csv読み込み *//
    {
        //headerはデフォルトでは無視される
        println!("headerはデフォルトでは無視される");
        {
            let mut reader = csv::Reader::from_path("./data/sample.csv")?;
            for result in reader.records() {
                let record = result?;
                println!("{:?}", record);
            }
        }
        //1行目をheaderとして扱わずに読み込む
        println!();
        println!("1行目をheaderとして扱わずに読み込む");
        {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_path("./data/sample.csv")?;
            for result in reader.records() {
                let record = result?;
                println!("{:?}", record);
            }
        }
        //headerを呼び出す
        println!();
        println!("headerを呼び出す");
        {
            let mut reader = csv::Reader::from_path("./data/sample.csv")?;
            let headers = reader.headers()?;
            println!("header: {:?}", headers);
            for result in reader.records() {
                let record = result?;
                println!("{:?}", record);
            }
        }
        // headerを読み込んでstructに変換
        println!();
        println!("headerを読み込んでstructに変換");
        {
            let mut reader = csv::Reader::from_path("./data/sample.csv")?;
            let headers = reader.headers()?;
            println!("header: {:?}", headers);
            for result in reader.deserialize() {
                let record: MyRecord = result?;
                println!("{:?}", record);
            }
        }
    }
    //* csv書き込む *//
    {
        //structの型を指定して書き込む
        {
            let mut writer = csv::Writer::from_path("./data/sample_out.csv")?;
            writer.serialize(MyRecord {
                name: "Alice".to_string(),
                age: 32,
                sex: "F".to_string(),
            })?;
            writer.serialize(MyRecord {
                name: "Bob".to_string(),
                age: 71,
                sex: "M".to_string(),
            })?;
            writer.serialize(MyRecord {
                name: "Carol".to_string(),
                age: 54,
                sex: "F".to_string(),
            })?;
            // CSVライターは内部的にバッファを用いている
            // よって処理の終わりで常にflushを行う必要がある
            writer.flush()?;
        }
        Ok(())
    }
}
