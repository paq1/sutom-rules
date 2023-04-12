use std::fmt::{Debug, Formatter};
use std::process::abort;
use sutom_rules::core::message_handler::handle_message;
use sutom_rules::models::party::Party;

#[test]
fn formattage_message_ok() {
    let message_from_sutom_partage =
        "#SUTOM #460 3/6\n\n🟥🟥🟥🟥🟦🟦🟦\n🟥🟥🟥🟡🟦🟡🟥\n🟥🟥🟥🟥🟥🟥🟥\n\nhttps://sutom.nocle.fr";

    let message = handle_message(&message_from_sutom_partage.to_string());

    assert_eq!(message.is_ok(), true);
}

#[test]
fn parsing_message_ok() {
    let message_from_sutom_partage =
        "#SUTOM #460 3/6\n\n🟥🟥🟥🟥🟦🟦🟦\n🟥🟥🟥🟡🟦🟡🟥\n🟥🟥🟥🟥🟥🟥🟥\n\nhttps://sutom.nocle.fr";

    let message = handle_message(&message_from_sutom_partage.to_string());

    match message {
        Ok(party) => {
            assert_eq!(
                party,
                Party::new(7, 6, 3)
            )
        },
        Err(_) => {
            assert!(false)
        }
    }
}

#[test]
fn fail_parsing_when_message_formatting_is_wrong() {
    let message_from_sutom_partage =
        "#SUTOM #460 3🟥🟥🟥🟥🟦🟦🟦\n🟥🟥🟥🟡🟦🟡🟥\n🟥🟥🟥🟥🟥🟥🟥\n\nhttps://sutom.nocle.fr";

    let message = handle_message(&message_from_sutom_partage.to_string());

    match message {
        Ok(_) => {
            assert!(false)
        },
        Err(err) => {
            assert_eq!(err, "une erreur est survenue".to_string())
        }
    }
}