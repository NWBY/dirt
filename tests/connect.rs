use dirt::commands::connect::test_connection;
use dirt::utils::ssh::SshRunner;
use mockall::{mock, predicate::*};
use ssh2::Session;
use std::error::Error;

mock! {
    SshRunner {}
    impl SshRunner for SshRunner {
        fn run_command(&self, session: &Session, command: &str) -> Result<(), Box<dyn Error>>;
        fn run_command_with_output(&self, session: &Session, command: &str) -> Result<String, Box<dyn Error>>;
    }
}

mock! {
    pub Session {}
    impl Clone for Session {
        fn clone(&self) -> Self;
    }
}

#[test]
fn test_connection_command() {
    let mut mock_ssh_runner = MockSshRunner::new();
    let mock_session = Session::new().unwrap();

    mock_ssh_runner
        .expect_run_command_with_output()
        .with(
            mockall::predicate::always(), // We can't easily match on mock_session, so we accept any session
            eq("echo 'SSH connection test successful'"),
        )
        .times(1)
        .returning(|_, _| Ok(String::from("SSH connection test successful")));

    let result = test_connection(&mock_ssh_runner, &mock_session);

    assert!(result.is_ok());
}
