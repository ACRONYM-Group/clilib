#![allow(dead_code)]

use std::collections::HashMap;

/// Levels of severity for a CliError
#[derive(Debug, Clone, Copy)]
pub enum ErrorLevel
{
    /// Fatal error: the application cannot continue
    Error,
    /// Warning: Execution can continue, although unexpected behavior may follow
    Warning
}

/// Error for a CLI application
#[derive(Debug, Clone)]
pub struct CliError
{
    /// Error Message
    pub error: String,
    /// Error Code
    pub error_code: i32,
    /// Error Level
    pub error_level: ErrorLevel,
    /// Has the error been reported to the user yet?
    reported: bool
}

impl CliError
{
    /// Generate a new CliError object
    pub fn new(msg: &str, error_code: i32, error_level: ErrorLevel) -> CliError
    {
        CliError
        {
            error: String::from(msg),
            error_code: error_code,
            error_level: error_level,
            reported: false
        }
    }

    /// Generate a new Err(CliError) with level ErrorLevel::Warning
    pub fn warn<T>(msg: &str, error_code: i32) -> Result<T, CliError>
    {
        Err(CliError::new(msg, error_code, ErrorLevel::Warning))
    }

    /// Generate a new Err(CliError) with level ErrorLevel::Error
    pub fn error<T>(msg: &str, error_code: i32) -> Result<T, CliError>
    {
        Err(CliError::new(msg, error_code, ErrorLevel::Error))
    }

    /// Handles the error appropriately, either by simply alerting the user, or
    /// halting execution
    pub fn handle(&mut self) -> Result<(), CliError>
    {
        match self.error_level
        {
            ErrorLevel::Error => 
            {
                // If the error has not been reported, report it
                if !self.reported
                {
                    eprintln!("{} has encountered an error: '{}'", env!("CARGO_PKG_NAME"), self.error);
                    self.reported = true;
                }
                
                // Pass the error up the call stack
                Err(self.clone())
            },
            ErrorLevel::Warning =>
            {
                // If the warning has not been reported, report it
                if !self.reported
                {
                    eprintln!("{} has encountered a warning: '{}'", env!("CARGO_PKG_NAME"), self.error);
                }

                // Do not pass the error up the call stack
                Ok(())
            }
        }
    }

    /// Dismiss error of a given error code
    pub fn dismiss_by_code(&self, error_code: i32) -> Result<(), CliError>
    {
        if self.error_code == error_code
        {
            Ok(())
        }
        else
        {
            Err(self.clone())
        }
    }

    /// Dismiss error if the error code is in a vector of error codes
    pub fn dismiss_by_codes(&self, error_codes: Vec<i32>) -> Result<(), CliError>
    {
        if error_codes.contains(&self.error_code)
        {
            Ok(())
        }
        else
        {
            Err(self.clone())
        }
    }

    /// Dismisses the error (A clearer syntax than just ignoring and returning Ok(()))
    pub fn dismiss(&self) -> Result<(), CliError>
    {
        Ok(())
    }
}


/// Command line arguments
#[derive(Debug)]
pub struct Arguments
{
    /// Defined Arguments
    pub args: Vec<String>,
    /// Argument Values
    pub values: HashMap<String, Vec<String>>,
    /// List of raw arguments
    pub naked_values: Vec<String>
}


impl Arguments
{
    /// Generate a new Arguments object from the command line arguments
    pub fn new(raw_args: std::env::Args) -> Arguments
    {
        let mut args: Vec<String> = vec![];
        let mut naked_values: Vec<String> = vec![];

        let mut values: HashMap<String, Vec<String>> = HashMap::new();

        let mut last_arg = String::from("");

        let arg_str_array: Vec<String> = raw_args.collect();

        for arg in &arg_str_array[1..]
        {
            if arg.starts_with("--")
            {
                if naked_values.len() != 0
                {
                    values.insert(last_arg.clone(), naked_values);
                    naked_values = vec![];
                }

                args.push(arg.clone());
                last_arg = arg.clone();
            }
            else if arg.starts_with("-")
            {
                if naked_values.len() != 0
                {
                    values.insert(last_arg.clone(), naked_values);
                    naked_values = vec![];
                }

                if arg.len() == 2
                {
                    args.push(arg.clone());
                    last_arg = arg.clone();
                }
                else
                {
                    for c in arg.chars()
                    {
                        if c != '-'
                        {
                            let current_arg = String::from("-") + &c.to_string();
                            args.push(current_arg.clone());
                            last_arg = current_arg;
                        }
                    }
                }
            }
            else
            {
                naked_values.push(arg.clone());
            }
        }

        if naked_values.len() != 0
        {
            values.insert(last_arg.clone(), naked_values.clone());
        }

        Arguments
        {
            args,
            values,
            naked_values
        }
    }

