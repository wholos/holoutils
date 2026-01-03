use libc::{sync, reboot, LINUX_REBOOT_CMD_RESTART, LINUX_REBOOT_CMD_POWER_OFF};
fn main(){
    unsafe{
        sync();
        reboot(LINUX_REBOOT_CMD_POWER_OFF);
    }
}