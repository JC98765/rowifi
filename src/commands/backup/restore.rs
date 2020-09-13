use crate::framework::prelude::*;
use crate::models::guild::RoGuild;

pub static BACKUP_RESTORE_OPTIONS: CommandOptions = CommandOptions {
    allowed_roles: &[],
    bucket: None,
    names: &["restore"],
    desc: None,
    usage: None,
    examples: &[],
    required_permissions: Permissions::empty(),
    hidden: false,
    owners_only: false,
    sub_commands: &[],
    group: None
};

pub static BACKUP_RESTORE_COMMAND: Command = Command {
    fun: backup_restore,
    options: &BACKUP_RESTORE_OPTIONS
};

#[command]
pub async fn backup_restore(ctx: &Context, msg: &Message, mut args: Arguments<'fut>) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let name = match args.next() {
        Some(g) => g.to_owned(),
        None => return Ok(())
    };
    let existing = ctx.database.get_guild(guild_id.0).await?.is_some();

    let backup = match ctx.database.get_backup(msg.author.id.0, name).await? {
        Some(b) => b,
        None => return Ok(())
    };

    let server_roles = ctx.cache.roles(guild_id);
    let mut roles = Vec::new();
    for role in server_roles {
        let cached = ctx.cache.role(role);
        if let Some(cached) = cached {
            roles.push(cached);
        }
    }

    let guild = RoGuild::from_backup(backup, ctx, guild_id, &roles).await;
    ctx.database.add_guild(guild, existing).await?;
    let _ = ctx.http.create_message(msg.channel_id).content("Backup successfully restored").unwrap().await?;
    Ok(())
}