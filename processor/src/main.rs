use std::sync::Arc;
use std::thread;
use processor::Processor;

fn main() {
    let processor= Arc::new(Processor::new());

    thread::scope(|s|{

        let processor_t1= processor.clone();
        s.spawn(move ||{
            processor_t1.send("thread 1, send 1");
            processor_t1.send("thread 1, send 2");
            processor_t1.send("thread 1, send 3");
        });


        let processor_t2= processor.clone();
        s.spawn(move ||{
            processor_t2.send("thread 2, send 1");
            processor_t2.send("thread 2, send 2");
            processor_t2.send("thread 2, send 3");
        });


        let processor_t3= processor.clone();
        s.spawn(move ||{
            processor_t3.send("thread 3, send 1");
            processor_t3.send("thread 3, send 2");
            processor_t3.send("thread 3, send 3");
        });


        let processor_t4= processor.clone();
        s.spawn(move ||{
            processor_t4.close();
        });
    });
}
