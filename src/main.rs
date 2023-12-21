use rust_bert::pipelines::translation::{Language, TranslationModelBuilder
};
use anyhow;

fn main() -> anyhow::Result<()> {
    let model = TranslationModelBuilder::new()
        .with_source_languages(vec![Language::English])
        .with_target_languages(vec![Language::French])
        .create_model()?;
    let input_text = "Clara is a pretty little girl.";
    let output = model.translate(&[input_text], None, Language::French)?;
    for sentence in output {
        println!("{}", sentence);
    }
    Ok(())       
}