use eij_tracker::eij::{Attribute, AttributeType, AttributeValue, Player};

fn main() {
    let mut player_attrs = Attribute::new();
    player_attrs.add_optional("age", AttributeType::Int);
    player_attrs.add_int("strength");
    player_attrs.add_bool("is_alive");

    let mut item_attrs = Attribute::new();
    item_attrs.add_text("name");
    item_attrs.add_float("weight");
    item_attrs.add_int("quantity");

    player_attrs.add_record("inventory", item_attrs);

    println!("Player Attributes: {:#?}", player_attrs);

    let mut player = Player::new("Alice");
    player.set_attribute("age", AttributeValue::Int(30));
    player.set_attribute("strength", AttributeValue::Int(5));
    player.set_attribute("is_alive", AttributeValue::Bool(true));

    player.set_attribute(
        "inventory",
        AttributeValue::Record(
            vec![
                (
                    "name".to_string(),
                    AttributeValue::Text("Sword".to_string()),
                ),
                ("weight".to_string(), AttributeValue::Float(3.5)),
                ("quantity".to_string(), AttributeValue::Int(1)),
            ]
            .into_iter()
            .collect(),
        ),
    );

    println!("Player: {:#?}", player);
}
