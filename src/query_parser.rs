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

  Ok(query::Query::Create(query::CreateQuery::new(
    table_name.to_owned(),
    fields,
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

  Ok(query::Query::Select(query::SelectQuery::new(
    table, columns,
  )))
}
