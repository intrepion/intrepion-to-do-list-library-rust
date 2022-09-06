use intrepion_to_do_list_library_rust::item::Item;

#[test]
fn item_should_have_title_when_created() {
    let _expected = "Untitled";

    let item = Item::default();
    let actual = item.title;
}
