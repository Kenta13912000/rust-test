use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
//<<課題1>>
//設定ファイルの型を定義
pub enum ConfigValue {
    String(String),
    Map(HashMap<String, ConfigValue>),
}

//
pub fn parse_file(path: &str) -> Result<HashMap<String, ConfigValue>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?; //ファイルを読み込む
    let mut root = HashMap::new(); //Mapを作成

    //1行ずつ処理
    for line in content.lines() {
        let line = line.trim(); //前後の空白を削除

        //空行or#で始まる行は無視
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, '=').collect(); //=で2つに分割
        //不正な行は無視
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].trim(); //空白を削除してキーを抽出
        let value = parts[1].trim().to_string(); //空白を削除してvalueを抽出

        let keys: Vec<&str> = key.split('.').collect(); //キーを.で分割
        insert_recursive(&mut root, &keys, value); //再帰呼び出し
    }

    Ok(root)
}

//再帰関数
fn insert_recursive(map: &mut HashMap<String, ConfigValue>, keys: &[&str], value: String) {
    //最後のキーなら値を格納
    if keys.len() == 1 {
        map.insert(keys[0].to_string(), ConfigValue::String(value));
        return;
    }

    //途中キーの場合
    //キーがなければMapを作成
    let entry = map
        .entry(keys[0].to_string())
        .or_insert(ConfigValue::Map(HashMap::new()));

    //残りのキーで再帰
    if let ConfigValue::Map(inner) = entry {
        insert_recursive(inner, &keys[1..], value);
    }
}

//<<課題2>>
//スキーマファイルの読み込み
pub fn parse_schema(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?; //ファイルを読み込む
    let mut schema = HashMap::new(); //スキーマを入れるMapを作成

    //1行ずつ処理
    for line in content.lines() {
        let line = line.trim(); //前後の空白を削除

        //空行or#で始まる行は無視
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        //不正な行は無視
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            continue;
        }

        //キーと型を保存
        schema.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
    }

    Ok(schema)
}

//値を取得
fn get_value<'a>(map: &'a HashMap<String, ConfigValue>, key: &str) -> Option<&'a String> {
    let keys: Vec<&str> = key.split('.').collect(); //キーを.で分割

    let mut current = map; //現在のMapを格納

    for (i, k) in keys.iter().enumerate() {
        //キーを順番にたどる
        //最後のキーである場合文字列なら返す
        if i == keys.len() - 1 {
            if let Some(ConfigValue::String(s)) = current.get(*k) {
                return Some(s);
            } else {
                return None;
            }
        }

        //途中のキーである場合Mapであるなら中に入る
        if let Some(ConfigValue::Map(inner)) = current.get(*k) {
            current = inner;
        } else {
            return None;
        }
    }

    None
}

//入力チェック
pub fn validate(
    config: &HashMap<String, ConfigValue>,
    schema: &HashMap<String, String>,
) -> Result<(), String> {
    //スキーマの中身を1つずつ確認
    for (key, expected_type) in schema {
        //値を取得
        let value = match get_value(config, key) {
            Some(v) => v,
            None => return Err(format!("Missing key: {}", key)),
        };

        //型がboolならboolに変換できるかチェック
        if expected_type == "bool" {
            if value.parse::<bool>().is_err() {
                return Err(format!("Invalid bool: {}", key));
            }
        }

        //型がIntegerならIntegerに変換できるかチェック
        if expected_type == "integer" {
            if value.parse::<i64>().is_err() {
                return Err(format!("Invalid integer: {}", key));
            }
        }

        // stringは何もしない
    }

    Ok(())
}

//単体テスト
#[cfg(test)]
mod tests {
    use super::*;

    fn schema_path() -> &'static str {
        "tests/schema.txt"
    }

    //入力値に誤りがないパターン
    #[test]
    fn test_valid_config() {
        let config = parse_file("tests/valid_config.txt").unwrap();
        let schema = parse_schema(schema_path()).unwrap();
        assert!(validate(&config, &schema).is_ok());
    }

    //boolではないパターン
    #[test]
    fn test_invalid_bool() {
        let config = parse_file("tests/invalid_bool.txt").unwrap();
        let schema = parse_schema(schema_path()).unwrap();
        assert!(validate(&config, &schema).is_err());
    }

    //integerではないパターン
    #[test]
    fn test_invalid_integer() {
        let config = parse_file("tests/invalid_integer.txt").unwrap();
        let schema = parse_schema(schema_path()).unwrap();
        assert!(validate(&config, &schema).is_err());
    }

    //キーが不足しているパターン
    #[test]
    fn test_missing_key() {
        let config = parse_file("tests/missing_key.txt").unwrap();
        let schema = parse_schema(schema_path()).unwrap();
        assert!(validate(&config, &schema).is_err());
    }
}
