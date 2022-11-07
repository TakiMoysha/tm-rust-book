fn fix_order() {}

fn cook_order() {}

pub fn fix_incorrect_order() {
    cook_order();
    fix_order();
}

