use std::str;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let bomb: u8 = b'*';
    let spot: u8 = b' ';
    let mut ascii_chars: Vec<Vec<u8>> = Vec::new();

    for row_columns in minefield.into_iter() {
        let columns: &[u8] = row_columns.as_bytes();
        ascii_chars.push(columns.to_vec());
    }

    if ascii_chars.len() == 0 {
        return minefield.iter().map(|a| a.to_string()).collect();
    }

    let l_i = ascii_chars.len() - 1;
    for i in 0..ascii_chars.len() {
        let (i_min, i_max) = match true {
            _ if { i == 0 && i == l_i } => (i, i),
            _ if { i == 0 } => (i, i + 1),
            _ if { i == l_i } => (i - 1, i),
            _ => (i - 1, i + 1),
        };
        if ascii_chars[i].len() == 0 {
            return minefield.iter().map(|a| a.to_string()).collect();
        }

        let l_j = ascii_chars[i].len() - 1;
        'inner: for j in 0..=l_j {
            if ascii_chars[i][j] != bomb {
                continue 'inner;
            }
            let (j_min, j_max) = match true {
                _ if { j == 0 && j == l_j } => (j, j),
                _ if { j == 0 } => (j, j + 1),
                _ if { j == l_j } => (j - 1, j),
                _ => (j - 1, j + 1),
            };
            for x in i_min..=i_max {
                for y in j_min..=j_max {
                    match true {
                        _ if { ascii_chars[x][y] == bomb } => {}
                        _ if { ascii_chars[x][y] == spot } => {
                            ascii_chars[x][y] = b'1';
                        }
                        _ => ascii_chars[x][y] = ascii_chars[x][y] + 1,
                    }
                }
            }
        }
    }

    ascii_chars
        .iter()
        .map(|v| String::from_utf8(v.to_vec()).unwrap())
        .collect()
}
