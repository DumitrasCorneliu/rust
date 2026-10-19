

fn add_space(s: &mut String, n: i32) {
    let mut i = 0;

    while i < n {
        s.push(' ');
        i += 1;
    }
}

fn add_str(s: &mut String, text: &str) {
    s.push_str(text);
}

fn add_integer(s: &mut String, mut n: i64) {
    if n < 0 {
        s.push('-');
        n = -n;
    }

    if n == 0 {
        s.push('0');
        return;
    }

    let mut temp = n;
    let mut digits = 0;

    while temp > 0 {
        temp = temp / 10;
        digits += 1;
    }

    let mut divisor = 1;

    while divisor <= n / 10 {
        divisor = divisor * 10;
    }

    while divisor > 0 {
        let digit = (n / divisor) % 10;
        s.push((digit as u8 + b'0') as char);

        divisor = divisor / 10;
        digits -= 1;

        if divisor > 0 && digits % 3 == 0 {
            s.push('_');
        }
    }
}

fn add_float(s: &mut String, n: f64) {
    let integer_part = n as i64;
    let decimal_part = ((n - integer_part as f64) * 1000.0) as i64;

    add_integer(s, integer_part);
    s.push('.');

    let mut divisor = 100;
    let mut decimal = decimal_part;

    while divisor > 0 {
        let digit = decimal / divisor;
        s.push((digit as u8 + b'0') as char);

        decimal %= divisor;
        divisor /= 10;
    }
}

fn main() {
    let mut s = String::new();

    // Primul rand
    add_space(&mut s, 40);
    add_str(&mut s, "I");
    add_space(&mut s, 1);
    add_str(&mut s, "💚");
    add_str(&mut s, "\n");

    // Al doilea rand
    add_space(&mut s, 40);
    add_str(&mut s, "RUST.");
    add_str(&mut s, "\n\n");

    // Al treilea rand
    add_space(&mut s, 4);
    add_str(&mut s, "Most");

    add_space(&mut s, 12);
    add_str(&mut s, "crate");

    add_space(&mut s, 6);
    add_integer(&mut s, 306437968);

    add_space(&mut s, 11);
    add_str(&mut s, "and");

    add_space(&mut s, 5);
    add_str(&mut s, "lastest");

    add_space(&mut s, 9);
    add_str(&mut s, "is");

    add_str(&mut s, "\n");

    // Al patrulea rand
    add_space(&mut s, 9);
    add_str(&mut s, "downloaded");

    add_space(&mut s, 8);
    add_str(&mut s, "has");

    add_space(&mut s, 13);
    add_str(&mut s, "downloads");

    add_space(&mut s, 5);
    add_str(&mut s, "the");

    add_space(&mut s, 9);
    add_str(&mut s, "version");

    add_space(&mut s, 4);
    add_float(&mut s, 2.038);

    add_str(&mut s, "\n");

    print!("{}", s);
}