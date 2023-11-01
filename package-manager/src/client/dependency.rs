// Standard Uses
use std::io::Read;
use std::path::Path;
use std::fmt::Write;

// Crate Uses

// External Uses
use comline::project::ir::frozen::loader as package_loader;
use comline::project::ir::frozen::loader::PackageVersion;

use eyre::{bail, Result};
use reqwest::{
    blocking::{Client, Response},
    StatusCode, header::CONTENT_LENGTH
};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};



pub fn add_remote_dependency(url: String, package_path: &Path) -> Result<()> {
    let mut dependency_data = get_remote_dependency(url)?;
    unpack_package_into_path(&mut dependency_data, package_path)?;


    Ok(())
}

fn get_remote_dependency(url: String) -> Result<Vec<u8>> {
    let client = Client::builder().build()?;
    let mut response = client.get(url).send()?;

    if !response.status().is_success() {
        match response.status() {
            StatusCode::OK => {},
            code => panic!("Could not get package: {}", code)
        }
    }

    let headers = response.headers();
    let content_length = headers.get(CONTENT_LENGTH);
    // let content_type = headers.get(CONTENT_TYPE);

    let Some(len) = content_length else {
        bail!("Package servers are always expected to give len")
    };
    let content_length = len.to_str()?.parse()?;

    download(&mut response, content_length)
}


pub(crate) fn get_local_dependency(path: &Path)
    -> Result<Vec<PackageVersion>>
{
    package_loader::load_package(&path)
}

fn download(response: &mut Response, content_length: usize) -> Result<Vec<u8>> {
    let chunk_size = match content_length {
        0 => content_length / 99,
        _ => 1024usize
    };

    let mut buf = Vec::new();

    let pb = ProgressBar::new(content_length as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"));

    loop {
        let mut buffer = vec![0; chunk_size];
        let count = response.read(&mut buffer[..]).unwrap();
        buffer.truncate(count);

        if buffer.is_empty() { break; }

        buf.extend(buffer.into_boxed_slice().into_vec().iter().cloned());

        pb.inc(count as u64);
    }

    pb.finish_with_message("Finished package data download");

    Ok(buf)
}

#[allow(unused)]
fn unpack_package_into_path(package_data: &[u8], path: &Path) -> Result<()> {
    todo!()
}

#[allow(unused)]
pub fn add_local_dependencies(package_paths: &[&str]) -> Result<()> {
    for package_path in package_paths {
        add_local_dependency(package_path.to_string());
    }

    Ok(())
}

#[allow(unused)]
pub fn add_local_dependency(package_path: String) -> Result<()> {
    todo!()
}


