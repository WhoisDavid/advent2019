use advent2019::intcode;
use advent2019::{get_input, AdventResult};
use std::collections::VecDeque;
use std::fmt;

fn main() -> AdventResult<()> {
    let program = &get_input::<isize>(23)?.first_row();
    solve_part1(program);
    solve_part2(program);
    Ok(())
}

fn solve_part1(input: &[isize]) {
    let mut network = Network::new(input);
    network.run(false)
}

fn solve_part2(input: &[isize]) {
    let mut network = Network::new(input);
    network.run(true)
}

#[derive(Clone, PartialEq, Eq)]
struct Packet {
    address: usize,
    x: isize,
    y: isize,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Packet(address: {}, x: {}, y: {})",
            self.address, self.x, self.y
        )
    }
}

struct Network {
    computers: Vec<intcode::IntCode>,
    packets: Vec<VecDeque<Packet>>,
    nat: Option<Packet>,
    last_sent_from_nat: Option<Packet>,
}

impl Network {
    fn new(nic: &[isize]) -> Self {
        let mut computers = Vec::with_capacity(50);
        for address in 0..50 {
            let mut intcode = intcode::IntCode::new(nic);
            intcode.set_input(&[address]);
            computers.push(intcode);
        }
        let packets = vec![VecDeque::new(); 50];
        Self {
            computers,
            packets,
            nat: None,
            last_sent_from_nat: None,
        }
    }

    fn run(&mut self, part2: bool) {
        let mut idle_cycles = 0;
        loop {
            for (current_address, intcode) in self.computers.iter_mut().enumerate() {
                match intcode.run_till_io(&[]) {
                    None => continue,
                    Some(intcode::IO::Input) => {
                        if !intcode.is_input_empty() {
                            continue;
                        }

                        if self.packets[current_address].is_empty() {
                            intcode.set_input(&[-1]);
                        }

                        while let Some(packet) = self.packets[current_address].pop_front() {
                            intcode.set_input(&[packet.x, packet.y]);
                        }
                    }
                    Some(intcode::IO::Output) => {
                        if intcode.output_len() == 3 {
                            let y = intcode.pop_output();
                            let x = intcode.pop_output();
                            let address = intcode.pop_output() as usize;
                            let packet = Packet { address, x, y };

                            if packet.address == 255 {
                                if !part2 {
                                    println!(
                                        "Packet sent to NAT by {}: {}",
                                        current_address, &packet
                                    );
                                    return;
                                }
                                self.nat = Some(packet);
                            } else {
                                self.packets[address].push_back(packet)
                            }
                        }
                    }
                }
            }

            idle_cycles += self.packets.iter().all(|p| p.is_empty()) as usize;

            if idle_cycles == 100 {
                idle_cycles = 0;
                // println!("Idle network!");
                if let Some(nat_packet) = self.nat.take() {
                    // println!("Don't worry, packet in NAT: {}", &nat_packet);
                    self.computers[0].set_input(&[nat_packet.x, nat_packet.y]);
                    if let Some(last_one) = self.last_sent_from_nat.take() {
                        if nat_packet.y == last_one.y {
                            println!("Packet Y sent by NAT twice in a row: {}", &nat_packet);
                            return;
                        }
                    }
                    self.last_sent_from_nat = Some(nat_packet);
                }
            }
        }
    }
}
