use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder};
use std::ffi::CString;

#[repr(C)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Point {
    pub description: CString,
    pub SKU: CString,
    pub designation: CString,
    pub price: CString,
}

impl Point {
    pub fn new() -> Self {
        Point {
            description: CString::new("").expect("CString::new failed"),
            SKU: CString::new("").expect("CString::new failed"),
            designation: CString::new("").expect("CString::new failed"),
            price: CString::new("").expect("CString::new failed"),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn read_xlsx() -> Vec<Point> {
    let path = format!("{}/../assets/test.xlsx", env!("CARGO_MANIFEST_DIR"));
    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();

    let mut ret = Vec::<Point>::new();
    if let Some(Ok(r)) = workbook.worksheet_range("Sheet1") {
        println!("Number of rows = {:?}", r.rows().count());
        for row in r.rows() {
            let mut record = Point::new();
            record.SKU = CString::new(row[0].to_string()).expect("CString::new failed");
            record.designation = CString::new(row[1].to_string()).expect("CString::new failed");
            record.price = CString::new(row[2].to_string()).expect("CString::new failed");
            ret.push(record);
        }

        println!("{:?}", ret);
    }

    return ret;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        unsafe {
            read_xlsx();
        }
    }
}
