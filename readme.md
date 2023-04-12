# SUTOM-RULES

Cette lib permet de convertir le message partage sutom.

## Exemple

Avec cette donnée en input :

```text
#SUTOM #460 3/6

🟥🟥🟥🟥🟦🟦🟦
🟥🟥🟥🟡🟦🟡🟥
🟥🟥🟥🟥🟥🟥🟥

https://sutom.nocle.fr
```

On obtient cette struct :

```rust
pub struct Party {
    pub taille_du_mot: u32,
    pub nombre_essaies: u32,
    pub nombre_essaies_total: u32
}
```

Voici un test qui vérifie ce fonctionnement :

```rust
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
```