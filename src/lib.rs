/// This is an API for modifying the general data files.
///
/// **Author: @_Yunhao Xu_**
///
/// **Version: v1.0.0**

pub mod fileapi {
    use std::fmt::Debug;
    use std::fs::{File, remove_file};
    use std::io::{Read, Write};
    use std::path::Path;
    use std::str::FromStr;

    /// A structure of file modified API. This class is used to change, read, write, remove a file in the project.
    ///
    /// **You can custom the split character by using [split] function.**
    ///
    /// To initialize the [FileAPI], you can use the [from] function.
    ///
    /// # Example
    ///
    /// create a new FileAPI instance:
    /// ```no_run
    /// use self::simple_file_manager::fileapi::FileAPI;
    ///
    /// let file = FileAPI::from("filename.gph");
    /// ```
    ///
    /// collect a [Reader] type (same with [Builder], [Changer]):
    /// ```no_run
    /// use self::simple_file_manager::fileapi::FileAPI;
    ///
    /// // custom the split character.
    /// let file = FileAPI::from("filename.gph").split(',');
    /// let reader = file.reader();
    ///
    /// // read the header of the file
    /// let header = reader.read_header::<usize>(1)[0].clone();
    ///
    /// assert_eq!(header, vec![1, 2, 3]);
    ///
    /// ```
    /// then you will receive a [Vec] recording the value in the first line, which are also parsed to [usize] type.
    ///
    /// [split]: FileAPI::split
    /// [from]: FileAPI::from
    pub struct FileAPI {
        pub path: String,
        split: char
    }

    impl FileAPI {
        /// Initialize the [FileAPI], you can use the [from] function.
        /// # Example
        ///
        /// create a new FileAPI instance:
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph");
        /// ```
        pub fn from(path: &str) -> FileAPI {
            FileAPI {
                path: path.to_string(),
                split: ' '
            }
        }

        /// Set the split character for all process (except [read_csv]). The default character is ' ' (whitespace).
        /// # Example
        ///
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph").split(',');
        /// // the values will divided by ','.
        /// let reader = file.reader();
        /// let body = reader.read_body::<usize>(1, 1);
        ///
        /// assert_eq!(body, vec![vec![4, 5, 6], vec![7, 8, 9]]);
        /// ```
        ///
        /// [read_csv]: Reader::read_csv
        pub fn split(mut self, split: char) -> Self {
            self.split = split.clone();
            self
        }

        /// Get a Reader object for reading several values of the same file in succession.
        ///
        /// # Example
        /// collect a [Reader] type (same with [Builder], [Changer]):
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph");
        /// let reader = file.reader();
        ///
        /// // read the header of the file
        /// let header = reader.read_header::<usize>(1)[0].clone();
        ///
        /// assert_eq!(header, vec![1, 2, 3]);
        /// ```
        /// then you will receive a [Vec] recording the value in the first line, which are also parsed to [usize] type.
        pub fn reader(&self) -> Reader {
            Reader::from(self)
        }

        /// Get a Changer object for modifying several values of the same file in succession.
        ///
        /// # Example
        /// collect a [Changer] type (same with [Builder], [Reader]):
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph").split(',');
        /// let changer = file.changer();
        ///
        /// // change the value in the 2th line and 2th row to value '123':
        /// // you can consecutive change values:
        /// changer.change_value(2, 2, "123")
        ///     .change_value(3, 2, "567")
        ///     .change_value(4, 2, "560")
        ///     .execute();  // after modifying the value, you will need to execute your changes.
        /// ```
        pub fn changer(&self) -> Changer {
            Changer::from(self)
        }

        /// Get a Builder object for writing several values for a new file in succession.
        ///
        /// # Example
        /// collect a [Builder] type (same with [Changer], [Reader]):
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph");
        /// let mut builder = file.builder();
        ///
        /// // write a line:
        /// // you can consecutive write lines:
        /// builder.write_line("This is the second line.")
        ///     .write_line("This is the third line.")
        ///     .write_line("This is the forth line.")
        ///     .execute(); // you will also need to execute your changes:
        ///
        /// ```
        pub fn builder(&self) -> Builder {
            Builder::from(self)
        }

        /// A function to remove the file and delete the object.
        ///
        /// # Example
        ///
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// FileAPI::from("filename.gph").remove();
        /// ```
        pub fn remove(&self) {
            remove_file(self.path.clone()).unwrap();
        }

