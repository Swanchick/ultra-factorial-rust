use std::io::{Error, ErrorKind, Result, Write};
use std::fs::OpenOptions;

#[derive(Clone)]
pub struct DNumber {
    numbers: Vec<u8>
}

impl DNumber {
    pub fn new() -> DNumber {
        let numbers: Vec<u8> = Vec::new();

        DNumber{
            numbers: numbers
        }
    }

    pub fn from(number: i32) -> Result<DNumber> {
        if number < 0 {
            return Err(
                Error::new(
                    ErrorKind::InvalidInput, 
                    "Number is less than 0!"
                )
            );
        }

        if number == 0 {
            return Ok(DNumber::new());
        }

        let string_number = number.to_string();
        let mut numbers: Vec<u8> = Vec::new();

        for c in string_number.chars() {
            let n = c.to_digit(10).unwrap() as u8;
            
            numbers.push(n);
        }

        Ok(DNumber {
            numbers: numbers
        })
    }
    
    pub fn show(&self) {
        if self.numbers.len() == 0 {
            println!("0");
            
            return;
        }

        for n in self.numbers.iter() {
            print!("{}", n);
        }

        println!("");
    }

    pub fn save(&self) {
        let file = OpenOptions::new()
            .append(true)
            .open("data.txt");

        if let Ok(mut file) = file {
            for number in self.numbers.iter() {
                _ = file.write(number.to_string().as_bytes());
            }
        } else {
            println!("Something went wrong");
        }
    }

    pub fn add(&mut self, dnumber: &DNumber) {
        let my_length = self.get_length() as i32;
        let dnumber_length = dnumber.get_length() as i32;
        
        if dnumber_length > my_length {
            let mut fake_dnumber = dnumber.clone();
            fake_dnumber.add(self);
            self.numbers = fake_dnumber.get_numbers().to_vec();
            
            return;
        }

        let numbers = dnumber.get_numbers();

        let mut additional_number = 0;

        for i in 0..self.get_length() as i32 {
            let last_index1 = my_length - i - 1;
            let last_index2 = dnumber_length - i - 1;
            let mut number1 = self.numbers[last_index1 as usize];
            
            let mut number2 = additional_number.clone();

            if last_index2 >= 0 {
                number2 += numbers[last_index2 as usize];
            }

            number1 += number2;
            if number1 >= 10 {
                number1 -= 10;
                additional_number = 1;

            } else {
                additional_number = 0;
            }

            self.numbers[last_index1 as usize] = number1;
        }

        if additional_number == 1 {
            self.numbers.insert(0, additional_number);
        }
    }

    pub fn multiply(&mut self, dnumber: &DNumber) {
        let my_length = self.get_length();
        let dnumber_length = dnumber.get_length();
        let dnumbers = dnumber.get_numbers();

        let mut additional_number: u8 = 0;
        let mut matrix: Vec<DNumber> = Vec::new();

        for j in 0..dnumber_length {
            let last_index2 = dnumber_length - j - 1;
            let number2 = dnumbers[last_index2];
            let mut numbers = DNumber::new();
            let mut last = 0;

            for i in 0..my_length {
                let last_index1 = my_length - i - 1;
                let number1 = self.numbers[last_index1];
                
                let mut multiplication = number1 * number2 + additional_number;

                if multiplication >= 10 {
                    additional_number = (multiplication as f64 / 10.0) as u8;
                    multiplication -= additional_number * 10;
                } else {
                    additional_number = 0;
                }
                
                let mut result = DNumber::new();
                
                for _ in 0..(i+j) {
                    result.insert(0);
                }
                
                result.insert(multiplication);
                numbers.add(&result);

                last = i;
            }

            if additional_number != 0 {
                let mut result = DNumber::new();
                for _ in 0..(last+j+1) {
                    result.insert(0);
                }

                result.insert(additional_number);

                numbers.add(&result);
            }
            matrix.insert(0, numbers);

            additional_number = 0;
        }

        let mut final_result = DNumber::new();

        for number in matrix.iter() {
            final_result.add(number);
        }

        self.numbers = final_result.get_numbers().to_vec();
    }

    pub fn insert(&mut self, number: u8) {
        self.numbers.insert(0, number);
    }

    pub fn factorial(&mut self, number: i32) {
        let mut dnumber = DNumber::from(1).unwrap();
        
        for i in 0..number {
            let factorial: i32 = i + 1;
            println!("{}", factorial);

            let n = DNumber::from(factorial as i32).unwrap();
            dnumber.multiply(&n);
        }

        self.numbers = dnumber.get_numbers().to_vec();
    }

    pub fn get_length(&self) -> usize {
        self.numbers.len()
    }

    pub fn get_numbers(&self) -> &Vec<u8> {
        &self.numbers
    }
}