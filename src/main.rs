use std::io;
use std::io::Write;
use prettytable::{Table, table};

fn main() {
    const SOCIAL_TAX_RATE: f32 = 0.23;

    // To understand these values, refer to this website
    // https://www.economie.gouv.fr/particuliers/tranches-imposition-impot-revenu#:~:text=Tranche%20de%20revenu%20jusqu'%C3%A0,%25%20%3D%20960%2C90%20%E2%82%AC.
    const TAX_SCALE: [(u32, u32, f32); 5] = [
        (0, 11_294, 0.0), 
        (11_295, 28_797, 0.11), 
        (28_798, 82_341, 0.3), 
        (82_342, 177_106, 0.41), 
        (177_107, u32::MAX, 0.45)
    ];

    let gross_salary: i32 = read_salary();

    // Apply social taxes to salary
    let social_taxes: f32 = (gross_salary as f32) * SOCIAL_TAX_RATE;

    let net_salary: f32 = (gross_salary as f32) - social_taxes;

    let tax_bracket_index: usize = get_tax_bracket_index(net_salary, TAX_SCALE);

    let mut total_taxes: f32 = 0.0;

    for i in 0..(tax_bracket_index + 1) {
        total_taxes += calculate_tax_by_bracket(net_salary, TAX_SCALE[i]);
    }

    let take_home_salary: f32 = net_salary - total_taxes;

    let table: Table = table!(
        ["/", "Annuel", "Mensuel"],
        ["Salaire net", take_home_salary as u32, (take_home_salary as u32 / 12)],
        ["Impot", total_taxes as u32, (total_taxes as u32 / 12)],
        ["Charges sociales", social_taxes as u32, (social_taxes as u32 / 12)]
    );

    table.printstd();
    
}


fn read_salary() -> i32 {
    let mut salary: String = String::new();

    print!("Salaire annuel brut > ");

    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin().read_line(&mut salary).expect("Failed to read line");

    salary.trim().parse().unwrap()
}


fn get_tax_bracket_index(salary: f32, tax_scale: [(u32, u32, f32); 5]) -> usize {
    for i in 0..tax_scale.len() {
        if salary <= (tax_scale[i].1 as f32) {
            return i;
        }
    }

    tax_scale.len() - 1 as usize
}


fn calculate_tax_by_bracket(salary: f32, tax_bracket: (u32, u32, f32)) -> f32 {
    let min_bracket: u32 = tax_bracket.0;
    let max_bracket: u32 = tax_bracket.1;
    let tax_rate: f32    = tax_bracket.2;

    if salary >= max_bracket as f32 {
        return ((max_bracket - min_bracket) as f32) * tax_rate;
    }

    (salary - (min_bracket as f32 - 1.0)) * tax_rate
}