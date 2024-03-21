
// 发给1001 client的所有设备的信息存储在收件箱里
// 每个收件箱里只存储XXX条最新信息,超过的将会被丢掉
// inbox:clientid:deviceid
// inbox:1001
#[allow(dead_code)]
static INBOX_PREFIX:  &str = "inbox:";
