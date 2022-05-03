#[allow(dead_code)]
mod use_thread {
    use std::cell::RefCell;
    use std::ops::Sub;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{mpsc, Arc, Barrier, Condvar, Mutex, Once, RwLock};
    use std::thread;
    use std::thread::{sleep, JoinHandle};
    use std::time::{Duration, Instant};

    fn start() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                sleep(Duration::from_millis(1));
            }
        });
        handle.join().unwrap();
        for i in 1..3 {
            println!("hi number {} from the main thread!", i);
            sleep(Duration::from_millis(1));
        }

        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here is a vector: {:?}", v);
        });
        handle.join().unwrap();
        // print!("{:?}", v);//error. v has been moved into the new thread

        let new_thread = thread::spawn(move || {
            thread::spawn(move || loop {
                println!("I am a new thread.");
            })
        });
        new_thread.join().unwrap();
        println!("child thread is finished!");
        sleep(Duration::from_millis(2));
    }
    fn try_barrier() {
        let mut handles = Vec::with_capacity(6);
        let barrier = Arc::new(Barrier::new(6));
        for code in 0..6 {
            let b = barrier.clone();
            handles.push(thread::spawn(move || {
                println!("before wait {}", code);
                b.wait();
                println!("after wait {}", code);
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
    fn thread_local_variable() {
        thread_local! (static FOO: RefCell<u32> = RefCell::new(1));

        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 2;
        });

        let t = thread::spawn(move || {
            FOO.with(|f| {
                assert_eq!(*f.borrow(), 1);
                *f.borrow_mut() = 3;
            });
        });
        t.join().unwrap();
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 2);
        });

        struct Bar;
        impl Bar {
            thread_local! {
                static FOO: RefCell<usize> = RefCell::new(1);
            }
        }
        Bar::FOO.with(|x| println!("{:?}", x));
    }

    fn cond_var() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = pair.clone();

        thread::spawn(move || {
            println!("do some work ...");
            sleep(Duration::from_millis(10));
            println!("finish work");
            let (ref lock, ref cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
            println!("child thread {}", *started);
        });

        let (ref lock, ref cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        println!("before wait {}", *started);
        if !*started {
            println!("waiting");
            started = cvar.wait(started).unwrap();
            println!("waiting end");
        }
        println!("after wait {}", *started);
    }
    fn exec_once() {
        static mut VAL: usize = 0;
        static INIT: Once = Once::new();

        let handle1 = thread::spawn(move || {
            INIT.call_once(|| unsafe {
                VAL = 1;
            });
        });

        let handle2 = thread::spawn(move || {
            INIT.call_once(|| unsafe {
                VAL = 2;
            });
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
        println!("{}", unsafe { VAL }); //VAL may be 1 or 2.it is decided by the first thread which run INIT
    }

    fn send_msg_thread() {
        fn use_mpsc() {
            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                sleep(Duration::from_millis(10));
                tx.send(1).unwrap();
                // tx.send((Some(1))).unwrap()//error. tx is Sender<i32> not Sender<Option<i32>>
            });
            println!("receive {}", rx.recv().unwrap());

            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                sleep(Duration::from_millis(10));
                tx.send(1).unwrap();
            });
            println!("try to receive: {:?}", rx.try_recv());

            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                let s = String::from("go");
                tx.send(s).unwrap();
                // println!("val is {}", s);//error.s has been moved
            });
            let received = rx.recv().unwrap();
            println!("Got: {}", received);

            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    sleep(Duration::from_millis(10));
                }
            });

            for received in rx {
                println!("Got: {}", received);
            }
            println!();
            multi_senders();
            //
            println!();
            sync_chan();
            //
            println!();
            send_multi_type_data();
        }

        fn multi_senders() {
            let (tx, rx) = mpsc::channel();
            let tx1 = tx.clone();
            thread::spawn(move || {
                tx.send(String::from("hi from raw tx")).unwrap();
            });
            thread::spawn(move || {
                tx1.send(String::from("hi from cloned tx")).unwrap();
            });
            for received in rx {
                println!("Got: {}", received);
            }
        }

        fn sync_chan() {
            fn try_with_usize(n: usize) {
                println!("sync_channel({}):", n);
                let (tx, rx) = mpsc::sync_channel(n);
                let handle = thread::spawn(move || {
                    println!("son: before sending");
                    tx.send(1).unwrap();
                    println!("son: after sending");
                });
                println!("main: before sleeping");
                sleep(Duration::from_millis(10));
                println!("main: after sleeping");
                println!("main: received: {}", rx.recv().unwrap());
                handle.join().unwrap();
                println!();
            }
            for i in 0..2 {
                try_with_usize(i);
            }
        }

        fn send_multi_type_data() {
            enum Fruit {
                Apple(u8),
                Orange(String),
            }
            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                tx.send(Fruit::Orange("sweet".to_string())).unwrap();
                tx.send(Fruit::Apple(1)).unwrap();
            });
            for x in rx {
                match x {
                    Fruit::Apple(cnt) => println!("received {} apples", cnt),
                    Fruit::Orange(s) => println!("received {} oranges", s),
                }
            }
            println!("ok");
        }

        fn trap() {
            let (send, recv) = mpsc::channel();
            let num_threads = 3;
            for i in 0..num_threads {
                let thread_send = send.clone();
                thread::spawn(move || {
                    thread_send.send(i).unwrap();
                    println!("thread {:?} finished", i);
                });
            }
            drop(send); //if we do not drop it here.
                        // the following loop will not stop
                        // because recv is not drop totally.
                        // only recv.clone(s) were dropped
            for x in recv {
                println!("Got: {}", x);
            }
            println!("finished iterating");
        }

        use_mpsc(); //mpsc::channel is async
        println!("===========");
        trap();
    }
    fn all_kinds_locks() {
        fn single_thread_mutex() {
            let m = Mutex::new(1);
            {
                let mut num = m.lock().unwrap();
                *num = 2;
                //num (lock) is dropped and calls unlock() automatically
            }
            println!("m = {:#?}", m);
        }

        fn multi_threads_mutex() {
            let counter = Arc::new(Mutex::new(0));
            let mut handles = vec![];

            for _ in 0..10 {
                let counter = Arc::clone(&counter);
                let handle = thread::spawn(move || {
                    sleep(Duration::from_millis(1));
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
            println!("{}", *counter.lock().unwrap());
        }

        fn read_write_lock() {
            let lock = RwLock::new(1);
            {
                let r1 = lock.read().unwrap();
                let r2 = lock.read().unwrap();
                assert_eq!(*r1, 1);
                assert_eq!(*r2, 1);
            }
            {
                let mut w = lock.write().unwrap();
                *w += 1;
                assert_eq!(*w, 2);

                // let r1 = lock.read().unwrap(); //error
                // print!("{:?}", r1);
                //w is dropped here. we can't read before write_lock is unlocked
            }
        }

        fn condavar() {
            let flag = Arc::new(Mutex::new(false));
            let cond = Arc::new(Condvar::new());
            let cflag = flag.clone();
            let ccond = cond.clone();

            let handle = thread::spawn(move || {
                let mut m = *cflag.lock().unwrap();
                let mut cnt = 0;

                while cnt < 3 {
                    while !m {
                        m = *ccond.wait(cflag.lock().unwrap()).unwrap();
                    }
                    {
                        m = false;
                        *cflag.lock().unwrap() = false;
                    }
                    cnt += 1;
                    println!("inner cnt: {}", cnt);
                }
            });

            let mut cnt = 0;
            loop {
                sleep(Duration::from_millis(1));
                *flag.lock().unwrap() = true;
                cnt += 1;
                if cnt > 3 {
                    break;
                }
                println!("outside cnt: {}", cnt);
                cond.notify_one();
            }
            handle.join().unwrap();
            println!("{:?}", flag);
        }

        condavar();
        println!("=========");
        single_thread_mutex();
        println!("=========");
        multi_threads_mutex();
        println!("=========");
        read_write_lock();
        //
        // {
        //     let mut handles = vec![];
        //     for i in 0..2 {
        //         let handle = thread::spawn(move || {
        //             println!("running thead {}...", i);
        //             if i == 0 {
        //                 thread::sleep(Duration::from_millis(1000));
        //             }
        //             println!("thread {} finished.", i);
        //         });
        //         handles.push(handle);
        //     }
        //     for (i, handle) in handles.into_iter().enumerate() {
        //         println!("join {}", i);
        //         handle.join().unwrap();
        //     }
        // }
    }

    fn atomic() {
        const N_TIMES: u64 = 100;
        const N_THREADS: usize = 10;

        static R: AtomicU64 = AtomicU64::new(0);

        fn add_n_times(n: u64) -> JoinHandle<()> {
            thread::spawn(move || {
                for _ in 0..n {
                    R.fetch_add(1, Ordering::Relaxed);
                }
            })
        }

        let start_time = Instant::now();
        let mut threads = Vec::with_capacity(N_THREADS);
        for _ in 0..N_THREADS {
            threads.push(add_n_times(N_TIMES));
        }
        for thread in threads {
            thread.join().unwrap();
        }
        assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
        println!("{:?}", Instant::now().sub(start_time));
    }
    pub fn test() {
        // start();
        //try_barrier();
        atomic();
        println!("------------------------");
        all_kinds_locks();
        println!("------------------------");
        thread_local_variable();
        println!("------------------------");
        cond_var();
        println!("------------------------");
        exec_once();
        println!("------------------------");
        send_msg_thread();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use_thread::test();
    }
}
