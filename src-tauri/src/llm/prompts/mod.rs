pub mod title_generation;
pub mod description_optimization;
pub mod card_commentary;
pub mod card_commentary_with_context;
pub mod multiple_cards_commentary;
pub mod discovery_chat;

pub use title_generation::TITLE_GENERATION_PROMPT;
pub use description_optimization::DESCRIPTION_OPTIMIZATION_PROMPT;
pub use card_commentary::CARD_COMMENTARY_PROMPT;
pub use card_commentary_with_context::CARD_COMMENTARY_WITH_CONTEXT_PROMPT;
pub use multiple_cards_commentary::MULTIPLE_CARDS_COMMENTARY_PROMPT;
pub use discovery_chat::DISCOVERY_CHAT_SYSTEM_PROMPT;
