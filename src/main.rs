fn main() 
{   
    let user_input: Vec<u8> = std::env::args()
        .nth(1)
        .expect("No Provided input")
        .split(['/','.'])
        .map(|x| x
            .parse::<u8>()
            .expect("Invalid octet value")
        ).collect();

    let ip: u32 = u32::from_be_bytes(user_input[..4]
        .try_into()
        .expect("IP must have exactly 4 bytes"));

    let cidr: u32 = match user_input.len() == 5 {
        true => 
        {
            assert!((1..=32).contains(&user_input[4]), "CIDR must be between 1 and 32");
            (!0u32) << (32 - user_input[4])
        }
        
        false => 
        {
            assert!(user_input.len() == 4, "Too few arguments");
            match user_input[0]
            {
                0..=127 => (!0u32) << 24,
                128..=191 => (!0u32) << 16,
                192..=223 => (!0u32) << 8,
                _ => !0u32
            }
        }
    };

    println!("--------------------------------------------------");
    println!("IP ADDRESS: {}", u32_to_dotted(ip));
    println!("SUBNET MASK: {}", u32_to_dotted(cidr));
    println!("WILDCARD MASK: {}", u32_to_dotted(!cidr));
    println!("SUBNET ADDRESS: {}", u32_to_dotted(ip & cidr));
    println!("BROADCAST ADDRESS: {}", u32_to_dotted((ip & cidr) | (!cidr)));
    
    println!("First usable host: {}", u32_to_dotted
        (
            match cidr.count_zeros()
            {
                0..=1 => ip & cidr,
                _ => (ip & cidr) + 1
            }
        ));

    println!("Last usable host: {}", u32_to_dotted(
        match cidr.count_zeros()
        {
            0..=1 => (ip & cidr) | (!cidr),
            _ => ((ip & cidr) | (!cidr)) - 1
        })
    );

    println!("IP CLASS: {}", match user_input[0]
    {
        0..=127 => "A",
        128..=191 => "B",
        192..=223 => "C",
        224..=239 => "D",
        _ => "E"
    });

    println!("Number of hosts: {}", match cidr.count_zeros()
    {
        0..=1 => 0u32,
        _ => {1u32 << cidr.count_zeros()}
    });

    println!("Number of usable hosts: {}",match cidr.count_zeros()
    {
        0..=1 => 0u32,
        _ => {(1u32 << cidr.count_zeros()).saturating_sub(2)}
    });

    println!("--------------------------------------------------");
}

fn u32_to_dotted (u: u32) -> String
{
    let b = u.to_be_bytes();
    format!("{}.{}.{}.{}", b[0], b[1], b[2], b[3])
}