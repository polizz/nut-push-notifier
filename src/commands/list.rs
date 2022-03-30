use color_eyre::Report;

use crate::ups::{UpsConnection};

pub fn execute(mut conn: impl UpsConnection) -> Result<(), Report> {
    conn.list_ups()?.iter().for_each(|(name, desc)| {
        println!("UPS Name: {}, Description: {}", &name, &desc);

        conn.list_vars(&name)
            .unwrap()
            .iter()
            .for_each(|val| println!("\t- {}", &val))
    });

    Ok(())
}
