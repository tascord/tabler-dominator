#[cfg(test)]
mod tests {
    use tabler_dominator::icon;
    use dominator::{html, with_node};

    #[test]
    fn it_works() {
        let ic = icon!("ad-2");
        // assert_eq!(ic.to_string(), "<svg class=\"icon\"><use xlink:href=\"#ad-2\"></use></svg>");
    }
}
