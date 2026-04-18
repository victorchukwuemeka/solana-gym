use dummy_client::gossip::GossipService;
//use crate::gossip::GossipService;
use std::thread;
use std::time::Duration;


fn main()-> anyhow::Result<()>{

    let mut node_a = GossipService::new(
         "alice".to_string(),
         "127.0.0.1:9000",
         2
    )?;

    let mut node_b = GossipService::new(
         "bob".to_string(),
         "127.0.0.1:9001",
         2
    )?;

    let mut node_c = GossipService::new(
         "charlie".to_string(),
         "127.0.0.1:9002",
         2
    )?;

    node_a.add_seed("bob".to_string(), "127.0.0.1:9001".parse()?);
    node_a.add_seed("charlie".to_string(), "127.0.0.1:9002".parse()?);

    node_b.add_seed("alice".to_string(), "127.0.0.1:9000".parse()?);
    node_b.add_seed("charlie".to_string(), "127.0.0.1:9002".parse()?);

    node_c.add_seed("alice".to_string(), "127.0.0.1:9000".parse()?);
    node_c.add_seed("bob".to_string(), "127.0.0.1:9001".parse()?);

    node_a.set_data("temperature".to_string(), b"25C".to_vec());



    println!("Starting gossip network...\n");

    
    let handle_a = thread::spawn(move || {
        node_a.start();
    });

    let handle_b = thread::spawn(move || {
        node_b.start();
    });

    let handle_c = thread::spawn(move || {
        node_c.start();
    });

    thread::sleep(Duration::from_secs(10));

    println!("Gossip test complete!");

    Ok(())



}