        /// A function to check if the file exist.
        ///
        /// # Example
        ///
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph");
        /// if file.is_exist() {
        ///     file.remove();
        /// };
        /// ```
        pub fn is_exist(&self) -> bool {
            Path::new(&self.path).exists()
        }
    }

    impl Clone for FileAPI {
        fn clone(&self) -> Self {
            FileAPI {
                path: self.path.clone(),
                split: self.split.clone()
            }
        }
    }

    /// A reader structure for reading specific values of the same file in succession.
    ///
    /// # Example
    /// collect a [Reader] type (same with [Builder], [Changer]):
    /// ```no_run
    /// use self::simple_file_manager::fileapi::FileAPI;
    ///
    /// let file = FileAPI::from("filename.gph");
    /// let reader = file.reader();
    ///
    /// // read the header of the file
    /// let header = reader.read_header::<usize>(1)[0].clone();
    ///
    /// assert_eq!(header, vec![1, 2, 3]);
    /// ```
    /// then you will receive a [Vec] recording the value in the first line, which are also parsed to [usize] type.
    pub struct Reader<'a> {
        pub lines: String,
        file: &'a FileAPI,
        pub values: Vec<String>
    }

    impl Reader<'_> {
        fn from(file: &FileAPI) -> Reader<'_> {
            let mut the_file = File::open(&file.path).unwrap();
            let mut lines = String::new();
            let _ = the_file.read_to_string(&mut lines).unwrap();
            Reader { lines, file , values: Vec::new()}
        }

        /// Read all text in the file.
        ///
        /// # Example
        /// collect a [Reader] type (same with [Builder], [Changer]):
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph");
        /// let reader = file.reader();
        ///
        /// // read all the lines in the file
        /// let context = reader.read_to_string();
        ///
        /// assert_eq!(context, String::from("1,2,3\n4,5,6\n7,8,9\n10,12"))
        /// ```
        pub fn read_to_string(&self) -> String {
            self.lines.to_string()
        }

        /// A function to read a value in this data storage file.
        ///
        /// # Example
        /// collect a [Reader] type (same with [Builder], [Changer]):
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph").split(',');
        /// let reader = file.reader();
        ///
        /// // read the value in the 5th line and 7th row :
        /// // you can consecutive read values:
        /// let results = reader.read_value(1, 2)
        ///     .read_value(3, 2)
        ///     .read_value(2, 1)
        ///     .execute::<usize>(); // after select the values, you will need to execute and receive the values stored in a [Vec].
        ///
        /// assert_eq!(results, vec![2, 8, 4]);
        /// ```
        pub fn read_value(mut self, line:usize, row:usize) -> Self {
            let a_line = self.lines
                .lines()
                .collect::<Vec<&str>>()[line-1]
                .split(self.file.split.clone())
                .collect::<Vec<&str>>();
            self.values.push(a_line[row - 1].to_string());
            self
        }

        /// Confirm and receive the selected values.
        ///
        /// # Example
        /// collect a [Reader] type (same with [Builder], [Changer]):
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph").split(',');
        /// let reader = file.reader();
        ///
        /// // read the value in the 5th line and 7th row :
        /// // you can consecutive read values:
        /// let results = reader.read_value(1, 2)
        ///     .read_value(3, 2)
        ///     .read_value(2, 1)
        ///     .execute::<usize>(); // after select the values, you will need to execute and receive the values stored in a [Vec].
        ///
        /// assert_eq!(results, vec![2, 8, 4]);
        /// ```
        pub fn execute<T: FromStr>(&self) -> Vec<T>
            where
                <T as FromStr>::Err: Debug,
        {
            self.values.iter().map(|v| v.trim().parse::<T>().unwrap()).collect::<Vec<T>>()
        }

        /// Read the specific lines of header and parse them into a certain type.
        ///
        /// # Example
        /// collect a [Reader] type (same with [Builder], [Changer]):
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph");
        /// let reader = file.reader();
        ///
        /// // read the header of the file
        /// let header = reader.read_header::<usize>(1)[0].clone();
        ///
        /// assert_eq!(header, vec![1, 2, 3]);
        /// ```
        /// then you will receive a [Vec] recording the value in the first line, which are also parsed to [usize] type.
        pub fn read_header<T: FromStr>(&self, len: usize) -> Vec<Vec<T>>
            where
                <T as FromStr>::Err: Debug,
        {
            if len < 1 {
                panic!("The 'len' parameter should not less than 1.")
            }
            let mut reader = self.lines.lines();
            let mut header: Vec<Vec<T>> = Vec::new();
            for _ in 0..len {
                let line: Vec<T> = Self::read_line_parse(reader.next().unwrap(), self.file.split);
                header.push(line);
            }
            header
        }

        /// Read the last line and parse them into a certain type.
        ///
        /// # Example
        /// collect a [Reader] type (same with [Builder], [Changer]):
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph");
        /// let reader = file.reader();
        ///
        /// // read the footer of the file
        /// let footer = reader.read_footer::<usize>();
        ///
        /// assert_eq!(footer, vec![10, 12]);
        /// ```
        /// then you will receive a [Vec] recording the value in the last line, which are also parsed to [usize] type.
        pub fn read_footer<T: FromStr>(&self) -> Vec<T>
            where
                <T as FromStr>::Err: Debug,
        {
            Self::read_line_parse(self.lines.lines().last().unwrap(), self.file.split)
        }

        /// Read the main context and parse them into a certain type.
        ///
        /// # Example
        /// collect a [Reader] type (same with [Builder], [Changer]):
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph");
        /// let reader = file.reader();
        ///
        /// // read the body
        /// // skip the first two line and the last line.
        /// let body = reader.read_body::<usize>(1, 1);
        ///
        /// assert_eq!(body, vec![vec![4, 5, 6], vec![7, 8, 9]]);
        /// ```
        /// Then you will receive a [Vec<Vec<usize>>] recording the value in the body, which are also parsed to [usize] type.
        pub fn read_body<T: FromStr>(&self, header: usize, footer: usize) -> Vec<Vec<T>>
            where
                <T as FromStr>::Err: Debug,
        {
            let mut reader = self.lines.lines();
            let len = reader.clone().count();
            let mut context: Vec<Vec<T>> = Vec::new();
            for i in 0..len - footer {
                if i < header {
                    reader.next();
                    continue;
                }
                let line: Vec<T> = Self::read_line_parse(reader.next().unwrap(), self.file.split);
                context.push(line);
            }
            context
        }

        // read a line and parse them into certain type.
        fn read_line_parse<T: FromStr>(line: &str, split: char) -> Vec<T>
            where
                <T as FromStr>::Err: Debug,
        {
            line.split(split)
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| s.trim().parse::<T>().unwrap())
                .collect::<Vec<T>>()
        }

        /// Count the lines.
        ///
        /// # Example
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let len = FileAPI::from("filename.gph").reader().count_lines();
        ///
        /// assert_eq!(len, 4);
        /// ```
        pub fn count_lines(&self) -> usize {
            self.lines.lines().count()
        }

        /// A function to read the specific row in this csv file.
        ///
        /// # Example
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// // read the specific row of the csv file.
        /// let row = FileAPI::from("filename.gph").reader().read_csv::<usize>(5);
        ///
        /// assert_eq!(row, vec![4, 7, 10]);
        /// ```
        /// Then you will receive a [Vec] recording the data in this row, which are also parsed to [usize] type.
        pub fn read_csv<T: FromStr>(&self, row: usize) -> Vec<T>
            where
                <T as FromStr>::Err: Debug,
        {
            let mut reader = self.lines.lines();
            reader.next();
            let data: Vec<T> = reader.map(|l| {
                l.split(',')
                    .collect::<Vec<&str>>()[row-1]
                    .parse::<T>()
                    .unwrap()
            }).collect();
            data
        }
    }

    /// A changer structure for change some specific values in the file in succession.
    ///
    /// # Example
    /// collect a [Changer] type (same with [Builder], [Reader]):
    /// ```no_run
    /// use self::simple_file_manager::fileapi::FileAPI;
    ///
    /// let file = FileAPI::from("filename.gph");
    /// let changer = file.changer();
    ///
    /// // change the value in the 1th line and 2th row to value '123':
    /// // you can consecutive change values:
    /// changer.change_value(1, 2, "234")
    ///     .change_value(2, 2, "567")
    ///     .change_value(3, 2, "560")
    ///     .execute(); // after modifying the value, you will need to execute your changes.
    /// ```
    pub struct Changer<'a> {
        lines: Vec<String>,
        file: &'a FileAPI
    }

    impl Changer<'_> {
        fn from(file: &FileAPI) -> Changer<'_> {
            let mut the_file = File::open(&file.path).unwrap();
            let mut lines = String::new();
            let _ = the_file.read_to_string(&mut lines).unwrap();
            let lines = lines.lines().collect::<Vec<&str>>().iter().map(|l| l.to_string()).collect();
            Changer { lines, file }
        }

        /// A function to change a value in this data storage file.
        ///
        /// # Example
        /// collect a [Changer] type (same with [Builder], [Reader]):
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph");
        /// let changer = file.changer();
        ///
        /// // change the value in the 1th line and 2th row to value '123':
        /// // you can consecutive change values:
        /// changer.change_value(1, 2, "234")
        ///     .change_value(2, 2, "567")
        ///     .change_value(3, 2, "560")
        ///     .execute(); // after modifying the value, you will need to execute your changes.
        /// ```
        pub fn change_value(mut self, line: usize, row: usize, value: &str) -> Self {
            let a_line = self.lines[line-1].clone();
            let mut a_line = a_line.split(self.file.split.clone())
                .collect::<Vec<&str>>();
            a_line[row-1] = value;
            self.lines[line-1] =  a_line.join(&*self.file.split.to_string());
            self
        }

        /// Confirm and implement the changes.
        ///
        /// # Example
        /// collect a [Changer] type (same with [Builder], [Reader]):
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph");
        /// let changer = file.changer();
        ///
        /// // change the value in the 1th line and 2th row to value '123':
        /// // you can consecutive change values:
        /// changer.change_value(1, 2, "234")
        ///     .change_value(2, 2, "567")
        ///     .change_value(3, 2, "560")
        ///     .execute(); // after modifying the value, you will need to execute your changes.
        /// ```
        pub fn execute(&self) -> &FileAPI {
            let mut file = File::create(&self.file.path).unwrap();
            for line in &self.lines {
                writeln!(file, "{}", line).unwrap();
            }
            self.file
        }
    }

    /// A changer class for for writing several values for a new file in succession.
    ///
    /// # Example
    /// collect a [Builder] type (same with [Changer], [Reader]):
    /// ```no_run
    /// use self::simple_file_manager::fileapi::FileAPI;
    ///
    /// let file = FileAPI::from("filename.gph");
    /// let mut builder = file.builder();
    ///
    /// // write a line:
    /// // you can consecutive write lines:
    /// builder.write_line("This is the second line.")
    ///     .write_line("This is the third line.")
    ///     .write_line("This is the forth line.")
    ///     .execute(); // you will also need to execute your changes:
    ///
    /// ```
    pub struct Builder<'a> {
        lines: Vec<String>,
        file: &'a FileAPI
    }
    impl Builder<'_> {
        fn from(file: &FileAPI) -> Builder<'_> {
            Builder {
                lines: Vec::new(),
                file
            }
        }

        /// A function to write a new line in the new file.
        ///
        /// # Example
        /// collect a [Builder] type (same with [Changer], [Reader]):
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph");
        /// let mut builder = file.builder();
        ///
        /// // write a line:
        /// // you can consecutive write lines:
        /// builder.write_line("This is the second line.")
        ///     .write_line("This is the third line.")
        ///     .write_line("This is the forth line.")
        ///     .execute(); // you will also need to execute your changes:
        ///
        /// ```
        pub fn write_line(mut self, line: &str) -> Self {
            self.lines.push(line.to_string());
            self
        }

        /// Confirm and implement.
        ///
        /// # Example
        /// collect a [Builder] type (same with [Changer], [Reader]):
        /// ```no_run
        /// use self::simple_file_manager::fileapi::FileAPI;
        ///
        /// let file = FileAPI::from("filename.gph");
        /// let mut builder = file.builder();
        ///
        /// // write a line:
        /// // you can consecutive write lines:
        /// builder.write_line("This is the second line.")
        ///     .write_line("This is the third line.")
        ///     .write_line("This is the forth line.")
        ///     .execute(); // you will also need to execute your changes:
        ///
        /// ```
        pub fn execute(&self) -> &FileAPI {
            let mut file = File::create(&self.file.path).unwrap();
            for line in &self.lines {
                writeln!(file, "{}", line).unwrap();
            }
            self.file
        }
    }
}
