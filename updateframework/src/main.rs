/// 두 정수를 더한 값을 반환합니다.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 0), 0);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_create_file_in_root_dir_requires_sudo() {
        // 이 테스트는 Linux의 /root 디렉토리에 파일을 생성해야 하므로 sudo 권한이 필요합니다.
        let path = "/root/test_lib_rs_sudo";
        let result = File::create(path);
        assert!(
            result.is_ok(),
            "이 테스트는 sudo 권한이 필요합니다: 파일 생성 실패: {:?}",
            result.err()
        );
        // 성공한 경우 파일을 삭제합니다.
        let _ = std::fs::remove_file(path);
    }
}
