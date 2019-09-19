pub fn compile_port_range(range: &Vec<String>) -> Result<Vec<u16>, &'static str> {
    if range.len() == 0 {
        return Err("No range is present");
    }

    let mut pairs: Vec<(u16, u16)> = match range
        .iter()
        .map(|s| {
            let ss: Vec<&str> = s.split('-').collect();
            match ss.len() {
                1 => {
                    //Single number
                    let p: u16 = ss[0].parse().map_err(|e| "Invalid single port number")?;
                    if p == 0 {
                        Err("Port number is 0")
                    } else {
                        Ok((p, p))
                    }
                }
                2 => {
                    let p1: u16 = ss[0].parse().map_err(|e| "Invalid low port number")?;
                    let p2: u16 = ss[1].parse().map_err(|e| "Invalid high port number")?;
                    if p2 < p1 {
                        Err("High port number is lower then low port number")
                    } else if p1 == 0 {
                        Err("Port number is 0")
                    } else {
                        Ok((p1, p2))
                    }
                }
                _ => Err("Invalid port range format"),
            }
        })
        .collect()
    {
        Ok(ps) => ps,
        Err(e) => return Err(e),
    };

    pairs.sort_by(|l, r| {
        if l.0 < r.0 {
            std::cmp::Ordering::Less
        } else if l.0 == r.0 {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let mut max = 0;
    let mut rst: Vec<u16> = Vec::new();
    for p in pairs {
        if p.0 > max {
            for n in p.0..=p.1 {
                rst.push(n);
            }
            max = p.1;
        } else if p.1 > max {
            for n in (max + 1)..=p.1 {
                rst.push(n);
            }
            max = p.1;
        }
    }
    Ok(rst)
}
