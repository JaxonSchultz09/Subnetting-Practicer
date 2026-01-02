use std::{vec};
use rand::Rng;

slint::include_modules!();

fn main()
{
    let gui = MainWindow::new().unwrap();
    let gui_weak = gui.as_weak();

    gui.on_button_id(move |button_id| 
    {
        let input_vec: Vec<String> = if let Some(gui_strong) = gui_weak.upgrade() 
        {
            vec!
            [
                String::from(gui_strong.get_ip_string()),
                String::from(gui_strong.get_subnet_mask_string()),
                String::from(gui_strong.get_wildcard_string()),
                String::from(gui_strong.get_subnet_address_string()),
                String::from(gui_strong.get_broadcast_address_string()),
                String::from(gui_strong.get_first_host_string()),
                String::from(gui_strong.get_last_host_string()),
            ]
        } else 
        {
            vec![0.to_string(); 7]
        };
        
        
        let correct_vec: Vec<u32> = calculate_correct_values(input_vec.clone());
        let input_vec: Vec<u32> = vec!
        [
            convert_string_to_u32(input_vec[0].clone()),
            convert_string_to_u32(input_vec[1].clone()),
            convert_string_to_u32(input_vec[2].clone()),
            convert_string_to_u32(input_vec[3].clone()),
            convert_string_to_u32(input_vec[4].clone()),
            convert_string_to_u32(input_vec[5].clone()),
            convert_string_to_u32(input_vec[6].clone()),
        ];
        check_values_equal(correct_vec, input_vec, button_id, gui_weak.clone());
    });
    
    gui.run().unwrap();
}

fn calculate_correct_values (input_vec: Vec<String>) -> Vec<u32>
{
    let user_input: Vec<u8> = if !input_vec[0].is_empty() 
    {
        input_vec[0]
            .trim()
            .split(&['/', '.'][..])
            .filter_map(|s| s.parse::<u8>().ok())
            .collect()
    } else 
    {
        vec![0, 0, 0, 0, 0]
    };
  
    let ip: u32 = match user_input.get(0..4) 
    {
        Some(slice) => 
        {
            u32::from_be_bytes(slice.try_into().unwrap())
        }
        None => 0u32
    };

    let cidr = match user_input.len()
    { 
        5 => 
        { 
            if !(user_input[4] == 0) 
            { 
                (!0u32) << (32 - user_input[4]) 
            } else 
            { 
                0 
            } 
        },
        4 => 
        {
            match ip >> 24
            {
                0 => 0u32,
                1..=127 => (!0u32) << 24,
                128..=191 => (!0u32) << 16,
                192..=223 => (!0u32) << 8,
                _ => !0u32
            }
        },
        _ => 0
    };

    return vec!
    [
        ip,
        cidr,
        !cidr,
        ip & cidr,
        ip | !cidr,

        //First Usable host
        match cidr.count_zeros()
        {
            0..=1 => ip & cidr,
            _ => (ip & cidr) + 1
        },

        //Last Usable Host
        match cidr.count_zeros()
        {
            0..=1 => (ip & cidr) | (!cidr),
            _ => ((ip & cidr) | (!cidr)) - 1
        },

        //Class
        match ip >> 28
        {
            0b0000..=0b0111 => 1,
            0b1000..=0b1011 => 2,
            0b1100..=0b1101 => 3,
            0b1110          => 4,
            _               => 5
        }
    ];
}

