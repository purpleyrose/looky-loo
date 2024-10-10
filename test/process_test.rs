
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let mut process = Process::new(1, 0, 1024);
        assert_eq!(process.get_pid(), 1);
        assert_eq!(process.get_priority(), 0);
        assert_eq!(process.get_memory(), 1024);
        assert_eq!(process.get_state(), ProcessState::Ready);
        process.set_state(ProcessState::Running);
        assert_eq!(process.get_state(), ProcessState::Running);
        assert_geq!(process.get_memory(), 1024);
    }
}