mod cli;

use cli::*;

fn main() 
{
    let mut grid = GridDisplay::empty();
    let headers = vec![String::from("col0"), String::from("col1"), String::from("col2"), String::from("col3")];

    let row0 = vec![String::from("asdlfjds"), String::from("dsfk"), String::from("dfdjgrgei")];
    let row1 = vec![String::from("asdlds"), String::from("dsdfk"), String::from("dfdi"), String::from("dfdfdfdi"), String::from("dfdi")];
    let row2 = vec![String::from("asdlfjddfs"), String::from("dsfsdk"), String::from("drgei"), String::from("dffdi")];

    grid.set_header(headers);

    grid.add_row(row0);
    grid.add_row(row1);
    grid.add_row(row2);

    grid.display();
}
