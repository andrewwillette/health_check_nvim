use nvim_oxi as oxi;

#[oxi::module]
fn health_check_nvim() -> oxi::Result<i32> {
    Ok(42)
}
