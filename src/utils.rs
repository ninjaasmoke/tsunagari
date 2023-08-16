use uuid::Uuid;

pub fn get_ack_id() -> String {
    let tsuna_prefix = "TSUNA";
    let random_uuid_part = Uuid::new_v4()
        .simple()
        .to_string()
        [tsuna_prefix.len()..].chars()
        .filter(|c| *c != '-')
        .collect::<String>();

    let custom_uuid = format!("{}{}", tsuna_prefix, random_uuid_part);
    custom_uuid
}
