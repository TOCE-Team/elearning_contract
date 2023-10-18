use unidecode::unidecode;

pub(crate) fn convert_coure_title_to_cousrse_id(title: &str, account_id: String) -> String {
  let account = account_id.replace(".testnet", "").replace(".near", "");
  let unaccented = unidecode(title);
  let lowercased = unaccented.to_ascii_lowercase();
  let result = lowercased + " " + &account;
  result.replace(' ', "_")
}
