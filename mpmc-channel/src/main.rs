use std::sync::Arc;
use std::thread;
use mpmc_channel::{MpMcChannelMine, MpMcChannelProf};

fn main() {
    let channel= Arc::new(MpMcChannelMine::new(4));

    thread::scope(|s|{
        for i in 0..12{
            let channel_clone= channel.clone();
            s.spawn(move ||{
                if i==8{
                    channel_clone.shutdown();
                    println!("Thread {} closed the channel", i);
                }else {
                    if i%2==0{
                        let op= channel_clone.send(i);
                        if op.is_some(){
                            println!("Thread {} sent message {}", i, i);
                        }else{
                            println!("Thread {}: it tried to send, but the channel is closed", i);
                        }
                    }else{
                        let value= channel_clone.recv();
                        if value.is_some(){
                            println!("Thread {} received message {}", i , value.unwrap());
                        }else{
                            println!("Thread {}: it tried to receive, but the channel is closed and there is nothing more to read", i);
                        }
                    }
                }
            });
        }
    });
}
