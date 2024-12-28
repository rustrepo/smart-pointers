struct File {
    name: String,
}

struct Directory {
    name: String,
    contents: Vec<Box<Directory>>, // Subdirectories
    files: Vec<File>,             // Files in the directory
}

impl Directory {
    fn new(name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            contents: Vec::new(),
            files: Vec::new(),
        }
    }

    fn add_subdirectory(&mut self, subdirectory: Directory) {
        self.contents.push(Box::new(subdirectory));
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    // Function to print files in the current directory
    fn print_files(&self) {
        if self.files.is_empty() {
            println!("No files in directory: {}", self.name);
        } else {
            println!("Files in directory {}:", self.name);
            for file in &self.files {
                println!("- {}", file.name);
            }
        }
    }

    // Function to print all files in subdirectories recursively
    fn print_subdirectory_files(&self) {
        self.print_files(); // Print files in the current directory
        for sub_dir in &self.contents {
            sub_dir.print_subdirectory_files(); // Recursively print files in subdirectories
        }
    }
}

fn main() {
    let mut root = Directory::new("root");

    let mut sub_dir1 = Directory::new("sub_dir1");
    sub_dir1.add_file(File { name: "file1.txt".to_string() });
    sub_dir1.add_file(File { name: "file2.txt".to_string() });

    let mut sub_dir2 = Directory::new("sub_dir2");
    sub_dir2.add_file(File { name: "file3.txt".to_string() });
    
    let mut sub_sub_dir = Directory::new("sub_sub_dir");
    sub_sub_dir.add_file(File { name: "file4.txt".to_string() });

    // Add subdirectories to the root directory
    root.add_subdirectory(sub_dir1);
    root.add_subdirectory(sub_dir2);
    root.add_subdirectory(sub_sub_dir);

    // Print all files inside the root directory and its subdirectories
    root.print_subdirectory_files();
}
