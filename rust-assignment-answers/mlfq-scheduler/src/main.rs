mod mlfq;

use mlfq::{MLFQ, Process};

fn main() {
    println!("MLFQ Scheduler Implementation");
    // You can add any demonstration code here if you wish
    // added code using id and all functions created to remove any warnings
    let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);

    //creating Processes
    let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
    let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
    let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };
    
    //using id
    println!("process 2 id is: {}", process2.id);

    //adding processes
    mlfq.add_process(process1);
    mlfq.add_process(process2);
    mlfq.add_process(process3);

    //"executing" process
    mlfq.execute_process(0);

    //updating time to 100
    mlfq.update_time(100);
    
    //boosting all processes
    mlfq.priority_boost();
}