fn check_values_equal(correct_vec: Vec<u32>, input_vec: Vec<u32>, button_id: i32, gui_weak: slint::Weak<MainWindow>)
{
    let gui = gui_weak.upgrade().unwrap();

    if button_id == 0 || !(correct_vec[0] == 0)
    {
        match button_id
        {
            0  => gui.set_ip_string(random_ip().into()),                                                                                       //Randomize
            1  =>                                                                                                                              //Show All
            {
                gui.set_subnet_mask_string(u32_to_ip(correct_vec[1]).into());
                gui.set_wildcard_string(u32_to_ip(correct_vec[2]).into());
                gui.set_subnet_address_string(u32_to_ip(correct_vec[3]).into());
                gui.set_broadcast_address_string(u32_to_ip(correct_vec[4]).into());
                gui.set_first_host_string(u32_to_ip(correct_vec[5]).into());
                gui.set_last_host_string(u32_to_ip(correct_vec[6]).into());
                gui.set_button_text(
                    match correct_vec[7]
                    {
                        1 => "A".into(),
                        2 => "B".into(),
                        3 => "C".into(),
                        4 => "D".into(),
                        5 => "E".into(),
                        _ => "ERROR".into(),
                    }
                );
            },                                                                             
            2  => gui.set_subnet_mask_string(u32_to_ip(compare_values(correct_vec[1], input_vec[1])).into()),       //Check  Subnet Mask
            3  => gui.set_subnet_mask_string(u32_to_ip(correct_vec[1]).into()),                                                                //Show Subnet Mask
            4  => gui.set_wildcard_string(u32_to_ip(compare_values(correct_vec[2], input_vec[2])).into()),          //Check Wildcard Mask
            5  => gui.set_wildcard_string(u32_to_ip(correct_vec[2]).into()),                                                                   //Show  Wildcard Mask
            6  => gui.set_subnet_address_string(u32_to_ip(compare_values(correct_vec[3], input_vec[3])).into()),    //Check Subnet Address
            7  => gui.set_subnet_address_string(u32_to_ip(correct_vec[3]).into()),                                                             //Show  Subnet Address
            8  => gui.set_broadcast_address_string(u32_to_ip(compare_values(correct_vec[4], input_vec[4])).into()), //Check Broadcast Address
            9  => gui.set_broadcast_address_string(u32_to_ip(correct_vec[4]).into()),                                                          //Show  Broadcast Address
            10 => gui.set_first_host_string(u32_to_ip(compare_values(correct_vec[5], input_vec[5])).into()),        //Check First Host
            11 => gui.set_first_host_string(u32_to_ip(correct_vec[5]).into()),                                                                 //Show  First Host
            12 => gui.set_last_host_string(u32_to_ip(compare_values(correct_vec[6], input_vec[6])).into()),         //Check Last Host
            13 => gui.set_last_host_string(u32_to_ip(correct_vec[6]).into()),                                                                  //Show  Last Host
            14 => gui.set_button_text(compare_class(correct_vec[7], 1).into()),                                     //Class A
            15 => gui.set_button_text(compare_class(correct_vec[7], 2).into()),                                     //Class B
            16 => gui.set_button_text(compare_class(correct_vec[7], 3).into()),                                     //Class C
            17 => gui.set_button_text(compare_class(correct_vec[7], 4).into()),                                     //Class D
            18 => gui.set_button_text(compare_class(correct_vec[7], 5).into()),                                     //Class E
            _  => gui.set_ip_string("ERROR".into()),                                                                                           //Incase Error
        }
    } else 
    {
        gui.set_ip_string("Type an IPv4 address".into())
    }
}

fn compare_values (correct_value: u32, input_value: u32) -> u32
{
    return if input_value == correct_value 
    {
        println!("correct: {}\ninput: {}", correct_value, input_value);
        correct_value
    } else 
    {
        println!("correct: {}\ninput: {}", correct_value, input_value);
        input_value
    }
}

fn convert_string_to_u32 (input: String) -> u32
{
    let input_vec: Vec<u8> = if !input.is_empty() 
    {
        input
            .trim()
            .split(&['/', '.'][..])
            .filter_map(|s| s.parse::<u8>().ok())
            .collect()
    } else 
    {
        vec![0, 0, 0, 0, 0]
    };
  
    match input_vec.get(0..4) 
    {
        Some(slice) => 
        {
            u32::from_be_bytes(slice.try_into().unwrap())
        }
        None => 0u32
    }
}

fn u32_to_ip (u: u32) -> String
{
    let b = u.to_be_bytes();
    format!("{}.{}.{}.{}", b[0], b[1], b[2], b[3])
}

fn random_ip () -> String
{
    let ip: [u8; _] = rand::random::<u32>().to_be_bytes();
    let cidr: u8 = rand::thread_rng().gen_range(0..=32);
    
    if cidr == 0 
    {
        format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])
    } else 
    {
        format!("{}.{}.{}.{}/{}", ip[0], ip[1], ip[2], ip[3], cidr)
    }
}

fn compare_class (correct_value: u32, input_value: u32) -> String
{
    if correct_value == input_value
    {
        "Correct".to_string()
    } else 
    {
        "Incorrect".to_string()
    }
}