use nif::Nif;
use std::io::Cursor;

fn round_trip(n: u32) {
    let path = format!("tests/{}.nif", n);
    let bytes = std::fs::read(&path).expect("read nif");

    let mut reader = Cursor::new(&bytes);
    let nif = Nif::parse(&mut reader).expect("parse nif");
    let consumed = reader.position() as usize;

    let mut written = Vec::new();
    nif.write(&mut Cursor::new(&mut written)).expect("write nif");

    let original = &bytes[..consumed];
    if written != original {
        let at = written
            .iter()
            .zip(original)
            .position(|(a, b)| a != b)
            .unwrap_or(written.len().min(original.len()));
        panic!(
            "{}: wrote {} bytes, read {}; first difference at {} ({:02x?} vs {:02x?})",
            path,
            written.len(),
            consumed,
            at,
            written.get(at..(at + 8).min(written.len())),
            original.get(at..(at + 8).min(original.len())),
        );
    }
}

macro_rules! round_trip_tests {
    ($($name:ident => $n:literal),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                round_trip($n);
            }
        )*
    };
}

round_trip_tests! {
    nif_1 => 1, nif_2 => 2, nif_3 => 3, nif_4 => 4, nif_5 => 5,
    nif_6 => 6, nif_7 => 7, nif_8 => 8, nif_9 => 9, nif_10 => 10,
    nif_11 => 11, nif_12 => 12, nif_13 => 13, nif_14 => 14, nif_15 => 15,
    nif_16 => 16, nif_17 => 17, nif_18 => 18, nif_19 => 19, nif_20 => 20,
    nif_21 => 21, nif_22 => 22, nif_23 => 23, nif_24 => 24, nif_25 => 25,
    nif_26 => 26,
}
