use encoding_rs::WINDOWS_1252;
use odbc::*;
use odbc_safe::AutocommitOn;

fn main() {
    env_logger::init();

    match connect() {
        Ok(()) => println!("Success"),
        Err(diag) => println!("Error: {}", diag),
    }
}

fn connect() -> std::result::Result<(), DiagnosticRecord> {
    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    let buffer = r#"Driver={Microsoft Visual FoxPro Driver};SourceType=DBF;SourceDB=c:\test\;Exclusive=No;Collate=Machine;NULL=NO;DELETED=YES;BACKGROUNDFETCH=NO;"#;

    let conn = env.connect_with_connection_string(&buffer)?;
    execute_statement(&conn)
}

fn execute_statement<'env>(conn: &Connection<'env, AutocommitOn>) -> Result<()> {
    let stmt = Statement::with_parent(conn)?;

    let sql_text = "select * from Hallodatei".to_string();

    let (encode, _, _) = WINDOWS_1252.encode(&sql_text);

    let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };

    let mut count = 1;

    match stmt.exec_direct(&s)? {
        Data(mut stmt) => {
            let cols = stmt.num_result_cols()?;
            while let Some(mut cursor) = stmt.fetch()? {
                println!("Row {}:", count);
                for i in 1..=cols {
                    println!("  Column {}:", i);
                    let data = cursor.get_data::<Vec<u8>>(i as u16).unwrap().unwrap();
                    let (result, _, _) = WINDOWS_1252.decode(&data);
                    let s = result.to_string();
                    println!("    {}", s);
                }

                count += 1;
            }
        }
        NoData(_) => println!("Query executed, no data returned"),
    }
    println!("Count: {}", count);

    Ok(())
}

fn insert_statement<'env>(conn: &Connection<'env, AutocommitOn>) -> Result<()> {
    let stmt = Statement::with_parent(conn)?;
    let sql_text =
        "INSERT INTO Hallodatei (name, number, logical, id, memo, date, datetime) VALUES ('insert Ä ü ö ß', 1.35, .F., 5, 'Hier können wir auch mal etwas längeres schreiben', DATE(), DATETIME())"
            .to_string();

    let (encode, _, _) = WINDOWS_1252.encode(&sql_text);

    let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };

    match stmt.exec_direct(&s)? {
        Data(_) => println!("Query executed, data returned"),
        NoData(_) => println!("Query executed, no data returned"),
    }

    Ok(())
}
