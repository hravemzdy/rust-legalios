#[cfg(test)]
//#[cfg(test_report)]
pub mod service_examples_tests {
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use rust_decimal::Decimal;
    use rust_decimal::prelude::ToPrimitive;
    use rust_decimal::prelude::FromPrimitive;

    pub fn create_report_file(file_name: &str) -> Option<File> {
        const PARENT_REPORT_FOLDER_NAME: &str = "legalios";
        const REPORT_FOLDER_NAME: &str = "test_values";
        let res_curr_path = std::env::current_dir();
        if res_curr_path.is_err(){
            return None;
        }
        let mut curr_path = res_curr_path.unwrap();
        while !curr_path.ends_with(PARENT_REPORT_FOLDER_NAME) && curr_path.ancestors().count() != 1 {
            curr_path.pop();
        }
        let base_path = curr_path.join(REPORT_FOLDER_NAME);
        let res_norm_path = fs::canonicalize(&base_path);
        if res_norm_path.is_err(){
            return None;
        }
        let norm_path = res_norm_path.unwrap();

        let file_path = norm_path.join(file_name);

        let res_file_handle = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path);
        if res_file_handle.is_err(){
            return None;
        }
        let file_handle = res_file_handle.unwrap();

        Some(file_handle)
    }

    pub fn create_expected_file(file_name: &str) -> Option<File> {
        const PARENT_REPORT_FOLDER_NAME: &str = "legalios";
        const REPORT_FOLDER_NAME: &str = "test_expected";
        let res_curr_path = std::env::current_dir();
        if res_curr_path.is_err(){
            return None;
        }
        let mut curr_path = res_curr_path.unwrap();
        while !curr_path.ends_with(PARENT_REPORT_FOLDER_NAME) && curr_path.ancestors().count() != 1 {
            curr_path.pop();
        }
        let base_path = curr_path.join(REPORT_FOLDER_NAME);
        let res_norm_path = fs::canonicalize(&base_path);
        if res_norm_path.is_err(){
            return None;
        }
        let norm_path = res_norm_path.unwrap();

        let file_path = norm_path.join(file_name);

        let res_file_handle = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path);
        if res_file_handle.is_err(){
            return None;
        }
        let file_handle = res_file_handle.unwrap();

        Some(file_handle)
    }

    pub fn write_report_head(opt_file_handle: &mut Option<File>) {
        if opt_file_handle.is_none(){
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        write!(file_handle, "YEAR").unwrap();
        for month in 1..=12 {
            write!(file_handle, "\t{}", month).unwrap();
        }
        write!(file_handle, "\n").unwrap();
    }

    pub fn write_report_year_head(opt_file_handle: &mut Option<File>, year: i16) {
        if opt_file_handle.is_none(){
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        write!(file_handle, "{}", year).unwrap();
    }

    pub fn write_report_year_int_value(opt_file_handle: &mut Option<File>, value: i32) {
        if opt_file_handle.is_none(){
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        write!(file_handle, "\t{}", value).unwrap();
    }

    pub fn write_report_year_dec_value(opt_file_handle: &mut Option<File>, value: Decimal) {
        let dec_option = value*Decimal::from_i32(100).unwrap();
        let int_option = dec_option.to_i32();
        let int_value: i32 = match int_option {
            Some(value) => value,
            None=> 0i32,
        };
        if opt_file_handle.is_none(){
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        write!(file_handle, "\t{}", int_value).unwrap();
    }

    pub fn write_report_year_ends(opt_file_handle: &mut Option<File>) {
        if opt_file_handle.is_none(){
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        write!(file_handle, "\n").unwrap();
    }

    pub fn close_report_file(opt_file_handle: &mut Option<File>) {
        if opt_file_handle.is_none(){
            return;
        }
        let file_handle: &mut File = opt_file_handle.as_mut().unwrap();
        file_handle.flush().unwrap();
    }
}