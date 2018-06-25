use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct QueryParser;

use query;

impl QueryParser {
    pub fn looks_like_query(raw: &String) -> bool {
        let slice: &str = &raw[..];
        if slice.to_lowercase().starts_with("?") {
            return true;
        }
        if slice.to_lowercase().starts_with("+") {
            return true;
        }
        if slice.to_lowercase().starts_with(">") {
            return true;
        }
        if slice.to_lowercase().starts_with(":") {
            return true;
        }

        return false;
    }

    pub fn parse(&self, raw: &String) -> Result<query::Query, ()> {
        let mut tokens = tokenize(raw);

        if tokens.len() == 0 {
            return Err(());
        }

        match &tokens[0].to_lowercase()[..] {
            "+" => parse_create_table(&mut tokens),
            "?" => parse_select(&mut tokens),
            ">" => parse_insert(&mut tokens),
            ":db" => Ok(query::Query::Describe(query::DescribeQuery)),
            _ => {
                error!("Unknown query: {:#?}", raw);
                return Err(());
            }
        }
    }
}

fn tokenize(raw: &String) -> Vec<&str> {
    let slice: &str = (&raw[..]).trim();
    slice.split(' ').collect()
}

fn parse_create_table(tokens: &mut Vec<&str>) -> Result<query::Query, ()> {
    assert!(tokens.len() >= 4);
    assert_eq!("+", tokens.remove(0));

    let table_name = tokens.remove(0);
    let mut fields: Vec<query::FieldDef> = vec![];

    while tokens.len() > 0 {
        // Indices are next.
        if tokens[0] == ":" {
            break;
        }

        assert!(tokens.len() >= 2);
        let field_name = tokens.remove(0);
        let type_name = tokens.remove(0);

        let data_type: query::Type = match &type_name {
            &"int" => query::Type::Int,
            &"varchar" => {
                let size: u8 = match u8::from_str_radix(tokens.remove(0), 10) {
                    Ok(n) => n,
                    Err(e) => {
                        error!("Cannot read varchar size: {:?}", e);
                        return Err(());
                    }
                };
                query::Type::Varchar(size)
            }
            other => {
                error!("Unknown type: {}", other);
                return Err(());
            }
        };

        fields.push(query::FieldDef::new(field_name.to_owned(), data_type));
    }

    let mut indices: Vec<String> = vec![];

    if tokens.len() > 0 {
        assert_eq!(":", tokens.remove(0));

        // @TODO Must be some kind of unrolling.
        while tokens.len() > 0 {
            indices.push(tokens.remove(0).to_owned());
        }
    }

    Ok(query::Query::Create(query::CreateQuery::new(
        table_name.to_owned(),
        fields,
        indices,
    )))
}

fn parse_select(tokens: &mut Vec<&str>) -> Result<query::Query, ()> {
    assert!(tokens.len() >= 4);
    assert_eq!("?", tokens.remove(0));

    let mut columns: Vec<String> = vec![];

    while tokens[0] != ">" {
        let column_name = tokens.remove(0);
        columns.push(column_name.to_owned());
    }

    assert_eq!(">", tokens.remove(0));
    let table = tokens.remove(0).to_owned();

    let mut conditions: Vec<query::FieldCondition> = vec![];
    if tokens.len() > 0 {
        assert_eq!(":", tokens.remove(0));

        while tokens.len() > 0 {
            let field_name = tokens.remove(0).to_owned();
            let op_raw = tokens.remove(0).to_owned();
            let value_raw = tokens.remove(0).to_owned();
            conditions.push(query::FieldCondition::new(field_name, op_raw, value_raw));
        }
    }

    Ok(query::Query::Select(query::SelectQuery::new(
        table, columns, conditions,
    )))
}

fn parse_insert(tokens: &mut Vec<&str>) -> Result<query::Query, ()> {
    assert!(tokens.len() >= 4);
    assert_eq!(">", tokens.remove(0));

    let table_name = tokens.remove(0).to_owned();
    let mut raw_inserts: HashMap<String, String> = HashMap::new();

    while tokens.len() > 0 {
        raw_inserts.insert(tokens.remove(0).to_owned(), tokens.remove(0).to_owned());
    }

    Ok(query::Query::Insert(query::InsertQuery::new(
        table_name,
        raw_inserts,
    )))
}
