use std::collections::HashMap;

struct Validator {
    magic: Vec<u8>,
    version: u8,
    s_terminator: u8,
    s_code: u8,
    s_data: u8,
}

impl Validator {
    fn new() -> Validator {

        let magic = from_hex("ef00");
        Validator {
            magic,
            version: 0x01,
            s_terminator: 0x00,
            s_code: 0x01, 
            s_data: 0x02,
        }
    }

    fn is_eof(&self, code: Vec<u8>) -> bool {
        if code.len() >= 2 {
            code[0..2] == self.magic[..]
        } else {
            false
        }
    }

    #[allow(arithmetic_overflow)]
    fn validate_eof(&self, code: Vec<u8>) -> Result<(), &str> {
        // Check version
        if code.len() < 3 || code[2] != self.version {
            return Err("invalid version");
        }

        let mut pos = 3;
        let mut section_sizes: HashMap<u8,u8> = HashMap::new();
        section_sizes.insert(self.s_code, 0);
        section_sizes.insert(self.s_data, 0);

        loop {
            // Terminator not found
            if pos >= code.len() {
                return Err("no section terminator");
            }

            let section_id = code[pos];
            pos += 1;
            if section_id == self.s_terminator {
                break;
            }

            // Disallow unknown sections
            if ! section_sizes.clone().into_keys().collect::<Vec<u8>>().contains(&section_id) {
            //if section_id != self.s_code && section_id != self.s_data {
                return Err("invalid section id");
            }

            // Data section preceding code section
            if section_id == self.s_data && section_sizes[&self.s_code] == 0 {
                return Err("data section preceding code section");
            }

            // Multiple sections with the same id
            if section_sizes[&section_id] != 0 {
                return Err("multiple sections with same id");
            }

            // Truncated section size
            if pos + 1 >= code.len() {
                return Err("truncated section size");
            }

            let inter = ((code[pos] as i32) << (8 as u8)) | code[pos + 1] as i32;
            section_sizes.insert(section_id, inter as u8);
            pos += 2;

            // Empty section
            if section_sizes[&section_id] == 0 {
                return Err("empty section");
            }
        }

        // Code section cannot be absent
        if section_sizes[&self.s_code] == 0 {
            return Err("no code section");
        }

        // The entire container must be scanned
        if code.len() != (pos + section_sizes[&self.s_code] as usize + section_sizes[&self.s_data] as usize) {
            return Err("container size not equal to sum of section sizes");
        }

        return Ok(());
    }

    fn is_valid_container(&self, code: &Vec<u8>) -> bool {
        if self.is_eof(code.to_vec()) {
            return self.validate_eof(code.to_vec()).is_ok()
        }
        return true;
    }
}

fn from_hex(src: &str) -> Vec<u8> {
    let mut byte = String::new();
    let mut result: Vec<u8> = Vec::new();
    for c in src.chars() {
        if c == ' ' {
            continue;
        }
        byte.push(c);
        if byte.len() == 2 {
            let byte_u8 = u8::from_str_radix(&byte, 16).expect("Invalid Hex String");
           result.push(byte_u8);
           byte = String::new();
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_invalid_with_error(code: Vec<u8>, expected_error: &str) {
        let validator = Validator::new();
        let mut err_message: &str = "";

        let result = validator.validate_eof(code);
        let error = result.is_err();
        match result {
            Err(e) => err_message = e,
            Ok(_) => {}
        }
        assert_eq!(error, true);
        assert_eq!(expected_error == err_message, true);
    }

    #[test]
    fn test_legacy_contracts() {
        let validator = Validator::new();

        assert_eq!(validator.is_valid_container(&from_hex("")), true);
        assert_eq!(validator.is_valid_container(&from_hex("00")), true);
        assert_eq!(validator.is_valid_container(&from_hex("ef")), true);
    }

    #[test]
    fn test_no_eof_magic() {
        let validator = Validator::new();
        
        for i in 1..=255 {
            let mut code = vec![0xef];
            code.push(i);
            assert_eq!(validator.is_valid_container(&code), true);
        }
    }

    #[test]
    fn test_eof1_container() {
        is_invalid_with_error(from_hex("ef00"), "invalid version");
        is_invalid_with_error(from_hex("ef0001"), "no section terminator");
        is_invalid_with_error(from_hex("ef0000"), "invalid version");
        is_invalid_with_error(from_hex("ef0002 010001 00 fe"), "invalid version"); // Valid except version
        is_invalid_with_error(from_hex("ef0001 00"), "no code section"); // Only terminator
        is_invalid_with_error(from_hex("ef0001 010001 00 fe aabbccdd"), "container size not equal to sum of section sizes"); // Trailing bytes
        is_invalid_with_error(from_hex("ef000101"), "truncated section size");
        is_invalid_with_error(from_hex("ef000101000102"), "truncated section size");
        is_invalid_with_error(from_hex("ef000103"), "invalid section id");
        is_invalid_with_error(from_hex("ef00010100"), "truncated section size");
        is_invalid_with_error(from_hex("ef00010100010200"), "truncated section size");
        is_invalid_with_error(from_hex("ef000101000000"), "empty section");
        is_invalid_with_error(from_hex("ef000101000102000000fe"), "empty section");
        is_invalid_with_error(from_hex("ef0001010001"), "no section terminator");
        is_invalid_with_error(from_hex("ef000101000100"), "container size not equal to sum of section sizes");  // Missing section contents
        is_invalid_with_error(from_hex("ef000102000100aa"), "data section preceding code section");  // Only data section
        is_invalid_with_error(from_hex("ef000101000101000100fefe"), "multiple sections with same id");  // Multiple code sections
        is_invalid_with_error(from_hex("ef000101000102000102000100feaabb"), "multiple sections with same id");  // Multiple data sections
        is_invalid_with_error(from_hex("ef000101000101000102000102000100fefeaabb"), "multiple sections with same id"); // Multiple code and data sections
        is_invalid_with_error(from_hex("ef000102000101000100aafe"), "data section preceding code section"); 
    }

}