    /// Extract a single value passed as the value of an option
    pub fn get_single(&self, key: &str) -> Option<String>
    {
        match self.values.get(&String::from(key))
        {
            Some(s) => 
            {
                if s.len() > 0
                {
                    Some(s[0].clone())
                }
                else
                {
                    None
                }
            },
            None => None
        }
    }

    /// Checks if an argument was given
    pub fn check_arg(&self, arg: &str) -> bool
    {
        self.args.contains(&String::from(arg))
    }

    /// Get Passed value
    pub fn get_passed<T: std::str::FromStr>(&self, arg: &str) -> Option<T>
    {
        if !self.check_arg(arg)
        {
            return None;
        }

        match self.get_single(arg)
        {
            Some(s) =>
            {
                match s.parse::<T>()
                {
                    Ok(v) => Some(v),
                    Err(_) => None
                }
            },
            None =>
            {
                None
            }
        }
    }

    /// Get a passed value while checking if the result is None, and if so it returns a CliError
    pub fn get_passed_checked<T: std::str::FromStr>(&self, arg: &str) -> Result<T, CliError>
    {
        if !self.check_arg(arg)
        {
            return CliError::error(&format!("No '{}' option passed", arg), 1);
        }

        match self.get_single(arg)
        {
            Some(s) =>
            {
                match s.parse::<T>()
                {
                    Ok(v) => Ok(v),
                    Err(_) => CliError::error(&format!("Cannot parse argument to '{}'", arg), 1)
                }
            },
            None =>
            {
                CliError::error(&format!("No argument passed to '{}'", arg), 1)
            }
        }
    }
}


/// Colors which can be used in the console
pub enum AnsiColor
{
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightPurple,
    BrightCyan,
    BrightWhite
}


/// Styles which can use used in the console
pub enum AnsiStyle
{
    Bold,
    Underline,
    Strikethrough
}

/// Clear all of the decorations from a string
pub fn clear_decoration(s: String) -> String
{
    let mut result = String::from("");

    let mut within = false;

    for c in s.chars()
    {
        if c == '\x1B'
        {
            within = true;
        }

        if within
        {
            if c == 'm'
            {
                within = false;
            }
        }
        else
        {
            result += &c.to_string();
        }
    }

    result
}

/// Decorate a string with a color
pub fn decorate_color(s: String, color: AnsiColor) -> String
{
    let start = String::from(match color
    {
        AnsiColor::Black => "\x1B[30m",
        AnsiColor::Red => "\x1B[31m",
        AnsiColor::Green => "\x1B[32m",
        AnsiColor::Yellow => "\x1B[33m",
        AnsiColor::Blue => "\x1B[34m",
        AnsiColor::Purple => "\x1B[35m",
        AnsiColor::Cyan => "\x1B[36m",
        AnsiColor::White => "\x1B[37m",
        AnsiColor::BrightBlack => "\x1B[90m",
        AnsiColor::BrightRed => "\x1B[91m",
        AnsiColor::BrightGreen => "\x1B[92m",
        AnsiColor::BrightYellow => "\x1B[93m",
        AnsiColor::BrightBlue => "\x1B[94m",
        AnsiColor::BrightPurple => "\x1B[95m",
        AnsiColor::BrightCyan => "\x1B[96m",
        AnsiColor::BrightWhite => "\x1B[97m"
    });

    start + &clear_decoration(s) + &String::from("\x1B[0m")
}

/// Decorate a string with a style
pub fn decorate_style(s: String, style: AnsiStyle) -> String
{
    let start = String::from(match style
    {
        AnsiStyle::Bold => "\x1B[1m",
        AnsiStyle::Strikethrough => "\x1B[9m",
        AnsiStyle::Underline => "\x1B[4m"
    });

    start + &clear_decoration(s) + &String::from("\x1B[0m")
}

/// Decorate with a color and style
pub fn decorate(s: String, color: AnsiColor, style: AnsiStyle) -> String
{
    let start = String::from(match style
        {
            AnsiStyle::Bold => "\x1B[1m",
            AnsiStyle::Strikethrough => "\x1B[9m",
            AnsiStyle::Underline => "\x1B[4m"
        });
    
    start + &decorate_color(s, color)
}

/// Decorate with a color and multiple styles
pub fn decorate_multiple(s: String, color: AnsiColor, styles: Vec<AnsiStyle>) -> String
{
    let mut result = decorate_color(s, color);

    for style in &styles
    {
        let start = String::from(match style
            {
                AnsiStyle::Bold => "\x1B[1m",
                AnsiStyle::Strikethrough => "\x1B[9m",
                AnsiStyle::Underline => "\x1B[4m"
            });

        result = start + &result;
    }

    result
}


