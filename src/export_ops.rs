use std::{env, fs};
use std::path::Path;
use std::process::Command;
use crate::config::Config;
use crate::generator::Test;
use crate::tex_ops::{write_questions_into_tex, ExamOptions};

pub fn export_tests(config: &Config, exam_options: &ExamOptions, tests: Vec<Test>) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let out_base_path = current_dir.join(&config.out_dir);

    if !fs::exists(&out_base_path)? {
        fs::create_dir(&out_base_path)?;
    }

    let mut idx = 1;
    for test in tests {
        let tex_code = write_questions_into_tex(&exam_options.tex, &test)?;

        let test_name = format!("test-{idx}");
        let tex_path = out_base_path.join(format!("{test_name}.tex"));
        let pdf_path = out_base_path.join(format!("{test_name}.pdf"));
        let tmp_dir = format!("tmp_build_{}", idx);

        if !fs::exists(&tmp_dir)? {
            fs::create_dir(&tmp_dir)?;
        }

        fs::write(&tex_path, tex_code)?;
        let status = Command::new("pdflatex")
            .arg("-interaction=nonstopmode")
            .arg("-output-directory")
            .arg(&tmp_dir)
            .arg(&tex_path)
            .status()?;

        if status.success() {
            fs::rename(Path::new(&tmp_dir).join(format!("{}.pdf", test_name)), &pdf_path)?;
            fs::remove_dir_all(&tmp_dir)?;
        } else {
            eprintln!("❌ Failed to convert {}", tex_path.display());
        }

        idx += 1;
    }

    Ok({})
}