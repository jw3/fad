use fanotify::high_level::*;
use fanotify::high_level::FanEvent::OpenExecPerm;
use fanotify::high_level::FanotifyResponse::Allow;
use fanotify::low_level::{FAN_OPEN_EXEC_PERM, FAN_OPEN_PERM, fanotify_read};
use fanotify::low_level;
use nix::poll::{poll, PollFd, PollFlags};
use sysinfo::{ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

fn main() {
    let mut system = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::everything()));

    let fan = Fanotify::new_with_nonblocking(FanotifyMode::CONTENT);
    fan.add_mountpoint(FAN_OPEN_EXEC_PERM | FAN_OPEN_PERM, "/tmp/foo").unwrap();

    more_magic::initialize();
    println!("polling");

    let mut fds = [PollFd::new(fan.as_raw_fd(), PollFlags::POLLIN)];
    loop {
        let poll_num = poll(&mut fds, -1).unwrap();
        if poll_num > 0 {
            system.refresh_processes();
            for ext in fan.read_events_ext() {
                let event = ext.event;
                let p = system.process(event.pid as i32).unwrap();

                let perm = if event.events.contains(&OpenExecPerm) { "execute" } else { "open" };
                let uid = p.uid;
                let gid = p.gid;
                let exe = p.exe().to_str().unwrap();
                let path = event.path.as_str();
                let mime = ext.ftype;

                // rule=9 dec=allow perm=execute uid=1003 gid=100 pid=5555 exe=/usr/bin/bash : path=/usr/bin/vi ftype=application/x-executable
                println!("rule=? dec=? perm={} uid={} gid={} pid={} exe={} : path={} ftype={}", perm, uid, gid, event.pid, exe, path, mime);

                fan.send_response(event.fd, Allow);
            }
        } else {
            eprintln!("poll_num <= 0!");
            break;
        }
    }

    more_magic::destroy();
}

struct EventExt {
    event: Event,
    ftype: String,
}

trait ExtEvents {
    fn read_events_ext(&self) -> Vec<EventExt>;
}

impl ExtEvents for fanotify::high_level::Fanotify {
    fn read_events_ext(&self) -> Vec<EventExt>  {
        let mut result = Vec::new();
        let events = fanotify_read(self.as_raw_fd());
        for metadata in events {
            let event: Event = metadata.into();
            let ftype = more_magic::get_ftype(event.fd, &event.path);
            result.push(EventExt {
                event,
                ftype,
            });
            low_level::close_fd(metadata.fd);
        }
        result
    }
}