/// An object to allow data to be displayed in the console as a grid
pub struct GridDisplay
{
    /// Optional Headers for each column
    headers: Option<Vec<String>>,
    /// Rows to be displayed
    rows: Vec<Vec<String>>
}


impl GridDisplay
{
    /// Generate an empty GridDisplay object
    pub fn empty() -> GridDisplay
    {
        GridDisplay
        {
            headers: None,
            rows: vec![]
        }
    }

    /// Generate a GridDisplay object with the given header
    pub fn new(headers: Vec<String>) -> GridDisplay
    {
        GridDisplay
        {
            headers: Some(headers),
            rows: vec![]
        }
    }

    /// Set the header for a GridDispaly object
    pub fn set_header(&mut self, headers: Vec<String>)
    {
        self.headers = Some(headers);
    }

    /// Add a row to the GridDisplay
    pub fn add_row(&mut self, row: Vec<String>)
    {
        self.rows.push(row);
    }

    /// Pad a string on the right with spaces to match a given length
    fn pad(s: String, width: usize) -> String
    {
        let mut result = s.clone();

        for _ in 0..(width - clear_decoration(s).len())
        {
            result += " ";
        }

        result
    }

    /// Render the GridDisplay to a string
    pub fn render(&self) -> String
    {
        let mut result = String::from("");

        let mut max_column_sizes: Vec<usize> = vec![];

        match &self.headers
        {
            Some(headers) =>
            {
                for header in headers
                {
                    max_column_sizes.push(clear_decoration(header.clone()).len());
                }
            },
            None => 
            {
                if self.rows.len() > 0
                {
                    for val in &self.rows[0]
                    {
                        max_column_sizes.push(clear_decoration(val.clone()).len());
                    }
                }
            }
        }

        let mut i: usize;

        for row in &self.rows
        {
            i = 0;

            for val in row
            {
                if i >= max_column_sizes.len()
                {
                    max_column_sizes.push(clear_decoration(val.clone()).len());
                }
                else
                {
                    if clear_decoration(val.clone()).len() > max_column_sizes[i]
                    {
                        max_column_sizes[i] = clear_decoration(val.clone()).len();
                    }
                }
                i += 1;
            }
        }

        match &self.headers
        {
            Some(headers) =>
            {
                i = 0;
                for header in headers
                {
                    result += &GridDisplay::pad(header.clone(), max_column_sizes[i] + 2);
                    i += 1;
                }

                result += "\n";
            },
            None => {}
        }

        for row in &self.rows
        {
            i = 0;
            for val in row
            {
                result += &GridDisplay::pad(val.clone(), max_column_sizes[i] + 2);
                i += 1;
            } 
            result += "\n";
        }

        result
    }

    /// Display the GridDisplay object
    pub fn display(&self)
    {
        print!("{}", self.render());
    }
}

/// Help Option Entry
#[derive(Debug, Clone)]
pub struct OptionEntry
{
    /// Short name
    short: String,
    /// Long name
    long: String,
    /// Extra Info
    extra: String,
    /// Description
    description: String
}

impl OptionEntry
{
    /// Generate a new OptionEntry object
    pub fn new(short: &str, long: &str, extra: &str, description: &str) -> Self
    {
        Self
        {
            short: String::from(short),
            long: String::from(long),
            extra: String::from(extra),
            description: String::from(description)
        }
    }
}

impl std::fmt::Display for OptionEntry
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let short_part = 
        if self.short != ""
        {
            if self.long != ""
            {
                format!("-{},", self.short)
            }
            else
            {
                format!("-{}", self.short)
            }
        }
        else
        {
            String::new()
        };

        let long_part = 
        if self.long != ""
        {
            format!("--{:10} {}", self.long, self.extra)
        }
        else
        {
            format!("            {}", self.extra)
        };

        write!(f, "  {:4}{:27} {}", short_part, long_part, self.description)
    }
}

/// Help Display
#[derive(Debug, Clone)]
pub struct HelpDisplay
{
    /// Usage
    usage: String,
    /// Description
    description: String,
    /// Entries
    entries: Vec<OptionEntry>
}

impl HelpDisplay
{
    /// Generate a new HelpDisplay object from the usage string and the description
    /// string
    pub fn new(usage: &str, description: &str) -> Self
    {
        Self
        {
            usage: String::from(usage),
            description: String::from(description),
            entries: vec![]
        }
    }

    /// Add another command line option
    pub fn add_option(&mut self, entry: OptionEntry)
    {
        self.entries.push(entry);
    }
}

impl std::fmt::Display for HelpDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Usage: {}\n{}\n\n", self.usage, self.description)?;

        for entry in &self.entries
        {
            write!(f, "{}\n", entry)?;
        }

        write!(f, "\n")
    }
}