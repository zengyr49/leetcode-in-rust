///
/// PAYPALISHIRING turning into
/// P   A   H   N
/// A P L S I I G
/// Y   I   R
///
/// we define a unit like "PAYP", we calculate unit and related cols to create a new matrix
/// we go up and down the matrix to set value in it
/// finally we scan each row and get outcome
///


pub fn convert(s: String, num_rows: i32) -> String {
    // unit rows, how many units,
    // let unit_rows = num_rows - 1;
    // let unit_count = s.len() as i32 / unit_rows;
    // let remain = s.len() as i32 - (unit_rows * unit_count);
    // let extra = max(0, remain-num_rows) + 1;
    // let num_cols = unit_rows * unit_count + extra;
    if num_rows == 1 {
        return s;
    }

    let mut z_mat = Vec::new();
    let num_rows = num_rows as usize;
    for i in 0..num_rows {
        z_mat.insert(i, String::new())
    }

    // going up and down
    let mut now_to = 0;
    let mut down = true;
    for i in 0..s.len() {
        if let Some(vec2) = z_mat.get_mut(now_to) {
            if down {
                // go down
                vec2.push_str(&s[i..i + 1]);
                now_to+=1;
                down =  now_to < (num_rows-1);
            } else {
                // go up
                vec2.push_str(&s[i..i + 1]);
                now_to-=1;
                down = now_to == 0;
            }
        }
    }
    // scan
    let mut res = String::new();
    for x in z_mat {
        res.push_str(&x);
    }

    // define 2d matrix
    res.to_string()
}

#[cfg(test)]
mod tests {
    use crate::zconvert::convert;

    #[test]
    fn test_convert() {
        // let s = "PAYPALISHIRING".to_string();
        // let num_rows = 3;
        // let res = convert(s, num_rows);
        // assert_eq!(res, "PAHNAPLSIIGYIR");
        let s = "AB".to_string();
        let num_rows = 1;
        let res = convert(s, num_rows);

    }
}