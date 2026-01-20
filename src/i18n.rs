use fluent::{FluentBundle, FluentResource};
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "locales"]
pub struct Locales;

lazy_static::lazy_static! {
    static ref BUNDLES: HashMap<String, FluentBundle<FluentResource>> = {
        let mut bundles = HashMap::new();
        
        for lang in &["en", "pt-BR", "es"] {
            if let Ok(ftl_string) = Locales::get(&format!("{}/main.ftl", lang)) {
                if let Ok(ftl_str) = std::str::from_utf8(&ftl_string.data) {
                    if let Ok(resource) = FluentResource::try_new(ftl_str.to_string()) {
                        let lang_id: LanguageIdentifier = lang.parse()
                            .unwrap_or_else(|_| "en".parse().unwrap());
                        let mut bundle = FluentBundle::new(vec![lang_id]);
                        let _ = bundle.add_resource(resource);
                        bundles.insert(lang.to_string(), bundle);
                    }
                }
            }
        }
        
        bundles
    };
}

/// Get a localized string
pub fn get_string(lang: &str, key: &str, args: Option<&HashMap<String, String>>) -> String {
    let bundle = BUNDLES.get(lang).or_else(|| BUNDLES.get("en"));
    
    if let Some(bundle) = bundle {
        match bundle.get_message(key) {
            Some(message) => {
                if let Some(pattern) = message.value() {
                    let mut errors = vec![];
                    let value = bundle.format_pattern(
                        pattern,
                        args.map(|a| {
                            a.iter()
                                .map(|(k, v)| {
                                    (k.as_str(), fluent::FluentValue::from(v.clone()))
                                })
                                .collect::<HashMap<_, _>>()
                        })
                        .as_ref(),
                        &mut errors,
                    );
                    value.to_string()
                } else {
                    key.to_string()
                }
            }
            None => key.to_string(),
        }
    } else {
        key.to_string()
    }
}

/// Helper to get string without arguments
pub fn t(lang: &str, key: &str) -> String {
    get_string(lang, key, None)
}

/// Helper to get string with arguments
pub fn t_with_args(lang: &str, key: &str, args: &HashMap<String, String>) -> String {
    get_string(lang, key, Some(args))
}

/// Detect system language
pub fn detect_system_language() -> String {
    #[cfg(target_os = "windows")]
    {
        // Try to get Windows locale
        if let Ok(output) = std::process::Command::new("powershell")
            .args(&["-Command", "Get-Culture | Select-Object -ExpandProperty Name"])
            .output()
        {
            if let Ok(locale) = String::from_utf8(output.stdout) {
                let locale = locale.trim();
                if locale.starts_with("pt-BR") {
                    return "pt-BR".to_string();
                } else if locale.starts_with("pt") {
                    return "pt-BR".to_string();
                } else if locale.starts_with("es") {
                    return "es".to_string();
                }
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        if let Ok(lang) = std::env::var("LANG") {
            if lang.contains("pt_BR") {
                return "pt-BR".to_string();
            } else if lang.contains("pt") {
                return "pt-BR".to_string();
            } else if lang.contains("es") {
                return "es".to_string();
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("defaults")
            .args(&["read", "-g", "AppleLanguages"])
            .output()
        {
            if let Ok(result) = String::from_utf8(output.stdout) {
                if result.contains("pt_BR") || result.contains("pt-BR") {
                    return "pt-BR".to_string();
                } else if result.contains("pt") {
                    return "pt-BR".to_string();
                } else if result.contains("es") {
                    return "es".to_string();
                }
            }
        }
    }
    
    "en".to_string()
}
