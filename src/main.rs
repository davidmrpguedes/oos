

extern crate simplelog;

use std::fs::File;
// multi-thread()
use std::thread;
use std::time::Duration;
// import gerais
use std::io;
use std::fs::read;
use std::io::{Read, stdin};
use std::env;
use std::fmt::format;
use std::net::ToSocketAddrs;
use std::process::exit;
use rand::distributions::Standard;
use crate::p2pkademlia::Knode;

// submodulos de main.rs / CRATE:
mod grpc;
mod auction_app;
mod distributed_ledger;
mod p2pkademlia;

// grpc module
mod hello;





fn main() {

    // Initial Vars
    let args: Vec<String> = env::args().collect();
    let arg_consola = &args[1];
    let arg_ip_address = &args[2];
    let arg_port_number = &args[3];
    loginit(format!("g1p2p-{arg_ip_address}-{arg_port_number}.log"));

    // Iniciatilize node (not yet in the p2p network)

    let mynode = p2pkademlia::Knode::new(arg_ip_address.to_string(), arg_port_number.to_string());

    // mostra GUI do nodo (se for pedido)
    consola(&mynode, arg_consola, &arg_ip_address.to_string(), &arg_port_number.to_string());

    // threat do nodo que recebe requests dos outros nodos
    kademlia_request_deamon(&mynode,arg_ip_address.to_string(), arg_port_number.to_string() );

    loop {
        // println!("Thread principal - iteration: trata os pedidos da queue ");
        Dispach_request_from_queue(&mynode);
    }

}

// LOG

fn loga(ipaddr: &String, port: &String, mensagem: &String) {


}

fn loginit(nome_fich: String) {

}




// GUI

fn consola(mynode: &p2pkademlia::Knode, arg_consola: &String, ip: &String, port: &String) {

    match arg_consola.as_str() {
        ("consola") => unsafe {
            menu_consola(mynode,ip, port);
        },
        "batch" => {
            // nao faz nada em termos de GUI
        },
        _ => {
            println!("Opcao consola ou batch é obrigatoria antes do IP e Porta. Execucao nao submetida.");
            panic!("argumento invalido.")
        },
    }
}

unsafe fn menu_consola(mynode: &p2pkademlia::Knode, ip: &String, port: &String) {
       let mut done = false;
       let mut option = String::new();
       let stdin = io::stdin();

       while done == false {
           println!(
               "Introduz opçao pretendida:\n\n\n \
               1- Entrar na rede P2P, \n \
               2- Sair da rede P2P \n \
               7- Testar ligacao com outro node \n");

           option = String::default();
           stdin.read_line(&mut option);
           println!("{}", option);

           match option.as_str().trim() {

               "1"=> kademlia_enter(&mynode),
               "2" => kademlia_exit(&mynode),
               "3" => kademlia_print(&mynode),
               "7" => kademlia_grpctest(&mynode),
               _ => {}
           }
       }

   }


// Kademlia console ops

fn kademlia_print(mynode: &p2pkademlia::Knode) {

}

fn kademlia_enter(mynode: &p2pkademlia::Knode) {

    let mut ip_boot: String = "".to_string();
    let mut port_boot: String = "".to_string();
    
    stdin().read_line(&mut ip_boot).expect("Erro leitura ip bootstrap.");
    stdin().read_line(&mut port_boot).expect("Erro leitura port bootstrap.");
    
    p2pkademlia::Knode::bootstrap(mynode, ip_boot, port_boot).expect("Insucesso ao entrar na rede P2P.");


}

fn kademlia_exit(mynode: &p2pkademlia::Knode) {

}

unsafe fn kademlia_grpctest(mynode: &p2pkademlia::Knode) {
    
    let mut ip_add1: String = "".to_string();
    let mut ip_port1: String = "".to_string();

    // lê qual o nodo pretendido:
    stdin().read_line(&mut ip_add1);
    stdin().read_line(&mut ip_port1);

    ip_add1 = ip_add1.trim().parse().unwrap();
    ip_port1 = ip_port1.trim().parse().unwrap();

    grpc::snd_request(ip_add1, ip_port1);
        
}


// Kademlia Request Thead

pub fn auction_daemons_launch() {

    // create a thread
    thread::spawn(|| {

        // everything in here runs
        // in its own separate thread
        loop {
            println!("thread 2 SPAWN SECUNDARIA iteration");
            thread::sleep(Duration::from_millis(500));
        }
    });


}

fn kademlia_request_deamon(mynode: &Knode, ip: String, port: String) {

    let ip_thread: String = ip;
    let port_thread: String = port;

    println!("vou entrar no daemon_start.");

    // create a thread for request receiving
    let handle = thread::spawn(  move || {

        // everything in here runs in its own separate thread

        println!("estou na thread!!!");

        // lança a thead com o listener que fica a receber pedidos dos outros nodes
        let result = grpc::server_init(ip_thread, port_thread);
        match result {
            Ok(_) => println!("thread- server de grpc esta ok."),
            Err(e) => println!("Problem creating the server - channel: {:?}", e),
            }
    });

    handle.join().expect("The thread panicked lourenco");

    println!("passei o join handle da thread.")

}

fn Dispach_request_from_queue(mynode: &Knode)
{
    thread::sleep(Duration::from_millis(1500));
}

