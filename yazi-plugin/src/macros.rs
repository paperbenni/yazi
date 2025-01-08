#[macro_export]
macro_rules! impl_style_method {
	($methods:ident, $($field:tt).+) => {
		$methods.add_function_mut("style", |_, (ud, value): (mlua::AnyUserData, mlua::Value)| {
			ud.borrow_mut::<Self>()?.$($field).+ = $crate::elements::Style::try_from(value)?.0;
			Ok(ud)
		});
	};
}

#[macro_export]
macro_rules! impl_area_method {
	($methods:ident) => {
		use mlua::{AnyUserData, IntoLua};

		$methods.add_function_mut("area", |lua, (ud, area): (AnyUserData, Option<AnyUserData>)| {
			if let Some(v) = area {
				ud.borrow_mut::<Self>()?.area = $crate::elements::Area::try_from(v)?;
				ud.into_lua(lua)
			} else {
				ud.borrow::<Self>()?.area.into_lua(lua)
			}
		});
	};
}

#[macro_export]
macro_rules! impl_style_shorthands {
	($methods:ident, $($field:tt).+) => {
		$methods.add_function_mut("fg", |_, (ud, color): (mlua::AnyUserData, String)| {
			ud.borrow_mut::<Self>()?.$($field).+.fg = yazi_shared::theme::Color::try_from(color).ok().map(Into::into);
			Ok(ud)
		});
		$methods.add_function_mut("bg", |_, (ud, color): (mlua::AnyUserData, String)| {
			ud.borrow_mut::<Self>()?.$($field).+.bg = yazi_shared::theme::Color::try_from(color).ok().map(Into::into);
			Ok(ud)
		});
		$methods.add_function_mut("bold", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::BOLD;
			Ok(ud)
		});
		$methods.add_function_mut("dim", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::DIM;
			Ok(ud)
		});
		$methods.add_function_mut("italic", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::ITALIC;
			Ok(ud)
		});
		$methods.add_function_mut("underline", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::UNDERLINED;
			Ok(ud)
		});
		$methods.add_function_mut("blink", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::SLOW_BLINK;
			Ok(ud)
		});
		$methods.add_function_mut("blink_rapid", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::RAPID_BLINK;
			Ok(ud)
		});
		$methods.add_function_mut("reverse", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::REVERSED;
			Ok(ud)
		});
		$methods.add_function_mut("hidden", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::HIDDEN;
			Ok(ud)
		});
		$methods.add_function_mut("crossed", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::CROSSED_OUT;
			Ok(ud)
		});
		$methods.add_function_mut("reset", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier = ratatui::style::Modifier::empty();
			Ok(ud)
		});
	};
}

#[macro_export]
macro_rules! impl_file_fields {
	($fields:ident) => {
		use mlua::UserDataFields;
		use $crate::bindings::Cast;

		$fields.add_field_method_get("cha", |_, me| Ok($crate::bindings::Cha::from(me.cha)));
		$fields.add_field_method_get("url", |lua, me| $crate::url::Url::cast(lua, me.url_owned()));
		$fields.add_field_method_get("link_to", |lua, me| {
			me.link_to.clone().map(|u| $crate::url::Url::cast(lua, u)).transpose()
		});

		$fields.add_field_method_get("name", |lua, me| {
			Some(me.name())
				.filter(|s| !s.is_empty())
				.map(|s| lua.create_string(s.as_encoded_bytes()))
				.transpose()
		});
	};
}

#[macro_export]
macro_rules! impl_file_methods {
	($methods:ident) => {
		use mlua::UserDataMethods;

		$methods.add_method("icon", |_, me, ()| {
			use yazi_shared::theme::IconCache;
			use $crate::bindings::Icon;

			Ok(match me.icon.get() {
				IconCache::Missing => {
					let matched = yazi_config::THEME.icons.matches(me);
					me.icon.set(matched.map_or(IconCache::Undefined, IconCache::Icon));
					matched.map(Icon::from)
				}
				IconCache::Undefined => None,
				IconCache::Icon(cached) => Some(Icon::from(cached)),
			})
		});
	};
}

#[macro_export]
macro_rules! deprecate {
	($lua:ident, $tt:tt) => {{
		static WARNED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
		if !WARNED.swap(true, std::sync::atomic::Ordering::Relaxed) {
			let id = match $lua.named_registry_value::<$crate::RtRef>("rt")?.current() {
				Some(id) => &format!("`{id}.yazi` plugin"),
				None => "`init.lua` config",
			};
			yazi_proxy::AppProxy::notify(yazi_proxy::options::NotifyOpt {
				title:   "Deprecated API".to_owned(),
				content: format!($tt, id),
				level:   yazi_proxy::options::NotifyLevel::Warn,
				timeout: std::time::Duration::from_secs(20),
			});
		}
	}};
}
