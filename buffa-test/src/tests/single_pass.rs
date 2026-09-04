//! Single-pass encode: `encode_to_vec_single_pass` must emit byte-identical
//! output to the two-pass `encode_to_vec` across field kinds (nested,
//! oneof, message/scalar maps, groups, empty, deep nesting).

use crate::basic::{Address, Person};
use crate::map_type::{Item, Maps};
use crate::nested::{corecursive, Corecursive};
use crate::proto2::{with_groups, WithGroups};
use buffa::Message;

fn assert_same_bytes<M: Message + PartialEq + core::fmt::Debug>(msg: &M) {
    let two_pass = msg.encode_to_vec();
    let one_pass = msg.encode_to_vec_single_pass();
    assert_eq!(one_pass, two_pass, "single-pass bytes differ");
    assert_eq!(
        M::decode_from_slice(&one_pass),
        M::decode_from_slice(&two_pass)
    );
}

#[test]
fn test_single_pass_nested_and_oneof() {
    let msg = Person {
        id: 7,
        name: "sp".into(),
        address: buffa::MessageField::some(Address {
            street: "s".into(),
            city: "c".into(),
            zip_code: 1,
            ..Default::default()
        }),
        contact: Some(crate::basic::__buffa::oneof::person::Contact::Email(
            "e@x.io".into(),
        )),
        ..Default::default()
    };
    assert_same_bytes(&msg);
}

#[test]
fn test_single_pass_message_and_scalar_maps() {
    let mut msg = Maps::default();
    msg.scores.insert("a".into(), 1);
    msg.items.insert(
        "k".into(),
        Item {
            id: 42,
            ..Default::default()
        },
    );
    msg.big_scores.insert(-5, 9);
    assert_same_bytes(&msg);
    assert_same_bytes(&Maps::default());
}

#[test]
fn test_single_pass_groups() {
    let msg = WithGroups {
        mygroup: buffa::MessageField::some(with_groups::MyGroup {
            a: Some(7),
            b: Some("inner".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_same_bytes(&msg);
}

#[test]
fn test_single_pass_deep_nesting() {
    let msg = Corecursive {
        name: "root".into(),
        nested: buffa::MessageField::some(corecursive::Nested {
            value: 1,
            back: buffa::MessageField::some(Corecursive {
                name: "leaf".into(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_same_bytes(&msg);
    assert_same_bytes(&Person::default());
}
