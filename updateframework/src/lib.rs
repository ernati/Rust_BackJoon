/// 두 정수를 더한 값을 반환합니다.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::path::Path;

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
        let path = "/root/test_lib_rs_sudo";
        // sudo를 통해 파일 생성 요청 (비밀번호 입력 프롬프트 발생)
        let status = Command::new("sudo")
            .arg("touch")
            .arg(path)
            .status()
            .expect("failed to execute sudo touch");
        assert!(status.success(), "sudo touch failed with status: {:?}", status);

        // 파일이 정상적으로 생성되었는지 확인
        assert!(Path::new(path).exists(), "파일이 생성되지 않았습니다");

        // sudo를 통해 파일 삭제
        let status = Command::new("sudo")
            .arg("rm")
            .arg(path)
            .status()
            .expect("failed to execute sudo rm");
        assert!(status.success(), "sudo rm failed with status: {:?}", status);
    }
}
