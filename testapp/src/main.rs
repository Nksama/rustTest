use std::fs;
use std::fs::File;

fn main() {
    let filename = String::from("k.gay");
    let gay = GayType{
        filename,
    };

    gay.create_gay();
    gay.write_gay("Elon Is Gay".to_owned());
}

struct GayType {
    filename: String,
}

impl GayType {
    fn create_gay(&self) {
        if self.filename.contains(".gay") {
            File::create(self.filename.to_owned()).expect("Could Not Create The File");
        } else {
            println!("FileType Must Be Gay")
        };
    }

    fn write_gay(&self , content : String) {
        if self.filename.contains(".gay") {
            let y = fs::write(self.filename.to_owned(), content);
        }
    }
}
