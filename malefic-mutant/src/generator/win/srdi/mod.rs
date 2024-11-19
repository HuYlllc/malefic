use link_srdi::link_shellcode_rdi_from_bytes;

use crate::GenerateArch;

pub mod shellcode;
pub mod link_srdi;

pub fn link_srdi_generator(
    src_path: &str,
    arch: GenerateArch,
    target_path: &str,
    function_name: &String,
    user_data: &String
) -> anyhow::Result<()> {
    println!("[+] Src PE file is {}", src_path);
    let src_path = std::path::Path::new(src_path);
    if !src_path.exists() {
        anyhow::bail!("src_path does not exist.");
    }
    let dll_bytes = std::fs::read(src_path)?;
    let data = link_shellcode_rdi_from_bytes(
        &arch, &dll_bytes, function_name, user_data);
    if data.is_empty() {
        anyhow::bail!("Failed to link shellcode.");
    }
    let target_path = std::path::Path::new(target_path);
    std::fs::write(target_path, &data)?;
    println!("[+] Successfully linked shellcode to {}", target_path.display());

    Ok(())
}