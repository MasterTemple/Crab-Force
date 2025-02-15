use serenity::all::{CreateActionRow, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter};

pub struct BotConfig {
    pub locale: String,
    pub explorer_url: String,
    pub explorer_res_url: String,
    pub author_url: Option<String>,
    pub author_name: Option<String>,
    pub author_icon: Option<String>,
    pub color: Option<String>,
    pub footer_message: Option<String>,
    pub footer_icon: Option<String>,
}

fn join_paths(front: &str, back: &str) -> String {
    let front = front.strip_suffix("/").unwrap_or(&front);
    let back = back.strip_prefix("/").unwrap_or(&back);
    format!("{}/{}", front, back)
}

impl BotConfig {
    pub fn explorer_uri(&self, path: impl AsRef<str>) -> String {
        join_paths(&self.explorer_url, path.as_ref())
    }

    pub fn explorer_res_uri(&self, path: &str) -> String {
        join_paths(&self.explorer_res_url, path)
            .replace(" ", "%20")
            .to_lowercase()
            .replace(".dds", ".png")
    }
}

impl BotConfig {
    pub fn default_embed(&self) -> CreateEmbed {
        let author = self.author_name.as_ref().map(|name| {
            let mut author = CreateEmbedAuthor::new(name);
            if let Some(ref author_icon) = self.author_icon {
                author = author.icon_url(author_icon);
            }
            if let Some(ref author_url) = self.author_url {
                author = author.url(author_url);
            }
            author
        });
        let footer = self.footer_message.as_ref().map(|msg| {
            let mut footer = CreateEmbedFooter::new(msg);
            if let Some(ref icon_url) = self.footer_icon {
                footer = footer.icon_url(icon_url);
            }
            footer
        });
        let mut embed = CreateEmbed::new();
        // embed = embed.color((0x42, 0xb9, 0xf5));
        embed = embed.color((0x1a, 0x87, 0xe8));
        // if let Some(color) = self.color {
        //     embed = embed.color(color);
        // }
        if let Some(author) = author {
            embed = embed.author(author);
        }
        if let Some(footer) = footer {
            embed = embed.footer(footer);
        }
        embed
    }

    pub fn error_embed(
        &self,
        msg: impl Into<String>,
    ) -> (CreateEmbed, Option<Vec<CreateActionRow>>) {
        let embed = self
            .default_embed()
            .title("Error")
            .description(msg)
            .color((0xff, 0x00, 0x00));
        // let response = CreateInteractionResponseMessage::new().embed(embed);
        // response
        (embed, None)
    }
}

impl Default for BotConfig {
    fn default() -> Self {
        BotConfig {
            locale: String::from("en_US"),
            explorer_url: String::from("https://explorer.lu/"),
            explorer_res_url: String::from("https://explorer.lu/lu-res/"),
            author_url: Some(String::from("https://github.com/MasterTemple/Crab-Force")),
            author_name: Some(String::from("Crab Force")),
            // author_icon: Some(String::from("https://explorer.lu/lu-res/textures/ui/inventory/models/amb_crab.png")),
            author_icon: Some(String::from("https://cdn.discordapp.com/avatars/1340084890342785055/4c83403b3a82920365a5007c1aa580ec.webp")),
            color: None, //(0x42, 0xb9, 0xf5),
            footer_message: Some(String::from("LEGO® is a trademark of the LEGO Group which does not sponsor, authorize, or endorse this bot. The data and assets are presented purely for informational purposes.")),
            footer_icon: None,
            // footer_icon: Some(String::from("https://cdn.discordapp.com/attachments/813618981247516715/1339979649328877627/170px-LEGO_logo.png?ex=67b0b0cc&is=67af5f4c&hm=7e9d7b9258682dae296a525bc2fb46a7835a3b9ebefbe5cc192519c32cd66402&format=webp&quality=lossless")),
        }
    }
}
