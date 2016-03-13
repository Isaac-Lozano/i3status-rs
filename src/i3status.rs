/* TODO: Clean up imports */
use block::Block;
use std::collections::{BinaryHeap, HashMap};
use rustc_serialize::json;
use std::{thread, cmp};
use time;
use std::time as stdtime;
use std::cell::RefCell;
use std::rc::Rc;

/********/
/* Task */
/********/
#[derive(Debug)]
struct Task<'a>
{
    block: Rc<RefCell<&'a mut Block>>,
    nanos: u64,
}

impl<'a> cmp::PartialEq for Task<'a>
{
    fn eq(&self, other: &Task) -> bool
    {
        self.nanos.eq(&other.nanos)
    }
}

impl<'a> cmp::Eq for Task<'a>
{
}

impl<'a> cmp::PartialOrd for Task<'a>
{
    fn partial_cmp(&self, other: &Task) -> Option<cmp::Ordering>
    {
        other.nanos.partial_cmp(&self.nanos)
    }
}

impl<'a> cmp::Ord for Task<'a>
{
    fn cmp(&self, other: &Task) -> cmp::Ordering
    {
        other.nanos.cmp(&self.nanos)
    }
}

/**********/
/* I3Stat */
/**********/
/// A type that manages blocks and outputs valid i3bar data.
#[derive(Debug)]
pub struct I3Status<'a>
{
    schedule: BinaryHeap<Task<'a>>,
    block_map: HashMap<String, Rc<RefCell<&'a mut Block>>>, 
    blocks: Vec<Rc<RefCell<&'a mut Block>>>,
}

impl<'a> I3Status<'a>
{
    /// Constructs a new `I3Status`.
    ///
    /// # Examples
    /// ```
    /// # use i3status::I3Status;
    /// let stat = I3Status::new();
    /// ```
    pub fn new() -> I3Status<'a>
    {
        print!("{{ \"version\": 1 }}[");
        I3Status
        {
            schedule: BinaryHeap::new(),
            block_map: HashMap::new(),
            blocks: Vec::new(),
        }
    }

    /// Adds a block to an `I3Status`.
    ///
    /// The resulting status will be in the order that `Block`s are
    /// added into the `I3Status`.
    pub fn add_block(&mut self, block: &'a mut Block, name: &str)
    {
        let block_cell = Rc::new(RefCell::new(block));
        self.blocks.push(block_cell.clone());
        self.block_map.insert(name.to_string(), block_cell.clone());
    }

    /// Runs an infinite loop, updating and printing out i3bar-compatable json data.
    pub fn run(&mut self)
    {
        /* Insert an update request in for each block */
        for block in self.blocks.iter()
        {
            self.schedule.push(
                Task
                {
                    block: block.clone(),
                    nanos: 0,
                }
            );
        }

        /* run the updaters */
        loop
        {
            /* XXX: Do not unwrap */
            let task = self.schedule.pop().unwrap();

            let now = time::precise_time_ns();
            if task.nanos > now
            {
                thread::sleep(stdtime::Duration::from_millis((task.nanos - now) / 1000000));
            }

            let dur = task.block.borrow_mut().update();
            let nanos = dur.as_secs() * 1000000000 + dur.subsec_nanos() as u64;

            self.schedule.push(
                Task
                {
                    block: task.block.clone(),
                    nanos: time::precise_time_ns() + nanos,
                }
            );

            self.update_status();

        }
    }

    fn update_status(&self)
    {
        print!("[");
        for (idx, block) in self.blocks.iter().enumerate()
        {
            print!("{}", json::encode(&block.borrow_mut().get_status()).unwrap());

            if idx != self.blocks.len()-1
            {
                print!(",");
            }
        }
        println!("],");
    }
}
