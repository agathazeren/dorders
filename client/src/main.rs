#[macro_use]
extern crate stdweb;


use stdweb::web::event::ChangeEvent;
use stdweb::web::event::ClickEvent;
use stdweb::web::document;
use stdweb::web::IParentNode;
use stdweb::web::IEventTarget;
use stdweb::web::html_element::TextAreaElement;
use stdweb::unstable::TryInto;
use dcore::order::Order;
use std::fmt::Write;
use blake2::{Blake2s,Digest};
use std::str::FromStr;
use dcore::order;
use stdweb::web::IElement;
use stdweb::web::INode;
use hex;
use rand::thread_rng;
use rand::Rng;



fn init_ui(){
    document().query_selector("#proccess").unwrap().unwrap()
        .add_event_listener(move |_:ClickEvent| {
            proccess();
        });
    document().query_selector("#hash-cmp").unwrap().unwrap()
        .add_event_listener(move |_:ChangeEvent| {
            update_hash_cmp();
        });
    document().query_selector("#salt-gen").unwrap().unwrap()
        .add_event_listener(move |_:ClickEvent|{
            generate_new_salt();
        });
    update_hash_cmp();
}
                            
fn proccess(){
    clear_parse_errors();
    clear_salt_error();
    
    let orders_el:TextAreaElement = document().query_selector("#orders-raw").unwrap().unwrap().try_into().unwrap();
    let orders_value = orders_el.value();
    let orders_res = orders_value.lines()
        .map(Order::from_str);
    let mut orders = Vec::new();
    for order_res in orders_res{
        match order_res {
            Ok(o) => orders.push(o),
            Err(e) => emit_parse_error(e),
        }
    }
    let orders = orders;
    let mut orders_out = String::new();
    let mut hasher = Blake2s::new();
    for order in orders {
        writeln!(orders_out,"{}",order.to_string()).unwrap();
        hasher.input(order.to_bytes());
    }
    let salt_el: TextAreaElement = document().query_selector("#salt").unwrap().unwrap().try_into().unwrap();
    hasher.input(hex::decode(salt_el.value()).unwrap_or_else(|_|{emit_salt_error();Vec::new()}));
    let orders_out = orders_out;
    let orders_out_el: TextAreaElement = document().query_selector("#orders-parsed").unwrap().unwrap().try_into().unwrap();
    orders_out_el.set_value(&orders_out);
    let hash = hasher.result();
    let hash_out_el: TextAreaElement = document().query_selector("#hash").unwrap().unwrap().try_into().unwrap();
    hash_out_el.set_value(&format!("{:x}",hash));

    update_hash_cmp();
}

fn update_hash_cmp(){
    let hash: TextAreaElement = document().query_selector("#hash").unwrap().unwrap().try_into().unwrap();
    let hash_cmp: TextAreaElement = document().query_selector("#hash-cmp").unwrap().unwrap().try_into().unwrap();
    let hash_match = hash.value() == hash_cmp.value();
    document().query_selector("#hash-cmp-match").unwrap().unwrap().set_attribute("style",if hash_match {"display:initial"} else {"display:none"}).unwrap();
    document().query_selector("#hash-cmp-differ").unwrap().unwrap().set_attribute("style",if hash_match {"display:none"} else {"display:initial"}).unwrap();
}

fn emit_parse_error(e: order::ParseError){
    let el = document().query_selector("#parse-err").unwrap().unwrap();
    let p = document().create_element("p").unwrap();
    p.append_child(&document().create_text_node(&e.to_string()));
    el.append_child(&p);
}

fn clear_parse_errors(){
    let el = document().query_selector("#parse-err").unwrap().unwrap();
    while let Some(child) = el.first_child(){
        el.remove_child(&child).unwrap();
    }
}

fn emit_salt_error(){
    let el = document().query_selector("#salt-err").unwrap().unwrap();
    el.set_attribute("style","display:initial").unwrap();
}

fn clear_salt_error(){
    let el = document().query_selector("#salt-err").unwrap().unwrap();
    el.set_attribute("style","display:none").unwrap();
}

fn generate_new_salt(){
    let mut salt = [0u8;32];
    thread_rng().fill(&mut salt);
    let el: TextAreaElement = document().query_selector("#salt").unwrap().unwrap().try_into().unwrap();
    el.set_value(&hex::encode(salt));
}
    

fn main(){
    init_ui();
}


    
