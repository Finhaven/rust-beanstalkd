extern crate bufstream;

use std::collections::HashMap;
use std::net::TcpStream;
use self::bufstream::BufStream;

use commands;
use error::{BeanstalkdError, BeanstalkdResult};
use parse;
use request::Request;
use response::{Response, Status};

macro_rules! try {
    ($e:expr) => (match $e { Ok(e) => e, Err(_) => return Err(BeanstalkdError::ConnectionError) })
}

pub struct Beanstalkd {
    stream: BufStream<TcpStream>,
}

impl Beanstalkd {
    /// Connect to a running beanstalkd server
    ///
    /// Example: `let mut beanstalkd = Beanstalkd::connect('localhost', 11300).unwrap();`
    pub fn connect(host: &str, port: u16) -> BeanstalkdResult<Beanstalkd> {
        let tcp_stream = try!(TcpStream::connect(&(host, port)));

        Ok(Beanstalkd { stream: BufStream::new(tcp_stream) })
    }

    /// Short hand method to connect to `localhost:11300`
    pub fn localhost() -> BeanstalkdResult<Beanstalkd> {
        Beanstalkd::connect("localhost", 11300)
    }

    /// Change the tube where put new messages (Standard tube is called `default`)
    pub fn tube(&mut self, tube: &str) -> BeanstalkdResult<()> {
        self.cmd(commands::tube(tube)).map(|_| ())
    }

    /// Inserts a job into the client's currently used tube
    pub fn put(&mut self,
               body: &str,
               priority: u32,
               delay: u32,
               ttr: u32)
               -> BeanstalkdResult<u64> {
        self.cmd(commands::put(body, priority, delay, ttr)).map(parse::id)
    }

    /// Get the next message out of the queue
    pub fn reserve(&mut self) -> BeanstalkdResult<(u64, String)> {
        self.cmd(commands::reserve()).map(|r| (parse::id(r.clone()), parse::body(r)))
    }

    /// Get the next message out of the queue with timeout. If the timeout runs out a None is returned
    /// in BeanstalkdResult.
    pub fn reserve_with_timeout(&mut self, timeout: u64) -> BeanstalkdResult<Option<(u64, String)>> {
        self.cmd(commands::reserve_with_timeout(timeout))
            .map(|r| {
                if r.status == Status::TIMED_OUT {
                    None
                } else {
                    Some((parse::id(r.clone()), parse::body(r)))
                }
            })
    } 

    /// Deletes a message out of the queue
    pub fn delete(&mut self, id: u64) -> BeanstalkdResult<()> {
        self.cmd(commands::delete(id)).map(|_| ())
    }

    /// Release a job in the queue
    pub fn release(&mut self, id: u64, priority: u32, delay: u32) -> BeanstalkdResult<()> {
        self.cmd(commands::release(id, priority, delay)).map(|_| ())
    }

    /// Bury a job in the queue
    pub fn bury(&mut self, id: u64, priority: u32) -> BeanstalkdResult<()> {
        self.cmd(commands::bury(id, priority)).map(|_| ())
    }

    /// Touch a job in the queue
    pub fn touch(&mut self, id: u64) -> BeanstalkdResult<()> {
        self.cmd(commands::touch(id)).map(|_| ())
    }

    /// Returns all available stats
    pub fn stats(&mut self) -> BeanstalkdResult<HashMap<String, String>> {
        self.cmd(commands::stats()).map(parse::hashmap)
    }

    /// Returns stats for the specified job
    pub fn stats_job(&mut self, id: u64) -> BeanstalkdResult<HashMap<String, String>> {
        self.cmd(commands::stats_job(id)).map(parse::hashmap)
    }

    /// Add new tube to watch list
    pub fn watch(&mut self, tube: &str) -> BeanstalkdResult<u64> {
        self.cmd(commands::watch(tube)).map(parse::id)
    }

    /// Removes the named tube from the watch list for the current connection
    pub fn ignore(&mut self, tube: &str) -> BeanstalkdResult<Option<u64>> {
        self.cmd(commands::ignore(tube)).map(parse::count)
    }

    /// Peeks the next ready job
    pub fn peek_ready(&mut self) -> BeanstalkdResult<Option<(u64, String)>> {
        self.peek_cmd(commands::peek_ready())
    }

    /// Peeks the next delayed job
    pub fn peek_delayed(&mut self) -> BeanstalkdResult<Option<(u64, String)>> {
        self.peek_cmd(commands::peek_delayed())
    }

    /// Peeks the next buried job
    pub fn peek_buried(&mut self) -> BeanstalkdResult<Option<(u64, String)>> {
        self.peek_cmd(commands::peek_buried())
    }

    /// Delete all the jobs in the ready state
    pub fn delete_all_ready(&mut self) -> BeanstalkdResult<()> {
        self.delete_all_cmd(Self::peek_ready)
    }

    /// Delete all the jobs in the delayed state
    pub fn delete_all_delayed(&mut self) -> BeanstalkdResult<()> {
        self.delete_all_cmd(Self::peek_delayed)
    }

    /// Delete all the jobs in the buried state
    pub fn delete_all_buried(&mut self) -> BeanstalkdResult<()> {
        self.delete_all_cmd(Self::peek_buried)
    }

    /// Delete all jobs (in any state)
    pub fn delete_all(&mut self) -> BeanstalkdResult<()> {
        self.delete_all_ready()?;
        self.delete_all_delayed()?;
        self.delete_all_buried()?;
        Ok(())
    }

    /// Returns:
    /// - Ok(Some(_)) if a job is found
    /// - Ok(None) if no job found
    /// - Err(_) if an error occurred
    fn peek_cmd(&mut self, message: String) -> BeanstalkdResult<Option<(u64, String)>> {
        self.cmd(message)
            .map(|r| {
                if r.status == Status::NOT_FOUND {
                    None
                }
                else {
                    Some((parse::id(r.clone()), parse::body(r)))
                }
            })
    }

    fn delete_all_cmd<PeekFn>(&mut self, peek: PeekFn) -> BeanstalkdResult<()>
        where PeekFn: Fn(&mut Self) -> BeanstalkdResult<Option<(u64, String)>>
    {
        loop {
            match peek(self)? {
                Some((job_id, _)) => self.delete(job_id)?,
                None => return Ok(()),
            }
        }
    }

    fn cmd(&mut self, message: String) -> BeanstalkdResult<Response> {
        let mut request = Request::new(&mut self.stream);

        request.send(message.as_bytes())
    }
}
