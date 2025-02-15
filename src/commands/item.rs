// pub struct ItemArguments {
//     item: i32,
// }
//
// impl ToCustomId for ItemArguments {
//     const CMD: &'static str = ItemCommand::NAME;
//
//     fn parameters(&self) -> String {
//         let ItemArguments { item } = self;
//         format!("item={item}")
//     }
// }
//
// impl<'a> TryFrom<CustomIdOptions<'a>> for ItemArguments {
//     type Error = String;
//
//     fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
//         Ok(ItemArguments {
//             item: options.parse("item")?,
//         })
//     }
// }
//
// impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for ItemArguments {
//     type Error = String;
//
//     fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
//         Ok(ItemArguments {
//             item: int_option!(options, "item"),
//         })
//     }
// }
