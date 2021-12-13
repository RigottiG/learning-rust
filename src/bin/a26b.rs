// Topic Inline Modules

mod msg {
    pub fn trim(msg: &str) -> &str {
        msg.trim()
    }

    pub fn capitalize(msg: &str) -> std::borrow::Cow<'_, str> {
        if let Some(letter) = msg.get(0..1) {
            format!("{}{}", letter.to_uppercase(), &msg[1..msg.len()]).into()
        } else {
            msg.into()
        }
    }

    pub fn exciting(msg: &str) -> String {
        format!("{}!", msg)
    }
}

mod math {
    pub fn add(lhs: isize, rhs: isize) -> isize {
        lhs + rhs
    }

    pub fn sub(lhs: isize, rhs: isize) -> isize {
        lhs - rhs
    }

    pub fn mul(lhs: isize, rhs: isize) -> isize {
        lhs * rhs
    }
}

fn main() {
    let result = {
        let two_plus_two = math::add(2, 2);
        let three = math::sub(two_plus_two, 1);
        math::mul(three, three)
    };

    assert_eq!(result, 9);
    println!("(2 + 2 - 1) * 3 = {}", result);

    {
        use msg::{capitalize, exciting, trim};

        let hello = {
            let msg = "hello";
            trim(msg);
            capitalize(msg)
        };

        let world = {
            let msg = "world";
            exciting(msg)
        };

        let msg = format!("{}, {}", hello, world);

        assert_eq!(&msg, "Hello, world!");
        println!("{}", msg);
    }
}
