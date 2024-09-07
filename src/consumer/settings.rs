use std::{path::PathBuf, time::Duration};

const DEFAULT_GRPC_SOCK_PATH: &str = "/tmp/grpc.sock";
const DEFAULT_FDPASS_SOCK_PATH: &str = "/tmp/fdpass.sock";
const DEFAULT_PROCESS_DURATION: Duration = Duration::from_millis(100);
const DEFAULT_RINGBUF_EXPIRE: Duration = Duration::from_secs(10);
const DEFAULT_RINGBUF_CHECK_INTERVAL: Duration = Duration::from_secs(3);

#[derive(Debug, Clone)]
pub struct ConsumerSettings {
    pub(super) grpc_sock_path: PathBuf,
    pub(super) fdpass_sock_path: PathBuf,
    pub(super) process_interval: Duration,
    pub(super) ringbuf_expire: Duration,
    pub(super) ringbuf_check_interval: Duration,
}

#[derive(Default)]
pub struct SettingsBuilder {
    grpc_sock_path: Option<PathBuf>,
    fdpass_sock_path: Option<PathBuf>,
    process_duration: Option<Duration>,
    ringbuf_expire: Option<Duration>,
    ringbuf_check_interval: Option<Duration>,
}

impl SettingsBuilder {
    pub fn new() -> Self {
        SettingsBuilder::default()
    }

    pub fn grpc_sock_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.grpc_sock_path = Some(path.into());
        self
    }

    /// Set the path of the unix socket for passing file descriptor and other things.
    pub fn fdpass_sock_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.fdpass_sock_path = Some(path.into());
        self
    }

    /// Set the process interval.
    pub fn process_interval(mut self, duration: Duration) -> Self {
        self.process_duration = Some(duration);
        self
    }

    /// Set the ringbuf expire duration.
    pub fn ringbuf_expire(mut self, duration: Duration) -> Self {
        self.ringbuf_expire = Some(duration);
        self
    }

    /// Set the ringbuf expire check interval.
    pub fn ringbuf_expire_check_interval(mut self, duration: Duration) -> Self {
        self.ringbuf_check_interval = Some(duration);
        self
    }

    pub fn build(self) -> ConsumerSettings {
        let grpc_sock_path =
            self.grpc_sock_path.unwrap_or(DEFAULT_GRPC_SOCK_PATH.into());

        let fdpass_sock_path = self
            .fdpass_sock_path
            .unwrap_or(DEFAULT_FDPASS_SOCK_PATH.into());

        let process_duration =
            self.process_duration.unwrap_or(DEFAULT_PROCESS_DURATION);

        let ringbuf_expire =
            self.ringbuf_expire.unwrap_or(DEFAULT_RINGBUF_EXPIRE);

        let ringbuf_check_interval = self
            .ringbuf_check_interval
            .unwrap_or(DEFAULT_RINGBUF_CHECK_INTERVAL);

        ConsumerSettings {
            grpc_sock_path,
            fdpass_sock_path,
            process_interval: process_duration,
            ringbuf_expire,
            ringbuf_check_interval,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::consumer::settings::{
        DEFAULT_FDPASS_SOCK_PATH, DEFAULT_GRPC_SOCK_PATH,
        DEFAULT_PROCESS_DURATION, DEFAULT_RINGBUF_CHECK_INTERVAL,
        DEFAULT_RINGBUF_EXPIRE,
    };

    use super::{ConsumerSettings, SettingsBuilder};

    #[test]
    fn test_settings_default() {
        let settings = SettingsBuilder::new().build();

        let ConsumerSettings {
            grpc_sock_path,
            fdpass_sock_path,
            process_interval: process_duration,
            ringbuf_expire,
            ringbuf_check_interval,
        } = settings;

        assert_eq!(grpc_sock_path.as_os_str(), DEFAULT_GRPC_SOCK_PATH);
        assert_eq!(fdpass_sock_path.as_os_str(), DEFAULT_FDPASS_SOCK_PATH);
        assert_eq!(process_duration, DEFAULT_PROCESS_DURATION);
        assert_eq!(ringbuf_expire, DEFAULT_RINGBUF_EXPIRE);
        assert_eq!(ringbuf_check_interval, DEFAULT_RINGBUF_CHECK_INTERVAL);
    }

    #[test]
    fn test_settings() {
        let settings = SettingsBuilder::new()
            .grpc_sock_path("/tmp/grpc_test.sock")
            .fdpass_sock_path("/tmp/fd_test.sock")
            .process_interval(Duration::from_millis(100))
            .ringbuf_expire(Duration::from_secs(20))
            .ringbuf_expire_check_interval(Duration::from_secs(6))
            .build();

        let ConsumerSettings {
            grpc_sock_path,
            fdpass_sock_path,
            process_interval: process_duration,
            ringbuf_expire,
            ringbuf_check_interval,
        } = settings;

        assert_eq!(grpc_sock_path.as_os_str(), "/tmp/grpc_test.sock");
        assert_eq!(fdpass_sock_path.as_os_str(), "/tmp/fd_test.sock");
        assert_eq!(process_duration, Duration::from_millis(100));
        assert_eq!(ringbuf_expire, Duration::from_secs(20));
        assert_eq!(ringbuf_check_interval, Duration::from_secs(6));
    }
}