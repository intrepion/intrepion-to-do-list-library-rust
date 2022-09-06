#[cfg(test)]
mod test_item_should_have {
    use intrepion_to_do_list_library_rust::item::Item;

    #[test]
    fn title_when_created() {
        let expected = "Untitled";

        let item = Item::default();
        let actual = item.title;

        assert_eq!(expected, actual);
    }

    #[test]
    fn is_done_false_when_created() {
        let expected = false;

        let item = Item::default();
        let actual = item.is_done;

        assert_eq!(expected, actual);
    }

    #[test]
    fn is_visible_true_when_created() {
        let expected = true;

        let item = Item::default();
        let actual = item.is_visible;

        assert_eq!(expected, actual);
    }
}
