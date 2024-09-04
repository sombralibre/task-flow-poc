use task_flow::{Conduit, DefaultParser, Step, Task, TaskError, TaskStep};
use tokio::sync::mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::time::Duration;
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};

#[derive(Clone)]
struct SenderWrapper<Chan>(Chan);

impl<Chan> SenderWrapper<Chan> {
    fn new(chan: Chan) -> Self {
        Self(chan)
    }
}

impl Conduit for SenderWrapper<UnboundedSender<usize>> {
    type Item = usize;
    type Error = SendError<Self::Item>;
    type Output = ();

    async fn try_send(&self, msg: Self::Item) -> Result<Self::Output, Self::Error> {
        self.0.send(msg)
    }
}

// TASKS 1
// step
async fn step_task1(
    inbox: Option<usize>,
    sender: Option<SenderWrapper<UnboundedSender<usize>>>,
) -> Result<(), TaskError> {
    if let Some(msg) = inbox {
        if let Some(snd) = sender {
            //println!("step_task1 => Received: {msg}");
            snd.try_send(msg * 2)
                .await
                .map_err(|e| TaskError::Common(format!("{e}")))
        } else {
            Err(TaskError::Common(String::from("re paila!")))
        }
    } else {
        Err(TaskError::Common(String::from("Paila!")))
    }
}

// task
async fn start_task1(
    task: Task<
        usize,
        usize,
        SenderWrapper<UnboundedSender<usize>>,
        (),
        TaskStep<usize, SenderWrapper<UnboundedSender<usize>>, ()>,
    >,
) -> Result<(), TaskError> {
    let Task {
        outbox,
        step,
        timer,
        ..
    } = task;
    //let mut count: usize = 0;
    for x in 1..10000000_usize {
        let oub = outbox.clone();
        let _ = tokio::task::spawn(step.clone().run(Some(x), oub));
        tokio::time::sleep(timer.unwrap()).await;
/*        count += 1;
        if count.eq(&10) {
            count = 0;
            tokio::time::sleep(timer.unwrap()).await;
        }*/
    }
    Ok(())
}

// TASK 2

// step
async fn step_task2(
    inbox: Option<usize>,
    sender: Option<SenderWrapper<UnboundedSender<usize>>>,
) -> Result<(), TaskError> {
    if let Some(msg) = inbox {
        if let Some(snd) = sender {
            //println!("step_task2 => Received: {msg}");
            snd.try_send(msg * 2)
                .await
                .map_err(|e| TaskError::Common(format!("{e}")))
        } else {
            Err(TaskError::Common(String::from("re paila!")))
        }
    } else {
        Err(TaskError::Common(String::from("Paila!")))
    }
}

// task
async fn start_task2(
    task: Task<
        UnboundedReceiver<usize>,
        usize,
        SenderWrapper<UnboundedSender<usize>>,
        (),
        TaskStep<usize, SenderWrapper<UnboundedSender<usize>>, ()>,
    >,
) -> Result<(), TaskError> {
    let Task {
        outbox,
        step,
        inbox,
        ..
    } = task;

    let mut inb = UnboundedReceiverStream::new(inbox.unwrap());

    while let Some(msg) = inb.next().await {
        let oub = outbox.clone();
        let _ = tokio::task::spawn(step.clone().run(Some(msg * 2), oub));
    }

    Ok(())
}

// TASK 3

// step
async fn step_task3(
    inbox: Option<usize>,
    sender: Option<SenderWrapper<UnboundedSender<usize>>>,
) -> Result<(), TaskError> {
    Ok(())
}

// task
async fn start_task3(
    task: Task<
        UnboundedReceiver<usize>,
        usize,
        SenderWrapper<UnboundedSender<usize>>,
        (),
        TaskStep<usize, SenderWrapper<UnboundedSender<usize>>, ()>,
    >,
) -> Result<(), TaskError> {
    let Task {
        
        
        inbox,
        ..
    } = task;

    let mut inb = UnboundedReceiverStream::new(inbox.unwrap());

    let mut counter = 0;

    while let Some(msg) = inb.next().await {
        //let oub = outbox.clone();
        //let _ = tokio::task::spawn(step.clone().run(Some(msg * 2), oub));
        //println!("Worker 3: RCV => {msg}");
        counter += msg;
        //println!("Worker 3: INC VALUE => {counter}");
    }

    println!("Worker 3: FINAL VALUE => {counter}");

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (snd_task1, rcv_task2) = unbounded_channel::<usize>();
    let (snd_task2, rcv_task3) = unbounded_channel::<usize>();

    let task1_step = TaskStep::new(step_task1);
    let task2_step = TaskStep::new(step_task2);
    let task3_step = TaskStep::new(step_task3);

    let task1_outbox = SenderWrapper::new(snd_task1);
    let task2_outbox = SenderWrapper::new(snd_task2);

    let stage_1 = Task::new::<DefaultParser>(
        None,
        Some(task1_outbox),
        task1_step,
        Some(Duration::from_nanos(1)),
    )
    .unwrap();

    let stage_2 =
        Task::new::<DefaultParser>(Some(rcv_task2), Some(task2_outbox), task2_step, None).unwrap();

    let stage_3 = Task::new::<DefaultParser>(Some(rcv_task3), None, task3_step, None).unwrap();

    //tokio::time::sleep(std::time::Duration::from_secs(5) ).await;

    let _worker1 = tokio::task::spawn(Box::new(stage_1).start(start_task1));
    let _worker2 = tokio::task::spawn(Box::new(stage_2).start(start_task2));
    let _worker3 = tokio::task::spawn(Box::new(stage_3).start(start_task3));

    //let _ = tokio::join!(worker1, _worker2, _worker3);

    tokio::time::sleep(std::time::Duration::from_secs(600) ).await;
}
