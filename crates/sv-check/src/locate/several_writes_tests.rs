//! Backlog 0489, V2.3.3: `locate.several-writes` points the transactions question at functions that write twice or
//! more with no transaction around them, each once at its first line (`perFunction`). A transaction anywhere in the
//! function, a decorator among them, leaves it out, and so do writes that are not to the database.

use super::*;
use crate::test_support::scratch;

fn shipped() -> Locators {
    Locators::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/locators.json"),
    )
    .expect("the shipped locators load, every query compiled")
}

fn found(name: &str, text: &str) -> Vec<String> {
    let dir = scratch("");
    std::fs::write(dir.join(name), text).unwrap();
    let listing = sv_scan::files::Listing::of(&dir);
    // The setup worked: the file was listed as the app's own code, or finding nothing would mean nothing.
    assert_eq!(listing.app_files().count(), 1, "{name} was not listed");
    shipped()
        .find(&listing)
        .places
        .get("locate.several-writes")
        .map(|places| {
            places
                .iter()
                .map(|p| format!("{}:{}", p.file, p.line))
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn python_functions_that_write_twice_outside_a_transaction_are_found_once_each() {
    let places = found(
        "views.py",
        "def place_order(request):\n    order = Order.objects.create(user=request.user)\n    stock.save()\n    return order\n\
         \n\
         @transaction.atomic\ndef place_order_safely(request):\n    Order.objects.create(user=request.user)\n    stock.save()\n\
         \n\
         def place_order_in_a_block(request):\n    with transaction.atomic():\n        Order.objects.create()\n        stock.save()\n\
         \n\
         def add_rows(db):\n    db.session.add(a)\n    db.session.add(b)\n    db.session.commit()\n\
         \n\
         def one_write(request):\n    note.save()\n\
         \n\
         def not_the_database(d, s):\n    d.update(x=1)\n    s.add(2)\n    s.add(3)\n",
    );
    // `place_order` alone: atomic as a decorator or a block, a session committed as one, one write, and a dictionary's
    // `update` and a set's `add` are all left out.
    assert_eq!(places, ["views.py:1"]);
    // A SQLAlchemy session's adds, with no commit beside them, are writes too.
    assert_eq!(
        found(
            "models.py",
            "def add_two(db):\n    db.session.add(a)\n    db.session.add(b)\n"
        ),
        ["models.py:1"]
    );
}

#[test]
fn javascript_functions_that_write_twice_outside_a_transaction_are_found_once_each() {
    let places = found(
        "orders.js",
        "async function placeOrder(req) {\n  await prisma.order.create({ data: {} });\n  await prisma.stock.update({ where: {}, data: {} });\n}\n\
         async function placeOrderSafely(req) {\n  await prisma.$transaction([prisma.order.create({}), prisma.stock.update({})]);\n}\n\
         const handler = async (req, res) => { await Order.create({}); await Stock.destroy({}); };\n\
         function forget(cache) { cache.delete('a'); cache.delete('b'); }\n",
    );
    assert_eq!(places, ["orders.js:1", "orders.js:8"]);
}

#[test]
fn a_per_function_locator_keeping_every_place_is_refused() {
    let path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/locators.json");
    let mut data: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    for l in data["locators"].as_array_mut().unwrap() {
        if l["id"] == "locate.several-writes" {
            l["perFunction"]["atLeast"] = serde_json::json!(1);
        }
    }
    let dir = scratch("");
    let file = dir.join("locators.json");
    std::fs::write(&file, serde_json::to_string(&data).unwrap()).unwrap();
    let refused = Locators::load(&file)
        .err()
        .map(|e| e.to_string())
        .unwrap_or_default();
    assert!(refused.contains("use 2 or more"), "{refused}");
}
