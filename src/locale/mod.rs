use fluent::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;
use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Russian,
    English,
}

impl Language {
    pub fn as_langid(&self) -> LanguageIdentifier {
        match self {
            Language::Russian => "ru".parse().unwrap(),
            Language::English => "en".parse().unwrap(),
        }
    }

    pub fn native_name(&self) -> &'static str {
        match self {
            Language::Russian => "Русский",
            Language::English => "English",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::Russian, Self::English]
    }
}

impl Default for Language {
    fn default() -> Self {
        // Автоопределение языка системы
        if let Ok(lang) = std::env::var("LANG") {
            if lang.starts_with("ru") {
                return Language::Russian;
            }
        }
        Language::English
    }
}

pub struct Localization {
    bundles: HashMap<Language, FluentBundle<FluentResource>>,
}

impl Localization {
    pub fn new() -> Self {
        let mut bundles = HashMap::new();
        
        // Русская локализация
        if let Some(bundle) = Self::create_bundle(Language::Russian) {
            bundles.insert(Language::Russian, bundle);
        }
        
        // Английская локализация
        if let Some(bundle) = Self::create_bundle(Language::English) {
            bundles.insert(Language::English, bundle);
        }

        Self { bundles }
    }

    fn create_bundle(lang: Language) -> Option<FluentBundle<FluentResource>> {
        let mut bundle = FluentBundle::new(vec![lang.as_langid()]);
        
        let resources = match lang {
            Language::Russian => include_str!("../locales/ru.ftl"),
            Language::English => include_str!("../locales/en.ftl"),
        };

        let resource = FluentResource::try_new(resources.to_string()).ok()?;
        bundle.add_resource(resource).ok()?;
        
        Some(bundle)
    }

    pub fn translate(&self, lang: Language, key: &str) -> String {
        self.bundles
            .get(&lang)
            .and_then(|bundle| {
                let message = bundle.get_message(key)?;
                let pattern = message.value()?;
                let mut errors = Vec::new();
                let value = bundle.format_pattern(pattern, None, &mut errors);
                Some(value.to_string())
            })
            .unwrap_or_else(|| format!("[{}]", key))
    }
}

lazy_static! {
    pub static ref I18N: Localization = Localization::new();
}

// Макрос для удобного использования
#[macro_export]
macro_rules! t {
    ($lang:expr, $key:expr) => {
        $crate::locale::I18N.translate($lang, $key)
    };
}
