# zfs_snap_control
Create ZFS snapshots on a schedule and automatically delete old ones

## Examples

For the application to work properly, configure its launch at a convenient time for you through the task scheduler. I'm using cron:
```
$ sudo crontab -e 
```

We will enter the convenient time for creating a snapshot of the state and the command to call zfs_snap_control with the pool.
```
0 0 * * * zfs_snap_control -p rpool/ROOT/ubuntu -d 20
0 0 * * * zfs_snap_control -p rpool/ROOT/home -d 10
0 0 * * * zfs_snap_control -p rpool/ROOT/var/mail -d 5
```

It's done, now you can forget about creating snapshots!!!

## For restore from snapshot

1. You must select a snapshot from command output:
```
zfs list -t snapshot -r rpool
```

2. And call zfs rollback
```
 zfs rollback rpool/ROOT/ubuntu@gitlab1 && systemctl reboot
```

3. Enjoy!